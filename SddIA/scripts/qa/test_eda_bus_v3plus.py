# -*- coding: utf-8 -*-
"""Tests topología bus EDA V3+ simétrica."""

from __future__ import annotations

import json
import os
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

from eda_bus_utils import (
    ensure_event_bus_topology,
    ensure_processing_header,
    gh_executable,
    github_pr_merged,
    list_witnesses,
    load_eda_bus,
    maybe_purge_processing_header,
    parse_pr_number,
    resolve_pull_request_lifecycle,
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

    def test_archive_purges_domain_entity_created_processed_header(self) -> None:
        from eda_bus_utils import archive_event_after_sweep, header_path

        repo = _fake_repo()
        bus = ensure_event_bus_topology(repo)
        eid = "dddddddd-eeee-4fff-aaaa-bbbbbbbbbbbb"
        event = {
            "event_id": eid,
            "event_type": "Domain_Entity_Created",
            "payload": {"entity_uuid": "aaaaaaaa-bbbb-4ccc-dddd-eeeeeeeeeeee", "lifecycle_operation": "create"},
        }
        pending = repo / bus["pending"] / f"{eid}.json"
        pending.write_text(json.dumps(event), encoding="utf-8")
        processed = repo / header_path(bus, "processed", eid)
        processed.write_text(json.dumps(event), encoding="utf-8")

        counts = archive_event_after_sweep(repo, bus, eid, event_type="Domain_Entity_Created")

        self.assertFalse(pending.is_file())
        self.assertFalse(processed.is_file())
        self.assertEqual(counts.get("pending"), 1)

    def test_load_eda_bus_respects_event_bus_path_env(self) -> None:
        repo = _fake_repo()
        os.environ["EVENT_BUS_PATH"] = ".tmp/custom_bus"
        try:
            bus = load_eda_bus(repo)
            self.assertEqual(bus["event_bus"], ".tmp/custom_bus")
            self.assertEqual(bus["pending"], ".tmp/custom_bus/pending")
        finally:
            os.environ.pop("EVENT_BUS_PATH", None)

    def test_sweep_purges_when_no_applicable_subscribers(self) -> None:
        from eda_bus_utils import applicable_subscriber_ids_for_event, try_sweep_event

        repo = _fake_repo()
        bus = ensure_event_bus_topology(repo)
        subs = {
            "Domain_Entity_Created": [
                {"agent": "cumulo", "action": "sync-entity-index", "applies_to_origin_topology": ["core"]}
            ]
        }
        (repo / "SddIA" / "core" / "event-subscriptions.json").write_text(
            json.dumps(subs), encoding="utf-8"
        )
        eid = "ffffffff-1111-4222-8333-444444444444"
        event = {
            "event_id": eid,
            "event_type": "Domain_Entity_Created",
            "payload": {"origin_topology": "local", "entity_uuid": "aaaaaaaa-bbbb-4ccc-dddd-eeeeeeeeeeee"},
        }
        pending = repo / bus["pending"] / f"{eid}.json"
        pending.write_text(json.dumps(event), encoding="utf-8")
        applicable = applicable_subscriber_ids_for_event(
            subs, "Domain_Entity_Created", event["payload"]
        )
        self.assertEqual(applicable, [])
        sweep = try_sweep_event(repo, bus, eid, registry=subs)
        self.assertTrue(sweep.get("purged"))
        self.assertEqual(sweep.get("status"), "purged")
        self.assertFalse(pending.is_file())

    def test_archive_purges_non_domain_entity_processed_header(self) -> None:
        from eda_bus_utils import archive_event_after_sweep, header_path

        repo = _fake_repo()
        bus = ensure_event_bus_topology(repo)
        eid = "eeeeeeee-ffff-4aaa-bbbb-cccccccccccc"
        event = {"event_id": eid, "event_type": "PullRequest_Presented", "payload": {}}
        pending = repo / bus["pending"] / f"{eid}.json"
        pending.write_text(json.dumps(event), encoding="utf-8")
        processed = repo / header_path(bus, "processed", eid)
        processed.write_text(json.dumps(event), encoding="utf-8")

        archive_event_after_sweep(repo, bus, eid, event_type="PullRequest_Presented")

        self.assertFalse(pending.is_file())
        self.assertFalse(processed.is_file())


class TestPullRequestLifecycle(unittest.TestCase):
    def test_parse_pr_number(self) -> None:
        self.assertEqual(parse_pr_number("https://github.com/org/repo/pull/48"), 48)
        self.assertIsNone(parse_pr_number("not-a-url"))

    @patch("eda_bus_utils._gh_pr_state", return_value="MERGED")
    @patch("eda_bus_utils._branch_exists_on_remote", return_value=False)
    def test_lifecycle_gh_merged(self, _branch: object, _gh: object) -> None:
        repo = _fake_repo()
        out = resolve_pull_request_lifecycle(
            repo,
            branch="feat/x",
            pr_url="https://github.com/org/repo/pull/1",
        )
        self.assertTrue(out["merged"])
        self.assertEqual(out["source"], "gh")

    @patch("eda_bus_utils._gh_pr_state", return_value=None)
    @patch("eda_bus_utils._branch_exists_on_remote", return_value=False)
    @patch("eda_bus_utils._merged_via_pull_ref", return_value=True)
    def test_lifecycle_git_pull_ref_fallback(self, _pull: object, _branch: object, _gh: object) -> None:
        repo = _fake_repo()
        out = resolve_pull_request_lifecycle(
            repo,
            branch="feat/x",
            pr_url="https://github.com/org/repo/pull/48",
        )
        self.assertTrue(out["merged"])
        self.assertEqual(out["source"], "git-pull-ref")

    @patch("eda_bus_utils._gh_pr_state", return_value="OPEN")
    @patch("eda_bus_utils._branch_exists_on_remote", return_value=False)
    def test_lifecycle_open_branch_absent(self, _branch: object, _gh: object) -> None:
        repo = _fake_repo()
        out = resolve_pull_request_lifecycle(
            repo,
            branch="feat/x",
            pr_url="https://github.com/org/repo/pull/48",
        )
        self.assertIsNone(out["merged"])
        self.assertFalse(out["branch_on_remote"])

    @patch("eda_bus_utils._gh_pr_state", return_value="OPEN")
    @patch("eda_bus_utils._branch_exists_on_remote", return_value=True)
    def test_lifecycle_open_branch_present(self, _branch: object, _gh: object) -> None:
        repo = _fake_repo()
        out = resolve_pull_request_lifecycle(
            repo,
            branch="feat/x",
            pr_url="https://github.com/org/repo/pull/48",
        )
        self.assertFalse(out["merged"])
        self.assertTrue(out["branch_on_remote"])
        self.assertEqual(out["source"], "branch-remote")

    @patch("eda_bus_utils._gh_pr_state", return_value="MERGED")
    def test_github_pr_merged_delegates_to_gh_state(self, _state: object) -> None:
        self.assertTrue(github_pr_merged("https://github.com/org/repo/pull/1"))


if __name__ == "__main__":
    unittest.main()
