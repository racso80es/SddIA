#!/usr/bin/env bash
# Compat: delega en _run_daemon.sh (foreground continuo por defecto).
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec "$SCRIPT_DIR/_run_daemon.sh" "$@"
