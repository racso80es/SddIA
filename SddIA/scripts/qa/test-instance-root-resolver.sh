#!/usr/bin/env bash
# Smoke F-DEP-10 / F-CEN-PKILL (sin systemd).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
# shellcheck source=../common/sddia_shell_lib.sh
source "$ROOT/SddIA/scripts/common/sddia_shell_lib.sh"

fail() { echo "FAIL: $*" >&2; exit 1; }

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
mkdir -p "$tmp/ap/.SddIA" "$tmp/forge/.SddIA" "$tmp/other"

got="$(cd "$tmp/other" && SDDIA_INSTANCE_ROOT="$tmp/ap" _sddia_resolve_instance_root "$tmp/forge")"
[[ "$got" == "$(cd "$tmp/ap" && pwd)" ]] || fail "env AP: got=$got"

got="$(cd "$tmp/ap" && unset SDDIA_INSTANCE_ROOT && _sddia_resolve_instance_root "$tmp/forge")"
[[ "$got" == "$(cd "$tmp/ap" && pwd)" ]] || fail "cwd AP: got=$got"

got="$(cd "$tmp/other" && unset SDDIA_INSTANCE_ROOT && _sddia_resolve_instance_root "$tmp/forge")"
[[ "$got" == "$tmp/forge" ]] || fail "fallback: got=$got"

if grep -q 'pkill -x' "$ROOT/SddIA/scripts/daemons/_run_daemon.sh"; then
  fail "pkill -x en _run_daemon.sh"
fi
if grep -n 'pkill -x' "$ROOT/start-sddia.sh" | grep -v 'no pkill' >/dev/null; then
  fail "pkill -x operativo en start-sddia.sh"
fi
grep -q 'ExecStart=%f/SddIA/scripts/daemons/@@DAEMON_NAME@@.sh' \
  "$ROOT/SddIA/templates/systemd/sddia-daemon@.service.template" || fail "factory ExecStart %f"
grep -q 'ExecStart=%f/SddIA/daemons/email-watcher.sh' \
  "$ROOT/SddIA/templates/systemd/sddia-email-watcher@.service.template" || fail "email ExecStart %f"
if grep -q '@@SDDIA_CORE_ROOT@@' "$ROOT/SddIA/templates/systemd/"*.template; then
  fail "CORE_ROOT residual en plantillas"
fi

echo "OK instance-root-resolver"
