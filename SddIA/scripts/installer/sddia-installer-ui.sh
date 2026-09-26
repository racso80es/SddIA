#!/usr/bin/env bash
# Presentador TTY para deploy/teardown (invoca fachada; motor sin prompts).
set -euo pipefail
UI_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FORGE_ROOT="$(cd "$UI_DIR/../../.." && pwd)"
FACADE="$FORGE_ROOT/sddia-installer.sh"
IO_PY="$FORGE_ROOT/SddIA/scripts/installer/installer_io.py"
DEFAULT_ROOT="/home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA"

NO_HOLD=0
YES=0
PASSTHRU=()

_die_usage() {
  echo "Uso: sddia-installer-ui.sh deploy|teardown [opciones fachada] [--yes] [--no-hold]" >&2
  exit 1
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --no-hold) NO_HOLD=1; shift ;;
    --yes) YES=1; shift ;;
    -h|--help)
      _die_usage
      ;;
    *) PASSTHRU+=("$1"); shift ;;
  esac
done

[[ ${#PASSTHRU[@]} -ge 1 ]] || _die_usage
CMD="$(echo "${PASSTHRU[0]}" | tr '[:upper:]' '[:lower:]')"
case "$CMD" in
  deploy|teardown) ;;
  *) _die_usage ;;
esac

_ui_tty() {
  [[ -t 0 && -t 1 ]]
}

_color() {
  local code="$1"
  if [[ -n "${NO_COLOR:-}" ]] || ! _ui_tty; then
    return 0
  fi
  if tput setaf "$code" >/dev/null 2>&1; then
    tput setaf "$code"
  fi
}

_color_reset() {
  if [[ -n "${NO_COLOR:-}" ]] || ! _ui_tty; then
    return 0
  fi
  tput sgr0 >/dev/null 2>&1 || true
}

_resolve_install_root() {
  local i=0 arg
  while [[ $i -lt ${#PASSTHRU[@]} ]]; do
    arg="${PASSTHRU[$i]}"
    if [[ "$arg" == "--root" ]]; then
      echo "${PASSTHRU[$((i + 1))]:-}"
      return 0
    fi
    i=$((i + 1))
  done
  if [[ -n "${SDDIA_INSTALL_ROOT:-}" ]]; then
    echo "$SDDIA_INSTALL_ROOT"
    return 0
  fi
  echo "$DEFAULT_ROOT"
}

_root_is_live() {
  local root="$1"
  [[ -d "$root/.SddIA" ]]
}

_teardown_prepare_argv() {
  if [[ "$CMD" != "teardown" ]]; then
    return 0
  fi
  if [[ " ${PASSTHRU[*]} " == *" --force "* ]]; then
    return 0
  fi
  if [[ "$YES" -eq 1 ]]; then
    PASSTHRU+=(--force)
    return 0
  fi
  if ! _ui_tty; then
    return 0
  fi
  local root_raw esc root live
  root_raw="$(_resolve_install_root)"
  if [[ "$root_raw" != /* ]]; then
    root_raw="$FORGE_ROOT/$root_raw"
  fi
  root="$(realpath -m "$root_raw")"
  esc="$(systemd-escape -p "$root")"
  live="no"
  if _root_is_live "$root"; then
    live="sí (instancia detectada)"
  fi
  echo ""
  echo "Eliminar cliente SddIA"
  echo "  Destino: $root"
  echo "  esc:     $esc"
  echo "  Activo:  $live"
  echo ""
  echo -n "Escriba ELIMINAR para confirmar: "
  local confirm=""
  if ! IFS= read -r confirm; then
    confirm=""
  fi
  if [[ "$confirm" != "ELIMINAR" ]]; then
    python3 "$IO_PY" static-envelope \
      --command teardown \
      --root "$root" \
      --esc "$esc" \
      --exit-code 3 \
      --error-code TEARDOWN_REQUIRES_FORCE \
      --message "teardown cancelado: se requiere confirmación ELIMINAR o --yes"
    exit 3
  fi
  PASSTHRU+=(--force)
}

_run_facade_plain() {
  exec "$FACADE" "${PASSTHRU[@]}"
}

_print_step_line() {
  local line="$1"
  python3 -c '
import json, sys
line = sys.argv[1]
try:
    d = json.loads(line)
except json.JSONDecodeError:
    sys.exit(0)
if d.get("kind") != "step" or d.get("moment") != "end":
    sys.exit(0)
idx = d.get("index", "?")
tot = d.get("total", "?")
title = d.get("title", d.get("id", ""))
st = d.get("status", "ok")
print(f"{idx}/{tot} — {title}: {st}")
' "$line"
}

_render_summary() {
  local env_file="$1"
  python3 -c '
import json, sys
d = json.load(open(sys.argv[1]))
r = d.get("result") or {}
units = r.get("units") or {}
enabled = units.get("enabled") or []
skipped = units.get("skipped") or []
verify = r.get("verify") or {}
ok = d.get("success")
res = "OK" if ok else "FALLO"
port = r.get("wui_port") or r.get("plan", {}).get("wui_port") or "—"
act = ", ".join(enabled) if enabled else "—"
omit = ", ".join(skipped) if skipped else "—"
audit = verify.get("audit_ref") or "—"
log_ref = r.get("log_ref") or "—"
dur = d.get("durationMs", 0)
print(f"Resultado:      {res}")
print(f"Puerto WUI:     {port}")
print(f"Servicios:      {act}")
print(f"Omitidos:       {omit}")
print(f"Acta:           {audit}")
print(f"Log:            {log_ref}")
print(f"Duración:       {dur} ms")
err = r.get("error") or d.get("error")
if err and not ok:
    code = err.get("code", "")
    msg = err.get("message", d.get("message", ""))
    step = err.get("step", "")
    if step:
        print(f"Paso fallido:   {step}")
    if code:
        print(f"error.code:     {code}")
    if msg:
        print(f"Mensaje:        {msg}")
' "$env_file"
}

_hold_prompt() {
  if [[ "$NO_HOLD" -eq 1 ]] || [[ "${SDDIA_INSTALLER_HOLD:-1}" == "0" ]]; then
    return 0
  fi
  if ! _ui_tty; then
    return 0
  fi
  echo ""
  echo -n "Pulse una tecla para cerrar… "
  read -rsn1 -t 600 _ || true
  echo ""
}

_run_facade_tty() {
  local fifo out rc
  fifo="$(mktemp -u)"
  mkfifo "$fifo"
  out="$(mktemp)"
  trap 'rm -f "$fifo" "$out"' RETURN

  (
    exec 3>"$fifo"
    export SDDIA_INSTALLER_FD3=1
    set +e
    "$FACADE" "${PASSTHRU[@]}" >"$out"
    echo $? >"${out}.rc"
  ) &

  local child=$!
  while IFS= read -r line <"$fifo" 2>/dev/null || true; do
    [[ -z "$line" ]] && continue
    local rendered
    rendered="$(_print_step_line "$line" || true)"
    if [[ -n "$rendered" ]]; then
      if [[ "$rendered" == *": failed"* ]]; then
        _color 1
        echo "$rendered"
        _color_reset
      else
        echo "$rendered"
      fi
    fi
  done &
  local reader=$!

  wait "$child" 2>/dev/null || true
  wait "$reader" 2>/dev/null || true
  rc=1
  if [[ -f "${out}.rc" ]]; then
    rc="$(cat "${out}.rc")"
  fi

  if [[ ! -s "$out" ]]; then
    echo "Presentador: sin envelope; salida cruda del motor." >&2
    exit "${rc:-1}"
  fi

  if ! python3 -c 'import json,sys; json.load(open(sys.argv[1]))' "$out" 2>/dev/null; then
    cat "$out"
    exit "${rc:-1}"
  fi

  echo ""
  _render_summary "$out"
  _hold_prompt

  exit "$rc"
}

_teardown_prepare_argv

if ! _ui_tty; then
  _run_facade_plain
fi

_run_facade_tty
