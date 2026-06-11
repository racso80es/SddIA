#!/usr/bin/env bash
# Lanzador Linux (Mint): Despertador inerte — event-watcher.py
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
WATCHER_SCRIPT="$REPO_ROOT/SddIA/scripts/daemons/event-watcher.py"

if [[ ! -f "$WATCHER_SCRIPT" ]]; then
  echo "[ERROR] No se encuentra: $WATCHER_SCRIPT"
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

echo "[SH] Deteniendo instancias previas de event-watcher.py..."
pkill -f '[e]vent-watcher\.py' 2>/dev/null || true
sleep 1

echo "[SH] Iniciando Despertador Inerte (event-watcher)..."
echo "[SH] Repo: $REPO_ROOT"
echo "[SH] Modo: bucle continuo (use --once para un solo ciclo)"
echo "[SH] Lab IOTA simulado: export SDDIA_LAB_SIMULATE_IOTA=1"
echo "[SH] Bóvedas: .dev/.env + .SddIA/.dev/.env (carga automática en Python)"

_run_in_terminal() {
  local title="SddIA Event Watcher"
  local cmd="cd $(printf '%q' "$REPO_ROOT") && export PYTHONUTF8=1 && $(printf '%q' "$PYTHON") $(printf '%q' "$WATCHER_SCRIPT")"
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
    exec "$PYTHON" "$WATCHER_SCRIPT" "$@"
  fi
}

_run_in_terminal "$@"
echo "[SH] Watcher lanzado en ventana nueva."
