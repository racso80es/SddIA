#!/usr/bin/env bash
# Lanzador unificado de Centinelas: bucle continuo en foreground por defecto.
set -euo pipefail

if [[ $# -lt 1 ]]; then
  echo "[ERROR] Uso: $(basename "$0") <daemon-name> [args...]" >&2
  exit 1
fi

DAEMON="$1"
shift

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../common/sddia_shell_lib.sh
source "$SCRIPT_DIR/../common/sddia_shell_lib.sh"
_FALLBACK_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
REPO_ROOT="$(_sddia_resolve_instance_root "$_FALLBACK_ROOT")"
EXEC="$SCRIPT_DIR/_exec_daemon.sh"
LOCK_FILE="$REPO_ROOT/.SddIA/daemons/status/${DAEMON}.lock"

_use_terminal=false
args=()
for arg in "$@"; do
  case "$arg" in
    --terminal)
      _use_terminal=true
      ;;
    *)
      args+=("$arg")
      ;;
  esac
done

_stop_previous() {
  _sddia_stop_lock_pid "$LOCK_FILE"
}

_one_shot_mode() {
  local arg
  for arg in "$@"; do
    case "$arg" in
      --once | --dry-run | --event-file-path)
        return 0
        ;;
    esac
  done
  return 1
}

_run_in_terminal() {
  local title="SddIA ${DAEMON}"
  local cmd="set +e"
  cmd="$cmd; cd $(printf '%q' "$REPO_ROOT")"
  cmd="$cmd && export PYTHONUTF8=1"
  cmd="$cmd && $(printf '%q' "$EXEC") $(printf '%q' "$DAEMON")"
  local arg
  for arg in "${args[@]}"; do
    cmd="$cmd $(printf '%q' "$arg")"
  done
  cmd="$cmd; rc=\$?"
  cmd="$cmd; echo"
  cmd="$cmd; echo \"[${DAEMON}] finalizado (exit=\$rc). Pulse Enter para cerrar.\""
  cmd="$cmd; read -r _"

  if command -v gnome-terminal >/dev/null 2>&1; then
    if gnome-terminal --title="$title" --working-directory="$REPO_ROOT" -- bash -lc "$cmd"; then
      return 0
    fi
    echo "[SH] gnome-terminal no disponible; ejecutando en esta sesión." >&2
  elif command -v x-terminal-emulator >/dev/null 2>&1; then
    if x-terminal-emulator -T "$title" -e bash -lc "$cmd"; then
      return 0
    fi
    echo "[SH] x-terminal-emulator falló; ejecutando en esta sesión." >&2
  elif command -v xterm >/dev/null 2>&1; then
    xterm -T "$title" -e bash -lc "$cmd" &
    return 0
  else
    echo "[SH] Terminal gráfica no detectada; ejecutando en esta sesión." >&2
  fi

  exec "$EXEC" "$DAEMON" "${args[@]}"
}

if [[ ! -x "$EXEC" ]]; then
  echo "[ERROR] No se encuentra ejecutor: $EXEC" >&2
  exit 1
fi

if [[ -z "${INVOCATION_ID:-}" ]]; then
  _stop_previous
fi

if _one_shot_mode "${args[@]}"; then
  exec "$EXEC" "$DAEMON" "${args[@]}"
fi

if [[ "$_use_terminal" == true ]]; then
  _run_in_terminal
  echo "[SH] Centinela ${DAEMON} lanzado en ventana nueva."
  exit 0
fi

echo "[${DAEMON}] Modo continuo — esperando estímulos (Ctrl+C para detener)"
echo "[${DAEMON}] Repo: $REPO_ROOT"
echo "[${DAEMON}] Bóvedas: .dev/.env + .SddIA/.dev/.env"
echo "[${DAEMON}] Un solo ciclo: añada --once"
exec "$EXEC" "$DAEMON" "${args[@]}"
