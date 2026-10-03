#!/usr/bin/env bash
# PBI-MERGE-THERMO-02: bus→gh, F-DEP-07 sddia-qa, skip present sin gh.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
HOOK_DIR="$ROOT/SddIA/scripts/qa/git-hooks"
# shellcheck source=/dev/null
source "$HOOK_DIR/hook_common.sh"

fail() {
  echo "FAIL: $*" >&2
  exit 1
}

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

# AC-6: Presented en bus → skip sin gh en PATH
mkdir -p "$tmp/.events/pending"
cat >"$tmp/.events/pending/01-presented.json" <<EOF
{"event_type":"PullRequest_Presented","branch":"feat/thermo-skip-bus"}
EOF
REPO="$tmp"
CUMULO_PATH="$tmp/__no_cumulo__.json"

mkdir -p "$tmp/bin"
printf '#!/bin/sh\necho "gh must not run" >&2\nexit 127\n' >"$tmp/bin/gh"
chmod +x "$tmp/bin/gh"
export PATH="$tmp/bin:$PATH"

if should_skip_pre_push_present "feat/thermo-skip-bus"; then
  :
else
  fail "should_skip debió ser true con Presented en bus y sin gh"
fi

# F-DEP-07: release cuando debug no es más nuevo (ELF nativo, no scripts)
TRUE_BIN="/bin/true"
[[ -x "$TRUE_BIN" ]] || TRUE_BIN="$(command -v true)"
[[ -x "$TRUE_BIN" ]] || fail "no se encontró binario true"
QA_REPO="$tmp/qa-resolve"
mkdir -p "$QA_REPO/SddIA/target/debug" "$QA_REPO/SddIA/target/release"
cp "$TRUE_BIN" "$QA_REPO/SddIA/target/debug/sddia-qa"
cp "$TRUE_BIN" "$QA_REPO/SddIA/target/release/sddia-qa"
chmod +x "$QA_REPO/SddIA/target/debug/sddia-qa" "$QA_REPO/SddIA/target/release/sddia-qa"
touch "$QA_REPO/SddIA/target/debug/sddia-qa"
sleep 1
touch "$QA_REPO/SddIA/target/release/sddia-qa"
REPO="$QA_REPO"
unset SDDIA_QA_BIN
resolve_sddia_qa || fail "resolve_sddia_qa"
[[ "$SDDIA_QA_BIN" == *"/release/sddia-qa" ]] || fail "esperaba release, got $SDDIA_QA_BIN"

sleep 1
touch "$QA_REPO/SddIA/target/debug/sddia-qa"
[[ "$(_sddia_elf_mtime "$QA_REPO/SddIA/target/debug/sddia-qa")" -gt "$(_sddia_elf_mtime "$QA_REPO/SddIA/target/release/sddia-qa")" ]] \
  || fail "debug debe ser más nuevo para segunda prueba"
unset SDDIA_QA_BIN
resolve_sddia_qa || fail "resolve_sddia_qa debug-newer"
[[ "$SDDIA_QA_BIN" == *"/debug/sddia-qa" ]] || fail "esperaba debug más nuevo, got $SDDIA_QA_BIN"

echo "OK test-merge-thermo-02-net-prune"
