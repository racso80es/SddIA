#!/usr/bin/env bash
# Smoke del orquestador físico (dry-run; cero systemctl enable).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
# shellcheck source=../common/sddia_shell_lib.sh
source "$ROOT/SddIA/scripts/common/sddia_shell_lib.sh"

fail() { echo "FAIL: $*" >&2; exit 1; }

INSTALLER="$ROOT/sddia-installer.sh"
[[ -x "$INSTALLER" ]] || fail "wrapper no ejecutable: $INSTALLER"

SMOKE_ROOT="/tmp/sddia-installer-smoke-$$"
FORGE_ABS="$(realpath -m "$ROOT")"

json_field() {
  python3 -c 'import json,sys; print(json.load(sys.stdin)[sys.argv[1]])' "$1"
}

# 1) dry-run con --root temporal
out="$("$INSTALLER" deploy --root "$SMOKE_ROOT" --dry-run)"
echo "$out" | python3 -c 'import json,sys; json.load(sys.stdin)' || fail "JSON dry-run inválido"
[[ "$(echo "$out" | json_field bundle_profile)" == "full-node" ]] || fail "bundle_profile != full-node"
got_root="$(echo "$out" | json_field root)"
[[ "$(realpath -m "$got_root")" == "$(realpath -m "$SMOKE_ROOT")" ]] || fail "root dry-run=$got_root"
esc="$(echo "$out" | json_field esc)"
want_esc="$(systemd-escape -p "$(realpath -m "$SMOKE_ROOT")")"
[[ "$esc" == "$want_esc" ]] || fail "esc=$esc want=$want_esc"

# 2) ROOT = forja → abort
if "$INSTALLER" deploy --root "$FORGE_ABS" --dry-run >/dev/null 2>&1; then
  fail "deploy --root FORGE debía abortar"
fi

# 3) teardown dry-run: esc coherente
tout="$("$INSTALLER" teardown --root "$SMOKE_ROOT" --dry-run)"
[[ "$(echo "$tout" | json_field esc)" == "$want_esc" ]] || fail "teardown esc divergente"
[[ "$(echo "$tout" | json_field command)" == "teardown" ]] || fail "command != teardown"

# 4) live-gate: stub .SddIA sin --force → exit 2
mkdir -p "$SMOKE_ROOT/.SddIA"
set +e
"$INSTALLER" deploy --root "$SMOKE_ROOT" --dry-run >/tmp/sddia-installer-live.json 2>/tmp/sddia-installer-live.err
live_rc=$?
set -e
[[ "$live_rc" -eq 2 ]] || fail "live-gate rc=$live_rc (esperado 2)"
rm -rf "$SMOKE_ROOT"

# 5) full-node discovery ⊇ CONSUMER_BINS + crate extra
caps="$("$ROOT/SddIA/scripts/build-release-bundle.sh" --profile full-node --list-capsules)"
for must in execute-process kalma2-bridge event-watcher llm-router thought-graph-access; do
  echo "$caps" | grep -qx "$must" || fail "full-node sin $must"
done
echo "$caps" | grep -qx "sddia-qa" || echo "$caps" | grep -qx "github-bridge-watcher" \
  || fail "full-node sin crate extra (sddia-qa|github-bridge-watcher)"

# 6) consumer no se rompe (list-capsules sigue existiendo)
cons="$("$ROOT/SddIA/scripts/build-release-bundle.sh" --profile consumer --list-capsules)"
echo "$cons" | grep -qx "execute-process" || fail "consumer list-capsules roto"

# 7) wrapper --help
"$INSTALLER" --help >/dev/null || fail "wrapper --help"

echo "OK test-sddia-installer"
