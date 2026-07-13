#!/usr/bin/env bash
# Lanzador Unix: tool schema-corruptor (stdin JSON → stdout envelope JSON)
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec "$SCRIPT_DIR/invoke.sh" schema-corruptor "$@"
