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
import uuid
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
    resolve_pull_request_lifecycle,
    resolve_origin_topology,
    subscriber_applies_to_topology,
    subscriber_id,
    terminal_witness_exists,
    try_sweep_event,
    write_processing_witness,
)
from ecst_validation import load_event_class_schemas, validate_ecst_instance

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


def _invoke_iota_publisher(
    repo: Path, event: dict[str, Any]
) -> tuple[bool, str, int, str | None]:
    if os.environ.get("SDDIA_LAB_SIMULATE_IOTA", "").strip().lower() in ("1", "true", "yes"):
        digest = f"lab-sim-{uuid.uuid4().hex[:24]}"
        return True, "lab-simulated", 0, digest
    tool_dir = repo / "SddIA" / "scripts" / "tools" / "iota-immutable-publisher"
    entry = tool_dir / "index.ts"
    if not entry.is_file():
        return False, "iota-immutable-publisher entry not found", 1, None
    npx = shutil.which("npx")
    if not npx:
        return False, "npx not found on PATH", 1, None
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
        return False, "iota-immutable-publisher timeout", 1, None
    except OSError as e:
        return False, str(e), 1, None
    if proc.returncode != 0:
        return (
            False,
            (proc.stderr or proc.stdout or "iota publish failed").strip(),
            proc.returncode,
            None,
        )
    try:
        body = json.loads(proc.stdout.strip() or "{}")
    except json.JSONDecodeError:
        return False, "invalid JSON from iota-immutable-publisher", 1, None
    ok = bool(body.get("success"))
    digest = (body.get("result") or {}).get("transaction_digest")
    if isinstance(digest, str) and digest.strip():
        digest = digest.strip()
    else:
        digest = None
    return ok, body.get("feedback", "ok"), 0 if ok else 1, digest


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
        process_key = process_name.strip()
        if process_key == "pull-request-review":
            ok_precheck, precheck_err, lifecycle = _pull_request_review_precheck(
                repo, branch=branch, pr_url=pr_url if isinstance(pr_url, str) else None, payload=payload
            )
            if not ok_precheck:
                return sid, "failed", precheck_err, 1
            if lifecycle.get("merged") is True:
                process_inputs["merge_already_done"] = True
        elif isinstance(pr_url, str) and pr_url.strip() and github_pr_merged(pr_url):
            process_inputs["merge_already_done"] = True
        inferred = infer_persist_ref_from_branch(repo, branch)
        if inferred:
            process_inputs["persist_ref"] = inferred
        if process_name.strip() == "pull-request-review":
            process_inputs.setdefault("code_diff", "origin/main...HEAD")
            process_inputs.setdefault("tasks_path", "docs/todos")
            if inferred:
                process_inputs.setdefault("document_context", inferred)
            elif isinstance(process_inputs.get("persist_ref"), str) and process_inputs["persist_ref"].strip():
                process_inputs.setdefault("document_context", process_inputs["persist_ref"].strip())
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
    if tool == "send-telegram-notification":
        from telegram_notify_core import (
            build_telegram_message_from_event,
            invoke_send_telegram_notification,
        )

        message = build_telegram_message_from_event(event)
        if not message:
            return sid, "skipped-empty-message", None, 0
        ok, body = invoke_send_telegram_notification(repo, message)
        if ok:
            return sid, "success", None, 0
        err = body.get("error") or "send-telegram-notification failed"
        return sid, "failed", str(err), 1

    if tool == "iota-immutable-publisher":
        emitter = event.get("emitter_agent")
        if is_backfill_emitter(emitter if isinstance(emitter, str) else None):
            return sid, "skipped-backfill", None, 0
        pl = event.get("payload")
        if isinstance(pl, dict) and pl.get("dlt_anchor_address"):
            return sid, "skipped-pre-anchored", None, 0
        ok_thresh, reason = dlt_threshold_ok(event)
        if not ok_thresh:
            return sid, "skipped-dlt-threshold", reason, 0
        ok, feedback, code, digest = _invoke_iota_publisher(repo, event)
        if ok:
            if digest:
                ds = event.get("delivery_state")
                if isinstance(ds, dict):
                    ds["cumulo"] = "success"
                    ds["transaction_digest"] = digest
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


def _status_is_terminal_ok(status: str) -> bool:
    return status == "success" or status.startswith("skipped")


def _pull_request_review_precheck(
    repo: Path,
    *,
    branch: str,
    pr_url: str | None,
    payload: dict[str, Any],
) -> tuple[bool, str | None, dict[str, Any]]:
    """Resuelve ciclo de vida PR antes de subprocess pull-request-review."""
    target = payload.get("target_branch", "main")
    if not isinstance(target, str) or not target.strip():
        target = "main"
    lifecycle = resolve_pull_request_lifecycle(
        repo,
        branch=branch,
        pr_url=pr_url.strip() if isinstance(pr_url, str) else None,
        target_branch=target.strip(),
    )
    merged = lifecycle.get("merged")
    on_remote = bool(lifecycle.get("branch_on_remote"))
    if merged is True:
        return True, None, lifecycle
    if merged is False and not on_remote:
        return False, (
            "pull-request-review: rama ausente en origin y PR no mergeado "
            f"(branch={branch}, pr={lifecycle.get('pr_number')})"
        ), lifecycle
    if merged is None and not on_remote:
        diag = lifecycle.get("diagnostics") or []
        return False, (
            "pull-request-review: no se pudo resolver ciclo de vida del PR "
            f"(branch={branch}; diagnostics={diag})"
        ), lifecycle
    return True, None, lifecycle


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
    iota_digest: str | None = None

    if not subscribers:
        sweep = try_sweep_event(repo, bus, event_uuid, registry=registry)
        return {
            "success": True,
            "exitCode": 0,
            "data": {
                "success": True,
                "delivery_status": {},
                "parent_path": _rel_event_path(repo, event_path),
                "processing_header_path": _rel_event_path(repo, processing_header),
                "dispatch_mode": dispatch_mode,
                "sweep": sweep,
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

    def _capture_iota_digest() -> None:
        nonlocal iota_digest
        ds = event.get("delivery_state")
        if isinstance(ds, dict):
            raw = ds.get("transaction_digest")
            if isinstance(raw, str) and raw.strip():
                iota_digest = raw.strip()

    if _sync_dispatch_mode():
        for sub in subscribers:
            sid, status = run_one(sub)
            delivery_status[sid] = status
            _capture_iota_digest()
    else:
        with ThreadPoolExecutor(max_workers=min(len(subscribers), 8)) as pool:
            futures = {pool.submit(run_one, sub): sub for sub in subscribers}
            for fut in as_completed(futures):
                sid, status = fut.result()
                delivery_status[sid] = status
                _capture_iota_digest()

    skip_only = delivery_status and all(
        v.startswith("skipped") for v in delivery_status.values()
    )
    all_success = not delivery_status or all(
        _status_is_terminal_ok(v) for v in delivery_status.values()
    )

    result_data: dict[str, Any] = {
        "success": all_success or skip_only,
        "delivery_status": delivery_status,
        "parent_path": _rel_event_path(repo, event_path),
        "processing_header_path": _rel_event_path(repo, processing_header),
        "dispatch_mode": dispatch_mode,
    }
    if iota_digest:
        result_data["transaction_digest"] = iota_digest
    result: dict[str, Any] = {
        "success": all_success or skip_only,
        "exitCode": 0 if (all_success or skip_only) else 1,
        "data": result_data,
    }
    if not all_success and not skip_only:
        result["error"] = "one or more subscribers failed"

    result["data"]["sweep"] = try_sweep_event(repo, bus, event_uuid, registry=registry)
    return result
