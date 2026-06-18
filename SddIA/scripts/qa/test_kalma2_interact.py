# -*- coding: utf-8 -*-
"""Tests kalma2-interact — proceso PoC cliente Kalma2."""

from __future__ import annotations

import sys
import unittest
from pathlib import Path

_QA = Path(__file__).resolve().parent
if str(_QA) not in sys.path:
    sys.path.insert(0, str(_QA))

from execute_process_capsules import run_process  # noqa: E402
from kalma2_interact_core import run_kalma2_interact  # noqa: E402


def _repo_root() -> Path:
    for parent in Path(__file__).resolve().parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError("repo root no encontrado")


class TestKalma2InteractCore(unittest.TestCase):
    def test_empty_prompt(self) -> None:
        out = run_kalma2_interact(None, "   ")
        self.assertFalse(out.get("ok"))

    def test_synthesis(self) -> None:
        out = run_kalma2_interact(None, "Señal Kalma2")
        self.assertTrue(out.get("ok"))
        lines = [ln for ln in (out.get("response") or "").splitlines() if ln.strip()]
        self.assertLessEqual(len(lines), 2)


class TestKalma2InteractProcess(unittest.TestCase):
    def test_run_process(self) -> None:
        repo = _repo_root()
        out = run_process(repo, "kalma2-interact", {"prompt": "ping kalma2"})
        self.assertTrue(out.get("success"))
        data = out.get("data") or {}
        self.assertIn("Tormentosa", data.get("response") or "")


if __name__ == "__main__":
    unittest.main()
