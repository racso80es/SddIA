#!/usr/bin/env bash
# Smoke Filtro C — library/codexes en perfil consumer (sin codex-software-engineering).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
# shellcheck source=../common/sddia_shell_lib.sh
source "$ROOT/SddIA/scripts/common/sddia_shell_lib.sh"

fail() { echo "FAIL: $*" >&2; exit 1; }

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

SDDIA_BUNDLE_SKIP_WITNESS=1 "$ROOT/SddIA/scripts/build-release-bundle.sh" \
  --out "$tmp/stage" \
  --codex codex-kalma2-assistant \
  --profile consumer \
  --skip-build

test ! -e "$tmp/stage/SddIA/library/codexes/codex-software-engineering.md" \
  || fail "codex-software-engineering.md filtrado"
test ! -d "$tmp/stage/SddIA/library/codexes/codex-software-engineering" \
  || fail "codex-software-engineering/ filtrado"
test -f "$tmp/stage/SddIA/library/codexes/codex-kalma2-assistant.md" \
  || fail "codex-kalma2-assistant.md ausente"
test -d "$tmp/stage/SddIA/library/codexes/codex-kalma2-assistant" \
  || fail "codex-kalma2-assistant/ ausente"

rg -q 'llm-router' "$ROOT/SddIA/scripts/build-release-bundle.sh" \
  || fail "CONSUMER_BINS sin llm-router"
rg -q 'gemini-http-infer' "$ROOT/SddIA/scripts/build-release-bundle.sh" \
  || fail "CONSUMER_BINS sin gemini-http-infer"
rg -q 'antigravity-cli-executor' "$ROOT/SddIA/scripts/build-release-bundle.sh" \
  || fail "CONSUMER_BINS sin antigravity-cli-executor"
rg -q 'thought-graph-access' "$ROOT/SddIA/scripts/build-release-bundle.sh" \
  || fail "CONSUMER_BINS sin thought-graph-access"

test -f "$tmp/stage/SddIA/tools/llm-router.md" || fail "llm-router.md ausente"
test -f "$tmp/stage/SddIA/tools/gemini-http-infer.md" || fail "gemini-http-infer.md ausente"
test -f "$tmp/stage/SddIA/tools/thought-graph-access.md" || fail "thought-graph-access.md ausente"
test -f "$tmp/stage/SddIA/skills/antigravity-cli-executor.md" || fail "antigravity-cli-executor.md ausente"
test -f "$tmp/stage/SddIA/conscience/aiua_core.md" || fail "conscience/aiua_core.md ausente"

for elf in llm-router gemini-http-infer antigravity-cli-executor thought-graph-access; do
  if [[ -x "$tmp/stage/SddIA/target/release/$elf" ]]; then
    echo "OK elf $elf"
  else
    echo "WARN elf $elf ausente en skip-build (CA-BINS exige build real)"
  fi
done

echo "OK build-release-bundle-filtro-c"
