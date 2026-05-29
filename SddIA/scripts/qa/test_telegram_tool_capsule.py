# -*- coding: utf-8 -*-
"""Cápsula scripts/tools/telegram-gateway — stdout ECST."""

from __future__ import annotations

import json
import subprocess
import sys
import unittest
from pathlib import Path

_QA = Path(__file__).resolve().parent


def _repo() -> Path:
    for parent in _QA.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError("repo root not found")


class TestToolCapsule(unittest.TestCase):
    def test_idea_pattern(self) -> None:
        repo = _repo()
        script = repo / "SddIA" / "scripts" / "tools" / "telegram-gateway" / "main.py"
        proc = subprocess.run(
            [sys.executable, str(script)],
            input=json.dumps({"text": "IDEA: refactor Argos"}),
            capture_output=True,
            text=True,
            encoding="utf-8",
            cwd=str(repo),
            check=False,
        )
        body = json.loads(proc.stdout.strip().splitlines()[-1])
        self.assertTrue(body.get("success"))
        self.assertTrue(body.get("emitted"))
        ev = body.get("event") or {}
        self.assertEqual(ev.get("event_type"), "Kaizen_Idea_Captured")
        self.assertEqual(ev["payload"]["idea_text"], "refactor Argos")


if __name__ == "__main__":
    unittest.main()
