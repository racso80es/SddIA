#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Despertador inerte: monitoriza docs/events/pending/ y delega en route-domain-event.

Variables de entorno:
  SDDIA_LAB_SIMULATE_IOTA=1     Simula éxito de iota-immutable-publisher (laboratorio).
  SDDIA_IOTA_TIMEOUT_SECONDS=N  Timeout de publicación IOTA (default 45).

Uso:
  python SddIA/scripts/daemons/event-watcher.py           # bucle continuo
  python SddIA/scripts/daemons/event-watcher.py --once  # un ciclo de sondeo
  python SddIA/scripts/daemons/event-watcher.py --event-file-path docs/events/processing/x.json
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
from pathlib import Path
from typing import Any

_SCRIPT_DIR = Path(__file__).resolve().parent
_QA_DIR = _SCRIPT_DIR.parent / "qa"
if str(_QA_DIR) not in sys.path:
    sys.path.insert(0, str(_QA_DIR))

from eda_bus_utils import (  # noqa: E402
    dlt_threshold_ok,
    inject_domain_entity_topology_defaults,
    is_backfill_emitter,
    resolve_origin_topology,
    subscriber_applies_to_topology,
)

POLL_SECONDS = 2
MAX_ROUTE_ATTEMPTS = 3
IOTA_TIMEOUT_SECONDS = int(os.environ.get("SDDIA_IOTA_TIMEOUT_SECONDS", "45"))

_SUBPROCESS_UTF8 = {"text": True, "encoding": "utf-8", "errors": "replace"}


def _run_subprocess(cmd: list[str], *, input_text: str | None = None, **kwargs: Any) -> subprocess.CompletedProcess[str]:
    """subprocess.run con canal UTF-8 tolerante (Windows / demonios)."""
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


def _load_eda_bus(repo: Path) -> dict[str, str]:
    """Rutas del bus desde cumulo.paths.json (fallback literales)."""
    defaults = {
        "pending": "docs/events/pending",
        "processing": "docs/events/processing",
        "processed": "docs/events/processed",
        "dead_letter": "docs/events/dead-letter",
        "subscriptions": "SddIA/core/event-subscriptions.json",
    }
    cfg_path = repo / "SddIA" / "core" / "cumulo.paths.json"
    try:
        import json as _json
        cfg = _json.loads(cfg_path.read_text(encoding="utf-8"))
        bus = cfg.get("eda_bus") or {}
        out = dict(defaults)
        for k in defaults:
            if isinstance(bus.get(k), str) and bus[k]:
                out[k] = bus[k]
        return out
    except (OSError, ValueError):
        return defaults


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
            timeout=IOTA_TIMEOUT_SECONDS,
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


def _infer_persist_ref_from_branch(branch: str) -> str | None:
    b = branch.strip()
    if b.startswith("feat/"):
        return f"docs/features/{b[5:]}"
    if b.startswith("fix/"):
        return f"docs/features/{b[4:]}"
    return None


def _dispatch_subscriber(
    repo: Path, subscriber: dict[str, Any], event: dict[str, Any]
) -> tuple[str, str]:
    agent = subscriber.get("agent")
    if not isinstance(agent, str) or not agent:
        return "unknown", "failed"

    payload = event.get("payload") or {}
    origin_topology = resolve_origin_topology(payload if isinstance(payload, dict) else {})
    if event.get("event_type", "").startswith("Domain_Entity_"):
        if not subscriber_applies_to_topology(subscriber, origin_topology):
            return agent, "skipped-topology"

    process_name = subscriber.get("process")
    if isinstance(process_name, str) and process_name.strip():
        runner = repo / "SddIA" / "scripts" / "qa" / "execute-process.py"
        if not runner.is_file():
            return agent, "failed"
        if not isinstance(payload, dict):
            return agent, "failed"
        branch = payload.get("branch")
        if not isinstance(branch, str) or not branch.strip():
            return agent, "failed"
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
        inferred = _infer_persist_ref_from_branch(branch)
        if inferred:
            process_inputs["persist_ref"] = inferred
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
        except OSError:
            return agent, "failed"
        stdout = (proc.stdout or "").strip()
        if not stdout:
            return agent, "failed"
        last_line = stdout.splitlines()[-1]
        try:
            envelope = json.loads(last_line)
        except json.JSONDecodeError:
            return agent, "failed"
        ok = bool(envelope.get("success")) and envelope.get("status_code", 1) == 0
        return agent, "success" if ok else "failed"

    tool = subscriber.get("tool")
    if tool == "iota-immutable-publisher":
        emitter = event.get("emitter_agent")
        if is_backfill_emitter(emitter if isinstance(emitter, str) else None):
            event.setdefault("delivery_state", {})["dlt"] = "skipped-backfill-v1"
            return agent, "skipped-backfill"
        ok_thresh, reason = dlt_threshold_ok(event)
        if not ok_thresh:
            event.setdefault("delivery_state", {})["dlt"] = f"skipped:{reason}"
            return agent, "skipped-dlt-threshold"
        ok, _ = _invoke_iota_publisher(repo, event)
        return agent, "success" if ok else "failed"
    action = subscriber.get("action")
    if isinstance(action, str) and action:
        if action == "sync-entity-index" and os.environ.get(
            "SDDIA_LAB_SIMULATE_SYNC_INDEX", ""
        ).strip().lower() in ("1", "true", "yes"):
            return agent, "success"
        runner = repo / "SddIA" / "scripts" / "qa" / "execute-action.py"
        if not runner.is_file():
            return agent, "failed"
        payload = event.get("payload", {})
        if not isinstance(payload, dict):
            return agent, "failed"
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
        except OSError:
            return agent, "failed"
        stdout = (proc.stdout or "").strip()
        if not stdout:
            return agent, "failed"
        last_line = stdout.splitlines()[-1]
        try:
            envelope = json.loads(last_line)
        except json.JSONDecodeError:
            return agent, "failed"
        data = envelope.get("data") or {}
        ok = bool(envelope.get("success")) and bool(data.get("success", True))
        return agent, "success" if ok else "failed"
    return agent, "failed"


def _parse_payload_fields(md_body: str, section: str) -> list[str]:
    """Extrae nombres de campo de una sección ### REQUIRED|OPTIONAL|FORBIDDEN."""
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
    """Mapa event_type → {required, optional, forbidden} desde genoma SddIA/events/."""
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
    """Valida instancia ECST frente a Clase catalogada (Fase 5 Ola C)."""
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


def route_domain_event(event_file_path: str) -> dict[str, Any]:
    repo = _repo_root()
    bus = _load_eda_bus(repo)
    processed = repo / bus["processed"]
    dead_letter = repo / bus["dead_letter"]
    processed.mkdir(parents=True, exist_ok=True)
    dead_letter.mkdir(parents=True, exist_ok=True)

    raw_path = Path(event_file_path)
    if not raw_path.is_absolute():
        event_path = (repo / raw_path).resolve()
    else:
        event_path = raw_path.resolve()

    if not event_path.is_file():
        _fail(f"event file not found: {event_path}")

    try:
        event = json.loads(event_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as e:
        _fail(f"invalid event JSON: {e}")

    event_type = event.get("event_type")
    if not isinstance(event_type, str) or not event_type:
        _fail("event_type missing")

    inject_domain_entity_topology_defaults(event)

    class_schemas = _load_event_class_schemas(repo)
    schema = class_schemas.get(event_type)
    ecst_ok, ecst_errors = _validate_ecst_instance(event, schema)
    if not ecst_ok:
        event["delivery_state"] = {
            **event.get("delivery_state", {}),
            "ecst_validation": "failed",
            "ecst_errors": ecst_errors,
        }
        dest = dead_letter / event_path.name
        dest.write_text(json.dumps(event, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        event_path.unlink()
        return {
            "success": False,
            "exitCode": 1,
            "data": {
                "success": False,
                "delivery_status": {"ecst_validation": "failed"},
                "target_path": _rel_event_path(repo, dest),
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
            agent, status = _dispatch_subscriber(repo, sub, event)
            delivery_status[agent] = status

    event["delivery_state"] = {**event.get("delivery_state", {}), **delivery_status}
    skip_only = delivery_status and all(
        v.startswith("skipped") for v in delivery_status.values()
    )
    all_success = not delivery_status or all(
        v == "success" or v.startswith("skipped") for v in delivery_status.values()
    )
    dest_dir = processed if (all_success or skip_only) else dead_letter
    dest = dest_dir / event_path.name
    dest.write_text(json.dumps(event, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    event_path.unlink()

    result = {
        "success": all_success,
        "exitCode": 0 if all_success else 1,
        "data": {
            "success": all_success,
            "delivery_status": delivery_status,
            "target_path": _rel_event_path(repo, dest),
        },
    }
    if not all_success:
        result["error"] = "one or more subscribers failed"
    return result


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
    bus = _load_eda_bus(repo)
    pending = repo / bus["pending"]
    processing = repo / bus["processing"]
    pending.mkdir(parents=True, exist_ok=True)
    processing.mkdir(parents=True, exist_ok=True)
    script = Path(__file__).resolve()
    attempts: dict[str, int] = {}
    in_flight: set[str] = set()

    print("[WATCHER] Iniciado. pending=", pending, flush=True)
    while True:
        for path in sorted(pending.glob("*.json")):
            key = path.name
            if key in in_flight:
                continue
            n = attempts.get(key, 0)
            if n >= MAX_ROUTE_ATTEMPTS:
                if path.is_file():
                    print(
                        f"[WATCHER] Skip {key}: max attempts ({MAX_ROUTE_ATTEMPTS})",
                        flush=True,
                    )
                continue

            processing_path = processing / key
            try:
                shutil.move(str(path), str(processing_path))
            except OSError as e:
                print(f"[WATCHER] No se pudo promover {key} a processing: {e}", flush=True)
                continue

            rel = _rel_event_path(repo, processing_path)
            print(f"[WATCHER] Detectado nuevo evento: {key} (promovido a processing)", flush=True)
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
            elif not processing_path.is_file():
                attempts.pop(key, None)
            else:
                print(
                    f"[WATCHER] {key} sigue en processing tras enrutar (intento {attempts[key]})",
                    flush=True,
                )

        time.sleep(POLL_SECONDS)
        if once:
            print("[WATCHER] Ciclo único (--once). Fin.", flush=True)
            break


def main() -> None:
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
