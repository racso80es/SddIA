#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Despertador inerte: monitoriza .events/pending/ y delega en route-domain-event.

Ola C V3: el JSON padre permanece inmutable en pending/; la trazabilidad
recae en testigos de suscriptor bajo .events/subscribers/.

Variables de entorno:
  SDDIA_LAB_SIMULATE_IOTA=1     Simula éxito de iota-immutable-publisher (laboratorio).
  SDDIA_IOTA_TIMEOUT_SECONDS=N  Timeout de publicación IOTA (default 45).

Uso:
  python SddIA/scripts/daemons/event-watcher.py           # bucle continuo
  python SddIA/scripts/daemons/event-watcher.py --once    # un ciclo de sondeo
  python SddIA/scripts/daemons/event-watcher.py --event-file-path .events/pending/x.json
"""

from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import time
import traceback
from pathlib import Path
from typing import Any

_SCRIPT_DIR = Path(__file__).resolve().parent
_QA_DIR = _SCRIPT_DIR.parent / "qa"
if str(_QA_DIR) not in sys.path:
    sys.path.insert(0, str(_QA_DIR))

from eda_bus_utils import (  # noqa: E402
    ECST_GATE_SUBSCRIBER,
    dlt_threshold_ok,
    ensure_event_bus_topology,
    github_pr_merged,
    infer_persist_ref_from_branch,
    inject_domain_entity_topology_defaults,
    is_backfill_emitter,
    load_eda_bus,
    list_witnesses,
    promote_witness,
    resolve_origin_topology,
    subscriber_applies_to_topology,
    subscriber_id,
    write_processing_witness,
)
from env_loader import load_hierarchical_env  # noqa: E402

POLL_SECONDS = 2
MAX_ROUTE_ATTEMPTS = 3


def _iota_timeout_seconds() -> int:
    return int(os.environ.get("SDDIA_IOTA_TIMEOUT_SECONDS", "45"))

_SUBPROCESS_UTF8 = {"text": True, "encoding": "utf-8", "errors": "replace"}


def _run_subprocess(cmd: list[str], *, input_text: str | None = None, **kwargs: Any) -> subprocess.CompletedProcess[str]:
    opts = {**_SUBPROCESS_UTF8, "capture_output": True, "check": False}
    opts.update(kwargs)
    if input_text is not None:
        opts["input"] = input_text
    return subprocess.run(cmd, **opts)


def _repo_root() -> Path:
    here = Path(__file__).resolve()
    for parent in here.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    _fail("No se encontró raíz del workspace (SddIA/core/cumulo.paths.json)")


def _fail(msg: str) -> None:
    print(json.dumps({"success": False, "exitCode": 1, "error": msg}), file=sys.stderr)
    sys.exit(1)


def _emit_route_result(out: dict[str, Any]) -> None:
    sys.stdout.write(json.dumps(out, ensure_ascii=False))


def _rel_event_path(repo: Path, event_path: Path) -> str:
    try:
        return event_path.resolve().relative_to(repo.resolve()).as_posix()
    except ValueError:
        return str(event_path.resolve())


def _invoke_iota_publisher(repo: Path, event: dict[str, Any]) -> tuple[bool, str]:
    if os.environ.get("SDDIA_LAB_SIMULATE_IOTA", "").strip().lower() in (
        "1",
        "true",
        "yes",
    ):
        return True, "lab-simulated"
    tool_dir = repo / "SddIA" / "scripts" / "tools" / "iota-immutable-publisher"
    entry = tool_dir / "index.ts"
    if not entry.is_file():
        return False, "iota-immutable-publisher entry not found"
    npx = shutil.which("npx")
    if not npx:
        return False, "npx not found on PATH"
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
        return False, "iota-immutable-publisher timeout"
    except OSError as e:
        return False, str(e)
    if proc.returncode != 0:
        return False, (proc.stderr or proc.stdout or "iota publish failed").strip()
    try:
        body = json.loads(proc.stdout.strip() or "{}")
    except json.JSONDecodeError:
        return False, "invalid JSON from iota-immutable-publisher"
    return bool(body.get("success")), body.get("feedback", "ok")


def _dispatch_subscriber(
    repo: Path, subscriber: dict[str, Any], event: dict[str, Any]
) -> tuple[str, str, str | None]:
    """Retorna (subscriber_id, status, error_trace)."""
    sid = subscriber_id(subscriber)
    agent = subscriber.get("agent")
    if not isinstance(agent, str) or not agent:
        return sid, "failed", "missing agent"

    payload = event.get("payload") or {}
    origin_topology = resolve_origin_topology(payload if isinstance(payload, dict) else {})
    if event.get("event_type", "").startswith("Domain_Entity_"):
        if not subscriber_applies_to_topology(subscriber, origin_topology):
            return sid, "skipped-topology", None

    process_name = subscriber.get("process")
    if isinstance(process_name, str) and process_name.strip():
        runner = repo / "SddIA" / "scripts" / "qa" / "execute-process.py"
        if not runner.is_file():
            return sid, "failed", "execute-process.py not found"
        if not isinstance(payload, dict):
            return sid, "failed", "payload must be object"
        branch = payload.get("branch")
        if not isinstance(branch, str) or not branch.strip():
            return sid, "failed", "branch missing in payload"
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
            return sid, "failed", str(e)
        stdout = (proc.stdout or "").strip()
        if not stdout:
            return sid, "failed", (proc.stderr or "empty stdout").strip()
        last_line = stdout.splitlines()[-1]
        try:
            envelope = json.loads(last_line)
        except json.JSONDecodeError:
            return sid, "failed", "invalid JSON from execute-process"
        ok = bool(envelope.get("success")) and envelope.get("status_code", 1) == 0
        if ok:
            return sid, "success", None
        return sid, "failed", envelope.get("error") or envelope.get("message") or "process failed"

    tool = subscriber.get("tool")
    if tool == "iota-immutable-publisher":
        emitter = event.get("emitter_agent")
        if is_backfill_emitter(emitter if isinstance(emitter, str) else None):
            return sid, "skipped-backfill", None
        ok_thresh, reason = dlt_threshold_ok(event)
        if not ok_thresh:
            return sid, "skipped-dlt-threshold", reason
        ok, feedback = _invoke_iota_publisher(repo, event)
        if ok:
            return sid, "success", None
        return sid, "failed", feedback
    action = subscriber.get("action")
    if isinstance(action, str) and action:
        if action == "sync-entity-index" and os.environ.get(
            "SDDIA_LAB_SIMULATE_SYNC_INDEX", ""
        ).strip().lower() in ("1", "true", "yes"):
            return sid, "success", None
        runner = repo / "SddIA" / "scripts" / "qa" / "execute-action.py"
        if not runner.is_file():
            return sid, "failed", "execute-action.py not found"
        payload = event.get("payload", {})
        if not isinstance(payload, dict):
            return sid, "failed", "payload must be object"
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
            return sid, "failed", str(e)
        stdout = (proc.stdout or "").strip()
        if not stdout:
            return sid, "failed", (proc.stderr or "empty stdout").strip()
        last_line = stdout.splitlines()[-1]
        try:
            envelope = json.loads(last_line)
        except json.JSONDecodeError:
            return sid, "failed", "invalid JSON from execute-action"
        data = envelope.get("data") or {}
        ok = bool(envelope.get("success")) and bool(data.get("success", True))
        if ok:
            return sid, "success", None
        return sid, "failed", envelope.get("error") or data.get("error") or "action failed"
    return sid, "failed", "no process/action/tool configured"


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


def _load_event_class_schemas(repo: Path) -> dict[str, dict[str, list[str]]]:
    events_dir = repo / "SddIA" / "events"
    index_path = events_dir / "index.md"
    if not index_path.is_file():
        return {}
    index_text = index_path.read_text(encoding="utf-8")
    schemas: dict[str, dict[str, list[str]]] = {}
    row_re = re.compile(
        r"\|\s*`([^`]+\.md)`\s*\|[^|]+\|[^|]+\|\s*(\S+)\s*\|"
    )
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


def _validate_ecst_instance(
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


def route_domain_event(event_file_path: str) -> dict[str, Any]:
    repo = _repo_root()
    bus = ensure_event_bus_topology(repo)

    raw_path = Path(event_file_path)
    event_path = (repo / raw_path).resolve() if not raw_path.is_absolute() else raw_path.resolve()

    if not event_path.is_file():
        _fail(f"event file not found: {event_path}")

    try:
        event = json.loads(event_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as e:
        _fail(f"invalid event JSON: {e}")

    event_type = event.get("event_type")
    event_uuid = event.get("event_id")
    if not isinstance(event_type, str) or not event_type:
        _fail("event_type missing")
    if not isinstance(event_uuid, str) or not event_uuid:
        _fail("event_id missing")

    inject_domain_entity_topology_defaults(event)

    class_schemas = _load_event_class_schemas(repo)
    schema = class_schemas.get(event_type)
    ecst_ok, ecst_errors = _validate_ecst_instance(event, schema)
    if not ecst_ok:
        write_processing_witness(
            repo,
            bus,
            event_uuid=event_uuid,
            subscriber_name=ECST_GATE_SUBSCRIBER,
            event_type=event_type,
        )
        promote_witness(
            repo,
            bus,
            event_uuid=event_uuid,
            subscriber_name=ECST_GATE_SUBSCRIBER,
            to_state="dead-letter",
            extra={"error_trace": "; ".join(ecst_errors), "ecst_errors": ecst_errors},
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

    subs_path = repo / bus["subscriptions"]
    try:
        registry = json.loads(subs_path.read_text(encoding="utf-8-sig"))
    except (OSError, json.JSONDecodeError) as e:
        _fail(f"cannot read event-subscriptions.json: {e}")

    subscribers = registry.get(event_type) or []
    delivery_status: dict[str, str] = {}

    if isinstance(subscribers, list):
        for sub in subscribers:
            if not isinstance(sub, dict):
                continue
            sid = subscriber_id(sub)
            try:
                write_processing_witness(
                    repo,
                    bus,
                    event_uuid=event_uuid,
                    subscriber_name=sid,
                    event_type=event_type,
                )
                _, status, err = _dispatch_subscriber(repo, sub, event)
                delivery_status[sid] = status
                if _status_is_terminal_ok(status):
                    promote_witness(
                        repo,
                        bus,
                        event_uuid=event_uuid,
                        subscriber_name=sid,
                        to_state="processed",
                        extra={"result_status": status},
                    )
                else:
                    promote_witness(
                        repo,
                        bus,
                        event_uuid=event_uuid,
                        subscriber_name=sid,
                        to_state="dead-letter",
                        extra={"error_trace": err or status},
                    )
            except Exception as exc:
                delivery_status[sid] = "failed"
                try:
                    promote_witness(
                        repo,
                        bus,
                        event_uuid=event_uuid,
                        subscriber_name=sid,
                        to_state="dead-letter",
                        extra={"error_trace": traceback.format_exc()},
                    )
                except OSError:
                    dead = repo / bus["subscriber_dead_letter"] / f"{event_uuid}.{sid}.json"
                    _write_dead_letter_fallback(dead, event_uuid, sid, event_type, str(exc))

    skip_only = delivery_status and all(
        v.startswith("skipped") for v in delivery_status.values()
    )
    all_success = not delivery_status or all(
        _status_is_terminal_ok(v) for v in delivery_status.values()
    )

    result = {
        "success": all_success or skip_only,
        "exitCode": 0 if (all_success or skip_only) else 1,
        "data": {
            "success": all_success or skip_only,
            "delivery_status": delivery_status,
            "parent_path": _rel_event_path(repo, event_path),
        },
    }
    if not all_success and not skip_only:
        result["error"] = "one or more subscribers failed"
    return result


def _write_dead_letter_fallback(
    path: Path, event_uuid: str, sid: str, event_type: str, error_trace: str
) -> None:
    from eda_bus_utils import _write_json_atomic, _iso_now

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


def _has_dead_letter_witnesses(repo: Path, bus: dict[str, str], event_uuid: str) -> bool:
    return bool(list_witnesses(repo, bus, "subscriber_dead_letter", event_uuid))


def _run_route_cli() -> None:
    parser = argparse.ArgumentParser(description="route-domain-event (cápsula física)")
    parser.add_argument(
        "--event-file-path",
        required=True,
        help="Ruta relativa o absoluta al JSON en pending/",
    )
    args = parser.parse_args()
    out = route_domain_event(args.event_file_path)
    _emit_route_result(out)
    sys.exit(0 if out.get("exitCode") == 0 else 1)


def _run_watcher(*, once: bool = False) -> None:
    repo = _repo_root()
    bus = ensure_event_bus_topology(repo)
    pending = repo / bus["pending"]
    script = Path(__file__).resolve()
    attempts: dict[str, int] = {}
    in_flight: set[str] = set()

    print("[WATCHER] Iniciado. pending=", pending, flush=True)
    while True:
        for path in sorted(pending.glob("*.json")):
            key = path.name
            event_uuid = path.stem
            if key in in_flight:
                continue
            if _has_dead_letter_witnesses(repo, bus, event_uuid):
                continue
            n = attempts.get(key, 0)
            if n >= MAX_ROUTE_ATTEMPTS:
                print(
                    f"[WATCHER] Skip {key}: max attempts ({MAX_ROUTE_ATTEMPTS})",
                    flush=True,
                )
                continue

            rel = _rel_event_path(repo, path)
            print(f"[WATCHER] Detectado nuevo evento: {key}", flush=True)
            in_flight.add(key)
            attempts[key] = n + 1

            proc = _run_subprocess(
                [
                    sys.executable,
                    str(script),
                    "--event-file-path",
                    rel,
                ],
                shell=False,
            )
            in_flight.discard(key)

            if proc.returncode != 0:
                print(
                    f"[WATCHER] route-domain-event falló ({key}): "
                    f"{(proc.stderr or proc.stdout or '').strip()}",
                    flush=True,
                )
            elif _has_dead_letter_witnesses(repo, bus, event_uuid):
                print(f"[WATCHER] {key}: testigo dead-letter — esperando sweeper/Kaizen", flush=True)
            else:
                attempts.pop(key, None)
                print(f"[WATCHER] {key}: enrutado (padre permanece en pending)", flush=True)

        time.sleep(POLL_SECONDS)
        if once:
            print("[WATCHER] Ciclo único (--once). Fin.", flush=True)
            break


def main() -> None:
    load_hierarchical_env(_repo_root())
    if "--event-file-path" in sys.argv:
        _run_route_cli()
    else:
        try:
            _run_watcher(once="--once" in sys.argv)
        except KeyboardInterrupt:
            print("[WATCHER] Detenido.", flush=True)
            sys.exit(0)


if __name__ == "__main__":
    main()
