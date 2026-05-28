# -*- coding: utf-8 -*-
"""Tests cumplimiento termodinámico — Fase 5."""

from __future__ import annotations

import json
import os
import tempfile
import unittest
from pathlib import Path

from eda_bus_utils import (
    COMPLIANCE_SUBSCRIBER_KEY,
    RADAMANTO_BATCH_SUBSCRIBER_KEY,
    build_raw_execution_finished_event,
    ensure_fractal_bus_topology,
    load_eda_fractal,
    maybe_purge_fractal_telemetry_when_terminal,
    resolve_ed_telemetry_contract,
    stamp_fractal_delivery_state,
    write_fractal_event,
)
from execute_process_capsules import extract_telemetry_receipt, run_thermodynamic_toll
from route_fractal_event_core import route_telemetry_event
from telemetry_compliance_audit_core import audit_telemetry_compliance


def _fake_repo() -> Path:
    root = Path(tempfile.mkdtemp())
    repo_root = Path(__file__).resolve().parents[2]
    core = root / "SddIA" / "core"
    core.mkdir(parents=True)
    for sub in ("process", "skills", "agents"):
        (root / "SddIA" / sub).mkdir(parents=True, exist_ok=True)
    for name in ("radamanto-batch.md", "telemetry-compliance-audit.md", "route-telemetry.md"):
        src = repo_root / "process" / name
        if src.is_file():
            (root / "SddIA" / "process" / name).write_text(
                src.read_text(encoding="utf-8"), encoding="utf-8"
            )
    src_skill = repo_root / "skills" / "text-metrics.md"
    if src_skill.is_file():
        (root / "SddIA" / "skills" / "text-metrics.md").write_text(
            src_skill.read_text(encoding="utf-8"), encoding="utf-8"
        )
    cumulo = {
        "version": "1.4.0",
        "event_bus": "./.events",
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
        "telemetry_compliance": {
            "emitted_registry": ".SddIA/telemetry-compliance/emitted.json",
        },
    }
    (core / "cumulo.paths.json").write_text(json.dumps(cumulo), encoding="utf-8")
    agents = root / "SddIA" / "agents"
    (agents / "radamanto.thresholds.json").write_text(
        json.dumps({"batch_min_events": 10, "redemption_success_count": 3}),
        encoding="utf-8",
    )
    for fname in (
        "event-telemetry-subscriptions.json",
        "event-orchestration-subscriptions.json",
        "event-domain-subscriptions.json",
    ):
        src = repo_root / "core" / fname
        if src.is_file():
            (core / fname).write_text(src.read_text(encoding="utf-8"), encoding="utf-8")
    ensure_fractal_bus_topology(root)
    return root


class TestTelemetryCompliance(unittest.TestCase):
    def tearDown(self) -> None:
        os.environ.pop("SDDIA_LAB_ROUTE_SYNC", None)

    def test_extract_telemetry_receipt_order(self) -> None:
        self.assertEqual(
            extract_telemetry_receipt({"telemetry_receipt": {"prompt_tokens": 1}}),
            {"prompt_tokens": 1},
        )
        self.assertEqual(
            extract_telemetry_receipt({"data": {"telemetry_receipt": {"prompt_tokens": 2}}}),
            {"prompt_tokens": 2},
        )
        self.assertIsNone(extract_telemetry_receipt({"success": True}))

    def test_resolve_ed_text_metrics(self) -> None:
        repo = _fake_repo()
        c = resolve_ed_telemetry_contract(repo, "text-metrics")
        self.assertTrue(c["telemetry_provided"])
        self.assertIn("prompt_tokens", c["telemetry_schema"] or [])

    def test_thermodynamic_receipt_attached(self) -> None:
        repo = _fake_repo()
        state = {
            "last_capsule_id": "text-metrics",
            "last_capsule_envelope": {
                "telemetry_receipt": {"prompt_tokens": 10, "completion_tokens": 5},
            },
        }
        toll = run_thermodynamic_toll(
            repo,
            "lab",
            state,
            {},
            exit_code=0,
            duration_ms=1,
            success=True,
        )
        path = repo / toll["telemetry"]["target_path"]
        body = json.loads(path.read_text(encoding="utf-8"))
        self.assertEqual(body["payload"]["capsule_id"], "text-metrics")
        self.assertEqual(body["payload"]["telemetry_receipt"]["prompt_tokens"], 10)

    def test_thermodynamic_no_receipt_success(self) -> None:
        repo = _fake_repo()
        toll = run_thermodynamic_toll(
            repo,
            "lab",
            {},
            {},
            exit_code=0,
            duration_ms=1,
            success=True,
        )
        path = repo / toll["telemetry"]["target_path"]
        body = json.loads(path.read_text(encoding="utf-8"))
        self.assertNotIn("telemetry_receipt", body["payload"])

    def test_compliance_breach_missing(self) -> None:
        repo = _fake_repo()
        os.environ["SDDIA_LAB_ROUTE_SYNC"] = "1"
        event = build_raw_execution_finished_event(
            event_id="11111111-2222-4333-8444-555555555555",
            asset_id="22222222-3333-4444-8555-666666666666",
            exit_code=0,
            duration_ms=1,
            process_name="lab",
            capsule_id="text-metrics",
        )
        seal = write_fractal_event(repo, event, "telemetry")
        result = audit_telemetry_compliance(repo, seal["target_path"])
        self.assertTrue(result.get("ok"))
        self.assertEqual(result.get("status"), "breach")
        domain_dir = repo / load_eda_fractal(repo)["domain"]
        self.assertGreaterEqual(len(list(domain_dir.glob("*.json"))), 1)

    def test_compliance_no_breach_when_false(self) -> None:
        repo = _fake_repo()
        event = build_raw_execution_finished_event(
            event_id="33333333-4444-4555-8666-777777777777",
            asset_id="44444444-5555-4666-8777-888888888888",
            exit_code=0,
            duration_ms=1,
            process_name="lab",
            capsule_id="unknown-skill",
        )
        seal = write_fractal_event(repo, event, "telemetry")
        result = audit_telemetry_compliance(repo, seal["target_path"])
        self.assertEqual(result.get("status"), "skipped")
        domain_dir = repo / load_eda_fractal(repo)["domain"]
        self.assertEqual(len(list(domain_dir.glob("*.json"))), 0)

    def test_compliance_schema_mismatch(self) -> None:
        repo = _fake_repo()
        event = build_raw_execution_finished_event(
            event_id="55555555-6666-4777-8888-999999999999",
            asset_id="66666666-7777-4888-8999-aaaaaaaaaaaa",
            exit_code=0,
            duration_ms=1,
            process_name="lab",
            capsule_id="text-metrics",
            telemetry_receipt={"prompt_tokens": 1},
        )
        seal = write_fractal_event(repo, event, "telemetry")
        result = audit_telemetry_compliance(repo, seal["target_path"])
        self.assertEqual(result.get("status"), "breach")

    def test_fan_out_no_competitive_purge(self) -> None:
        repo = _fake_repo()
        event = build_raw_execution_finished_event(
            event_id="77777777-8888-4999-8aaa-bbbbbbbbbbbb",
            asset_id="88888888-9999-4aaa-8bbb-cccccccccccc",
            exit_code=0,
            duration_ms=1,
            process_name="lab",
        )
        seal = write_fractal_event(repo, event, "telemetry")
        rel = seal["target_path"]
        path = repo / rel
        from radamanto_batch_core import process_telemetry_file

        process_telemetry_file(repo, rel)
        self.assertTrue(path.is_file())
        audit_telemetry_compliance(repo, rel)
        self.assertTrue(path.is_file())

    def test_purge_after_all_delivery_stamps(self) -> None:
        repo = _fake_repo()
        event = build_raw_execution_finished_event(
            event_id="99999999-aaaa-4bbb-8ccc-dddddddddddd",
            asset_id="aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee",
            exit_code=0,
            duration_ms=1,
            process_name="lab",
        )
        seal = write_fractal_event(repo, event, "telemetry")
        path = repo / seal["target_path"]
        stamp_fractal_delivery_state(repo, path, RADAMANTO_BATCH_SUBSCRIBER_KEY, "success")
        stamp_fractal_delivery_state(repo, path, COMPLIANCE_SUBSCRIBER_KEY, "success")
        registry = json.loads(
            (repo / "SddIA/core/event-telemetry-subscriptions.json").read_text(encoding="utf-8")
        )
        purged = maybe_purge_fractal_telemetry_when_terminal(
            repo, path, registry, "Raw_Execution_Finished"
        )
        self.assertTrue(purged)
        self.assertFalse(path.is_file())

    def test_route_telemetry_fan_out_purge(self) -> None:
        repo = _fake_repo()
        os.environ["SDDIA_LAB_ROUTE_SYNC"] = "1"
        event = build_raw_execution_finished_event(
            event_id="bbbbbbbb-cccc-4ddd-8eee-ffffffffffff",
            asset_id="cccccccc-dddd-4eee-8fff-000000000000",
            exit_code=0,
            duration_ms=1,
            process_name="lab",
        )
        seal = write_fractal_event(repo, event, "telemetry")
        rel = seal["target_path"]
        out = route_telemetry_event(repo, rel)
        self.assertTrue(out.get("success"), out)
        self.assertFalse((repo / rel).is_file())


if __name__ == "__main__":
    unittest.main()
