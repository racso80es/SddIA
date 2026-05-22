# -*- coding: utf-8 -*-
"""Tests topología bus EDA V3+ simétrica."""

from __future__ import annotations

import json
import os
import tempfile
import unittest
from pathlib import Path

from eda_bus_utils import (
    ensure_event_bus_topology,
    ensure_processing_header,
    list_witnesses,
    load_eda_bus,
    maybe_purge_processing_header,
    terminal_witness_exists,
    write_processing_witness,
)


def _fake_repo() -> Path:
    root = Path(tempfile.mkdtemp())
    (root / "SddIA" / "core").mkdir(parents=True)
    cumulo = {
        "event_bus": "./.events",
        "eda_bus": {
            "pending": "./.events/pending",
            "processing": "./.events/processing",
            "processed": "./.events/processed",
            "dead_letter": "./.events/dead-letter",
            "subscriptions": "SddIA/core/event-subscriptions.json",
        },
    }
    (root / "SddIA" / "core" / "cumulo.paths.json").write_text(
        json.dumps(cumulo), encoding="utf-8"
    )
    (root / "SddIA" / "core" / "event-subscriptions.json").write_text("{}", encoding="utf-8")
    return root


class TestEdaBusV3Plus(unittest.TestCase):
    def test_bootstrap_creates_symmetric_tree(self) -> None:
        repo = _fake_repo()
        bus = ensure_event_bus_topology(repo)
        for key in (
            "pending",
            "processing",
            "processing_subscribers",
            "processed",
            "processed_subscribers",
            "dead_letter",
            "dead_letter_subscribers",
        ):
            self.assertTrue((repo / bus[key]).is_dir(), key)

    def test_processing_header_copy(self) -> None:
        repo = _fake_repo()
        bus = ensure_event_bus_topology(repo)
        event = {"event_id": "aaaaaaaa-bbbb-4ccc-dddd-eeeeeeeeeeee", "event_type": "Test"}
        pending = repo / bus["pending"] / f"{event['event_id']}.json"
        pending.write_text(json.dumps(event), encoding="utf-8")
        header = ensure_processing_header(repo, bus, event["event_id"], pending)
        self.assertTrue(header.is_file())
        self.assertEqual(json.loads(header.read_text()), event)

    def test_purge_processing_when_all_terminal(self) -> None:
        repo = _fake_repo()
        bus = ensure_event_bus_topology(repo)
        eid = "bbbbbbbb-cccc-4ddd-eeee-ffffffffffff"
        pending = repo / bus["pending"] / f"{eid}.json"
        pending.write_text("{}", encoding="utf-8")
        ensure_processing_header(repo, bus, eid, pending)
        registry = {"T": [{"agent": "cumulo", "action": "sync-entity-index"}]}
        write_processing_witness(
            repo, bus, event_uuid=eid, subscriber_name="cumulo.sync-entity-index", event_type="T"
        )
        from eda_bus_utils import promote_witness

        promote_witness(
            repo,
            bus,
            event_uuid=eid,
            subscriber_name="cumulo.sync-entity-index",
            to_state="processed",
            pending_header=pending,
        )
        purged = maybe_purge_processing_header(repo, bus, eid, registry, "T", "core")
        self.assertTrue(purged)
        self.assertFalse((repo / bus["processing"] / f"{eid}.json").is_file())

    def test_terminal_witness_idempotent(self) -> None:
        repo = _fake_repo()
        bus = ensure_event_bus_topology(repo)
        eid = "cccccccc-dddd-4eee-ffff-000000000001"
        write_processing_witness(
            repo, bus, event_uuid=eid, subscriber_name="x", event_type="T", dispatch_mode="async"
        )
        from eda_bus_utils import promote_witness

        promote_witness(repo, bus, event_uuid=eid, subscriber_name="x", to_state="processed")
        self.assertTrue(terminal_witness_exists(repo, bus, eid, "x"))
        self.assertEqual(len(list_witnesses(repo, bus, "processing_subscribers", eid)), 0)


if __name__ == "__main__":
    unittest.main()
