#!/usr/bin/env bash
# P4 — Regenera la tabla de cobertura en spec-test-matrix.md (solo documental).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
exec python3 "$ROOT/docs/features/eda-store-io-gate/_lab/gen_test_matrix_table.py"
