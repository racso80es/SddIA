#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Simula creación de PR remoto (Jules) sin acceso a wallet — fixture para lab H4.

Uso:
  SDDIA_LAB_SIMULATE_REMOTE_PR=1 python SddIA/scripts/qa/simulate_remote_pr.py
  python SddIA/scripts/qa/simulate_remote_pr.py --inputs-file docs/features/pull-request-automation-dlt/_smoke-remote-pr-dlt.json
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

_SCRIPT_DIR = Path(__file__).resolve().parent
if str(_SCRIPT_DIR) not in sys.path:
    sys.path.insert(0, str(_SCRIPT_DIR))

SIMULATION_REL = ".SddIA/.dev/remote_pr_simulation.json"


def _repo_root() -> Path:
    here = Path(__file__).resolve()
    for parent in here.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError("No se encontró raíz del workspace")


def main() -> None:
    parser = argparse.ArgumentParser(description="simulate_remote_pr — fixture Jules lab")
    parser.add_argument("--inputs-file", help="JSON con repository, branch, pr_url, origin_agent")
    parser.add_argument("--inputs", help="JSON inline")
    args = parser.parse_args()

    if args.inputs_file:
        raw = Path(args.inputs_file).read_text(encoding="utf-8-sig")
        fixture = json.loads(raw)
    elif args.inputs:
        fixture = json.loads(args.inputs)
    else:
        fixture = {
            "repository": "racso80es/SddIA",
            "branch": "feat/pull-request-automation-dlt",
            "pr_url": "https://github.com/racso80es/SddIA/pull/lab-sim-remote-pr",
            "origin_agent": "jules",
        }

    repo = _repo_root()
    target = repo / SIMULATION_REL
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(json.dumps(fixture, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")

    out = {
        "success": True,
        "simulation_path": SIMULATION_REL,
        "fixture": fixture,
    }
    sys.stdout.write(json.dumps(out, ensure_ascii=False) + "\n")
    sys.exit(0)


if __name__ == "__main__":
    main()
