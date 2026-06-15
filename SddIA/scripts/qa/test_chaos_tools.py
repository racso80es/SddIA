# -*- coding: utf-8 -*-
"""Tests Arsenal de Entropía — Fase 1 (cápsulas Rust WASI vía capsule_resolve)."""

from __future__ import annotations

import sys
import tempfile
import unittest
from pathlib import Path

_QA = Path(__file__).resolve().parent
if str(_QA) not in sys.path:
    sys.path.insert(0, str(_QA))

from capsule_resolve import invoke_tool_capsule_json
from chaos_workspace_utils import assert_workspace_bound

REPO_ROOT = Path(__file__).resolve().parents[3]


def _run_capsule(tool_name: str, payload: dict) -> tuple[int, dict]:
    return invoke_tool_capsule_json(REPO_ROOT, tool_name, payload, prefer_wasm=True)


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
        with tempfile.TemporaryDirectory(dir=REPO_ROOT) as tmp:
            ws = Path(tmp).resolve()
            code, body = _run_capsule(
                "io-choke",
                {"workspace_path": str(ws), "target_file": "choke.txt"},
            )
            self.assertEqual(code, 0)
            self.assertTrue(body.get("success"))
            io_choked = (body.get("result") or {}).get("io_choked") or body.get("io_choked")
            self.assertTrue(io_choked)


class TestSchemaCorruptor(unittest.TestCase):
    def test_empty_mode_no_valid_receipt(self) -> None:
        code, body = _run_capsule("schema-corruptor", {"corruption_mode": "empty"})
        self.assertEqual(code, 0)
        self.assertTrue(body.get("success"))
        self.assertNotIn("telemetry_receipt", body)

    def test_partial_mode_incomplete_receipt(self) -> None:
        code, body = _run_capsule("schema-corruptor", {"corruption_mode": "partial"})
        self.assertEqual(code, 0)
        receipt = body.get("telemetry_receipt")
        self.assertIsInstance(receipt, dict)
        self.assertNotIn("completion_tokens", receipt)


class TestSandboxBreacher(unittest.TestCase):
    def test_blocks_escape(self) -> None:
        with tempfile.TemporaryDirectory(dir=REPO_ROOT) as tmp:
            ws = Path(tmp).resolve()
            code, body = _run_capsule(
                "sandbox-breacher",
                {"workspace_path": str(ws), "escape_target": "../breach-marker.txt"},
            )
            self.assertEqual(code, 1)
            self.assertFalse(body.get("success"))
            blocked = (body.get("result") or {}).get("breach_blocked") or body.get("breach_blocked")
            self.assertTrue(blocked)

    def test_allows_inside_workspace(self) -> None:
        with tempfile.TemporaryDirectory(dir=REPO_ROOT) as tmp:
            ws = Path(tmp).resolve()
            code, body = _run_capsule(
                "sandbox-breacher",
                {"workspace_path": str(ws), "escape_target": "inside.txt"},
            )
            self.assertEqual(code, 0)
            self.assertTrue(body.get("success"))
            blocked = (body.get("result") or {}).get("breach_blocked")
            if blocked is None:
                blocked = body.get("breach_blocked")
            self.assertFalse(blocked)


if __name__ == "__main__":
    unittest.main()
