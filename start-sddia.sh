#!/bin/bash
# ==============================================================================
# SddIA - Ignición del Ecosistema Operativo
# Documentación: start-sddia.md
# ==============================================================================
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$REPO_ROOT"

_sddia_ensure_hooks_path() {
  local expected="SddIA/scripts/qa/git-hooks"
  local current
  current=$(git config --get core.hooksPath 2>/dev/null || true)
  if [[ "$current" != "$expected" ]]; then
    git config core.hooksPath "$expected"
    echo "[SddIA] core.hooksPath -> ${expected}"
  fi
}
_sddia_ensure_hooks_path

# shellcheck source=SddIA/scripts/common/sddia_shell_lib.sh
source "$REPO_ROOT/SddIA/scripts/common/sddia_shell_lib.sh"
_sddia_load_vault "$REPO_ROOT"

KALMA_PORT="${SDDIA_CLIENT_PORT:-8765}"
KALMA_HOST="127.0.0.1"
KALMA_URL="http://${KALMA_HOST}:${KALMA_PORT}"
DAEMON_LAUNCHER_DIR="SddIA/scripts/daemons"
REQUIRED_DAEMONS=(event-watcher event-sweeper)
# Perfil runtime (L-PROFILE): consumer | engineering. Default lab = engineering.
RUNTIME_PROFILE="$(echo "${SDDIA_RUNTIME_PROFILE:-engineering}" | tr '[:upper:]' '[:lower:]')"
EMAIL_DAEMON="email-watcher"
EMAIL_DAEMON_STARTED=0
STATUS_DIR=".SddIA/daemons/status"
IOTA_RELAY_DIR="${SDDIA_IOTA_RELAY_DIR:-.SddIA/services/iota-publish-relay}"
IOTA_DLT_REQUIRED=0
HEARTBEAT_AUDIT=".SddIA/daemons/state/heartbeat-audit.json"
CLEANUP_DONE=0
IGNITION_EPOCH="$(date -u +%s)"

# Filtro C: consumidor sin github-bridge-watcher (F-04).
if [[ "$RUNTIME_PROFILE" == "consumer" || "$RUNTIME_PROFILE" == "consumidor" ]]; then
    OPTIONAL_DAEMONS=(telegram-watcher)
else
    OPTIONAL_DAEMONS=(telegram-watcher github-bridge-watcher)
fi

# L-REQUIRED: aduana DLT = centinela obligatorio si no-consumer + sin simulación + URL loopback + hijo Node presente.
_iota_dlt_required() {
    if [[ "$RUNTIME_PROFILE" == "consumer" || "$RUNTIME_PROFILE" == "consumidor" ]]; then
        return 1
    fi
    if [[ "${SDDIA_LAB_SIMULATE_IOTA:-0}" != "0" ]]; then
        return 1
    fi
    local url="${IOTA_PUBLISH_RELAY_URL:-}"
    if [[ -z "$url" ]]; then
        return 1
    fi
    if [[ ! "$url" =~ 127\.0\.0\.1|/localhost|localhost ]]; then
        return 1
    fi
    if [[ ! -f "$IOTA_RELAY_DIR/server.mjs" ]]; then
        return 1
    fi
    return 0
}

_iota_relay_health_url() {
    local url="${IOTA_PUBLISH_RELAY_URL:-}"
    if [[ -n "$url" && "$url" == */v1/publish ]]; then
        echo "${url%/v1/publish}/health"
        return 0
    fi
    local host="${IOTA_PUBLISH_RELAY_HOST:-127.0.0.1}"
    local port="${IOTA_PUBLISH_RELAY_PORT:-8787}"
    echo "http://${host}:${port}/health"
}

if _iota_dlt_required; then
    IOTA_DLT_REQUIRED=1
    REQUIRED_DAEMONS+=(iota-publish-relay)
fi
DAEMON_NAMES=("${REQUIRED_DAEMONS[@]}" "${OPTIONAL_DAEMONS[@]}")

# R-07: jurisdicción sensorial systemd ⇒ script no spawnea email/telegram.
_sensorial_under_systemd() {
    local juris
    juris="$(echo "${SDDIA_SENSORIAL_JURISDICTION:-}" | tr '[:upper:]' '[:lower:]')"
    if [[ "$juris" == "systemd" ]]; then
        return 0
    fi
    if command -v systemctl >/dev/null 2>&1; then
        if systemctl --user list-units --type=service --state=running --no-legend 2>/dev/null \
            | awk '{print $1}' | grep -E '^sddia-email-watcher@.+\.service$' >/dev/null 2>&1; then
            return 0
        fi
    fi
    return 1
}
SENSORIAL_SYSTEMD=0
if _sensorial_under_systemd; then
    SENSORIAL_SYSTEMD=1
fi

_user_systemd_bus_ok() {
    command -v systemctl >/dev/null 2>&1 || return 1
    systemctl --user show-environment >/dev/null 2>&1
}

_resolved_daemon_jurisdiction() {
    local j
    j="$(echo "${SDDIA_DAEMON_JURISDICTION:-}" | tr '[:upper:]' '[:lower:]')"
    if [[ "$j" == "systemd" || "$j" == "script" ]]; then
        printf '%s\n' "$j"
        return 0
    fi
    if _user_systemd_bus_ok; then
        printf '%s\n' "systemd"
    else
        printf '%s\n' "script"
    fi
}

DAEMON_JURIS="$(_resolved_daemon_jurisdiction)"

_ensure_orchestrator() {
    echo "[SddIA] Asegurando orquestador nativo (execute-process)..."
    if [[ -f "$REPO_ROOT/MANIFEST.json" ]] || [[ ! -f "$REPO_ROOT/SddIA/Cargo.toml" ]]; then
        _sddia_discard_foreign_orchestrator_pin "$REPO_ROOT"
        if _sddia_resolve_orchestrator "$REPO_ROOT"; then
            echo "  -> orquestador (bundle): ${SDDIA_EXECUTE_PROCESS_BIN}"
            return 0
        fi
        echo "  -> [ERROR] bundle hermético: ELF execute-process ausente. Reinyectar bundle (no cargo)." >&2
        return 1
    fi
    local target_dir="$REPO_ROOT/SddIA/target"
    local build_log
    if ! build_log="$(cd "$REPO_ROOT/SddIA" && CARGO_TARGET_DIR="$target_dir" cargo build -p execute-process -p iota-immutable-publisher -q 2>&1)"; then
        echo "  -> [ERROR] cargo build -p execute-process -p iota-immutable-publisher falló (CARGO_TARGET_DIR=${target_dir})."
        [[ -n "$build_log" ]] && echo "$build_log" >&2
        return 1
    fi
    _sddia_resolve_orchestrator "$REPO_ROOT" || return 1
    echo "  -> orquestador: ${SDDIA_EXECUTE_PROCESS_BIN}"
}

echo "[SddIA] Iniciando secuencia de ignición del núcleo..."
echo "[SddIA] Repo: $REPO_ROOT"
echo "[SddIA] Perfil runtime: ${RUNTIME_PROFILE} (SDDIA_RUNTIME_PROFILE)"
if [[ "$SENSORIAL_SYSTEMD" -eq 1 ]]; then
    echo "[SddIA] Jurisdicción sensorial: systemd (R-07 — sin spawn email/telegram desde script)"
fi
echo "[SddIA] Jurisdicción centinelas: ${DAEMON_JURIS} (SDDIA_DAEMON_JURISDICTION)"

cleanup() {
    local exit_code="${1:-0}"
    if [[ "$CLEANUP_DONE" -eq 1 ]]; then
        exit "$exit_code"
    fi
    CLEANUP_DONE=1

    echo ""
    echo "[SddIA] Interrupción detectada. Apagando el Sistema Nervioso y Kalma2..."

    if [[ "${DAEMON_JURIS:-script}" == "systemd" ]]; then
        echo "[SddIA] Jurisdicción systemd: no pkill de centinelas (supervisor = systemd --user)."
        exit "$exit_code"
    fi

    kill $(jobs -p) 2>/dev/null || true

    local daemon
    for daemon in "${DAEMON_NAMES[@]}"; do
        _sddia_stop_lock_pid "${STATUS_DIR}/${daemon}.lock"
    done
    _sddia_stop_lock_pid "${STATUS_DIR}/kalma2-bridge.lock"
    _sddia_stop_lock_pid "${STATUS_DIR}/${EMAIL_DAEMON}.lock"

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
        if [[ "$name" == "iota-publish-relay" ]]; then
            local health
            health="$(_iota_relay_health_url)"
            if ! _wait_http "$health" "IOTA Relay" 30 0.5; then
                echo "  -> [ERROR] ${name}: sin binding /health en ${health} (prohibido ACTIVO no verificado)"
                return 1
            fi
            echo "  -> ${name}: ACTIVO (pid=${pid}, health=${health})"
            return 0
        fi
        echo "  -> ${name}: ACTIVO (pid=${pid}, binario nativo=${executable})"
        return 0
    fi

    echo "  -> [ERROR] ${name} no arrancó como binario nativo (revisar .SddIA/daemons/logs/${name}.log)"
    return 1
}

_warn_xdg_linger() {
    if [[ -z "${XDG_RUNTIME_DIR:-}" ]]; then
        echo "[SddIA] [WARN] XDG_RUNTIME_DIR vacío: systemctl --user no tiene bus. Active sesión o: loginctl enable-linger $(id -un)"
    fi
    if command -v loginctl >/dev/null 2>&1; then
        local linger
        linger="$(loginctl show-user "$(id -un)" -p Linger --value 2>/dev/null || true)"
        if [[ "$linger" != "yes" ]]; then
            echo "[SddIA] [WARN] Linger inactivo: reboot no re-ignita unidades --user. Active: loginctl enable-linger $(id -un)"
        fi
    fi
}

_instance_unit_escape() {
    systemd-escape -p "$REPO_ROOT"
}

_materialize_systemd_units() {
    local dest="$REPO_ROOT/.SddIA/systemd"
    local tpl="$REPO_ROOT/SddIA/templates/systemd"
    local factory="$tpl/sddia-daemon@.service.template"
    local email="$tpl/sddia-email-watcher@.service.template"
    local name body
    mkdir -p "$dest"
    if [[ -f "$factory" ]]; then
        for name in event-watcher event-sweeper kalma2-bridge telegram-watcher github-bridge-watcher iota-publish-relay; do
            body="$(cat "$factory")"
            body="${body//@@DAEMON_NAME@@/$name}"
            printf '%s\n' "$body" >"$dest/sddia-${name}@.service"
        done
    fi
    if [[ -f "$email" ]]; then
        body="$(cat "$email")"
        printf '%s\n' "$body" >"$dest/sddia-email-watcher@.service"
    fi
}

_sync_user_systemd_units() {
    local src dest f
    src="$REPO_ROOT/.SddIA/systemd"
    dest="${XDG_CONFIG_HOME:-$HOME/.config}/systemd/user"
    if [[ ! -d "$src" ]]; then
        echo "[SddIA] [ERROR] unidades ausentes en ${src} (re-ejecute instance-creator o copie SddIA/templates/systemd)." >&2
        return 1
    fi
    mkdir -p "$dest"
    local found=0
    for f in "$src"/*.service; do
        [[ -f "$f" ]] || continue
        found=1
        cp -f "$f" "$dest/"
    done
    if [[ "$found" -eq 0 ]]; then
        echo "[SddIA] [ERROR] ningún .service en ${src}" >&2
        return 1
    fi
    systemctl --user daemon-reload
}

_enable_instance_unit() {
    local stem="$1"
    local esc
    esc="$(_instance_unit_escape)"
    systemctl --user enable --now "${stem}@${esc}.service"
}

_systemd_ignite() {
    _warn_xdg_linger
    if ! command -v systemd-escape >/dev/null 2>&1; then
        echo "[SddIA] [ERROR] systemd-escape ausente." >&2
        return 1
    fi
    _materialize_systemd_units
    _sync_user_systemd_units || return 1

    _enable_instance_unit "sddia-event-watcher" || return 1
    _enable_instance_unit "sddia-event-sweeper" || return 1
    _enable_instance_unit "sddia-kalma2-bridge" || return 1

    if [[ -n "${SDDIA_EMAIL_IMAP_HOST:-}" || "$SENSORIAL_SYSTEMD" -eq 1 ]]; then
        if [[ -f "$REPO_ROOT/.SddIA/systemd/sddia-email-watcher@.service" ]]; then
            _enable_instance_unit "sddia-email-watcher" || echo "  -> [WARN] sddia-email-watcher@%f no enable"
        fi
    fi

    if [[ -n "${TELEGRAM_BOT_TOKEN:-}" ]]; then
        _enable_instance_unit "sddia-telegram-watcher" || echo "  -> [WARN] sddia-telegram-watcher@%f no enable"
    fi

    if [[ "$RUNTIME_PROFILE" != "consumer" && "$RUNTIME_PROFILE" != "consumidor" ]]; then
        _enable_instance_unit "sddia-github-bridge-watcher" || echo "  -> [WARN] sddia-github-bridge-watcher@%f no enable"
    fi

    # L-REQUIRED: enable relay ANTES del exit 0 de jurisdicción systemd (ceguera histórica L441).
    if [[ "$IOTA_DLT_REQUIRED" -eq 1 ]]; then
        _enable_instance_unit "sddia-iota-publish-relay" || return 1
        local health
        health="$(_iota_relay_health_url)"
        if ! _wait_http "$health" "IOTA Relay" 30 0.5; then
            echo "  -> [ERROR] iota-publish-relay sin /health en ${health}"
            return 1
        fi
        echo "  -> iota-publish-relay: ACTIVO (systemd + ${health})"
    fi
    return 0
}

_required_heartbeats_ready() {
    python3 - "$HEARTBEAT_AUDIT" "$IGNITION_EPOCH" "${REQUIRED_DAEMONS[@]}" <<'PY'
import json, sys, time
from pathlib import Path
from datetime import datetime, timezone

audit_path = Path(sys.argv[1])
ignition = int(sys.argv[2])
required = sys.argv[3:]
if not audit_path.is_file():
    sys.exit(1)
try:
    body = json.loads(audit_path.read_text(encoding="utf-8"))
except Exception:
    sys.exit(1)
daemons = body.get("daemons") or {}
now = int(time.time())
for name in required:
    entry = daemons.get(name) or {}
    last = entry.get("last_heartbeat_at")
    if not last:
        sys.exit(1)
    try:
        # Accept Z or +00:00
        ts = last.replace("Z", "+00:00")
        dt = datetime.fromisoformat(ts)
        if dt.tzinfo is None:
            dt = dt.replace(tzinfo=timezone.utc)
        epoch = int(dt.timestamp())
    except Exception:
        sys.exit(1)
    # Latido posterior al arranque, o al menos fresco (< 90s).
    if epoch < ignition and (now - epoch) > 90:
        sys.exit(1)
    missed = int(entry.get("missed_cycles") or 0)
    if missed >= 3:
        sys.exit(1)
sys.exit(0)
PY
}

# Ingesta directa de Daemon_Heartbeat (evita inanición de route-telemetry
# cuando pending/ satura event-watcher). Actualiza heartbeat-audit.json vía Argos.
_ingest_telemetry_heartbeats() {
    [[ -x ./sddia-run.sh ]] || return 0
    local rel
    while IFS= read -r rel; do
        [[ -z "$rel" ]] && continue
        ./sddia-run.sh --process daemon-heartbeat-audit \
            --inputs "{\"event_file_path\":\"${rel}\"}" >/dev/null 2>&1 || true
    done < <(python3 - "$REPO_ROOT" "${REQUIRED_DAEMONS[@]}" <<'PY'
import json, sys
from pathlib import Path

repo = Path(sys.argv[1])
required = sys.argv[2:]
tel = repo / ".events" / "telemetry"
if not tel.is_dir():
    sys.exit(0)

latest = {name: None for name in required}  # name -> (mtime, rel)
for path in tel.glob("*.json"):
    try:
        body = json.loads(path.read_text(encoding="utf-8"))
    except Exception:
        continue
    if body.get("event_type") != "Daemon_Heartbeat":
        continue
    payload = body.get("payload") or {}
    name = payload.get("daemon_name")
    if name not in latest:
        continue
    mtime = path.stat().st_mtime
    prev = latest[name]
    if prev is None or mtime > prev[0]:
        rel = path.relative_to(repo).as_posix()
        latest[name] = (mtime, rel)

for name in required:
    entry = latest.get(name)
    if entry:
        print(entry[1])
PY
)
}

_wait_required_heartbeats() {
    local attempts="${1:-45}"
    local delay="${2:-1}"
    local i

    echo "[SddIA] Esperando Daemon_Heartbeat auditado de obligatorios..."
    for ((i = 1; i <= attempts; i++)); do
        # 1) Ingestar latidos recientes desde telemetry (gate no depende solo del fan-out).
        _ingest_telemetry_heartbeats
        # 2) Sweep Argos (staleness / fracturas; no bloqueante si falla el orquestador).
        if [[ -x ./sddia-run.sh ]]; then
            ./sddia-run.sh --process daemon-heartbeat-audit --inputs '{"sweep":true}' >/dev/null 2>&1 || true
        fi
        if _required_heartbeats_ready; then
            echo "  -> heartbeats obligatorios: OK (audit fresco, missed_cycles<3)"
            return 0
        fi
        sleep "$delay"
    done

    echo "  -> [ERROR] heartbeats de ${REQUIRED_DAEMONS[*]} no confirmados en ${attempts}s."
    return 1
}

if ! _ensure_orchestrator; then
    cleanup 1
fi

if [[ "$DAEMON_JURIS" == "systemd" ]]; then
    echo "[SddIA] Levantando Sistema Nervioso vía systemd --user (@%f)..."
    if ! _systemd_ignite; then
        echo "[SddIA] [ERROR] enable --now de unidades de instancia falló."
        cleanup 1
    fi

    if ! _wait_http "${KALMA_URL}/" "Kalma2" 30 0.5; then
        echo "  -> [ERROR] Kalma2 no alcanzable; revise sddia-kalma2-bridge@%f y SDDIA_CLIENT_PORT."
        cleanup 1
    fi
    echo "  -> Kalma2: ACTIVO (${KALMA_URL}) [systemd]"

    if ! _wait_required_heartbeats 45 1; then
        cleanup 1
    fi

    echo "===================================================================="
    echo "[SddIA] Ecosistema S+ Grade operativo (systemd --user)."
    echo "[SddIA] Unidades enable --now con instancia $(systemd-escape -p "$REPO_ROOT")."
    echo "[SddIA] Kalma2: ${KALMA_URL}"
    echo "[SddIA] Este script no retiene hijos; stop: systemctl --user stop 'sddia-*@$(systemd-escape -p "$REPO_ROOT").service'"
    echo "===================================================================="
    exit 0
fi

# 1. Centinelas (Sistema Nervioso EDA) — jurisdicción script
echo "[SddIA] Levantando Sistema Nervioso (Demonios)..."

for name in "${REQUIRED_DAEMONS[@]}"; do
    _start_daemon "$name" || {
        echo "[SddIA] [ERROR] Centinela obligatorio no disponible: ${name}."
        cleanup 1
    }
done

OPTIONAL_DAEMONS_OK=0
for name in "${OPTIONAL_DAEMONS[@]}"; do
    # R-07: telegram-watcher bajo systemd sensorial no se spawnea desde script.
    if [[ "$SENSORIAL_SYSTEMD" -eq 1 && "$name" == "telegram-watcher" ]]; then
        echo "  -> ${name}: omitido (jurisdicción systemd R-07)"
        continue
    fi
    if _start_daemon "$name"; then
        ((OPTIONAL_DAEMONS_OK++)) || true
    fi
done

if [[ "$SENSORIAL_SYSTEMD" -eq 1 ]]; then
    echo "[SddIA] ${EMAIL_DAEMON} omitido (jurisdicción systemd R-07 — unidad @%f responsable)."
elif [[ -n "${SDDIA_EMAIL_IMAP_HOST:-}" ]]; then
    echo "[SddIA] Centinela sensorial IMAP (${EMAIL_DAEMON})..."
    if _start_daemon "$EMAIL_DAEMON"; then
        EMAIL_DAEMON_STARTED=1
        ((OPTIONAL_DAEMONS_OK++)) || true
    else
        echo "  -> [WARN] ${EMAIL_DAEMON} no arrancó (revise SDDIA_EMAIL_* en bóveda instancia)."
    fi
else
    echo "[SddIA] ${EMAIL_DAEMON} omitido (SDDIA_EMAIL_IMAP_HOST no configurado en bóveda)."
fi

# 2. Kalma2 (puente HTTP nativo Rust) — hereda bóveda ya exportada
echo "[SddIA] Levantando el puente de Kalma2 (kalma2-bridge)..."

if [[ -n "${SDDIA_LLM_CHAT_COMMAND:-}${SDDIA_LLM_CLI_COMMAND:-}" ]]; then
    echo "  -> bóveda LLM: SDDIA_LLM_*_COMMAND exportada (chat SSE habilitado)"
else
    echo "  -> [WARN] bóveda sin SDDIA_LLM_CHAT_COMMAND/CLI_COMMAND; POST /api/chat emitirá System_Fracture_Detected"
fi

BRIDGE_BIN="$(_resolve_bridge_bin || true)"
if [[ -z "$BRIDGE_BIN" ]]; then
    echo "  -> [ERROR] kalma2-bridge nativo no encontrado. Compilar: cd SddIA && cargo build -p kalma2-bridge"
    cleanup 1
fi
echo "  -> kalma2-bridge: binario nativo=${BRIDGE_BIN}"

# Aduana DLT: centinela iota-publish-relay ya arrancado vía REQUIRED_DAEMONS / _start_daemon
# (L-SUPERVISOR + L-HEALTH). Sin hijo Node inline ni ACTIVO no verificado.
if [[ "$IOTA_DLT_REQUIRED" -eq 0 ]]; then
    echo "  -> IOTA Relay: omitido (no L-REQUIRED: consumer/simulación/URL no-loopback/hijo ausente)"
fi

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

if ! _wait_required_heartbeats 45 1; then
    cleanup 1
fi

echo "===================================================================="
echo "[SddIA] Ecosistema S+ Grade operativo."
echo "[SddIA] Centinelas obligatorios: ${#REQUIRED_DAEMONS[@]}/${#REQUIRED_DAEMONS[@]}; opcionales: ${OPTIONAL_DAEMONS_OK}/$(( ${#OPTIONAL_DAEMONS[@]} + ($EMAIL_DAEMON_STARTED) ))"
if [[ "$EMAIL_DAEMON_STARTED" -eq 1 ]]; then
    echo "[SddIA] ${EMAIL_DAEMON}: ACTIVO (circuito sensorial correo)"
fi
echo "[SddIA] Kalma2 disponible en: ${KALMA_URL}"
echo "[SddIA] Presiona Ctrl+C para realizar una desconexión limpia."
echo "===================================================================="

wait
