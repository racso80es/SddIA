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

env_plan() {
  python3 -c 'import json,sys; d=json.load(sys.stdin); print(json.dumps(d["result"]["plan"]))'
}

plan_field() {
  local field="$1"
  python3 -c 'import json,sys; p=json.load(sys.stdin); print(p[sys.argv[1]])' "$field"
}

envelope_ok() {
  python3 -c '
import json,sys
d=json.load(sys.stdin)
assert d.get("meta",{}).get("entityId")=="sddia-installer"
assert d.get("success") is True
assert d.get("exitCode")==0
' 
}

# 1) dry-run con --root temporal
out="$("$INSTALLER" deploy --root "$SMOKE_ROOT" --dry-run)"
echo "$out" | envelope_ok || fail "envelope dry-run inválido"
plan="$(echo "$out" | env_plan)"
[[ "$(echo "$plan" | plan_field bundle_profile)" == "full-node" ]] || fail "bundle_profile != full-node"
got_root="$(echo "$plan" | plan_field root)"
[[ "$(realpath -m "$got_root")" == "$(realpath -m "$SMOKE_ROOT")" ]] || fail "root dry-run=$got_root"
esc="$(echo "$plan" | plan_field esc)"
want_esc="$(systemd-escape -p "$(realpath -m "$SMOKE_ROOT")")"
[[ "$esc" == "$want_esc" ]] || fail "esc=$esc want=$want_esc"

# 2) ROOT = forja → abort
if "$INSTALLER" deploy --root "$FORGE_ABS" --dry-run >/dev/null 2>&1; then
  fail "deploy --root FORGE debía abortar"
fi

# 3) teardown dry-run: esc coherente
tout="$("$INSTALLER" teardown --root "$SMOKE_ROOT" --dry-run)"
echo "$tout" | envelope_ok || fail "envelope teardown dry-run"
tpl="$(echo "$tout" | env_plan)"
[[ "$(echo "$tpl" | plan_field esc)" == "$want_esc" ]] || fail "teardown esc divergente"
[[ "$(echo "$tpl" | plan_field command)" == "teardown" ]] || fail "command != teardown"

# 4) live-gate: stub .SddIA sin --force → exit 2
mkdir -p "$SMOKE_ROOT/.SddIA"
set +e
out_live="$("$INSTALLER" deploy --root "$SMOKE_ROOT" --dry-run 2>/tmp/sddia-installer-live.err)"
live_rc=$?
set -e
[[ "$live_rc" -eq 2 ]] || fail "live-gate rc=$live_rc (esperado 2)"
echo "$out_live" | python3 -c 'import json,sys; d=json.load(sys.stdin); assert d["exitCode"]==2; assert d["result"]["error"]["code"]=="ROOT_LIVE_REQUIRES_FORCE"' \
  || fail "live-gate sin error.code"
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

# 8) teardown sin --force → exit 3
set +e
out_t3="$("$INSTALLER" teardown --root "$SMOKE_ROOT" 2>/tmp/sddia-installer-tear3.err)"
tear_rc=$?
set -e
[[ "$tear_rc" -eq 3 ]] || fail "teardown sin --force rc=$tear_rc (esperado 3)"
echo "$out_t3" | python3 -c 'import json,sys; d=json.load(sys.stdin); assert d["exitCode"]==3' || fail "teardown sin envelope 3"

# 9) ROOT inseguro / y forja
if "$INSTALLER" teardown --root / --dry-run >/dev/null 2>&1; then
  fail "teardown --root / debía abortar"
fi
if "$INSTALLER" teardown --root "$FORGE_ABS" --force --dry-run >/dev/null 2>&1; then
  fail "teardown --root FORGE debía abortar"
fi

# 10) teardown --force --dry-run sobre stub (no wipe real)
mkdir -p "$SMOKE_ROOT/.SddIA"
toutf="$("$INSTALLER" teardown --root "$SMOKE_ROOT" --force --dry-run)"
echo "$toutf" | envelope_ok || fail "force dry-run envelope"
tplf="$(echo "$toutf" | env_plan)"
[[ "$(echo "$tplf" | plan_field command)" == "teardown" ]] || fail "force dry-run command"
[[ "$(echo "$tplf" | plan_field force)" == "True" || "$(echo "$tplf" | plan_field force)" == "true" ]] \
  || fail "force dry-run force!=true"
test -d "$SMOKE_ROOT/.SddIA" || fail "dry-run no debe borrar ROOT"
rm -rf "$SMOKE_ROOT"

# 11) dry-run deploy: vault compuesto + wui_port derivado
out11="$("$INSTALLER" deploy --root "$SMOKE_ROOT" --dry-run)"
plan11="$(echo "$out11" | env_plan)"
vrs="$(echo "$plan11" | python3 -c 'import json,sys; print(json.load(sys.stdin).get("vault_root_source") or "")')"
[[ -n "$vrs" ]] || fail "vault_root_source vacío en dry-run"
vis="$(echo "$plan11" | python3 -c 'import json,sys; print(json.load(sys.stdin).get("vault_instance_source") or "")')"
[[ "$vis" == "starter-kit" || -n "$vis" ]] || fail "vault_instance_source ausente"
wp="$(echo "$plan11" | python3 -c 'import json,sys; print(json.load(sys.stdin).get("wui_port"))')"
[[ "$wp" != "8765" && -n "$wp" ]] || fail "wui_port debe derivarse ≠ 8765 forja"
ps="$(echo "$plan11" | python3 -c 'import json,sys; print(json.load(sys.stdin).get("port_source"))')"
[[ "$ps" == "derived" ]] || fail "port_source=$ps"

# 12) motor sin curl/ss (palabras completas)
if rg -w 'curl|ss' "$ROOT/SddIA/scripts/sddia-installer.sh" 2>/dev/null; then
  fail "motor installer contiene curl/ss"
fi

# 13) request-file equivalente a argv (plan)
req="$(mktemp)"
cat >"$req" <<EOF
{"meta":{"schemaVersion":"2.0","entityKind":"tool","entityId":"sddia-installer"},"request":{"command":"deploy","root":"$SMOKE_ROOT","dry_run":true}}
EOF
out13="$("$INSTALLER" --request-file "$req")"
plan13="$(echo "$out13" | env_plan)"
out13b="$("$INSTALLER" deploy --root "$SMOKE_ROOT" --dry-run)"
plan13b="$(echo "$out13b" | env_plan)"
python3 -c 'import json,sys; a=json.loads(sys.argv[1]); b=json.loads(sys.argv[2]); 
for k in a:
  if k in ("live",): continue
  if a[k]!=b.get(k): raise SystemExit(f"diff {k}")' "$plan13" "$plan13b" || fail "request-file plan diverge"
rm -f "$req"

echo "OK test-sddia-installer"
