#!/bin/bash
set -e
REPO_ROOT="$(cd "$(dirname "$0")" && pwd)"
QA="$REPO_ROOT/SddIA/scripts/qa"
export PYTHONPATH="$QA${PYTHONPATH:+:$PYTHONPATH}"
exec python3 "$QA/orchestrator_resolve.py" "$@"
