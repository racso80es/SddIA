#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Golden harness P8/P9: compara envelope Rust vs Python (normaliza no-deterministas)."""

from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
import uuid
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

SCRIPT = Path(__file__).resolve()
QA = SCRIPT.parent
if str(QA) not in sys.path:
    sys.path.insert(0, str(QA))

UUID_RE = re.compile(
    r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}",
    re.I,
)

LAB_FEATURE_ENV = {
    "SDDIA_LAB_SKIP_PBI_ARCHIVE": "1",
    "SDDIA_LAB_SKIP_DELIVERY_CLOSE": "1",
    "SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF": "1",
    "SDDIA_LAB_SKIP_GIT": "1",
}

LAB_DELIVERY_ENV = {
    "SDDIA_LAB_SKIP_SNAPSHOT": "1",
    "SDDIA_LAB_SKIP_GIT_PUSH": "1",
    "SDDIA_LAB_SIMULATE_GH_PR": "1",
    "SDDIA_LAB_SKIP_HIGIENE": "1",
    "SDDIA_LAB_SKIP_IMPACT_ASSESSMENT": "1",
}


def repo_root() -> Path:
    for parent in SCRIPT.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError("repo root not found")


def normalize(obj: Any) -> Any:
    """Elimina campos no deterministas para comparación estructural."""
    if isinstance(obj, dict):
        skip_keys = {
            "asset_id",
            "execution_id",
            "duration_ms",
            "event_id",
            "target_path",
            "thermodynamic_toll",
            "created",
            "workspace_path",
            "branch_name",
            "objectives_path",
            "git_steps",
            "handoff",
            "message_preview",
            "tool_result",
            "response_preview",
            "dispatch_mode",
            "seal",
            "sensorial_seal",
            "orchestration_event_id",
            "orchestration_event_path",
            "purge_report",
            "stale_locks_removed",
            "fractures_emitted",
            "os_result",
            "parent_path",
            "processing_header_path",
            "sweep",
            "delivery_status",
            "pr_url",
            "gh_stdout",
            "eda_audit",
            "orphan_count",
            "argos_verdict",
            "argos_noise",
            "capsule_result",
            "capsule_invoked",
        }
        out = {}
        for k, v in obj.items():
            if k in skip_keys:
                continue
            if k == "error" and (v is None or v == ""):
                continue
            out[k] = normalize(v)
        return out
    if isinstance(obj, list):
        return [normalize(x) for x in obj]
    if isinstance(obj, str):
        s = UUID_RE.sub("<UUID>", obj)
        if "/.SddIA/workspaces/" in s or s.startswith("/home/"):
            return "<PATH>"
        if "no encontrada" in s.lower() or "no encontrado" in s.lower():
            return "<CAPSULE_MISSING>"
        return s
    return obj


def phase_signature(body: dict[str, Any]) -> list[tuple[str, str]]:
    report = body.get("execution_report") or {}
    phases = report.get("phases") or []
    sig: list[tuple[str, str]] = []
    if isinstance(phases, list):
        for ph in phases:
            if isinstance(ph, dict):
                sig.append(
                    (
                        str(ph.get("phase_name", "")),
                        str(ph.get("status", "")),
                    )
                )
    return sig


def run_orchestrator(
    repo: Path,
    use_rust: bool,
    process: str,
    inputs: dict[str, Any],
    env_overlay: dict[str, str],
) -> dict[str, Any]:
    env = dict(os.environ)
    env.update(env_overlay)
    inputs_json = json.dumps(inputs, ensure_ascii=False)
    if use_rust:
        bin_path = repo / "SddIA/target/debug/execute-process"
        if not bin_path.is_file():
            bin_path = repo / "SddIA/target/release/execute-process"
        if not bin_path.is_file():
            raise FileNotFoundError("binario execute-process no compilado")
        cmd = [str(bin_path), "--process", process, "--inputs", inputs_json]
    else:
        cmd = [
            sys.executable,
            str(repo / "SddIA/scripts/qa/execute-process.py"),
            "--process",
            process,
            "--inputs",
            inputs_json,
        ]

    proc = subprocess.run(
        cmd,
        cwd=str(repo),
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        env=env,
        check=False,
    )
    line = (proc.stdout or "").strip().splitlines()[-1] if proc.stdout else ""
    if not line:
        raise RuntimeError(proc.stderr or "sin salida")
    return json.loads(line)


LAB_ROUTE_ENV = {"SDDIA_LAB_ROUTE_SYNC": "1"}

ENTITY_MANAGER_LAB_NAME = "golden-orchestrator-parity-em"

ENTITY_MANAGER_INPUTS: dict[str, Any] = {
    "entity_class": "tool",
    "entity_name": ENTITY_MANAGER_LAB_NAME,
    "lifecycle_operation": "create",
    "semantic_seed": {
        "tool_name": ENTITY_MANAGER_LAB_NAME,
        "scope": "local",
        "execution_logic": "golden P9 entity-manager smoke",
        "tool_outputs": [],
        "required_secrets": [],
        "dependencies": [],
    },
}


def cleanup_entity_manager_lab(repo: Path) -> None:
    """Teardown forge local + fila índice (P9 entity-manager)."""
    from lab_teardown import cleanup_lab_entity_forge

    cleanup_lab_entity_forge(
        repo,
        entity_class="tool",
        entity_name=ENTITY_MANAGER_LAB_NAME,
        event_id=None,
    )


def write_route_fixture(repo: Path) -> str:
    """Evento Daemon_Heartbeat ECST válido (sin fan-out en event-subscriptions.json)."""
    pending = repo / ".events" / "pending"
    pending.mkdir(parents=True, exist_ok=True)
    event_id = str(uuid.uuid4())
    event = {
        "event_id": event_id,
        "event_type": "Daemon_Heartbeat",
        "timestamp": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "emitter_agent": "golden-orchestrator-parity",
        "payload": {
            "daemon_name": "event-watcher",
            "daemon_uuid": str(uuid.uuid4()),
            "pid": os.getpid(),
            "uptime_seconds": 42,
            "status": "alive",
        },
        "delivery_state": {},
    }
    rel = f".events/pending/{event_id}.json"
    (repo / rel).write_text(json.dumps(event, ensure_ascii=False, indent=2), encoding="utf-8")
    return rel


CASES: list[tuple[str, dict[str, Any], dict[str, str], bool, bool]] = [
    ("kalma2-interact", {"prompt": "golden parity ping"}, {}, False, False),
    (
        "feature",
        {
            "feature_name": "migracion-execute-process-rust",
            "persist_ref": "docs/features/migracion-execute-process-rust",
            "document_context": "docs/features/migracion-execute-process-rust",
        },
        LAB_FEATURE_ENV,
        False,
        False,
    ),
    (
        "bug-fix",
        {
            "bug_summary": "golden parity bug-fix smoke",
            "branch_name": "fix/golden-parity-bug-fix",
            "persist_ref": "docs/fixes/golden-parity-bug-fix",
            "document_context": "docs/fixes/golden-parity-bug-fix",
        },
        LAB_FEATURE_ENV,
        False,
        False,
    ),
    (
        "telegram-fallback-responder",
        {"text": "/start"},
        {},
        False,
        False,
    ),
    (
        "telegram-fallback-responder",
        {"text": "golden parity ping"},
        {"TELEGRAM_ALLOWED_CHAT_ID": ""},
        False,
        False,
    ),
    (
        "telegram-gateway",
        {"text": "TODO: golden parity"},
        {"TELEGRAM_ALLOWED_CHAT_ID": ""},
        False,
        False,
    ),
    (
        "telegram-gateway",
        {"text": ""},
        {},
        False,
        False,
    ),
    ("daemon-heartbeat-audit", {}, {}, False, False),
    (
        "governance-daemon-manager",
        {"operation": "status", "daemon_id": "event-watcher"},
        {},
        False,
        False,
    ),
    ("daemon-kill-switch", {}, {}, False, False),
    ("route-domain-event", {}, {**LAB_ROUTE_ENV}, True, False),
    (
        "delivery-close-cycle",
        {
            "source_process": "feature",
            "persist_ref": "docs/features/migracion-execute-process-rust",
            "branch_name": "feat/migracion-execute-process-rust",
            "pr_title": "golden delivery close",
        },
        LAB_DELIVERY_ENV,
        False,
        False,
    ),
    ("capsule-invoke-smoke", {}, {}, False, False),
    ("entity-manager", ENTITY_MANAGER_INPUTS, {}, False, True),
]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--process", help="Proceso individual")
    parser.add_argument("--inputs", help="JSON inputs")
    args = parser.parse_args()
    repo = repo_root()

    cases = CASES
    if args.process:
        inputs = json.loads(args.inputs or "{}")
        overlay = LAB_FEATURE_ENV if args.process in ("feature", "bug-fix") else {}
        needs_entity_cleanup = args.process == "entity-manager"
        if args.process == "route-domain-event":
            overlay = {**LAB_ROUTE_ENV}
        elif args.process == "delivery-close-cycle":
            overlay = {**LAB_DELIVERY_ENV}
        cases = [
            (
                args.process,
                inputs,
                overlay,
                args.process == "route-domain-event",
                needs_entity_cleanup,
            )
        ]

    failed = 0
    for process, inputs, env_overlay, needs_route_fixture, needs_entity_cleanup in cases:
        label = process if len(cases) == len(CASES) else f"{process} custom"
        fixture_rels: list[str] = []
        try:
            if needs_entity_cleanup:
                cleanup_entity_manager_lab(repo)
            case_env = dict(env_overlay)
            if needs_route_fixture:
                case_env.update(LAB_ROUTE_ENV)
                py_fixture = write_route_fixture(repo)
                rust_fixture = write_route_fixture(repo)
                fixture_rels = [py_fixture, rust_fixture]
                py_inputs = {"event_file_path": py_fixture}
                rust_inputs = {"event_file_path": rust_fixture}
            else:
                py_inputs = dict(inputs)
                rust_inputs = dict(inputs)
            py_body = run_orchestrator(repo, False, process, py_inputs, case_env)
            rust_body = run_orchestrator(repo, True, process, rust_inputs, case_env)
            nr_data = normalize(rust_body.get("data"))
            np_data = normalize(py_body.get("data"))
            same_success = rust_body.get("success") == py_body.get("success")
            same_phases = phase_signature(rust_body) == phase_signature(py_body)
            if same_success and same_phases and nr_data == np_data:
                print(f"OK  {label}")
            else:
                failed += 1
                print(f"FAIL {label}")
                if not same_success:
                    print(f"  success rust={rust_body.get('success')} py={py_body.get('success')}")
                if not same_phases:
                    print(f"  phases rust={phase_signature(rust_body)}")
                    print(f"  phases py  ={phase_signature(py_body)}")
                if nr_data != np_data:
                    print("  data rust:", json.dumps(nr_data, ensure_ascii=False)[:240])
                    print("  data py  :", json.dumps(np_data, ensure_ascii=False)[:240])
        except Exception as exc:
            failed += 1
            print(f"ERR {label}: {exc}")
        finally:
            if needs_entity_cleanup:
                cleanup_entity_manager_lab(repo)
            for fixture_rel in fixture_rels:
                try:
                    (repo / fixture_rel).unlink(missing_ok=True)
                except OSError:
                    pass

    return 1 if failed else 0


if __name__ == "__main__":
    raise SystemExit(main())
