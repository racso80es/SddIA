#!/usr/bin/env bash
# Lanzador Unix: centinela telegram-watcher (bucle continuo en foreground por defecto).
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec "$SCRIPT_DIR/_run_daemon.sh" telegram-watcher "$@"
