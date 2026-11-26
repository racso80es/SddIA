#!/usr/bin/env bash
# AUT-120 KA-ENV-1: _sddia_load_vault exporta literales con metacaracteres sin evaluar shell.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
# shellcheck source=../common/sddia_shell_lib.sh
source "$ROOT/SddIA/scripts/common/sddia_shell_lib.sh"

fail() { echo "FAIL: $*" >&2; exit 1; }

TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT
mkdir -p "$TMP/.dev" "$TMP/.SddIA/.dev"

cat >"$TMP/.dev/.env" <<'EOF'
KA_ENV_PIPE=alpha|beta
KA_ENV_AMP=one&two
KA_ENV_SUB=$(echo pwned)
KA_ENV_SPACE=hello world
EOF

(
  unset KA_ENV_PIPE KA_ENV_AMP KA_ENV_SUB KA_ENV_SPACE pwned 2>/dev/null || true
  _sddia_load_vault "$TMP"
  [[ "${KA_ENV_PIPE:-}" == "alpha|beta" ]] || exit 11
  [[ "${KA_ENV_AMP:-}" == "one&two" ]] || exit 12
  [[ "${KA_ENV_SUB:-}" == '$(echo pwned)' ]] || exit 13
  [[ "${KA_ENV_SPACE:-}" == "hello world" ]] || exit 14
  [[ -z "${pwned:-}" ]] || exit 15
) || fail "carga literal dotenv"

# Paridad helper de entrecomillado (KA-ENV-4).
q="$(_sddia_quote_dotenv_value 'a|b & c')"
[[ "$q" == '"a|b & c"' ]] || fail "quote helper: $q"

echo "OK: test-vault-literal-dotenv-load"