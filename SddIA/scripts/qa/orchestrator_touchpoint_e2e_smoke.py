#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Smokes E2E CA-7: touchpoints del orquestador (binario nativo + SSOT)."""

from __future__ import annotations

import json
import os
import subprocess
import sys
from pathlib import Path
from typing import Any

SCRIPT = Path(__file__).resolve()
QA = SCRIPT.parent
if str(QA) not in sys.path:
    sys.path.insert(0, str(QA))

REPO = next(
    p for p in SCRIPT.parents if (p / "SddIA" / "core" / "cumulo.paths.json").is_file()
)

from env_loader import load_hierarchical_env
from orchestrator_resolve import resolve_orchestrator_cmd, resolve_orchestrator_executable


def _run_orchestrator(
    argv_tail: list[str],
    *,
    env: dict[str, str] | None = None,
    label: str = "",
) -> dict[str, Any]:
    cmd = resolve_orchestrator_cmd(REPO, argv_tail)
    proc = subprocess.run(
        cmd,
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(REPO),
        env=env or os.environ.copy(),
        check=False,
    )
    line = (proc.stdout or "").strip().splitlines()[-1] if proc.stdout else ""
    if not line:
        raise RuntimeError(f"{label or 'orchestrator'}: {proc.stderr or 'sin salida JSON'}")
    body = json.loads(line)
    if not body.get("success"):
        raise RuntimeError(f"{label or 'orchestrator'}: {body.get('error') or body}")
    return body


def smoke_ssot_resolves_native() -> None:
    exe = resolve_orchestrator_executable(REPO)
    if not exe.name == "execute-process":
        raise RuntimeError(f"SSOT no resolvió binario nativo: {exe}")
    if not exe.is_file():
        raise RuntimeError(f"binario ausente: {exe}")


def smoke_orchestrator_resolve_cli() -> None:
    _run_orchestrator(
        ["--process", "daemon-kill-switch", "--inputs", "{}"],
        label="orchestrator_resolve CLI",
    )


def smoke_sddia_run_sh() -> None:
    proc = subprocess.run(
        [str(REPO / "sddia-run.sh"), "--process", "daemon-kill-switch", "--inputs", "{}"],
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(REPO),
        check=False,
    )
    line = (proc.stdout or "").strip().splitlines()[-1] if proc.stdout else ""
    if proc.returncode != 0 or not line:
        raise RuntimeError(proc.stderr or "sddia-run.sh falló")
    body = json.loads(line)
    if not body.get("success"):
        raise RuntimeError(body.get("error") or "sddia-run envelope inválido")


def smoke_kalma2_bridge_invoke() -> None:
    bridge = REPO / ".SddIA" / "client" / "sddia-client-bridge.py"
    if not bridge.is_file():
        raise RuntimeError("sddia-client-bridge.py ausente")
    code = """
import json, sys, importlib.util
sys.path.insert(0, %r)
spec = importlib.util.spec_from_file_location("bridge", %r)
mod = importlib.util.module_from_spec(spec)
spec.loader.exec_module(mod)
out = mod.invoke_engine("e2e smoke ping")
if not out.get("success") or not out.get("response"):
    raise SystemExit(json.dumps(out))
print(json.dumps({"success": True}))
""" % (
        str(QA),
        str(bridge),
    )
    proc = subprocess.run(
        [sys.executable, "-c", code],
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(REPO),
        check=False,
    )
    line = (proc.stdout or "").strip().splitlines()[-1] if proc.stdout else ""
    if proc.returncode != 0 or not line:
        raise RuntimeError(proc.stderr or "kalma2 bridge falló")
    body = json.loads(line)
    if not body.get("success"):
        raise RuntimeError(body)


def smoke_hook_common_invoke() -> None:
    hook_common = QA / "git-hooks" / "hook_common.py"
    code = """
import sys
sys.path.insert(0, %r)
import importlib.util
spec = importlib.util.spec_from_file_location("hook_common", %r)
mod = importlib.util.module_from_spec(spec)
spec.loader.exec_module(mod)
rc = mod.invoke_process("daemon-kill-switch", {})
if rc != 0:
    raise SystemExit(f"hook invoke_process exit {rc}")
print('{"success": true}')
""" % (
        str(QA),
        str(hook_common),
    )
    proc = subprocess.run(
        [sys.executable, "-c", code],
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(REPO),
        check=False,
    )
    line = (proc.stdout or "").strip().splitlines()[-1] if proc.stdout else ""
    if proc.returncode != 0 or not line:
        raise RuntimeError(proc.stderr or "hook_common falló")
    body = json.loads(line)
    if not body.get("success"):
        raise RuntimeError(body)


def smoke_event_watcher_bin() -> None:
    from capsule_resolve import resolve_daemon_capsule

    watcher = resolve_daemon_capsule(REPO, "event-watcher")
    if not watcher.is_file():
        raise RuntimeError(f"event-watcher ausente: {watcher}")
    if not os.access(watcher, os.X_OK):
        raise RuntimeError(f"event-watcher no ejecutable: {watcher}")


def smoke_native_without_pyyaml() -> None:
    """CA-8 parcial: binario nativo no requiere intérprete Python."""
    exe = REPO / "SddIA" / "target" / "debug" / "execute-process"
    if not exe.is_file():
        raise RuntimeError("binario execute-process ausente")
    env = os.environ.copy()
    env["PATH"] = "/usr/bin:/bin"
    env.pop("PYTHONPATH", None)
    proc = subprocess.run(
        [str(exe), "--process", "daemon-kill-switch", "--inputs", "{}"],
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(REPO),
        env=env,
        check=False,
    )
    line = (proc.stdout or "").strip().splitlines()[-1] if proc.stdout else ""
    if not line:
        raise RuntimeError(proc.stderr or "binario nativo sin salida con PATH mínimo")
    body = json.loads(line)
    if not body.get("success"):
        raise RuntimeError(body.get("error") or "binario nativo falló CA-8 smoke")


def smoke_eda_e2e_lab() -> None:
    script = QA / "run-eda-e2e-lab.py"
    env = os.environ.copy()
    # Cadena watcher → route requiere bus canónico `.events/` (no overlay EVENT_BUS_PATH).
    env.pop("EVENT_BUS_PATH", None)
    env.setdefault("SDDIA_LAB_SIMULATE_IOTA", "1")
    env.setdefault("SDDIA_LAB_SIMULATE_SYNC_INDEX", "1")
    env.setdefault("SDDIA_LAB_ROUTE_SYNC", "1")
    proc = subprocess.run(
        [sys.executable, str(script), "--entity-class", "tool", "--json"],
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(REPO),
        env=env,
        check=False,
    )
    if proc.returncode != 0:
        raise RuntimeError(proc.stderr or proc.stdout or "run-eda-e2e-lab falló")
    report = json.loads(proc.stdout)
    if not report.get("success"):
        raise RuntimeError(report.get("error") or report)


SMOKES: list[tuple[str, callable]] = [
    ("ssot-native-binary", smoke_ssot_resolves_native),
    ("orchestrator-resolve-cli", smoke_orchestrator_resolve_cli),
    ("sddia-run-sh", smoke_sddia_run_sh),
    ("kalma2-bridge", smoke_kalma2_bridge_invoke),
    ("hook-common", smoke_hook_common_invoke),
    ("event-watcher-bin", smoke_event_watcher_bin),
    ("native-without-python", smoke_native_without_pyyaml),
    ("eda-e2e-lab", smoke_eda_e2e_lab),
]


def main() -> int:
    load_hierarchical_env(REPO)
    bin_path = REPO / "SddIA" / "target" / "debug" / "execute-process"
    if bin_path.is_file():
        os.environ.setdefault("SDDIA_EXECUTE_PROCESS_BIN", str(bin_path))
    failures: list[str] = []
    for name, fn in SMOKES:
        try:
            fn()
            print(f"OK  {name}")
        except Exception as exc:
            print(f"FAIL {name}: {exc}")
            failures.append(name)
    if failures:
        print(f"\n{len(failures)}/{len(SMOKES)} fallos: {', '.join(failures)}")
        return 1
    print(f"\nOK  orchestrator touchpoint E2E — {len(SMOKES)}/{len(SMOKES)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
