#!/usr/bin/env bash
# Lanzador Unix: tool sandbox-breacher (stdin JSON → stdout envelope JSON)
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec "$SCRIPT_DIR/invoke.sh" sandbox-breacher "$@"
