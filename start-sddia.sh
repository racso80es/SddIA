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
REQUIRED_DAEMONS=(event-watcher event-sweeper)
OPTIONAL_DAEMONS=(telegram-watcher github-bridge-watcher)
DAEMON_NAMES=("${REQUIRED_DAEMONS[@]}" "${OPTIONAL_DAEMONS[@]}")
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

    pkill -x kalma2-bridge 2>/dev/null || true

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

_resolve_bridge_bin() {
    if [[ -n "${SDDIA_KALMA2_BRIDGE_BIN:-}" ]] && _is_native_elf "${SDDIA_KALMA2_BRIDGE_BIN}"; then
        echo "${SDDIA_KALMA2_BRIDGE_BIN}"
        return 0
    fi
    local rel
    for rel in SddIA/target/debug/kalma2-bridge SddIA/target/release/kalma2-bridge; do
        if _is_native_elf "$rel"; then
            echo "$REPO_ROOT/$rel"
            return 0
        fi
    done
    return 1
}

_is_native_elf() {
    local candidate="$1"
    local mime
    [[ -x "$candidate" ]] || return 1
    mime="$(file -Lb --mime-type "$candidate" 2>/dev/null || true)"
    [[ "$mime" == "application/x-executable" || "$mime" == "application/x-pie-executable" ]]
}

_start_daemon() {
    local name="$1"
    local launcher="${DAEMON_LAUNCHER_DIR}/${name}.sh"

    if [[ ! -f "$launcher" ]]; then
        echo "  -> [ERROR] ${name}.sh no encontrado (${launcher})."
        return 1
    fi

    bash "$launcher" &
    local pid=$!
    sleep 1

    local executable
    executable="$(readlink -f "/proc/${pid}/exe" 2>/dev/null || true)"
    if kill -0 "$pid" 2>/dev/null && _is_native_elf "$executable"; then
        echo "  -> ${name}: ACTIVO (pid=${pid}, binario nativo=${executable})"
        return 0
    fi

    echo "  -> [ERROR] ${name} no arrancó como binario nativo (revisar .SddIA/daemons/logs/${name}.log)"
    return 1
}

# 1. Centinelas (Sistema Nervioso EDA)
echo "[SddIA] Levantando Sistema Nervioso (Demonios)..."

for name in "${REQUIRED_DAEMONS[@]}"; do
    _start_daemon "$name" || {
        echo "[SddIA] [ERROR] Centinela obligatorio no disponible: ${name}."
        cleanup 1
    }
done

OPTIONAL_DAEMONS_OK=0
for name in "${OPTIONAL_DAEMONS[@]}"; do
    if _start_daemon "$name"; then
        ((OPTIONAL_DAEMONS_OK++)) || true
    fi
done

# 2. Kalma2 (puente HTTP nativo Rust)
echo "[SddIA] Levantando el puente de Kalma2 (kalma2-bridge)..."

BRIDGE_BIN="$(_resolve_bridge_bin || true)"
if [[ -z "$BRIDGE_BIN" ]]; then
    echo "  -> [ERROR] kalma2-bridge nativo no encontrado. Compilar: cd SddIA && cargo build -p kalma2-bridge"
    cleanup 1
fi
echo "  -> kalma2-bridge: binario nativo=${BRIDGE_BIN}"

export SDDIA_REPO_ROOT="$REPO_ROOT"
"$BRIDGE_BIN" &
KALMA_PID=$!
sleep 0.5

if ! kill -0 "$KALMA_PID" 2>/dev/null; then
    echo "  -> [ERROR] kalma2-bridge terminó al arrancar."
    cleanup 1
fi

if ! _wait_http "${KALMA_URL}/" "Kalma2" 30 0.5; then
    echo "  -> [ERROR] Kalma2 no alcanzable; revise bundle UI (interfaces/kalma2)."
    cleanup 1
fi

echo "  -> Kalma2: ACTIVO (${KALMA_URL})"

echo "===================================================================="
echo "[SddIA] Ecosistema S+ Grade operativo."
echo "[SddIA] Centinelas obligatorios: ${#REQUIRED_DAEMONS[@]}/${#REQUIRED_DAEMONS[@]}; opcionales: ${OPTIONAL_DAEMONS_OK}/${#OPTIONAL_DAEMONS[@]}"
echo "[SddIA] Kalma2 disponible en: ${KALMA_URL}"
echo "[SddIA] Presiona Ctrl+C para realizar una desconexión limpia."
echo "===================================================================="

wait
