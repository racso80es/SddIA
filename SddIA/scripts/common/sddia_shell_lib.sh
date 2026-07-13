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
  local candidate
  for candidate in \
    "$repo_root/SddIA/target/debug/execute-process" \
    "$repo_root/SddIA/target/release/execute-process"; do
    if _sddia_is_native_elf "$candidate"; then
      SDDIA_EXECUTE_PROCESS_BIN="$candidate"
      export SDDIA_EXECUTE_PROCESS_BIN
      return 0
    fi
  done
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
      export "$key=$value"
    done < "$file"
  done
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
