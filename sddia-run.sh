#!/usr/bin/env bash
set -e
REPO_ROOT="$(cd "$(dirname "$0")" && pwd)"
# shellcheck source=SddIA/scripts/common/sddia_shell_lib.sh
source "$REPO_ROOT/SddIA/scripts/common/sddia_shell_lib.sh"
_sddia_resolve_orchestrator "$REPO_ROOT"
exec "$SDDIA_EXECUTE_PROCESS_BIN" "$@"
