#!/usr/bin/env bash
# Prepara target/release para AC-4 lab (CI installer-smoke). No despliega instancia.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT/SddIA"
echo "[prep-installer-lab] cargo build --release --workspace"
cargo build --release --workspace -q
[[ -x "$ROOT/SddIA/target/release/execute-process" ]] || {
  echo "[prep-installer-lab] ERROR: execute-process ausente tras build" >&2
  exit 1
}
echo "[prep-installer-lab] OK"
