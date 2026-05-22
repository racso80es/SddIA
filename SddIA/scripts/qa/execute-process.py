#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""CLI de laboratorio: intérprete dinámico universal de procesos SddIA.

Uso canónico (Ola C+):
  python SddIA/scripts/qa/execute-process.py --process feature --inputs '{"feature_name":"mi-feature",...}'
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any

_QA_DIR = Path(__file__).resolve().parent
if str(_QA_DIR) not in sys.path:
    sys.path.insert(0, str(_QA_DIR))

from execute_process_capsules import run_process
from execute_process_core import (
    emit,
    normalize_request,
    repo_root,
)


def _parse_inputs_arg(raw: str | None) -> dict[str, Any]:
    if raw:
        return json.loads(raw)
    stdin = sys.stdin.read()
    if stdin.strip():
        return json.loads(stdin)
    return {}


def main() -> None:
    parser = argparse.ArgumentParser(description="execute-process — intérprete dinámico (laboratorio SddIA)")
    parser.add_argument("--process", help="Nombre canónico del proceso (kebab-case)")
    parser.add_argument("--inputs", help="JSON de process_inputs")
    parser.add_argument("--inputs-file", help="Ruta a JSON de process_inputs (alternativa a --inputs)")
    args = parser.parse_args()

    try:
        if args.process:
            if args.inputs_file:
                process_inputs = json.loads(
                    Path(args.inputs_file).read_text(encoding="utf-8-sig")
                )
            else:
                process_inputs = _parse_inputs_arg(args.inputs)
            if not isinstance(process_inputs, dict):
                raise ValueError("--inputs debe ser objeto JSON")
            process_name = args.process.strip()
        else:
            raw = _parse_inputs_arg(None)
            if not raw:
                raise ValueError("Indique --process y --inputs (--inputs-file o stdin JSON)")
            process_name, process_inputs = normalize_request(raw)

        repo = repo_root()
        result = run_process(repo, process_name, process_inputs)
        emit(result, result.get("status_code", 0))
    except json.JSONDecodeError as e:
        emit({"success": False, "error": f"JSON inválido: {e}"}, 1)
    except Exception as e:
        emit({"success": False, "status_code": 1, "error": str(e)}, 1)


if __name__ == "__main__":
    main()
