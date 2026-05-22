# -*- coding: utf-8 -*-
"""Núcleo orquestador route-domain-event — topología V3+ simétrica."""

from __future__ import annotations

import json
import os
import re
import shutil
import subprocess
import sys
import traceback
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path
from typing import Any

from eda_bus_utils import (
    ECST_GATE_SUBSCRIBER,
    _delegation_meta,
    _iso_now,
    _write_json_atomic,
    dlt_threshold_ok,
    ensure_event_bus_topology,
    ensure_processing_header,
    github_pr_merged,
    inject_domain_entity_topology_defaults,
    infer_persist_ref_from_branch,
    is_backfill_emitter,
    list_witnesses,
    maybe_purge_processing_header,
    promote_witness,
    resolve_origin_topology,
    subscriber_applies_to_topology,
    subscriber_id,
    terminal_witness_exists,
    write_processing_witness,
)

_SUBPROCESS_UTF8 = {"text": True, "encoding": "utf-8", "errors": "replace"}


def _repo_root() -> Path:
    here = Path(__file__).resolve()
    for parent in here.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError("No se encontró raíz del workspace (SddIA/core/cumulo.paths.json)")


def _rel_event_path(repo: Path, event_path: Path) -> str:
    try:
        return event_path.resolve().relative_to(repo.resolve()).as_posix()
    except ValueError:
        return str(event_path.resolve())


def _sync_dispatch_mode() -> bool:
    return os.environ.get("SDDIA_LAB_ROUTE_SYNC", "").strip().lower() in (
        "1",
        "true",
        "yes",
    )


def _dispatch_mode_label() -> str:
    return "sync" if _sync_dispatch_mode() else "async"


def _iota_timeout_seconds() -> int:
    return int(os.environ.get("SDDIA_IOTA_TIMEOUT_SECONDS", "45"))


def _run_subprocess(cmd: list[str], *, input_text: str | None = None, **kwargs: Any) -> subprocess.CompletedProcess[str]:
    opts = {**_SUBPROCESS_UTF8, "capture_output": True, "check": False}
    opts.update(kwargs)
    if input_text is not None:
        opts["input"] = input_text
    return subprocess.run(cmd, **opts)


def _invoke_iota_publisher(repo: Path, event: dict[str, Any]) -> tuple[bool, str, int]:
    if os.environ.get("SDDIA_LAB_SIMULATE_IOTA", "").strip().lower() in ("1", "true", "yes"):
        return True, "lab-simulated", 0
    tool_dir = repo / "SddIA" / "scripts" / "tools" / "iota-immutable-publisher"
    entry = tool_dir / "index.ts"
    if not entry.is_file():
        return False, "iota-immutable-publisher entry not found", 1
    npx = shutil.which("npx")
    if not npx:
        return False, "npx not found on PATH", 1
    payload = {
        "action": "publish_immutable_data",
        "network": "testnet",
        "payload": json.dumps(event, ensure_ascii=False),
    }
    try:
        proc = _run_subprocess(
            [npx, "tsx", str(entry)],
            input_text=json.dumps(payload),
            cwd=str(tool_dir),
            timeout=_iota_timeout_seconds(),
            shell=False,
        )
    except subprocess.TimeoutExpired:
        return False, "iota-immutable-publisher timeout", 1
    except OSError as e:
        return False, str(e), 1
    if proc.returncode != 0:
        return False, (proc.stderr or proc.stdout or "iota publish failed").strip(), proc.returncode
    try:
        body = json.loads(proc.stdout.strip() or "{}")
    except json.JSONDecodeError:
        return False, "invalid JSON from iota-immutable-publisher", 1
    ok = bool(body.get("success"))
    return ok, body.get("feedback", "ok"), 0 if ok else 1


def dispatch_subscriber(
    repo: Path, subscriber: dict[str, Any], event: dict[str, Any]
) -> tuple[str, str, str | None, int]:
    """Retorna (subscriber_id, status, error_trace, exit_code)."""
    sid = subscriber_id(subscriber)
    agent = subscriber.get("agent")
    if not isinstance(agent, str) or not agent:
        return sid, "failed", "missing agent", 1

    payload = event.get("payload") or {}
    origin_topology = resolve_origin_topology(payload if isinstance(payload, dict) else {})
    if event.get("event_type", "").startswith("Domain_Entity_"):
        if not subscriber_applies_to_topology(subscriber, origin_topology):
            return sid, "skipped-topology", None, 0

    process_name = subscriber.get("process")
    if isinstance(process_name, str) and process_name.strip():
        runner = repo / "SddIA" / "scripts" / "qa" / "execute-process.py"
        if not runner.is_file():
            return sid, "failed", "execute-process.py not found", 1
        if not isinstance(payload, dict):
            return sid, "failed", "payload must be object", 1
        branch = payload.get("branch")
        if not isinstance(branch, str) or not branch.strip():
            return sid, "failed", "branch missing in payload", 1
        branch = branch.strip()
        process_inputs: dict[str, Any] = {
            "pr_branch": branch,
            "pr_id_or_path": payload.get("pr_url") or branch,
            "correlation_id": event.get("event_id") or "",
            "author": "eda-bus-watcher",
        }
        pr_url = payload.get("pr_url")
        if isinstance(pr_url, str) and pr_url.strip():
            process_inputs["pr_url"] = pr_url.strip()
            if github_pr_merged(pr_url):
                process_inputs["merge_already_done"] = True
        inferred = infer_persist_ref_from_branch(repo, branch)
        if inferred:
            process_inputs["persist_ref"] = inferred
        if process_name.strip() == "pull-request-review":
            process_inputs.setdefault("code_diff", "origin/main...HEAD")
            process_inputs.setdefault("tasks_path", "docs/todos")
            process_inputs.setdefault(
                "document_context",
                inferred or "docs/features/remove-cli-legacy-compat",
            )
        os.environ.setdefault("SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF", "0")
        try:
            proc = _run_subprocess(
                [
                    sys.executable,
                    str(runner),
                    "--process",
                    process_name.strip(),
                    "--inputs",
                    json.dumps(process_inputs, ensure_ascii=False),
                ],
                cwd=str(repo),
                shell=False,
            )
        except OSError as e:
            return sid, "failed", str(e), 1
        stdout = (proc.stdout or "").strip()
        if not stdout:
            return sid, "failed", (proc.stderr or "empty stdout").strip(), proc.returncode or 1
        last_line = stdout.splitlines()[-1]
        try:
            envelope = json.loads(last_line)
        except json.JSONDecodeError:
            return sid, "failed", "invalid JSON from execute-process", 1
        exit_code = int(envelope.get("status_code", 1 if not envelope.get("success") else 0))
        ok = bool(envelope.get("success")) and exit_code == 0
        if ok:
            return sid, "success", None, 0
        return sid, "failed", envelope.get("error") or envelope.get("message") or "process failed", exit_code

    tool = subscriber.get("tool")
    if tool == "iota-immutable-publisher":
        emitter = event.get("emitter_agent")
        if is_backfill_emitter(emitter if isinstance(emitter, str) else None):
            return sid, "skipped-backfill", None, 0
        ok_thresh, reason = dlt_threshold_ok(event)
        if not ok_thresh:
            return sid, "skipped-dlt-threshold", reason, 0
        ok, feedback, code = _invoke_iota_publisher(repo, event)
        if ok:
            return sid, "success", None, code
        return sid, "failed", feedback, code

    action = subscriber.get("action")
    if isinstance(action, str) and action:
        if action == "sync-entity-index" and os.environ.get(
            "SDDIA_LAB_SIMULATE_SYNC_INDEX", ""
        ).strip().lower() in ("1", "true", "yes"):
            return sid, "success", None, 0
        runner = repo / "SddIA" / "scripts" / "qa" / "execute-action.py"
        if not runner.is_file():
            return sid, "failed", "execute-action.py not found", 1
        if not isinstance(payload, dict):
            return sid, "failed", "payload must be object", 1
        try:
            proc = _run_subprocess(
                [
                    sys.executable,
                    str(runner),
                    "--action",
                    action,
                    "--inputs",
                    json.dumps(payload, ensure_ascii=False),
                ],
                cwd=str(repo),
                shell=False,
            )
        except OSError as e:
            return sid, "failed", str(e), 1
        stdout = (proc.stdout or "").strip()
        if not stdout:
            return sid, "failed", (proc.stderr or "empty stdout").strip(), proc.returncode or 1
        last_line = stdout.splitlines()[-1]
        try:
            envelope = json.loads(last_line)
        except json.JSONDecodeError:
            return sid, "failed", "invalid JSON from execute-action", 1
        data = envelope.get("data") or {}
        exit_code = int(envelope.get("status_code", 1 if not envelope.get("success") else 0))
        ok = bool(envelope.get("success")) and bool(data.get("success", True))
        if ok:
            return sid, "success", None, 0
        return sid, "failed", envelope.get("error") or data.get("error") or "action failed", exit_code

    return sid, "failed", "no process/action/tool configured", 1


def _parse_payload_fields(md_body: str, section: str) -> list[str]:
    pattern = rf"### {section}\s*\n((?:- .+\n?)*)"
    match = re.search(pattern, md_body)
    if not match:
        return []
    fields: list[str] = []
    for line in match.group(1).splitlines():
        field_match = re.search(r"`([^`]+)`", line)
        if field_match and not field_match.group(1).startswith("*"):
            fields.append(field_match.group(1))
    return fields


def load_event_class_schemas(repo: Path) -> dict[str, dict[str, list[str]]]:
    events_dir = repo / "SddIA" / "events"
    index_path = events_dir / "index.md"
    if not index_path.is_file():
        return {}
    index_text = index_path.read_text(encoding="utf-8")
    schemas: dict[str, dict[str, list[str]]] = {}
    row_re = re.compile(r"\|\s*`([^`]+\.md)`\s*\|[^|]+\|[^|]+\|\s*(\S+)\s*\|")
    for row_match in row_re.finditer(index_text):
        filename, event_type = row_match.group(1), row_match.group(2)
        class_path = events_dir / filename
        if not class_path.is_file():
            continue
        body = class_path.read_text(encoding="utf-8")
        if body.startswith("---"):
            parts = body.split("---", 2)
            body = parts[2] if len(parts) >= 3 else body
        schemas[event_type] = {
            "required": _parse_payload_fields(body, "REQUIRED"),
            "optional": _parse_payload_fields(body, "OPTIONAL"),
            "forbidden": _parse_payload_fields(body, "FORBIDDEN"),
        }
    return schemas


def validate_ecst_instance(
    event: dict[str, Any], schema: dict[str, list[str]] | None
) -> tuple[bool, list[str]]:
    errors: list[str] = []
    if schema is None:
        return False, ["event_type not cataloged in SddIA/events/index.md"]

    payload = event.get("payload")
    if not isinstance(payload, dict):
        return False, ["payload must be object"]

    for field in schema.get("required", []):
        if field not in payload or payload[field] is None:
            errors.append(f"missing required payload.{field}")

    for field in schema.get("forbidden", []):
        if field not in payload:
            continue
        value = payload[field]
        if field == "hash_signature":
            errors.append(f"forbidden payload.{field}")
        elif value is not None:
            errors.append(f"forbidden payload.{field} (must be null if present)")

    return not errors, errors


def _status_is_terminal_ok(status: str) -> bool:
    return status == "success" or status.startswith("skipped")


def _write_dead_letter_fallback(
    path: Path, event_uuid: str, sid: str, event_type: str, error_trace: str
) -> None:
    _write_json_atomic(
        path,
        {
            "event_uuid": event_uuid,
            "subscriber": sid,
            "state": "dead-letter",
            "started_at": _iso_now(),
            "failed_at": _iso_now(),
            "error_trace": error_trace,
            "event_type": event_type,
        },
    )


def _handle_subscriber(
    repo: Path,
    bus: dict[str, str],
    *,
    subscriber: dict[str, Any],
    event: dict[str, Any],
    event_uuid: str,
    event_type: str,
    pending_path: Path,
    registry: dict[str, Any],
    origin_topology: str,
    dispatch_mode: str,
) -> tuple[str, str]:
    sid = subscriber_id(subscriber)
    if terminal_witness_exists(repo, bus, event_uuid, sid):
        existing = list_witnesses(repo, bus, "processed_subscribers", event_uuid)
        for p in existing:
            if p.name == f"{event_uuid}.{sid}.json":
                return sid, "skipped-already-processed"
        return sid, "skipped-already-terminal"

    write_processing_witness(
        repo,
        bus,
        event_uuid=event_uuid,
        subscriber_name=sid,
        event_type=event_type,
        dispatch_mode=dispatch_mode,
    )
    _, status, err, exit_code = dispatch_subscriber(repo, subscriber, event)
    delegation = _delegation_meta(subscriber, exit_code)
    try:
        if _status_is_terminal_ok(status):
            promote_witness(
                repo,
                bus,
                event_uuid=event_uuid,
                subscriber_name=sid,
                to_state="processed",
                extra={
                    "result_status": status,
                    "delegation": delegation,
                },
                pending_header=pending_path,
            )
        else:
            promote_witness(
                repo,
                bus,
                event_uuid=event_uuid,
                subscriber_name=sid,
                to_state="dead-letter",
                extra={
                    "error_trace": err or status,
                    "delegation": delegation,
                },
                pending_header=pending_path,
            )
    except Exception as exc:
        dead = repo / bus["dead_letter_subscribers"] / f"{event_uuid}.{sid}.json"
        _write_dead_letter_fallback(dead, event_uuid, sid, event_type, traceback.format_exc())
        status = "failed"
        err = str(exc)

    maybe_purge_processing_header(
        repo, bus, event_uuid, registry, event_type, origin_topology
    )
    return sid, status


def route_domain_event(repo: Path, event_file_path: str) -> dict[str, Any]:
    bus = ensure_event_bus_topology(repo)
    raw_path = Path(event_file_path)
    event_path = (repo / raw_path).resolve() if not raw_path.is_absolute() else raw_path.resolve()

    if not event_path.is_file():
        return {
            "success": False,
            "exitCode": 1,
            "data": None,
            "error": f"event file not found: {event_path}",
        }

    try:
        event = json.loads(event_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as e:
        return {
            "success": False,
            "exitCode": 1,
            "data": None,
            "error": f"invalid event JSON: {e}",
        }

    event_type = event.get("event_type")
    event_uuid = event.get("event_id")
    if not isinstance(event_type, str) or not event_type:
        return {"success": False, "exitCode": 1, "data": None, "error": "event_type missing"}
    if not isinstance(event_uuid, str) or not event_uuid:
        return {"success": False, "exitCode": 1, "data": None, "error": "event_id missing"}

    inject_domain_entity_topology_defaults(event)
    payload = event.get("payload") if isinstance(event.get("payload"), dict) else {}
    origin_topology = resolve_origin_topology(payload)

    schemas = load_event_class_schemas(repo)
    schema = schemas.get(event_type)
    ecst_ok, ecst_errors = validate_ecst_instance(event, schema)
    if not ecst_ok:
        write_processing_witness(
            repo,
            bus,
            event_uuid=event_uuid,
            subscriber_name=ECST_GATE_SUBSCRIBER,
            event_type=event_type,
            dispatch_mode="sync",
        )
        promote_witness(
            repo,
            bus,
            event_uuid=event_uuid,
            subscriber_name=ECST_GATE_SUBSCRIBER,
            to_state="dead-letter",
            extra={"error_trace": "; ".join(ecst_errors), "ecst_errors": ecst_errors},
            pending_header=event_path,
        )
        return {
            "success": False,
            "exitCode": 1,
            "data": {
                "success": False,
                "delivery_status": {ECST_GATE_SUBSCRIBER: "failed"},
                "parent_path": _rel_event_path(repo, event_path),
            },
            "error": "; ".join(ecst_errors),
        }

    processing_header = ensure_processing_header(repo, bus, event_uuid, event_path)

    subs_path = repo / bus["subscriptions"]
    try:
        registry = json.loads(subs_path.read_text(encoding="utf-8-sig"))
    except (OSError, json.JSONDecodeError) as e:
        return {
            "success": False,
            "exitCode": 1,
            "data": None,
            "error": f"cannot read event-subscriptions.json: {e}",
        }

    subscribers = [
        sub
        for sub in (registry.get(event_type) or [])
        if isinstance(sub, dict)
        and subscriber_applies_to_topology(sub, origin_topology)
    ]

    dispatch_mode = _dispatch_mode_label()
    delivery_status: dict[str, str] = {}

    if not subscribers:
        return {
            "success": True,
            "exitCode": 0,
            "data": {
                "success": True,
                "delivery_status": {},
                "parent_path": _rel_event_path(repo, event_path),
                "processing_header_path": _rel_event_path(repo, processing_header),
                "dispatch_mode": dispatch_mode,
            },
        }

    def run_one(sub: dict[str, Any]) -> tuple[str, str]:
        return _handle_subscriber(
            repo,
            bus,
            subscriber=sub,
            event=event,
            event_uuid=event_uuid,
            event_type=event_type,
            pending_path=event_path,
            registry=registry,
            origin_topology=origin_topology,
            dispatch_mode=dispatch_mode,
        )

    if _sync_dispatch_mode():
        for sub in subscribers:
            sid, status = run_one(sub)
            delivery_status[sid] = status
    else:
        with ThreadPoolExecutor(max_workers=min(len(subscribers), 8)) as pool:
            futures = {pool.submit(run_one, sub): sub for sub in subscribers}
            for fut in as_completed(futures):
                sid, status = fut.result()
                delivery_status[sid] = status

    skip_only = delivery_status and all(
        v.startswith("skipped") for v in delivery_status.values()
    )
    all_success = not delivery_status or all(
        _status_is_terminal_ok(v) for v in delivery_status.values()
    )

    result: dict[str, Any] = {
        "success": all_success or skip_only,
        "exitCode": 0 if (all_success or skip_only) else 1,
        "data": {
            "success": all_success or skip_only,
            "delivery_status": delivery_status,
            "parent_path": _rel_event_path(repo, event_path),
            "processing_header_path": _rel_event_path(repo, processing_header),
            "dispatch_mode": dispatch_mode,
        },
    }
    if not all_success and not skip_only:
        result["error"] = "one or more subscribers failed"
    return result
