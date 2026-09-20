#!/usr/bin/env bash
# Smoke Filtro C — library/codexes en perfil consumer (sin codex-software-engineering).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
# shellcheck source=../common/sddia_shell_lib.sh
source "$ROOT/SddIA/scripts/common/sddia_shell_lib.sh"

fail() { echo "FAIL: $*" >&2; exit 1; }

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

"$ROOT/SddIA/scripts/build-release-bundle.sh" \
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

echo "OK build-release-bundle-filtro-c"
