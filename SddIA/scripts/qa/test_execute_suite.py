# -*- coding: utf-8 -*-
"""Tests orquestador execute-suite — Fase 3 Caos."""

from __future__ import annotations

import unittest
from pathlib import Path

from execute_process_capsules import (
    CREATOR_BY_CLASS,
    PILOT_ENTITY_CLASSES,
    creator_inputs_from_entity,
    load_suite_spec,
    run_process,
)
from execute_process_core import parse_frontmatter

REPO_ROOT = Path(__file__).resolve().parents[3]
SUITES_DIR = REPO_ROOT / "SddIA" / "suites"


class TestExecuteSuite(unittest.TestCase):
    def test_entity_manager_accepts_suite_class(self) -> None:
        self.assertIn("suite", PILOT_ENTITY_CLASSES)
        self.assertEqual(CREATOR_BY_CLASS.get("suite"), "suite-creator")
        inputs = creator_inputs_from_entity(
            "suite",
            "lab-suite-smoke",
            "create",
            {
                "execution_strategy": "run_all",
                "atomic_nodes": [
                    {
                        "process_name": "audit-sandbox-isolation-rbac",
                        "expected_exit_code": 0,
                    }
                ],
            },
        )
        self.assertEqual(inputs["suite_name"], "lab-suite-smoke")
        self.assertEqual(inputs["execution_strategy"], "run_all")
        self.assertEqual(len(inputs["atomic_nodes"]), 1)

    def test_core_full_stress_suite_spec_valid(self) -> None:
        spec = load_suite_spec(REPO_ROOT, "core-full-stress")
        self.assertEqual(spec.get("execution_strategy"), "run_all")
        nodes = spec.get("atomic_nodes") or []
        self.assertEqual(len(nodes), 3)
        names = {n.get("process_name") for n in nodes if isinstance(n, dict)}
        self.assertEqual(
            names,
            {
                "audit-thermodynamic-toll-failsoft",
                "audit-telemetry-compliance-breach",
                "audit-sandbox-isolation-rbac",
            },
        )
        path = SUITES_DIR / "core-full-stress.md"
        meta = parse_frontmatter(path)
        self.assertIn("hash_signature", meta)

    def test_execute_suite_core_full_stress_smoke(self) -> None:
        out = run_process(REPO_ROOT, "execute-suite", {"suite_id": "core-full-stress"})
        self.assertTrue(out.get("success"), out)
        self.assertEqual(out.get("status_code"), 0)
        data = out.get("data") or {}
        manifest = data.get("survival_manifest_path")
        self.assertIsInstance(manifest, str)
        self.assertTrue((REPO_ROOT / manifest).is_file())
        self.assertEqual(data.get("nodes_executed"), 3)
        self.assertEqual(data.get("execution_strategy"), "run_all")

    def test_execute_suite_isolated_sub_workspaces(self) -> None:
        out = run_process(REPO_ROOT, "execute-suite", {"suite_id": "core-full-stress"})
        self.assertTrue(out.get("success"), out)
        nodes = (out.get("execution_report") or {}).get("nodes") or []
        self.assertEqual(len(nodes), 3)
        paths = {n.get("workspace_path") for n in nodes}
        self.assertEqual(len(paths), 3)
        orchestrator_ws = (out.get("data") or {}).get("workspace_path")
        self.assertIsInstance(orchestrator_ws, str)
        for node in nodes:
            ws = node.get("workspace_path")
            self.assertIsInstance(ws, str)
            self.assertIn("/nodes/", ws.replace("\\", "/"))
            self.assertNotEqual(ws, orchestrator_ws)

    def test_execute_suite_fail_fast_aborts_after_first_fail(self) -> None:
        out = run_process(REPO_ROOT, "execute-suite", {"suite_id": "fail-fast-lab"})
        self.assertFalse(out.get("success"))
        self.assertEqual(out.get("status_code"), 1)
        data = out.get("data") or {}
        self.assertEqual(data.get("execution_strategy"), "fail_fast")
        self.assertEqual(data.get("nodes_executed"), 1)
        nodes = (out.get("execution_report") or {}).get("nodes") or []
        self.assertEqual(len(nodes), 1)
        self.assertEqual(nodes[0].get("verdict"), "fail")
        self.assertEqual(nodes[0].get("actual_exit_code"), 0)
        self.assertEqual(nodes[0].get("expected_exit_code"), 99)
        manifest = data.get("survival_manifest_path")
        self.assertIsInstance(manifest, str)
        self.assertTrue((REPO_ROOT / manifest).is_file())


if __name__ == "__main__":
    unittest.main()
