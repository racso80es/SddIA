#!/usr/bin/env bash
# PBI-MERGE-THERMO-06: regresión R-4 (gates existentes + atestación fail-closed).
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cd "$ROOT"
"$ROOT/SddIA/scripts/qa/test-merge-thermo-02-net-prune.sh"
"$ROOT/SddIA/scripts/qa/test-merge-thermo-03-delta-profile.sh"
"$ROOT/SddIA/scripts/qa/test-merge-thermo-04-attestation.sh"
"$ROOT/SddIA/target/release/sddia-qa" verify-domain-subscription-parity
echo "OK test-merge-thermo-06-regression"
