# -*- coding: utf-8 -*-
"""Tests procesos audit Caos — Fase 2."""

from __future__ import annotations

import json
import unittest
from pathlib import Path

from execute_process_capsules import (
    CHAOS_AUDIT_PROCESSES,
    CHAOS_OFFENSIVE_TOOLS,
    run_process,
)
from execute_process_core import load_process_def, parse_frontmatter

REPO_ROOT = Path(__file__).resolve().parents[3]
PROCESS_DIR = REPO_ROOT / "SddIA" / "process"


class TestChaosAuditProcesses(unittest.TestCase):
    def test_audit_thermodynamic_toll_failsoft_exit_zero(self) -> None:
        out = run_process(REPO_ROOT, "audit-thermodynamic-toll-failsoft", {})
        self.assertTrue(out.get("success"), out)
        self.assertEqual(out.get("status_code"), 0)
        data = out.get("data") or {}
        toll = data.get("thermodynamic_toll") or {}
        self.assertTrue(toll.get("telemetry_io_failed"))
        self.assertTrue(data.get("toll_failsoft_verified"))

    def test_audit_telemetry_compliance_breach_event(self) -> None:
        out = run_process(REPO_ROOT, "audit-telemetry-compliance-breach", {})
        self.assertTrue(out.get("success"), out)
        data = out.get("data") or {}
        breach_path = data.get("breach_event_path")
        self.assertIsInstance(breach_path, str)
        self.assertTrue(breach_path.strip())
        body = json.loads((REPO_ROOT / breach_path).read_text(encoding="utf-8"))
        self.assertEqual(body.get("event_type"), "Telemetry_Compliance_Breached")

    def test_audit_sandbox_isolation_blocks_escape(self) -> None:
        out = run_process(REPO_ROOT, "audit-sandbox-isolation-rbac", {})
        self.assertTrue(out.get("success"), out)
        data = out.get("data") or {}
        self.assertTrue(data.get("isolation_verified"))
        phases = (out.get("execution_report") or {}).get("phases") or []
        stimulus = next(p for p in phases if p.get("tool") == "sandbox-breacher")
        self.assertEqual(stimulus.get("status"), "executed")

    def test_chaos_audit_atomicity_one_tool_each(self) -> None:
        for name in sorted(CHAOS_AUDIT_PROCESSES):
            _, process_def, _ = load_process_def(REPO_ROOT, name)
            tools: set[str] = set()
            for phase in process_def.get("phases") or []:
                if not isinstance(phase, dict):
                    continue
                for delegate in phase.get("delegates_to") or []:
                    if isinstance(delegate, str) and delegate.startswith("tool:"):
                        tools.add(delegate.split(":", 1)[1])
            offensive = tools & CHAOS_OFFENSIVE_TOOLS
            self.assertEqual(len(offensive), 1, f"{name} debe tener exactamente una tool caos")

    def test_chaos_audit_processes_have_workspace_template(self) -> None:
        for name in sorted(CHAOS_AUDIT_PROCESSES):
            path = PROCESS_DIR / f"{name}.md"
            self.assertTrue(path.is_file(), f"falta {path}")
            meta = parse_frontmatter(path)
            self.assertIn("workspace_template", meta)
            self.assertIn("{execution_id}", meta["workspace_template"])


if __name__ == "__main__":
    unittest.main()
