#!/usr/bin/env bash
# Lanzador Linux (Mint): Recolector inerte — event-sweeper.py
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
SWEEPER_SCRIPT="$REPO_ROOT/SddIA/scripts/daemons/event-sweeper.py"

if [[ ! -f "$SWEEPER_SCRIPT" ]]; then
  echo "[ERROR] No se encuentra: $SWEEPER_SCRIPT"
  exit 1
fi

if command -v python3 >/dev/null 2>&1; then
  PYTHON=python3
elif command -v python >/dev/null 2>&1; then
  PYTHON=python
else
  echo "[ERROR] No se encontró Python 3. Instale python3."
  exit 1
fi

export PYTHONUTF8=1

echo "[SH] Deteniendo instancias previas de event-sweeper.py..."
pkill -f '[e]vent-sweeper\.py' 2>/dev/null || true
sleep 1

echo "[SH] Iniciando Recolector Inerte (event-sweeper)..."
echo "[SH] Repo: $REPO_ROOT"
echo "[SH] Modo: bucle continuo (use --once para un solo ciclo, --json para salida JSON)"
echo "[SH] Purga padres completados y alerta dead-letter del bus EDA V3+"

_run_in_terminal() {
  local title="SddIA Event Sweeper"
  local cmd="cd $(printf '%q' "$REPO_ROOT") && export PYTHONUTF8=1 && $(printf '%q' "$PYTHON") $(printf '%q' "$SWEEPER_SCRIPT")"
  for arg in "$@"; do
    cmd="$cmd $(printf '%q' "$arg")"
  done
  cmd="$cmd; exec bash"

  if command -v gnome-terminal >/dev/null 2>&1; then
    gnome-terminal --title="$title" --working-directory="$REPO_ROOT" -- bash -lc "$cmd"
  elif command -v x-terminal-emulator >/dev/null 2>&1; then
    x-terminal-emulator -T "$title" -e bash -lc "$cmd"
  elif command -v xterm >/dev/null 2>&1; then
    xterm -T "$title" -e bash -lc "$cmd" &
  else
    echo "[SH] Terminal gráfica no detectada; ejecutando en esta sesión."
    cd "$REPO_ROOT"
    exec "$PYTHON" "$SWEEPER_SCRIPT" "$@"
  fi
}

_run_in_terminal "$@"
echo "[SH] Sweeper lanzado en ventana nueva."
