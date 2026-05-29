# -*- coding: utf-8 -*-
"""Tests Radamanto Self-Healing (Fase 4)."""

from __future__ import annotations

import json
import os
import tempfile
import unittest
import uuid
from pathlib import Path

from cerbero_governance_react_core import is_entity_revoked, load_revoked
from eda_bus_utils import build_raw_execution_finished_event, write_fractal_event
from fix_tool_process_core import assert_sandbox_write, process_fix_tool
from radamanto_batch_core import build_domain_event, process_telemetry_file, load_stats
from route_fractal_event_core import route_telemetry_event


def _fake_repo(*, batch_min: int = 3, redemption: int = 2) -> Path:
    root = Path(tempfile.mkdtemp())
    core = root / "SddIA" / "core"
    agents = root / "SddIA" / "agents"
    process_dir = root / "SddIA" / "process"
    for d in (core, agents, process_dir):
        d.mkdir(parents=True)
    repo_root = Path(__file__).resolve().parents[2]
    for name in (
        "radamanto-batch.md",
        "cerbero-governance-react.md",
        "fix-tool-process.md",
        "route-telemetry.md",
        "route-domain.md",
    ):
        src = repo_root / "process" / name
        if src.is_file():
            (process_dir / name).write_text(src.read_text(encoding="utf-8"), encoding="utf-8")
    thresholds = {
        "version": "1.0.0",
        "success_rate_min": 0.85,
        "batch_min_events": batch_min,
        "redemption_success_count": redemption,
        "max_recovery_attempts": 3,
        "abrupt_drop_min_samples": 3,
    }
    (agents / "radamanto.thresholds.json").write_text(json.dumps(thresholds), encoding="utf-8")
    cumulo = {
        "version": "1.3.0",
        "eda_fractal": {
            "telemetry": "./.events/telemetry",
            "orchestration": "./.events/orchestration",
            "domain": "./.events/domain",
            "telemetry_subscriptions": "SddIA/core/event-telemetry-subscriptions.json",
            "orchestration_subscriptions": "SddIA/core/event-orchestration-subscriptions.json",
            "domain_subscriptions": "SddIA/core/event-domain-subscriptions.json",
        },
        "radamanto": {
            "stats": ".SddIA/radamanto/stats.json",
            "consumed": ".SddIA/radamanto/consumed.json",
            "thresholds": "SddIA/agents/radamanto.thresholds.json",
            "sandbox_root": ".SddIA/sandbox/",
            "revoked_entities": ".SddIA/cerbero/revoked_entities.json",
        },
    }
    (core / "cumulo.paths.json").write_text(json.dumps(cumulo), encoding="utf-8")
    for fname in (
        "event-telemetry-subscriptions.json",
        "event-domain-subscriptions.json",
    ):
        src = repo_root / "core" / fname
        if src.is_file():
            (core / fname).write_text(src.read_text(encoding="utf-8"), encoding="utf-8")
    for sub in (".events/telemetry", ".events/domain", ".SddIA/radamanto", ".SddIA/sandbox"):
        (root / sub).mkdir(parents=True, exist_ok=True)
    return root


def _emit_telemetry(repo: Path, entity: str, exit_code: int) -> str:
    event = build_raw_execution_finished_event(
        event_id=str(uuid.uuid4()),
        asset_id=str(uuid.uuid4()),
        exit_code=exit_code,
        duration_ms=10,
        process_name=entity,
    )
    event["payload"]["capsule_id"] = entity
    seal = write_fractal_event(repo, event, "telemetry")
    return seal["target_path"]


class TestRadamantoSelfHealing(unittest.TestCase):
    def test_full_self_healing_cycle(self) -> None:
        repo = _fake_repo(batch_min=3, redemption=2)
        entity = "skill:lab-test"
        os.environ["SDDIA_LAB_ROUTE_SYNC"] = "1"
        os.environ["SDDIA_LAB_SIMULATE_IOTA"] = "1"
        try:
            for _ in range(3):
                rel = _emit_telemetry(repo, entity, 1)
                route_telemetry_event(repo, rel)
            self.assertTrue(is_entity_revoked(repo, entity))
            stats = load_stats(repo)
            self.assertEqual(stats["entities"][entity]["status"], "pending_redemption")
            for _ in range(2):
                rel = _emit_telemetry(repo, entity, 0)
                route_telemetry_event(repo, rel)
            self.assertFalse(is_entity_revoked(repo, entity))
            stats = load_stats(repo)
            self.assertEqual(stats["entities"][entity]["status"], "healthy")
            domain_files = list((repo / ".events" / "domain").glob("*.json"))
            types = []
            for p in domain_files:
                types.append(json.loads(p.read_text(encoding="utf-8")).get("event_type"))
            self.assertIn("Domain_Entity_Degraded", types)
            self.assertIn("Domain_Entity_Restored", types)
        finally:
            os.environ.pop("SDDIA_LAB_ROUTE_SYNC", None)
            os.environ.pop("SDDIA_LAB_SIMULATE_IOTA", None)

    def test_argos_does_not_emit_status_restored(self) -> None:
        repo = _fake_repo()
        ev = build_domain_event(
            "Domain_Entity_Degraded",
            {
                "entity_type": "skill",
                "entity_id": "skill:x",
                "reason": "test",
                "success_rate": 0.5,
                "recovery_attempt": 1,
            },
        )
        seal = write_fractal_event(repo, ev, "domain")
        out = process_fix_tool(repo, seal["target_path"])
        self.assertTrue(out.get("ok"))
        self.assertFalse(out.get("status_restored_emitted"))
        gate = out.get("argos_gate") or {}
        self.assertTrue(gate.get("structure_valid"))
        self.assertFalse(gate.get("emits_status_restored"))

    def test_sandbox_blocks_production_write(self) -> None:
        repo = _fake_repo()
        sandbox = repo / ".SddIA" / "sandbox" / "skill_x" / "1"
        sandbox.mkdir(parents=True)
        prod = repo / "SddIA" / "tools" / "blocked.md"
        prod.parent.mkdir(parents=True)
        ok, err = assert_sandbox_write(repo, prod, sandbox)
        self.assertFalse(ok)
        self.assertIn("produccion", err or "")


class TestRadamantoMaxRecovery(unittest.TestCase):
    def test_deprecated_after_max_attempts(self) -> None:
        repo = _fake_repo(batch_min=3, redemption=2)
        entity = "skill:doomed"
        os.environ["SDDIA_LAB_ROUTE_SYNC"] = "1"
        try:
            for _ in range(3):
                rel = _emit_telemetry(repo, entity, 1)
                route_telemetry_event(repo, rel)
            stats = load_stats(repo)
            stats["entities"][entity]["recovery_attempts"] = 3
            stats["entities"][entity]["status"] = "pending_redemption"
            stats["entities"][entity]["structure_valid"] = True
            from radamanto_batch_core import save_stats

            save_stats(repo, stats)
            rel = _emit_telemetry(repo, entity, 1)
            result = process_telemetry_file(repo, rel)
            self.assertTrue(result.get("ok"))
            action_types = [a.get("type") for a in result.get("actions") or []]
            self.assertIn("Domain_Entity_Deprecated", action_types)
            revoked = load_revoked(repo)
            self.assertIn(entity, revoked.get("permanent", {}))
        finally:
            os.environ.pop("SDDIA_LAB_ROUTE_SYNC", None)


if __name__ == "__main__":
    unittest.main()
