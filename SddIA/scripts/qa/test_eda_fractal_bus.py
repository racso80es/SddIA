# -*- coding: utf-8 -*-
"""Tests bus EDA fractal (Fase 3): telemetría, Peaje Termodinámico y stub Radamanto."""

from __future__ import annotations

import json
import os
import tempfile
import unittest
from pathlib import Path

from eda_bus_utils import (
    build_raw_execution_finished_event,
    ensure_fractal_bus_topology,
    load_eda_fractal,
    write_fractal_event,
)
from execute_process_capsules import run_process
from route_fractal_event_core import route_telemetry_event


def _fake_repo() -> Path:
    root = Path(tempfile.mkdtemp())
    core = root / "SddIA" / "core"
    core.mkdir(parents=True)
    process_dir = root / "SddIA" / "process"
    process_dir.mkdir(parents=True)
    for name in (
        "telemetry-batch-stub.md",
        "route-telemetry.md",
        "workspace-smoke.md",
    ):
        src = Path(__file__).resolve().parents[2] / "process" / name
        if src.is_file():
            (process_dir / name).write_text(src.read_text(encoding="utf-8"), encoding="utf-8")
    cumulo = {
        "version": "1.2.0",
        "event_bus": "./.events",
        "eda_bus": {
            "pending": "./.events/pending",
            "processing": "./.events/processing",
            "processed": "./.events/processed",
            "dead_letter": "./.events/dead-letter",
            "subscriptions": "SddIA/core/event-domain-subscriptions.json",
        },
        "eda_fractal": {
            "telemetry": "./.events/telemetry",
            "orchestration": "./.events/orchestration",
            "domain": "./.events/domain",
            "telemetry_subscriptions": "SddIA/core/event-telemetry-subscriptions.json",
            "orchestration_subscriptions": "SddIA/core/event-orchestration-subscriptions.json",
            "domain_subscriptions": "SddIA/core/event-domain-subscriptions.json",
        },
        "paths": {"workspacesRoot": ".SddIA/workspaces/"},
    }
    (core / "cumulo.paths.json").write_text(json.dumps(cumulo), encoding="utf-8")
    repo_root = Path(__file__).resolve().parents[2]
    for fname in (
        "event-telemetry-subscriptions.json",
        "event-orchestration-subscriptions.json",
        "event-domain-subscriptions.json",
    ):
        src = repo_root / "core" / fname
        if src.is_file():
            (core / fname).write_text(src.read_text(encoding="utf-8"), encoding="utf-8")
    (root / ".SddIA" / "workspaces").mkdir(parents=True, exist_ok=True)
    return root


class TestEdaFractalBus(unittest.TestCase):
    def test_fractal_topology_bootstrap(self) -> None:
        repo = _fake_repo()
        fractal = ensure_fractal_bus_topology(repo)
        for key in ("telemetry", "orchestration", "domain"):
            self.assertTrue((repo / fractal[key]).is_dir(), key)

    def test_write_telemetry_event(self) -> None:
        repo = _fake_repo()
        event = build_raw_execution_finished_event(
            event_id="aaaaaaaa-bbbb-4ccc-dddd-eeeeeeeeeeee",
            asset_id="bbbbbbbb-cccc-4ddd-eeee-ffffffffffff",
            exit_code=0,
            duration_ms=42,
            process_name="workspace-smoke",
        )
        seal = write_fractal_event(repo, event, "telemetry")
        path = repo / seal["target_path"]
        self.assertTrue(path.is_file())
        body = json.loads(path.read_text(encoding="utf-8"))
        self.assertEqual(body["event_type"], "Raw_Execution_Finished")
        self.assertEqual(body["event_family"], "telemetry")

    def test_thermodynamic_toll_on_workspace_smoke(self) -> None:
        repo = _fake_repo()
        os.environ["SDDIA_LAB_SKIP_GIT"] = "1"
        try:
            out = run_process(repo, "workspace-smoke", {})
        finally:
            os.environ.pop("SDDIA_LAB_SKIP_GIT", None)
        self.assertTrue(out.get("success"), out)
        toll = (out.get("data") or {}).get("thermodynamic_toll") or {}
        telemetry = toll.get("telemetry") or {}
        self.assertIn("target_path", telemetry)
        telemetry_path = repo / telemetry["target_path"]
        self.assertTrue(telemetry_path.is_file())
        orch_dir = repo / load_eda_fractal(repo)["orchestration"]
        orch_files = list(orch_dir.glob("*.json"))
        self.assertEqual(len(orch_files), 1)

    def test_telemetry_route_and_stub_purge(self) -> None:
        repo = _fake_repo()
        os.environ["SDDIA_LAB_ROUTE_SYNC"] = "1"
        try:
            event = build_raw_execution_finished_event(
                event_id="cccccccc-dddd-4eee-ffff-000011112222",
                asset_id="dddddddd-eeee-4fff-aaaa-bbbbbbbbbbbb",
                exit_code=0,
                duration_ms=10,
                process_name="lab",
            )
            seal = write_fractal_event(repo, event, "telemetry")
            rel = seal["target_path"]
            out = route_telemetry_event(repo, rel)
            self.assertTrue(out.get("success"), out)
            self.assertFalse((repo / rel).is_file())
        finally:
            os.environ.pop("SDDIA_LAB_ROUTE_SYNC", None)

    def test_no_telemetry_in_orchestration_path(self) -> None:
        repo = _fake_repo()
        event = build_raw_execution_finished_event(
            event_id="eeeeeeee-ffff-4aaa-bbbb-cccccccccccc",
            asset_id="ffffffff-aaaa-4bbb-cccc-dddddddddddd",
            exit_code=0,
            duration_ms=5,
            process_name="lab",
        )
        write_fractal_event(repo, event, "telemetry")
        fractal = load_eda_fractal(repo)
        self.assertEqual(len(list((repo / fractal["telemetry"]).glob("*.json"))), 1)
        self.assertEqual(len(list((repo / fractal["orchestration"]).glob("*.json"))), 0)
        self.assertEqual(len(list((repo / fractal["domain"]).glob("*.json"))), 0)


if __name__ == "__main__":
    unittest.main()
