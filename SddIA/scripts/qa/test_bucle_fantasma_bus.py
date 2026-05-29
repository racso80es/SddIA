# -*- coding: utf-8 -*-
"""Tests [ARQUITECTURA] fix-bucle-fantasma-sistema-nervioso."""

from __future__ import annotations

import json
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

_QA = Path(__file__).resolve().parent
if str(_QA) not in sys.path:
    sys.path.insert(0, str(_QA))

from eda_bus_utils import safe_remove_path


def _minimal_cumulo(root: Path) -> None:
    core = root / "SddIA" / "core"
    core.mkdir(parents=True)
    cumulo = {
        "version": "1.2.0",
        "event_bus": "./.events",
        "eda_bus": {
            "pending": "./.events/pending",
            "processing": "./.events/processing",
            "processed": "./.events/processed",
            "dead_letter": "./.events/dead-letter",
            "processing_subscribers": "./.events/processing/subscribers",
            "processed_subscribers": "./.events/processed/subscribers",
            "dead_letter_subscribers": "./.events/dead-letter/subscribers",
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
    }
    (core / "cumulo.paths.json").write_text(json.dumps(cumulo), encoding="utf-8")
    repo_src = Path(__file__).resolve().parents[2] / "core"
    for fname in (
        "event-domain-subscriptions.json",
        "event-telemetry-subscriptions.json",
        "event-orchestration-subscriptions.json",
    ):
        src = repo_src / fname
        if src.is_file():
            (core / fname).write_text(src.read_text(encoding="utf-8"), encoding="utf-8")


class TestSafeRemovePath(unittest.TestCase):
    def test_safe_remove_missing_is_true(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "gone.json"
            self.assertTrue(safe_remove_path(path))

    def test_safe_remove_retries_then_succeeds(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "x.json"
            path.write_text("{}", encoding="utf-8")
            calls = {"n": 0}
            original_unlink = Path.unlink

            def flaky_unlink(self: Path, missing_ok: bool = False) -> None:
                calls["n"] += 1
                if calls["n"] < 3:
                    raise PermissionError("locked")
                original_unlink(self)

            with patch.object(Path, "unlink", flaky_unlink):
                with patch("eda_bus_utils.time.sleep"):
                    ok = safe_remove_path(path, retries=3, delay_s=0.0)
            self.assertTrue(ok)
            self.assertFalse(path.is_file())
            self.assertEqual(calls["n"], 3)


class TestWatcherIdempotency(unittest.TestCase):
    def test_skip_in_flight_and_routed_ok(self) -> None:
        daemon_dir = Path(__file__).resolve().parents[1] / "daemons"
        sys.path.insert(0, str(daemon_dir.parent / "qa"))
        import importlib.util

        spec = importlib.util.spec_from_file_location(
            "event_watcher_under_test",
            daemon_dir / "event-watcher.py",
        )
        assert spec and spec.loader
        mod = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(mod)

        with tempfile.TemporaryDirectory() as tmp:
            repo = Path(tmp)
            _minimal_cumulo(repo)
            bus = mod.ensure_event_bus_topology(repo)
            domain = repo / ".events" / "domain"
            domain.mkdir(parents=True)
            event_id = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee"
            path = domain / f"{event_id}.json"
            path.write_text(
                json.dumps({"event_id": event_id, "event_type": "X"}),
                encoding="utf-8",
            )
            key = f"domain/{path.name}"
            processing: set[str] = set()
            routed_ok: set[str] = set()
            attempts: dict[str, int] = {}

            processing.add(event_id)
            reason = mod._watcher_skip_reason(
                event_uuid=event_id,
                key=key,
                process_name="route-domain",
                path=path,
                processing_uuids=processing,
                routed_ok_pending_absent=routed_ok,
                repo=repo,
                bus=bus,
                attempts=attempts,
            )
            self.assertIsNotNone(reason)
            self.assertIn("in-flight", reason or "")

            processing.discard(event_id)
            routed_ok.add(event_id)
            reason2 = mod._watcher_skip_reason(
                event_uuid=event_id,
                key=key,
                process_name="route-domain",
                path=path,
                processing_uuids=processing,
                routed_ok_pending_absent=routed_ok,
                repo=repo,
                bus=bus,
                attempts=attempts,
            )
            self.assertIsNotNone(reason2)
            self.assertIn("routed-ok", reason2 or "")


class TestPurgeStaleDryRun(unittest.TestCase):
    def test_detects_delivery_complete(self) -> None:
        from purge_stale_events import purge_stale_events

        with tempfile.TemporaryDirectory() as tmp:
            repo = Path(tmp)
            _minimal_cumulo(repo)
            fractal_dir = repo / ".events" / "telemetry"
            fractal_dir.mkdir(parents=True)
            eid = "11111111-2222-4333-8444-555555555555"
            body = {
                "event_id": eid,
                "event_type": "Raw_Execution_Finished",
                "timestamp": "2020-01-01T00:00:00Z",
                "delivery_state": {
                    "radamanto.radamanto-batch": "success",
                    "argos.telemetry-compliance-audit": "success",
                },
            }
            (fractal_dir / f"{eid}.json").write_text(json.dumps(body), encoding="utf-8")
            report = purge_stale_events(repo, apply=False)
            self.assertGreaterEqual(report["candidate_count"], 1)
            ids = {c["event_id"] for c in report["candidates"]}
            self.assertIn(eid, ids)


if __name__ == "__main__":
    unittest.main()
