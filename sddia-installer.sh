#!/usr/bin/env bash
set -e
REPO_ROOT="$(cd "$(dirname "$0")" && pwd)"
exec "$REPO_ROOT/SddIA/scripts/sddia-installer.sh" "$@"
