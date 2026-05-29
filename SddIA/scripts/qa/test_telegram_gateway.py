# -*- coding: utf-8 -*-
"""Smoke telegram-gateway — transmutación y escritura domain."""

from __future__ import annotations

import json
import shutil
import sys
import tempfile
import unittest
from pathlib import Path

_QA = Path(__file__).resolve().parent
if str(_QA) not in sys.path:
    sys.path.insert(0, str(_QA))

from execute_process_capsules import run_process  # noqa: E402
from telegram_gateway_core import transmute_telegram_text  # noqa: E402


def _repo_with_events() -> Path:
    here = Path(__file__).resolve()
    for parent in here.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError("repo root not found")


class TestTransmute(unittest.TestCase):
    def test_todo_kaizen(self):
        t = transmute_telegram_text("TODO: Revisar auditorías")
        self.assertIsNotNone(t)
        assert t is not None
        self.assertEqual(t[0], "Kaizen_Idea_Captured")
        self.assertEqual(t[1]["idea_text"], "Revisar auditorías")

    def test_manual_task(self):
        t = transmute_telegram_text("Hacer backup")
        self.assertIsNotNone(t)
        assert t is not None
        self.assertEqual(t[0], "Manual_Task_Requested")


class TestGatewayProcess(unittest.TestCase):
    def test_emit_manual_task(self) -> None:
        repo = _repo_with_events()
        fractal_domain = repo / ".events" / "domain"
        before = set(fractal_domain.glob("*.json")) if fractal_domain.is_dir() else set()
        out = run_process(repo, "telegram-gateway", {"text": "Hacer backup"})
        self.assertTrue(out.get("success"))
        data = out.get("data") or {}
        self.assertTrue(data.get("emitted"))
        self.assertEqual(data.get("event_type"), "Manual_Task_Requested")
        eid = data.get("event_id")
        self.assertIsInstance(eid, str)
        path = fractal_domain / f"{eid}.json"
        self.assertTrue(path.is_file())
        body = json.loads(path.read_text(encoding="utf-8"))
        self.assertEqual(body.get("event_type"), "Manual_Task_Requested")
        path.unlink(missing_ok=True)


if __name__ == "__main__":
    unittest.main()
