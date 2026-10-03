#!/usr/bin/env bash
# PBI-MERGE-THERMO-04: atestación QA fail-closed y R-5 docs-only + delta activo.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
# shellcheck source=/dev/null
source "$ROOT/SddIA/scripts/qa/git-hooks/hook_common.sh"

fail() { echo "FAIL: $*" >&2; exit 1; }

REPO="$ROOT"
tmpdir=$(mktemp -d)
trap 'rm -rf "$tmpdir"' EXIT

proofs=$(resolve_eda_proofs_dir)
mkdir -p "$proofs/qa-attestations"
branch="feat/merge-thermo-04-attestation"
slug=$(branch_slug "$branch")
path="$proofs/qa-attestations/${slug}.json"
local_sha=$(git_run rev-parse HEAD)
tree=$(git_run rev-parse "${local_sha}^{tree}")

issued=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
python3 -c 'import json,sys; print(json.dumps({"schema":"qa-attestation/1.0","branch":sys.argv[1],"head_sha":sys.argv[2],"tree_sha":sys.argv[3],"verdict":"aprobado","qa_profile":"full","issued_at":sys.argv[4],"ttl_secs":86400,"correlation_id":"c","execution_id":"e-test"}))' \
  "$branch" "$local_sha" "$tree" "$issued" > "$path"

read_qa_attestation_hit "$branch" "$local_sha" "active" || fail "valid full attestation should hit"
[[ "$HOOK_TIMING_ATTESTATION_HIT" == "true" ]] || fail "attestation_hit flag"

python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); d["tree_sha"]="0000000000000000000000000000000000000000"; json.dump(d,open(sys.argv[1],"w"))' "$path"
read_qa_attestation_hit "$branch" "$local_sha" "active" && fail "tree mismatch must miss"

python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); d["tree_sha"]=sys.argv[2]; d["verdict"]="rechazado"; json.dump(d,open(sys.argv[1],"w"))' "$path" "$tree"
read_qa_attestation_hit "$branch" "$local_sha" "active" && fail "bad verdict must miss"

python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); d["verdict"]="aprobado"; d["qa_profile"]="docs-only"; json.dump(d,open(sys.argv[1],"w"))' "$path"
read_qa_attestation_hit "$branch" "$local_sha" "active" && fail "docs-only + active delta must miss"
read_qa_attestation_hit "$branch" "$local_sha" "passive" || fail "docs-only + passive should hit"

echo "OK test-merge-thermo-04-attestation"
