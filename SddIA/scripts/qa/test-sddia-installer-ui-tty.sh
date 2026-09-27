#!/usr/bin/env bash
# Smoke TTY presentador — AC-T1..T3 (historia §8 AC-9, AC-11).
set -euo pipefail
unset SDDIA_CAPSULE_REQUEST SDDIA_SKIP_STDIN 2>/dev/null || true
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
# shellcheck source=../common/sddia_shell_lib.sh
source "$ROOT/SddIA/scripts/common/sddia_shell_lib.sh"

fail() { echo "FAIL ui-tty: $*" >&2; exit 1; }

UI="$ROOT/SddIA/scripts/installer/sddia-installer-ui.sh"
[[ -x "$UI" ]] || fail "presentador no ejecutable"
command -v script >/dev/null 2>&1 || fail "script(1) ausente"
command -v expect >/dev/null 2>&1 || fail "expect ausente"

SMOKE_ROOT="/tmp/sddia-installer-ui-tty-$$"
TEAR_ROOT="/tmp/sddia-installer-ui-tear-$$"
rm -rf "$SMOKE_ROOT" "$TEAR_ROOT"
mkdir -p "$SMOKE_ROOT"

WORKDIR="$(mktemp -d /tmp/sddia-ui-tty-work.XXXXXX)"
trap 'rm -rf "$WORKDIR" "$SMOKE_ROOT" "$TEAR_ROOT"' EXIT

export SDDIA_INSTALLER_HOLD=0
export NO_COLOR=1

# AC-T1 — deploy dry-run bajo pseudo-TTY
T1_LOG="$WORKDIR/ac-t1.script"
set +e
script -qefc "cd '$ROOT' && '$UI' deploy --root '$SMOKE_ROOT' --dry-run --no-hold" "$T1_LOG" >/dev/null 2>"$WORKDIR/ac-t1.err"
t1_rc=$?
set -e
[[ "$t1_rc" -eq 0 ]] || fail "AC-T1 rc=$t1_rc"
grep -qE '[0-9]+/[0-9]+ — ' "$T1_LOG" || fail "AC-T1 sin línea k/N —"
grep -q 'Resultado:' "$T1_LOG" || fail "AC-T1 sin resumen Resultado"
grep -q 'Puerto WUI:' "$T1_LOG" || fail "AC-T1 sin Puerto"
grep -q 'Log:' "$T1_LOG" || fail "AC-T1 sin Log"
grep -q 'Duración:' "$T1_LOG" || fail "AC-T1 sin Duración"

# AC-T2 — teardown TTY: confirmación distinta de ELIMINAR → exit 3, sin pasos de motor
T2_OUT="$WORKDIR/ac-t2.out"
T2_ERR="$WORKDIR/ac-t2.err"
export ROOT UI TEAR_ROOT
set +e
expect <<'EXPECT' >"$T2_OUT" 2>"$T2_ERR"
set timeout 30
spawn -noecho bash -c "cd $env(ROOT) && SDDIA_INSTALLER_HOLD=0 NO_COLOR=1 ./SddIA/scripts/installer/sddia-installer-ui.sh teardown --root $env(TEAR_ROOT) --dry-run --no-hold"
expect {
  -re {ELIMINAR} {
    send "cancelar\r"
    exp_continue
  }
  eof
}
catch wait result
exit [lindex $result 3]
EXPECT
t2_rc=$?
set -e
[[ "$t2_rc" -eq 3 ]] || fail "AC-T2 rc=$t2_rc (esperado 3)"
python3 -c '
import json,sys,re
blob=open(sys.argv[1]).read()+open(sys.argv[2]).read()
m=re.search(r"\{.*\"entityId\":\"sddia-installer\".*\}", blob)
assert m, "sin envelope"
d=json.loads(m.group(0))
assert d["exitCode"]==3
assert d["result"]["error"]["code"]=="TEARDOWN_REQUIRES_FORCE"
' "$T2_OUT" "$T2_ERR" || fail "AC-T2 envelope"
if grep -qE '[0-9]+/[0-9]+ — ' "$T2_OUT" "$T2_ERR" 2>/dev/null; then
  fail "AC-T2 motor invocado (líneas de paso presentes)"
fi

# AC-T3 — teardown --yes + dry-run bajo TTY
T3_LOG="$WORKDIR/ac-t3.script"
set +e
script -qefc "cd '$ROOT' && '$UI' teardown --root '$TEAR_ROOT' --dry-run --yes --no-hold" "$T3_LOG" >/dev/null 2>"$WORKDIR/ac-t3.err"
t3_rc=$?
set -e
[[ "$t3_rc" -eq 0 ]] || fail "AC-T3 rc=$t3_rc"
grep -q 'Resultado:      OK' "$T3_LOG" || fail "AC-T3 resumen no OK"

echo "OK test-sddia-installer-ui-tty"
