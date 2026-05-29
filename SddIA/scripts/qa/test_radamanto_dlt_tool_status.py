# -*- coding: utf-8 -*-
"""Tests DLT Radamanto ventana dual (Fase 4)."""

from __future__ import annotations

import json
import os
import tempfile
import unittest
from pathlib import Path

from radamanto_batch_core import build_domain_event, emit_domain_and_route


def _fake_repo() -> Path:
    root = Path(tempfile.mkdtemp())
    core = root / "SddIA" / "core"
    process_dir = root / "SddIA" / "process"
    core.mkdir(parents=True)
    process_dir.mkdir(parents=True)
    repo_root = Path(__file__).resolve().parents[2]
    for name in ("cerbero-governance-react.md", "route-domain.md"):
        src = repo_root / "process" / name
        if src.is_file():
            (process_dir / name).write_text(src.read_text(encoding="utf-8"), encoding="utf-8")
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
    src = repo_root / "core" / "event-domain-subscriptions.json"
    if src.is_file():
        (core / "event-domain-subscriptions.json").write_text(
            src.read_text(encoding="utf-8"), encoding="utf-8"
        )
    (root / ".events" / "domain").mkdir(parents=True)
    (root / "SddIA" / "agents" / "radamanto.thresholds.json").parent.mkdir(parents=True)
    (root / "SddIA" / "agents" / "radamanto.thresholds.json").write_text("{}", encoding="utf-8")
    return root


class TestRadamantoDlt(unittest.TestCase):
    def test_dlt_via_route_fanout_simulated(self) -> None:
        repo = _fake_repo()
        os.environ["SDDIA_LAB_ROUTE_SYNC"] = "1"
        os.environ["SDDIA_LAB_SIMULATE_IOTA"] = "1"
        try:
            ev = build_domain_event(
                "Domain_Entity_Degraded",
                {
                    "entity_type": "skill",
                    "entity_id": "skill:dlt-test",
                    "reason": "lab",
                    "success_rate": 0.5,
                    "recovery_attempt": 1,
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


if __name__ == "__main__":
    unittest.main()
