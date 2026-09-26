#!/usr/bin/env bash
set -euo pipefail
_FORGE_ROOT="__FORGE_ROOT__"
exec "${_FORGE_ROOT}/SddIA/scripts/installer/sddia-installer-ui.sh" teardown "$@"
