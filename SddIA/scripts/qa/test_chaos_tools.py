# -*- coding: utf-8 -*-
"""Tests Arsenal de Entropía — Fase 1."""

from __future__ import annotations

import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

from chaos_workspace_utils import assert_workspace_bound

REPO_ROOT = Path(__file__).resolve().parents[3]
TOOLS = Path(__file__).resolve().parents[1] / "tools"


def _run_capsule(script: Path, payload: dict) -> tuple[int, dict]:
    proc = subprocess.run(
        [sys.executable, str(script)],
        input=json.dumps(payload),
        capture_output=True,
        text=True,
        encoding="utf-8",
        cwd=str(REPO_ROOT),
        check=False,
    )
    line = (proc.stdout or "").strip().splitlines()[-1] if proc.stdout else ""
    body = json.loads(line) if line else {}
    return proc.returncode, body


class TestAssertWorkspaceBound(unittest.TestCase):
    def test_accepts_inside_workspace(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            ws = Path(tmp).resolve()
            target = ws / "inner.txt"
            ok, err = assert_workspace_bound(REPO_ROOT, target, ws)
            self.assertTrue(ok)
            self.assertIsNone(err)

    def test_rejects_escape(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            ws = Path(tmp).resolve()
            target = (ws / ".." / "outside.txt").resolve()
            ok, err = assert_workspace_bound(REPO_ROOT, target, ws)
            self.assertFalse(ok)
            self.assertIsNotNone(err)


class TestIoChoke(unittest.TestCase):
    def test_chokes_write(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            ws = Path(tmp).resolve()
            code, body = _run_capsule(
                TOOLS / "io-choke" / "io_choke.py",
                {"workspace_path": str(ws), "target_file": "choke.txt"},
            )
            self.assertEqual(code, 0)
            self.assertTrue(body.get("success"))
            self.assertTrue(body.get("result", {}).get("io_choked"))


class TestSchemaCorruptor(unittest.TestCase):
    def test_empty_mode_no_valid_receipt(self) -> None:
        code, body = _run_capsule(
            TOOLS / "schema-corruptor" / "schema_corruptor.py",
            {"corruption_mode": "empty"},
        )
        self.assertEqual(code, 0)
        self.assertTrue(body.get("success"))
        self.assertNotIn("telemetry_receipt", body)

    def test_partial_mode_incomplete_receipt(self) -> None:
        code, body = _run_capsule(
            TOOLS / "schema-corruptor" / "schema_corruptor.py",
            {"corruption_mode": "partial"},
        )
        self.assertEqual(code, 0)
        receipt = body.get("telemetry_receipt")
        self.assertIsInstance(receipt, dict)
        self.assertNotIn("completion_tokens", receipt)


class TestSandboxBreacher(unittest.TestCase):
    def test_blocks_escape(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            ws = Path(tmp).resolve()
            code, body = _run_capsule(
                TOOLS / "sandbox-breacher" / "sandbox_breacher.py",
                {"workspace_path": str(ws), "escape_target": "../breach-marker.txt"},
            )
            self.assertEqual(code, 1)
            self.assertFalse(body.get("success"))
            self.assertTrue(body.get("result", {}).get("breach_blocked"))

    def test_allows_inside_workspace(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            ws = Path(tmp).resolve()
            code, body = _run_capsule(
                TOOLS / "sandbox-breacher" / "sandbox_breacher.py",
                {"workspace_path": str(ws), "escape_target": "inside.txt"},
            )
            self.assertEqual(code, 0)
            self.assertTrue(body.get("success"))
            self.assertFalse(body.get("result", {}).get("breach_blocked"))


if __name__ == "__main__":
    unittest.main()
