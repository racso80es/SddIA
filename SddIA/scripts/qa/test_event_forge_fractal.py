#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Regresion forja ECST fractal — event_family explicito (Kaizen)."""

from __future__ import annotations

import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

_QA = Path(__file__).resolve().parent
if str(_QA) not in sys.path:
    sys.path.insert(0, str(_QA))

from execute_process_capsules import resolve_effective_event_family, run_event_forge


def _minimal_repo(root: Path) -> Path:
    repo = root / "repo"
    for family in ("telemetry", "orchestration", "domain"):
        fam_dir = repo / "SddIA" / "events" / family
        fam_dir.mkdir(parents=True)
        (fam_dir / "index.md").write_text(
            """---
family: test
---

# Codice

| Archivo fuente | uuid | name | event_type | version | contract | context | Capabilities |
|----------------|------|------|------------|---------|----------|---------|--------------|
""",
            encoding="utf-8",
        )
    return repo


class TestEventForgeFractal(unittest.TestCase):
    def test_missing_event_family_raises(self) -> None:
        with self.assertRaisesRegex(ValueError, "event_family requerido"):
            resolve_effective_event_family({})

    def test_invalid_event_family_raises(self) -> None:
        with self.assertRaisesRegex(ValueError, "debe ser telemetry, orchestration o domain"):
            resolve_effective_event_family({"event_family": "legacy"})

    def test_forge_domain_path(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            repo = _minimal_repo(Path(tmp))
            with patch("execute_process_capsules.crypto", side_effect=["uuid-test", "hashhex"]):
                out = run_event_forge(
                    repo,
                    {
                        "event_name": "kaizen-test-event",
                        "event_type": "Kaizen_Test_Event",
                        "event_family": "domain",
                        "payload_required": [],
                        "payload_optional": [],
                        "payload_forbidden": [],
                    },
                )
            artifact = repo / out["artifact_event_md"]
            self.assertTrue(artifact.is_file())
            self.assertEqual(
                out["artifact_event_md"],
                "SddIA/events/domain/kaizen-test-event.md",
            )
            text = artifact.read_text(encoding="utf-8")
            self.assertIn('event_family: "domain"', text)
            idx = (repo / "SddIA" / "events" / "domain" / "index.md").read_text(encoding="utf-8")
            self.assertIn("kaizen-test-event", idx)

    def test_forge_telemetry_path(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            repo = _minimal_repo(Path(tmp))
            with patch("execute_process_capsules.crypto", side_effect=["uuid-tel", "hashhex2"]):
                out = run_event_forge(
                    repo,
                    {
                        "event_name": "kaizen-tel-event",
                        "event_type": "Kaizen_Tel_Event",
                        "event_family": "telemetry",
                    },
                )
            self.assertEqual(
                out["artifact_event_md"],
                "SddIA/events/telemetry/kaizen-tel-event.md",
            )


if __name__ == "__main__":
    unittest.main()
