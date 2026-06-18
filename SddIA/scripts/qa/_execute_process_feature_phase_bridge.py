#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Bridge: fases feature/bug-fix específicas (PBI archive, delivery-close)."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

_QA = Path(__file__).resolve().parent
if str(_QA) not in sys.path:
    sys.path.insert(0, str(_QA))

from execute_process_capsules import (  # noqa: E402
    execute_delivery_close_phase,
    execute_feature_phase,
)
from execute_process_core import repo_root  # noqa: E402


def main() -> int:
    raw = sys.stdin.read()
    req = json.loads(raw)
    process_name = req["process"]
    phase_name = req["phase_name"]
    inputs = req["inputs"]
    state = req.get("state") or {}
    repo = repo_root()
    if process_name == "feature":
        result = execute_feature_phase(repo, phase_name, inputs, state)
    elif process_name == "delivery-close-cycle":
        result = execute_delivery_close_phase(repo, phase_name, inputs, state)
    else:
        result = None
    out = {"phase": result, "state": state}
    print(json.dumps(out, ensure_ascii=False))
    return 0 if result else 1


if __name__ == "__main__":
    raise SystemExit(main())
