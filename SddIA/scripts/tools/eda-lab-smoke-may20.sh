#!/usr/bin/env bash
# Lanzador Unix: tool eda-lab-smoke-may20 (stdin JSON → stdout envelope JSON)
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec "$SCRIPT_DIR/invoke.sh" eda-lab-smoke-may20 "$@"
