#!/usr/bin/env bash
# Fixture AUT-79 — generaciones Cargo en deps (sin tocar SddIA/target del host).
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
PY="$ROOT/SddIA/scripts/qa/clean-project-ephemeral-cargo-deps.py"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

mkdir -p "$TMP/SddIA/core"
cp "$ROOT/SddIA/core/cumulo.paths.json" "$TMP/SddIA/core/"
DEPS="$TMP/SddIA/target/debug/deps"
mkdir -p "$DEPS"

# Hash viejo completo
echo old-rlib >"$DEPS/libfoo-aaaaaaaaaaaaaaaa.rlib"
echo old-meta >"$DEPS/libfoo-aaaaaaaaaaaaaaaa.rmeta"
sleep 1
# Hash nuevo completo
echo new-rlib >"$DEPS/libfoo-bbbbbbbbbbbbbbbb.rlib"
echo new-meta >"$DEPS/libfoo-bbbbbbbbbbbbbbbb.rmeta"

out="$("$PY" "$TMP" --apply)"
echo "$out" | grep -q '"cargo_deps":2' || { echo "FAIL: debían borrarse 2 ficheros del hash viejo: $out"; exit 1; }
[[ -f "$DEPS/libfoo-bbbbbbbbbbbbbbbb.rlib" ]] || { echo "FAIL: falta rlib nuevo"; exit 1; }
[[ -f "$DEPS/libfoo-aaaaaaaaaaaaaaaa.rlib" ]] && { echo "FAIL: rlib viejo no borrado"; exit 1; }

# Hold: nuevo solo rmeta, viejo con rlib
rm -rf "$DEPS"/*
echo old-rlib >"$DEPS/libbar-cccccccccccccccc.rlib"
sleep 1
echo new-meta >"$DEPS/libbar-dddddddddddddddd.rmeta"

out="$("$PY" "$TMP")"
echo "$out" | grep -q '"cargo_deps":0' || { echo "FAIL: hold debía impedir borrado: $out"; exit 1; }
echo "$out" | grep -q 'hold-incomplete' || { echo "FAIL: acción hold-incomplete: $out"; exit 1; }

echo "OK test-clean-project-ephemeral-cargo-deps"
