#!/usr/bin/env sh
# Instala hooks SddIA dinámicamente en .git/hooks/ (O5)
set -eu
REPO_ROOT=$(git rev-parse --show-toplevel 2>/dev/null) || {
  echo "Ejecutar desde un repositorio git." >&2
  exit 1
}
SRC="$REPO_ROOT/SddIA/scripts/qa/git-hooks"
DST="$REPO_ROOT/.git/hooks"
mkdir -p "$DST"
installed=0
for f in "$SRC"/*; do
  [ -f "$f" ] || continue
  base=$(basename "$f")
  case "$base" in
    *.py|*.ps1|*.sh|*.md|*.json|*.txt|install-hooks*) continue ;;
  esac
  case "$base" in
    *.*) continue ;;
  esac
  target="$DST/$base"
  if ln -sf "$f" "$target" 2>/dev/null; then
    :
  else
    cp -f "$f" "$target"
  fi
  echo "  - $base"
  installed=$((installed + 1))
done
if [ "$installed" -eq 0 ]; then
  echo "Advertencia: no se encontraron hooks instalables en $SRC" >&2
else
  echo "Hooks instalados en $DST"
fi
