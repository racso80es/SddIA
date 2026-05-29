# -*- coding: utf-8 -*-
"""Tests EDA Caos Fase 4 — Suite_Execution_Requested + System_Immunity_Certified."""

from __future__ import annotations

import importlib.util
import json
import os
import tempfile
import unittest
from pathlib import Path

from execute_process_capsules import emit_system_immunity_certified, run_process
from radamanto_batch_core import build_domain_event, emit_domain_and_route

REPO_ROOT = Path(__file__).resolve().parents[3]
DOMAIN_DIR = REPO_ROOT / ".events" / "domain"
_QA = Path(__file__).resolve().parent


def _load_execute_action_module():
    path = _QA / "execute-action.py"
    spec = importlib.util.spec_from_file_location("execute_action_lab", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("no se pudo cargar execute-action.py")
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


_emit_suite_requested = _load_execute_action_module()._run_emit_suite_execution_requested


def _domain_events_by_type(event_type: str) -> list[dict]:
    found: list[dict] = []
    if not DOMAIN_DIR.is_dir():
        return found
    for path in DOMAIN_DIR.glob("*.json"):
        try:
            body = json.loads(path.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError):
            continue
        if body.get("event_type") == event_type:
            found.append(body)
    return found


def _fake_repo() -> Path:
    root = Path(tempfile.mkdtemp())
    core = root / "SddIA" / "core"
    process_dir = root / "SddIA" / "process"
    events_domain = root / "SddIA" / "events" / "domain"
    core.mkdir(parents=True)
    process_dir.mkdir(parents=True)
    events_domain.mkdir(parents=True)
    repo_root = Path(__file__).resolve().parents[2]
    for name in ("cerbero-governance-react.md", "route-domain.md"):
        src = repo_root / "process" / name
        if src.is_file():
            (process_dir / name).write_text(src.read_text(encoding="utf-8"), encoding="utf-8")
    for src in (repo_root / "events" / "domain").glob("*.md"):
        if src.name != "index.md":
            (events_domain / src.name).write_text(src.read_text(encoding="utf-8"), encoding="utf-8")
    (core / "cumulo.paths.json").write_text(
        json.dumps(
            {
                "version": "1.3.0",
                "eda_fractal": {
                    "domain": "./.events/domain",
                    "domain_subscriptions": "SddIA/core/event-domain-subscriptions.json",
                },
                "radamanto": {
                    "revoked_entities": ".SddIA/cerbero/revoked_entities.json",
                    "thresholds": "SddIA/agents/radamanto.thresholds.json",
                    "stats": ".SddIA/radamanto/stats.json",
                    "consumed": ".SddIA/radamanto/consumed.json",
                },
            }
        ),
        encoding="utf-8",
    )
    src_subs = repo_root / "core" / "event-domain-subscriptions.json"
    if src_subs.is_file():
        (core / "event-domain-subscriptions.json").write_text(
            src_subs.read_text(encoding="utf-8"), encoding="utf-8"
        )
    (root / "SddIA" / "agents" / "radamanto.thresholds.json").parent.mkdir(parents=True)
    (root / "SddIA" / "agents" / "radamanto.thresholds.json").write_text("{}", encoding="utf-8")
    return root


class TestChaosImmunityEda(unittest.TestCase):
    def test_emit_suite_execution_requested_writes_domain_event(self) -> None:
        before = len(_domain_events_by_type("Suite_Execution_Requested"))
        out = _emit_suite_requested(
            REPO_ROOT,
            {"suite_id": "core-full-stress"},
            {},
        )
        self.assertTrue(out.get("success"))
        self.assertEqual(out.get("event_type"), "Suite_Execution_Requested")
        target = REPO_ROOT / str(out.get("target_path", ""))
        self.assertTrue(target.is_file())
        body = json.loads(target.read_text(encoding="utf-8"))
        self.assertEqual(body.get("payload", {}).get("suite_id"), "core-full-stress")
        after = len(_domain_events_by_type("Suite_Execution_Requested"))
        self.assertGreater(after, before)

    def test_emit_suite_execution_requested_rejects_missing_suite(self) -> None:
        with self.assertRaises(ValueError):
            _emit_suite_requested(REPO_ROOT, {"suite_id": "no-such-suite-xyz"}, {})

    def test_immunity_emitted_on_execute_suite_success(self) -> None:
        before = len(_domain_events_by_type("System_Immunity_Certified"))
        out = run_process(REPO_ROOT, "execute-suite", {"suite_id": "core-full-stress"})
        self.assertTrue(out.get("success"), out)
        data = out.get("data") or {}
        self.assertIsInstance(data.get("immunity_event_id"), str)
        after = len(_domain_events_by_type("System_Immunity_Certified"))
        self.assertGreater(after, before)
        phases = (out.get("execution_report") or {}).get("phases") or []
        cert_phases = [p for p in phases if p.get("phase_name") == "Certificación inmunidad"]
        self.assertEqual(len(cert_phases), 1)
        self.assertEqual(cert_phases[0].get("status"), "executed")

    def test_immunity_not_emitted_on_execute_suite_fail(self) -> None:
        before_ids = {
            e.get("event_id")
            for e in _domain_events_by_type("System_Immunity_Certified")
        }
        out = run_process(REPO_ROOT, "execute-suite", {"suite_id": "fail-fast-lab"})
        self.assertFalse(out.get("success"))
        data = out.get("data") or {}
        self.assertNotIn("immunity_event_id", data)
        after = _domain_events_by_type("System_Immunity_Certified")
        new_ids = {e.get("event_id") for e in after} - before_ids
        self.assertEqual(len(new_ids), 0)

    def test_immunity_certified_radamanto_dlt_witness(self) -> None:
        repo = _fake_repo()
        os.environ["SDDIA_LAB_ROUTE_SYNC"] = "1"
        os.environ["SDDIA_LAB_SIMULATE_IOTA"] = "1"
        try:
            ev = build_domain_event(
                "System_Immunity_Certified",
                {
                    "suite_id": "core-full-stress",
                    "survival_manifest_path": ".SddIA/workspaces/lab/manifest.md",
                    "orchestrator_execution_id": "lab-exec-id",
                    "nodes_passed": 3,
                    "nodes_total": 3,
                },
            )
            out = emit_domain_and_route(repo, ev)
            route = out.get("route") or {}
            self.assertTrue(route.get("success"), route)
            delivery = (route.get("data") or {}).get("delivery_status") or {}
            self.assertTrue(any("radamanto" in k for k in delivery))
        finally:
            os.environ.pop("SDDIA_LAB_ROUTE_SYNC", None)
            os.environ.pop("SDDIA_LAB_SIMULATE_IOTA", None)

    def test_emit_system_immunity_certified_helper(self) -> None:
        repo = _fake_repo()
        manifest = repo / "survival-manifest.md"
        manifest.write_text("# test\n", encoding="utf-8")
        out = emit_system_immunity_certified(
            repo,
            suite_id="lab",
            survival_manifest_path="survival-manifest.md",
            orchestrator_execution_id="exec-1",
            node_reports=[{"verdict": "pass"}, {"verdict": "pass"}],
        )
        self.assertIn("event_id", out)
        path = repo / str((out.get("seal") or {}).get("target_path", ""))
        self.assertTrue(path.is_file())
        body = json.loads(path.read_text(encoding="utf-8"))
        self.assertEqual(body.get("event_type"), "System_Immunity_Certified")
        self.assertEqual(body.get("payload", {}).get("nodes_passed"), 2)


if __name__ == "__main__":
    unittest.main()
