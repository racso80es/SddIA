# -*- coding: utf-8 -*-
"""Cápsula telegram-gateway Rust — stdout ECST."""

from __future__ import annotations

import sys
import unittest
from pathlib import Path

_QA = Path(__file__).resolve().parent
if str(_QA) not in sys.path:
    sys.path.insert(0, str(_QA))

from capsule_resolve import invoke_tool_capsule_json


def _repo() -> Path:
    for parent in _QA.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError("repo root not found")


class TestToolCapsule(unittest.TestCase):
    def test_idea_pattern(self) -> None:
        repo = _repo()
        _code, body = invoke_tool_capsule_json(
            repo, "telegram-gateway", {"text": "IDEA: refactor Argos"}
        )
        self.assertTrue(body.get("success"))
        self.assertTrue(body.get("emitted"))
        ev = body.get("event") or {}
        self.assertEqual(ev.get("event_type"), "Kaizen_Idea_Captured")
        self.assertEqual(ev["payload"]["idea_text"], "refactor Argos")


if __name__ == "__main__":
    unittest.main()
