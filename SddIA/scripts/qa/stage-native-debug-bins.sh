#!/usr/bin/env bash
# Copia a un directorio plano los binarios debug que verify-compiled-capsules exige.
# Misma exploración: un nivel bajo engine/skills/tools/daemons/interfaces,
# crate con src/main.rs, nombre = name de [package].
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
SRC="$ROOT/SddIA/target/debug"
DEST="$ROOT/SddIA/target/native-debug-bins"
rm -rf "$DEST"
mkdir -p "$DEST"

package_name() {
  awk '
    /^\[/ {
      if ($0 == "[package]") { in_pkg = 1 } else { in_pkg = 0 }
      next
    }
    in_pkg && /^name[[:space:]]*=/ {
      line = $0
      sub(/^name[[:space:]]*=[[:space:]]*/, "", line)
      gsub(/"/, "", line)
      gsub(/\047/, "", line)
      sub(/[[:space:]].*/, "", line)
      print line
      exit
    }
  ' "$1"
}

found=0
missing=0
for root_rel in SddIA/engine SddIA/skills SddIA/tools SddIA/daemons SddIA/interfaces; do
  dir="$ROOT/$root_rel"
  [[ -d "$dir" ]] || continue
  for crate in "$dir"/*; do
    [[ -d "$crate" ]] || continue
    [[ -f "$crate/src/main.rs" && -f "$crate/Cargo.toml" ]] || continue
    name="$(package_name "$crate/Cargo.toml")"
    [[ -n "$name" ]] || {
      echo "[stage-native-debug-bins] sin name en $crate/Cargo.toml" >&2
      missing=$((missing + 1))
      continue
    }
    bin="$SRC/$name"
    if [[ -f "$bin" ]]; then
      cp -a "$bin" "$DEST/$name"
      found=$((found + 1))
    else
      echo "[stage-native-debug-bins] ausente: $name" >&2
      missing=$((missing + 1))
    fi
  done
done

if [[ "$missing" -ne 0 || "$found" -eq 0 ]]; then
  echo "[stage-native-debug-bins] FAIL — copiados=$found ausentes=$missing" >&2
  exit 1
fi
for req in execute-process sddia-qa; do
  if [[ ! -f "$DEST/$req" ]]; then
    echo "[stage-native-debug-bins] FAIL — obligatorio ausente: $req" >&2
    exit 1
  fi
done
echo "[stage-native-debug-bins] OK — $found binarios"
