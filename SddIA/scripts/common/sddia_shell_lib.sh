#!/usr/bin/env bash
# Resuelve SDDIA_EXECUTE_PROCESS_BIN (SSOT orquestador nativo).
set -euo pipefail

_sddia_is_native_elf() {
  local candidate="$1"
  local mime
  [[ -x "$candidate" ]] || return 1
  mime="$(file -Lb --mime-type "$candidate" 2>/dev/null || true)"
  [[ "$mime" == "application/x-executable" || "$mime" == "application/x-pie-executable" ]]
}

_sddia_elf_mtime() {
  stat -c %Y "$1" 2>/dev/null || stat -f %m "$1" 2>/dev/null
}

# F-DEP-09: pin de forja no aplica a ignición/CLI de otra raíz.
_sddia_discard_foreign_orchestrator_pin() {
  local repo_root="$1"
  [[ -n "${SDDIA_EXECUTE_PROCESS_BIN:-}" ]] || return 0
  local pin_dir pin_base abs_pin abs_root
  pin_dir="$(dirname "$SDDIA_EXECUTE_PROCESS_BIN")"
  pin_base="$(basename "$SDDIA_EXECUTE_PROCESS_BIN")"
  abs_pin="$(cd "$pin_dir" 2>/dev/null && pwd)/$pin_base" || return 0
  abs_root="$(cd "$repo_root" && pwd)"
  if [[ "$abs_pin" != "$abs_root"/* ]]; then
    echo "[CONFIG] SDDIA_EXECUTE_PROCESS_BIN fuera de ${abs_root}; ignorado (F-DEP-09)" >&2
    unset SDDIA_EXECUTE_PROCESS_BIN
  fi
}

_sddia_resolve_orchestrator() {
  local repo_root="$1"
  if [[ -n "${SDDIA_EXECUTE_PROCESS_BIN:-}" ]]; then
    SDDIA_EXECUTE_PROCESS_BIN="$(cd "$(dirname "$SDDIA_EXECUTE_PROCESS_BIN")" && pwd)/$(basename "$SDDIA_EXECUTE_PROCESS_BIN")"
    if ! _sddia_is_native_elf "$SDDIA_EXECUTE_PROCESS_BIN"; then
      echo "[ERROR] SDDIA_EXECUTE_PROCESS_BIN no es un binario ELF nativo ejecutable: $SDDIA_EXECUTE_PROCESS_BIN" >&2
      return 1
    fi
    export SDDIA_EXECUTE_PROCESS_BIN
    return 0
  fi
  local debug_bin release_bin
  debug_bin="$repo_root/SddIA/target/debug/execute-process"
  release_bin="$repo_root/SddIA/target/release/execute-process"
  local d_ok=0 r_ok=0
  if _sddia_is_native_elf "$debug_bin"; then d_ok=1; fi
  if _sddia_is_native_elf "$release_bin"; then r_ok=1; fi
  # F-DEP-07: debug solo si es estrictamente más nuevo que release (lab). Empate o stale → release.
  if [[ "$d_ok" -eq 1 && "$r_ok" -eq 1 ]]; then
    local dm rm_
    dm="$(_sddia_elf_mtime "$debug_bin")"
    rm_="$(_sddia_elf_mtime "$release_bin")"
    if [[ -n "$dm" && -n "$rm_" && "$dm" -gt "$rm_" ]]; then
      SDDIA_EXECUTE_PROCESS_BIN="$debug_bin"
    else
      SDDIA_EXECUTE_PROCESS_BIN="$release_bin"
    fi
    export SDDIA_EXECUTE_PROCESS_BIN
    return 0
  fi
  if [[ "$d_ok" -eq 1 ]]; then
    SDDIA_EXECUTE_PROCESS_BIN="$debug_bin"
    export SDDIA_EXECUTE_PROCESS_BIN
    return 0
  fi
  if [[ "$r_ok" -eq 1 ]]; then
    SDDIA_EXECUTE_PROCESS_BIN="$release_bin"
    export SDDIA_EXECUTE_PROCESS_BIN
    return 0
  fi
  echo "[ERROR] binario execute-process no encontrado. Compilar: cd SddIA && cargo build -p execute-process" >&2
  return 1
}

_sddia_load_vault() {
  local repo_root="$1"
  local global="$repo_root/.dev/.env"
  local local_env="$repo_root/.SddIA/.dev/.env"
  if [[ -f "$global" && -f "$local_env" ]]; then
    echo "[CONFIG] Jerarquía detectada: Aplicando SddIA/.dev/.env sobre .dev/.env" >&2
  fi
  local file line key value
  for file in "$global" "$local_env"; do
    [[ -f "$file" ]] || continue
    while IFS= read -r line || [[ -n "$line" ]]; do
      line="${line%%#*}"
      line="${line#"${line%%[![:space:]]*}"}"
      line="${line%"${line##*[![:space:]]}"}"
      [[ -z "$line" ]] && continue
      if [[ "$line" == export\ * ]]; then
        line="${line#export }"
      fi
      if [[ "$line" != *"="* ]]; then
        echo "[ERROR] línea dotenv inválida en $file: $line" >&2
        return 1
      fi
      key="${line%%=*}"
      value="${line#*=}"
      value="${value#"${value%%[![:space:]]*}"}"
      value="${value%"${value##*[![:space:]]}"}"
      if [[ "$value" == \"*\" && "$value" == *\" ]]; then
        value="${value:1:${#value}-2}"
      elif [[ "$value" == \'*\' && "$value" == *\' ]]; then
        value="${value:1:${#value}-2}"
      fi
      if [[ "$key" == SDDIA_LAB_SIMULATE_IOTA || "$key" == SDDIA_IOTA_TIMEOUT_SECONDS ]]; then
        export "$key=$value"
      elif [[ -z "${!key+x}" ]]; then
        export "$key=$value"
      fi
    done < "$file"
  done
}

# F-DEP-10: raíz de instancia, no ubicación del wrapper.
# Jerarquía: SDDIA_INSTANCE_ROOT (absoluto existente) → cwd con .SddIA → fallback lab (SCRIPT_DIR).
_sddia_is_instance_root() {
  local d="$1"
  [[ -n "$d" && -d "$d/.SddIA" ]]
}

_sddia_resolve_instance_root() {
  local fallback="$1"
  local cand
  if [[ -n "${SDDIA_INSTANCE_ROOT:-}" && "${SDDIA_INSTANCE_ROOT}" == /* && -d "${SDDIA_INSTANCE_ROOT}" ]]; then
    (cd "${SDDIA_INSTANCE_ROOT}" && pwd)
    return 0
  fi
  cand="${PWD}"
  if _sddia_is_instance_root "$cand"; then
    (cd "$cand" && pwd)
    return 0
  fi
  printf '%s\n' "$fallback"
}

_sddia_lock_pid_from_file() {
  local lock_file="$1"
  if [[ ! -f "$lock_file" ]]; then
    return 0
  fi
  sed -n 's/.*"pid"[[:space:]]*:[[:space:]]*\([0-9][0-9]*\).*/\1/p' "$lock_file" | head -1
}

# F-CEN-PKILL: parada = PID del lock de esa raíz. Cero pkill -x.
_sddia_stop_lock_pid() {
  local lock_file="$1"
  local pid
  pid="$(_sddia_lock_pid_from_file "$lock_file")"
  if [[ -n "${pid:-}" ]] && kill -0 "$pid" 2>/dev/null; then
    kill "$pid" 2>/dev/null || true
    sleep 1
    if kill -0 "$pid" 2>/dev/null; then
      kill -9 "$pid" 2>/dev/null || true
      sleep 1
    fi
  fi
  pid="$(_sddia_lock_pid_from_file "$lock_file")"
  if [[ -z "${pid:-}" ]] || ! kill -0 "$pid" 2>/dev/null; then
    rm -f "$lock_file"
  fi
}

_sddia_resolve_daemon_binary() {
  local repo_root="$1"
  local daemon="$2"
  local candidate
  for candidate in \
    "$repo_root/SddIA/target/release/${daemon}" \
    "$repo_root/SddIA/target/debug/${daemon}"; do
    if [[ -f "$candidate" && -x "$candidate" ]]; then
      echo "$candidate"
      return 0
    fi
  done
  echo "[ERROR] Binario no encontrado para ${daemon} bajo SddIA/target/{release|debug}/" >&2
  return 1
}
