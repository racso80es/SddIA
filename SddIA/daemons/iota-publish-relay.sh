#!/usr/bin/env bash
# Lanzador Unix: centinela iota-publish-relay (supervisor Rust + hijo Node)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../scripts/common/sddia_shell_lib.sh
source "$SCRIPT_DIR/../scripts/common/sddia_shell_lib.sh"
_FALLBACK_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
REPO_ROOT="$(_sddia_resolve_instance_root "$_FALLBACK_ROOT")"
cd "$REPO_ROOT"

if BIN="$(_sddia_resolve_daemon_binary "$REPO_ROOT" iota-publish-relay)"; then
  exec "$BIN" "$@"
fi

echo "[iota-publish-relay] binario no encontrado (build: cd SddIA && cargo build -p iota-publish-relay)" >&2
exit 1
