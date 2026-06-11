#!/usr/bin/env bash
# Lanzador Linux (Mint): Centinela Capa 0 — telegram-watcher.py
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
WATCHER_SCRIPT="$REPO_ROOT/SddIA/scripts/daemons/telegram-watcher.py"

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

echo "[SH] Deteniendo instancias previas de telegram-watcher.py..."
pkill -f '[t]elegram-watcher\.py' 2>/dev/null || true
sleep 1

echo "[SH] Iniciando Centinela Capa 0 (telegram-watcher)..."
echo "[SH] Repo: $REPO_ROOT"
echo "[SH] Modo: bucle long-polling (use --once o --dry-run)"
echo "[SH] Requiere: TELEGRAM_BOT_TOKEN + TELEGRAM_ALLOWED_CHAT_ID en .SddIA/.dev/.env"
echo "[SH] Estado idempotencia: .SddIA/.state/telegram_last_id"

_run_in_terminal() {
  local title="SddIA Telegram Watcher"
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
echo "[SH] Centinela lanzado en ventana nueva."
