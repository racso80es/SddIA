#!/usr/bin/env bash
# Smoke lab Installer v3 — AC-4..AC-8 (historia §8).
set -euo pipefail
unset SDDIA_CAPSULE_REQUEST SDDIA_SKIP_STDIN 2>/dev/null || true
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
# shellcheck source=../common/sddia_shell_lib.sh
source "$ROOT/SddIA/scripts/common/sddia_shell_lib.sh"

fail() { echo "FAIL lab: $*" >&2; exit 1; }

INSTALLER="$ROOT/sddia-installer.sh"
MOTOR="$ROOT/SddIA/scripts/sddia-installer.sh"
[[ -x "$INSTALLER" ]] || fail "wrapper no ejecutable"

RVAULT_KEYS=(
  SDDIA_EMAIL_IMAP_USER
  SDDIA_EMAIL_IMAP_PASSWORD
  TELEGRAM_BOT_TOKEN
  TELEGRAM_ALLOWED_CHAT_ID
  IOTA_WALLET_SECRET
  IOTA_ANCHOR_PACKAGE_ID
  GEMINI_API_KEY
  CURSOR_API_KEY
)

_rvault_scan() {
  python3 - "$@" <<'PY'
import re, sys
keys = sys.argv[1:]
text = sys.stdin.read()
for k in keys:
    if re.search(rf"(?m)^[ \t]*{re.escape(k)}[ \t]*=[ \t]*[^ \t#]", text):
        raise SystemExit(f"valor R-VAULT-2 para {k}")
PY
}

# AC-4 — deploy lab skip-build: stdout único JSON; ruido en log
LAB4="/tmp/sddia-installer-lab-ac4-$$"
rm -rf "$LAB4"
mkdir -p "$LAB4"
export SDDIA_INSTALLER_LAB_SKIP_ENABLE=1
export SDDIA_INSTALLER_BUNDLE_PROFILE=engineering
lab_deploy_args=(deploy --root "$LAB4")
if [[ "${GITHUB_ACTIONS:-}" == "true" ]]; then
  # CI: sin ELF en target/release; bundle compila en el propio paso build_bundle.
  :
else
  export SDDIA_BUNDLE_SKIP_WITNESS=1
  lab_deploy_args+=(--skip-build)
fi
stdout4="${LAB4}.stdout"
stderr4="${LAB4}.stderr"
set +e
env -u SDDIA_CAPSULE_REQUEST "$MOTOR" "${lab_deploy_args[@]}" >"$stdout4" 2>"$stderr4"
rc4=$?
set -e
unset SDDIA_BUNDLE_SKIP_WITNESS SDDIA_INSTALLER_BUNDLE_PROFILE
[[ "$rc4" -eq 0 ]] || fail "AC-4 deploy rc=$rc4 (stderr tail: $(tail -3 "$stderr4"))"
python3 -c 'import json,sys; p=sys.argv[1]; raw=open(p).read().strip();
assert "\n" not in raw, "multilínea stdout"
d=json.loads(raw); assert d.get("success") is True' "$stdout4" \
  || fail "AC-4 stdout no es un JSON único"
if rg -q '^(cargo |systemctl )' "$stdout4" 2>/dev/null; then
  fail "AC-4 stdout contaminado (cargo/systemctl)"
fi
log_ref="$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["result"].get("log_ref") or "")' "$stdout4")"
[[ -n "$log_ref" ]] || fail "AC-4 sin log_ref"
log_abs="$ROOT/$log_ref"
[[ -f "$log_abs" ]] || fail "AC-4 log ausente: $log_abs"
if ! rg -q 'build-release-bundle|systemctl|\[installer\]' "$log_abs" 2>/dev/null; then
  fail "AC-4 log sin trazas de hijo/motor"
fi
rm -rf "$LAB4" "$stdout4" "$stderr4"

# AC-5 — perfil inválido → STEP_FAILED build_bundle
LAB5="/tmp/sddia-installer-lab-ac5-$$"
rm -rf "$LAB5"
mkdir -p "$LAB5"
set +e
out5="$(env -u SDDIA_CAPSULE_REQUEST SDDIA_INSTALLER_BUNDLE_PROFILE=invalid-lab-profile "$MOTOR" deploy --root "$LAB5" --skip-build 2>/dev/null)"
rc5=$?
set -e
[[ "$rc5" -eq 6 ]] || fail "AC-5 rc=$rc5 (esperado 6)"
echo "$out5" | python3 -c '
import json,sys
d=json.load(sys.stdin)
assert d["exitCode"]==6
assert d["result"]["error"]["code"]=="STEP_FAILED"
assert d["result"]["error"]["step"]=="build_bundle"
assert d["result"]["error"]["child_exit"]==1
steps=d["result"].get("steps") or []
bb=next(s for s in steps if s["id"]=="build_bundle")
assert bb["status"]=="failed"
mi=next(s for s in steps if s["id"]=="materialize_instance")
assert mi["status"]=="not_run"
' || fail "AC-5 envelope/steps"
rm -rf "$LAB5"

# AC-6 — progreso JSONL begin/end coherente (dry-run)
SMOKE_ROOT="/tmp/sddia-installer-lab-ac6-$$"
stderr6="${SMOKE_ROOT}.stderr"
"$INSTALLER" deploy --root "$SMOKE_ROOT" --dry-run 2>"$stderr6" >/dev/null
python3 - "$stderr6" <<'PY'
import json, re, sys
path = sys.argv[1]
lines = open(path).read().splitlines()
prog = []
for ln in lines:
    if ln.startswith("@sddia-progress "):
        prog.append(json.loads(ln[len("@sddia-progress "):]))
begins = {p["id"]: p for p in prog if p.get("moment") == "begin"}
ends = {p["id"]: p for p in prog if p.get("moment") == "end"}
for sid, b in begins.items():
    e = ends.get(sid)
    if not e:
        raise SystemExit(f"AC-6 sin end para {sid}")
    if b.get("index") != e.get("index") or b.get("total") != e.get("total"):
        raise SystemExit(f"AC-6 index/total divergente en {sid}")
    blob = json.dumps(b) + json.dumps(e)
    if "/home/" in blob or ".env" in blob.lower():
        raise SystemExit(f"AC-6 ruta sensible en progreso {sid}")
PY
rm -f "$stderr6"

# AC-7 — correlation_id → PTC en .events/progress/
CID="$(python3 -c 'import uuid; print(uuid.uuid4())')"
PROG_DIR="$ROOT/.events/progress/$CID"
rm -rf "$PROG_DIR"
req7="$(mktemp)"
cat >"$req7" <<EOF
{"meta":{"schemaVersion":"2.0","entityKind":"tool","entityId":"sddia-installer"},"request":{"command":"deploy","root":"$SMOKE_ROOT","dry_run":true,"correlation_id":"$CID"}}
EOF
"$INSTALLER" --request-file "$req7" >/dev/null
rm -f "$req7"
shopt -s nullglob
ptc=("$PROG_DIR"/*.json)
shopt -u nullglob
[[ ${#ptc[@]} -ge 1 ]] || fail "AC-7 sin PTC en $PROG_DIR"
python3 - "${ptc[@]}" <<'PY'
import json, sys
for p in sys.argv[1:]:
    d=json.load(open(p))
    assert d.get("source_agent")=="sddia-installer"
    assert d.get("correlation_id")
    assert d.get("phase")
PY
rm -rf "$PROG_DIR" "$SMOKE_ROOT"

# AC-8 — R-VAULT-2: sin valores en envelope + progreso + log (dry-run)
stderr8="${SMOKE_ROOT}.stderr"
out8="$("$INSTALLER" deploy --root "/tmp/sddia-installer-lab-ac8-$$" --dry-run 2>"$stderr8")"
log8="$(echo "$out8" | python3 -c 'import json,sys; print(json.load(sys.stdin)["result"].get("log_ref") or "")')"
bundle8="$(echo "$out8")$(
  cat "$stderr8"
)$([[ -n "$log8" && -f "$ROOT/$log8" ]] && cat "$ROOT/$log8")"
printf '%s' "$bundle8" | _rvault_scan "${RVAULT_KEYS[@]}" || fail "AC-8 valor prohibido"
rm -f "$stderr8"

echo "OK test-sddia-installer-lab"
