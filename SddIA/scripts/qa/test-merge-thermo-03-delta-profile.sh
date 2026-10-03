#!/usr/bin/env bash
# PBI-MERGE-THERMO-03: delta pasivo/activo y pre-commit condicional.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
# shellcheck source=/dev/null
source "$ROOT/SddIA/scripts/qa/git-hooks/hook_common.sh"

fail() { echo "FAIL: $*" >&2; exit 1; }

REPO="$ROOT"
path_is_passive "docs/foo.md" || fail "docs debe ser pasivo"
path_is_passive "README.md" && fail "README debe ser activo"
path_is_passive "notes.md" || fail "md raíz pasivo"
path_is_passive "SddIA/norms/x.md" && fail "SddIA md activo"

class=$(delta_class_for_paths $'docs/a.md\ndocs/b.md')
[[ "$class" == "passive" ]] || fail "delta docs passive got $class"
class=$(delta_class_for_paths $'docs/a.md\nSddIA/foo.rs')
[[ "$class" == "active" ]] || fail "delta mix active got $class"

echo "OK test-merge-thermo-03-delta-profile"
