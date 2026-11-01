#!/usr/bin/env bash
# Acceso directo al operador de ficheros temporales.
# Implementación: SddIA/scripts/qa/clean-project-ephemeral.sh
# Por defecto: --apply --with-cargo-deps (purga real incl. generaciones Cargo obsoletas).
# Inventario solo: SddIA/scripts/qa/clean-project-ephemeral.sh (sin esas banderas).
set -u
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET="$REPO_ROOT/SddIA/scripts/qa/clean-project-ephemeral.sh"

pause_for_close() {
  [[ -r /dev/tty && -w /dev/tty ]] || return 0
  printf '\nPulsa una tecla para cerrar.\n' >/dev/tty
  read -r -n 1 -s _sddia_key </dev/tty || true
}

if [[ ! -f "$TARGET" ]]; then
  printf 'No se encuentra el operador: %s\n' "$TARGET" >&2
  pause_for_close
  exit 1
fi

printf 'SddIA — limpieza de ficheros temporales\n\n'
"$TARGET" --repo "$REPO_ROOT" --apply --with-cargo-deps "$@"
status=$?
exit "$status"
