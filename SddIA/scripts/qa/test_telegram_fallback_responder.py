# -*- coding: utf-8 -*-
"""Tests telegram-fallback-responder — Filtro C y proceso."""

from __future__ import annotations

import sys
import unittest
from pathlib import Path

_QA = Path(__file__).resolve().parent
if str(_QA) not in sys.path:
    sys.path.insert(0, str(_QA))

from execute_process_capsules import run_process  # noqa: E402
from telegram_fallback_responder_core import (  # noqa: E402
    filter_c_should_abort,
    synthesize_mayeuta_response,
)


class TestFilterC(unittest.TestCase):
    def test_command_slash(self) -> None:
        self.assertTrue(filter_c_should_abort("/start"))

    def test_command_bang(self) -> None:
        self.assertTrue(filter_c_should_abort("!help"))

    def test_todo_reserved(self) -> None:
        self.assertTrue(filter_c_should_abort("TODO: kaizen"))

    def test_free_text_passes(self) -> None:
        self.assertFalse(filter_c_should_abort("Hola arquitectura"))


class TestSynthesis(unittest.TestCase):
    def test_two_lines_max(self) -> None:
        out = synthesize_mayeuta_response("Señal entrante")
        lines = [ln for ln in out.splitlines() if ln.strip()]
        self.assertLessEqual(len(lines), 2)


class TestFallbackProcess(unittest.TestCase):
    def test_filter_abort_success(self) -> None:
        here = Path(__file__).resolve()
        repo = None
        for parent in here.parents:
            if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
                repo = parent
                break
        self.assertIsNotNone(repo)
        assert repo is not None
        out = run_process(repo, "telegram-fallback-responder", {"text": "/start"})
        self.assertTrue(out.get("success"))
        data = out.get("data") or {}
        self.assertTrue(data.get("filtered"))


if __name__ == "__main__":
    unittest.main()
