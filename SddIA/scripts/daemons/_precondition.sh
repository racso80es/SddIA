#!/usr/bin/env bash
# ExecCondition helper: exit 0 si prerequisitos del daemon existen en bóveda instancia.
set -euo pipefail
NAME="${1:-}"
ROOT="${2:-%f}"
if [[ "$ROOT" == "%f" ]]; then
  ROOT="$(pwd)"
fi
ENV_FILE="$ROOT/.SddIA/.dev/.env"
[[ -f "$ENV_FILE" ]] || exit 1

_env_nonempty() {
  local key="$1"
  local line val
  line="$(grep -E "^${key}=" "$ENV_FILE" 2>/dev/null | tail -n1 || true)"
  val="${line#*=}"
  val="${val#\"}"
  val="${val%\"}"
  [[ -n "${val// /}" ]]
}

case "$NAME" in
  telegram-watcher)
    _env_nonempty TELEGRAM_BOT_TOKEN && _env_nonempty TELEGRAM_ALLOWED_CHAT_ID || exit 1
    ;;
  email-watcher)
    _env_nonempty SDDIA_EMAIL_IMAP_HOST \
      && _env_nonempty SDDIA_EMAIL_IMAP_USER \
      && _env_nonempty SDDIA_EMAIL_IMAP_SECRET || exit 1
    ;;
  iota-publish-relay)
    if [[ -n "${SDDIA_IOTA_RELAY_DIR:-}" && -d "${SDDIA_IOTA_RELAY_DIR}" ]]; then
      exit 0
    fi
    [[ -d "$ROOT/.SddIA/services/iota-publish-relay" ]] || exit 1
    ;;
  *)
    exit 0
    ;;
esac
exit 0
