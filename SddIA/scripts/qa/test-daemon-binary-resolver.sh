#!/usr/bin/env bash
# Smoke F-BUILD-DEV-DESALINEADO / RELAY-R1-CA6 (sin systemd).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
# shellcheck source=../common/sddia_shell_lib.sh
source "$ROOT/SddIA/scripts/common/sddia_shell_lib.sh"

fail() { echo "FAIL: $*" >&2; exit 1; }

for launcher in event-watcher telegram-watcher github-bridge-watcher event-sweeper email-watcher iota-publish-relay; do
  grep -q '_sddia_resolve_daemon_binary' "$ROOT/SddIA/daemons/${launcher}.sh" \
    || fail "launcher ${launcher}.sh no usa _sddia_resolve_daemon_binary"
  if grep -q 'NATIVE_DEBUG' "$ROOT/SddIA/daemons/${launcher}.sh"; then
    fail "launcher ${launcher}.sh aún prefija debug"
  fi
done

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
mkdir -p "$tmp/SddIA/daemons/probe-daemon/src" "$tmp/SddIA/target/debug" "$tmp/SddIA/target/release"
printf '[package]\nname = "probe-daemon"\nversion = "0.0.0"\nedition = "2021"\n' \
  > "$tmp/SddIA/daemons/probe-daemon/Cargo.toml"
printf 'fn main() {}\n' > "$tmp/SddIA/daemons/probe-daemon/src/main.rs"

cp /bin/true "$tmp/SddIA/target/debug/probe-daemon"
cp /bin/true "$tmp/SddIA/target/release/probe-daemon"
chmod +x "$tmp/SddIA/target/debug/probe-daemon" "$tmp/SddIA/target/release/probe-daemon"

touch -d '2020-01-01 00:00:00' \
  "$tmp/SddIA/daemons/probe-daemon/Cargo.toml" \
  "$tmp/SddIA/daemons/probe-daemon/src/main.rs"
touch -d '2021-01-01 00:00:00' "$tmp/SddIA/target/debug/probe-daemon"
touch -d '2019-01-01 00:00:00' "$tmp/SddIA/target/release/probe-daemon"

got="$(_sddia_resolve_daemon_binary "$tmp" probe-daemon)" \
  || fail "debug fresco + release fósil debía servir debug"
[[ "$got" == "$tmp/SddIA/target/debug/probe-daemon" ]] \
  || fail "esperado debug fresco, got=$got"

touch -d '2022-01-01 00:00:00' "$tmp/SddIA/daemons/probe-daemon/src/main.rs"
if _sddia_resolve_daemon_binary "$tmp" probe-daemon >/dev/null 2>&1; then
  fail "fuente más nueva que ambos ELF debía fallar"
fi

touch -d '2023-01-01 00:00:00' "$tmp/SddIA/target/release/probe-daemon"
got="$(_sddia_resolve_daemon_binary "$tmp" probe-daemon)" \
  || fail "release fresco debía servir"
[[ "$got" == "$tmp/SddIA/target/release/probe-daemon" ]] \
  || fail "esperado release, got=$got"

if _sddia_resolve_daemon_binary "$tmp" missing-daemon >/dev/null 2>&1; then
  fail "crate ausente debía fallar"
fi

echo "OK daemon-binary-resolver"
