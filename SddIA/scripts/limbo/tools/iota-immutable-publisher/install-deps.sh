#!/usr/bin/env bash
# Dependencias de iota-immutable-publisher (anclaje IOTA Testnet real)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

if ! command -v npm >/dev/null 2>&1; then
  echo '{"success":false,"exitCode":1,"error":"npm no encontrado; instale Node.js 20+ (nodejs + npm)"}' >&2
  exit 1
fi

if [[ -f package-lock.json ]]; then
  npm ci
else
  npm install
fi

echo '{"success":true,"exitCode":0,"message":"iota-immutable-publisher deps installed"}'
