#!/usr/bin/env bash
# kalma2-agent-runtime-cursor — entrada producción para SDDIA_AGENT_RUNTIME_COMMAND.
# Delega en kalma2-agent-runtime-cursor.py (CLI Cursor o SDK).
#
# Bóveda (.dev/.env):
#   SDDIA_AGENT_RUNTIME_COMMAND=SddIA/scripts/tools/kalma2-agent-runtime-cursor.sh
#   SDDIA_AGENT_RUNTIME_CLI='cursor-agent --print'   # opcional; fallback SDDIA_LLM_CLI_COMMAND
#   SDDIA_AGENT_RUNTIME_BACKEND=cli|sdk              # default cli
#   SDDIA_AGENT_RUNTIME_MODEL=composer-2.5           # sdk
#   CURSOR_API_KEY=...                               # sdk
#   SDDIA_AGENT_RUNTIME_TIMEOUT_SECS=600
#   SDDIA_AGENT_RUNTIME_MOCK=1                       # CI / lab sin Cursor
set -euo pipefail
DIR="$(cd "$(dirname "$0")" && pwd)"
exec python3 "$DIR/kalma2-agent-runtime-cursor.py"
