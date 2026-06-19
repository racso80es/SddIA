#!/bin/bash
# ==============================================================================
# SddIA - Ignición del Ecosistema Operativo
# Documentación: start-sddia.md
# ==============================================================================
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$REPO_ROOT"

KALMA_PORT="${SDDIA_CLIENT_PORT:-8765}"
KALMA_HOST="127.0.0.1"
KALMA_URL="http://${KALMA_HOST}:${KALMA_PORT}"
DAEMON_LAUNCHER_DIR="SddIA/scripts/daemons"
BRIDGE=".SddIA/client/sddia-client-bridge.py"
DAEMON_NAMES=(event-watcher event-sweeper telegram-watcher github-bridge-watcher)
OPTIONAL_DAEMONS=(telegram-watcher github-bridge-watcher)
CLEANUP_DONE=0

echo "[SddIA] Iniciando secuencia de ignición del núcleo..."
echo "[SddIA] Repo: $REPO_ROOT"

cleanup() {
    local exit_code="${1:-0}"
    if [[ "$CLEANUP_DONE" -eq 1 ]]; then
        exit "$exit_code"
    fi
    CLEANUP_DONE=1

    echo ""
    echo "[SddIA] Interrupción detectada. Apagando el Sistema Nervioso y Kalma2..."

    kill $(jobs -p) 2>/dev/null || true

    local daemon
    for daemon in "${DAEMON_NAMES[@]}"; do
        pkill -x "$daemon" 2>/dev/null || true
    done

    pkill -f "sddia-client-bridge.py" 2>/dev/null || true

    echo "[SddIA] Ecosistema detenido de forma segura."
    exit "$exit_code"
}

trap cleanup SIGINT SIGTERM

_wait_http() {
    local url="$1"
    local label="$2"
    local attempts="${3:-30}"
    local delay="${4:-0.5}"
    local i

    for ((i = 1; i <= attempts; i++)); do
        if curl -sf "$url" >/dev/null 2>&1; then
            return 0
        fi
        sleep "$delay"
    done

    echo "  -> [ERROR] ${label} no respondió en ${url} tras ${attempts} intentos."
    return 1
}

_start_daemon() {
    local name="$1"
    local required="${2:-true}"
    local launcher="${DAEMON_LAUNCHER_DIR}/${name}.sh"

    if [[ ! -f "$launcher" ]]; then
        if [[ "$required" == true ]]; then
            echo "  -> [AVISO] ${name}.sh no encontrado (${launcher})."
        fi
        return 1
    fi

    bash "$launcher" &
    local pid=$!
    sleep 1

    if kill -0 "$pid" 2>/dev/null || pgrep -x "$name" >/dev/null 2>&1; then
        echo "  -> ${name}: ACTIVO (pid wrapper=${pid})"
        return 0
    fi

    echo "  -> [ERROR] ${name} no arrancó (revisar .SddIA/daemons/logs/${name}.log)"
    return 1
}

# 1. Centinelas (Sistema Nervioso EDA)
echo "[SddIA] Levantando Sistema Nervioso (Demonios)..."

DAEMONS_OK=0
for name in event-watcher event-sweeper; do
    if _start_daemon "$name" true; then
        ((DAEMONS_OK++)) || true
    fi
done

for name in "${OPTIONAL_DAEMONS[@]}"; do
    if _start_daemon "$name" false; then
        ((DAEMONS_OK++)) || true
    fi
done

if [[ "$DAEMONS_OK" -lt 2 ]]; then
    echo "[SddIA] [ERROR] Centinelas obligatorios incompletos (event-watcher + event-sweeper)."
    cleanup 1
fi

# 2. Kalma2 (puente HTTP local)
echo "[SddIA] Levantando el puente de Kalma2..."

if [[ ! -f "$BRIDGE" ]]; then
    echo "  -> [ERROR] Puente no encontrado: ${BRIDGE}"
    cleanup 1
fi

python3 "$BRIDGE" &
KALMA_PID=$!
sleep 0.5

if ! kill -0 "$KALMA_PID" 2>/dev/null; then
    echo "  -> [ERROR] sddia-client-bridge.py terminó al arrancar."
    cleanup 1
fi

if ! _wait_http "${KALMA_URL}/" "Kalma2" 30 0.5; then
    echo "  -> [ERROR] Kalma2 no alcanzable; revise bundle UI (interfaces/kalma2) y dependencias Python."
    cleanup 1
fi

echo "  -> Kalma2: ACTIVO (${KALMA_URL})"

echo "===================================================================="
echo "[SddIA] Ecosistema S+ Grade operativo."
echo "[SddIA] Centinelas activos: ${DAEMONS_OK}/${#DAEMON_NAMES[@]}"
echo "[SddIA] Kalma2 disponible en: ${KALMA_URL}"
echo "[SddIA] Presiona Ctrl+C para realizar una desconexión limpia."
echo "===================================================================="

wait
