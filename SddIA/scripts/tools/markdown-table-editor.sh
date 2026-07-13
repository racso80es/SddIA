#!/usr/bin/env bash
# Lanzador Unix: tool markdown-table-editor (stdin JSON → stdout envelope JSON)
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec "$SCRIPT_DIR/invoke.sh" markdown-table-editor "$@"
