#!/usr/bin/env python3
"""Unit: timeout terminal + persist_ref fallback (KALMA2-AUD-4b9de6)."""
from __future__ import annotations

import importlib.util
import os
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
MOD_PATH = ROOT / "SddIA" / "scripts" / "tools" / "kalma2-agent-runtime-cursor.py"


def load_runtime():
    spec = importlib.util.spec_from_file_location("kalma2_agent_runtime_cursor", MOD_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"no se pudo cargar {MOD_PATH}")
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


class TimeoutPolicyTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.rt = load_runtime()

    def test_timeout_is_not_soft(self) -> None:
        self.assertFalse(self.rt.is_soft_config_error("timeout 600s"))
        self.assertFalse(self.rt.is_soft_config_error("TimeoutExpired"))
        self.assertFalse(self.rt.is_soft_config_error("CLI no encontrado: timeout 600s"))

    def test_config_absence_remains_soft(self) -> None:
        self.assertTrue(self.rt.is_soft_config_error("CLI no encontrado: cursor-agent"))
        self.assertTrue(self.rt.is_soft_config_error("not found"))
        self.assertTrue(self.rt.is_soft_config_error("401 auth"))
        self.assertFalse(self.rt.is_soft_config_error("agent crashed"))

    def test_timeout_secs_default_and_execution_override(self) -> None:
        os.environ.pop("SDDIA_AGENT_RUNTIME_TIMEOUT_SECS", None)
        os.environ.pop("SDDIA_AGENT_RUNTIME_TIMEOUT_SECS_EJECUCION", None)
        self.assertEqual(self.rt.resolve_timeout_secs("Ejecución"), 600)
        os.environ["SDDIA_AGENT_RUNTIME_TIMEOUT_SECS_EJECUCION"] = "1800"
        self.assertEqual(self.rt.resolve_timeout_secs("Ejecución"), 1800)
        self.assertEqual(self.rt.resolve_timeout_secs("Verificación"), 600)
        os.environ.pop("SDDIA_AGENT_RUNTIME_TIMEOUT_SECS_EJECUCION", None)

    def test_persist_ref_fallback_from_inputs(self) -> None:
        doc = {"persist_ref": "", "inputs": {"persist_ref": "docs/features/kalma2-x"}}
        self.assertEqual(self.rt.resolve_persist_ref(doc), "docs/features/kalma2-x")
        doc2 = {"persist_ref": "docs/features/top"}
        self.assertEqual(self.rt.resolve_persist_ref(doc2), "docs/features/top")


if __name__ == "__main__":
    sys.exit(0 if unittest.main(verbosity=2) else 1)
