#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""CLI de laboratorio: intérprete dinámico universal de procesos SddIA.

Uso canónico (Ola C+):
  python SddIA/scripts/qa/execute-process.py --process feature --inputs '{"feature_name":"mi-feature",...}'

Compatibilidad (deprecada, Ola C):
  python SddIA/scripts/qa/execute-process.py --input-file payload.json

Shim acciones legacy (deprecado):
  python SddIA/scripts/qa/execute-process.py --action emit-pr-merged-event --inputs '{...}'
  → delega en execute-action.py
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

from execute_process_capsules import run_process, shim_execute_action
from execute_process_core import (
    emit,
    normalize_request,
    repo_root,
    warn_deprecated_action_shim,
    warn_deprecated_input_file,
)


def _parse_inputs_arg(raw: str | None, input_file: str | None) -> dict[str, Any]:
    if input_file:
        return json.loads(Path(input_file).read_text(encoding="utf-8-sig"))
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
    parser.add_argument("--input-file", help="[DEPRECADO] Ruta a JSON envelope legacy")
    parser.add_argument("--action", help="[DEPRECADO] Shim → execute-action.py")
    args = parser.parse_args()

    used_legacy_input_file = False

    try:
        if args.action:
            if not args.inputs and not args.input_file:
                raise ValueError("--inputs o --input-file requerido con --action")
            warn_deprecated_action_shim(args.action.strip())
            action_inputs = _parse_inputs_arg(args.inputs, args.input_file)
            if not isinstance(action_inputs, dict):
                raise ValueError("inputs de acción deben ser objeto JSON")
            repo = repo_root()
            result = shim_execute_action(repo, args.action.strip(), action_inputs)
            emit(result, result.get("status_code", 0))
            return

        if args.process:
            if args.inputs_file:
                process_inputs = json.loads(
                    Path(args.inputs_file).read_text(encoding="utf-8-sig")
                )
            else:
                process_inputs = _parse_inputs_arg(args.inputs, None)
            if not isinstance(process_inputs, dict):
                raise ValueError("--inputs debe ser objeto JSON")
            process_name = args.process.strip()
        elif args.input_file:
            used_legacy_input_file = True
            raw = json.loads(Path(args.input_file).read_text(encoding="utf-8-sig"))
            process_name, process_inputs = normalize_request(raw)
        else:
            raw = _parse_inputs_arg(None, None)
            if not raw:
                raise ValueError("Indique --process y --inputs, --input-file o stdin JSON")
            process_name, process_inputs = normalize_request(raw)

        if used_legacy_input_file:
            warn_deprecated_input_file()

        repo = repo_root()
        result = run_process(repo, process_name, process_inputs)
        emit(result, result.get("status_code", 0))
    except json.JSONDecodeError as e:
        emit({"success": False, "error": f"JSON inválido: {e}"}, 1)
    except Exception as e:
        emit({"success": False, "status_code": 1, "error": str(e)}, 1)


if __name__ == "__main__":
    main()
