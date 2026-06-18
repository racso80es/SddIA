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


CASES: list[tuple[str, dict[str, Any], dict[str, str]]] = [
    ("kalma2-interact", {"prompt": "golden parity ping"}, {}),
    (
        "feature",
        {
            "feature_name": "migracion-execute-process-rust",
            "persist_ref": "docs/features/migracion-execute-process-rust",
            "document_context": "docs/features/migracion-execute-process-rust",
        },
        LAB_FEATURE_ENV,
    ),
    (
        "telegram-fallback-responder",
        {"text": "/start"},
        {},
    ),
    (
        "telegram-fallback-responder",
        {"text": "golden parity ping"},
        {"TELEGRAM_ALLOWED_CHAT_ID": ""},
    ),
    (
        "telegram-gateway",
        {"text": "TODO: golden parity"},
        {"TELEGRAM_ALLOWED_CHAT_ID": ""},
    ),
    (
        "telegram-gateway",
        {"text": ""},
        {},
    ),
    ("daemon-heartbeat-audit", {}, {}),
    (
        "governance-daemon-manager",
        {"operation": "status", "daemon_id": "event-watcher"},
        {},
    ),
    ("daemon-kill-switch", {}, {}),
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
        cases = [(args.process, inputs, overlay)]

    failed = 0
    for process, inputs, env_overlay in cases:
        label = process if len(cases) == len(CASES) else f"{process} custom"
        try:
            py_body = run_orchestrator(repo, False, process, inputs, env_overlay)
            rust_body = run_orchestrator(repo, True, process, inputs, env_overlay)
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

    return 1 if failed else 0


if __name__ == "__main__":
    raise SystemExit(main())
