#!/bin/bash

# Abortar en caso de error
set -e

REPO_ROOT="$(cd "$(dirname "$0")" && pwd)"
QA="$REPO_ROOT/SddIA/scripts/qa"
export PYTHONPATH="$QA${PYTHONPATH:+:$PYTHONPATH}"

# Bootstrap venv para fallback Python (PyYAML en execute-process.py)
VENV_DIR="$REPO_ROOT/.venv"
if [ ! -d "$VENV_DIR" ]; then
    python3 -m venv "$VENV_DIR"
fi
# shellcheck source=/dev/null
source "$VENV_DIR/bin/activate"
pip install -r "$REPO_ROOT/requirements.txt" -q 2>/dev/null || true

exec python3 "$QA/orchestrator_resolve.py" "$@"
