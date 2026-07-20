#!/usr/bin/env bash
# Inferencia de laboratorio (cableado S1) — NO es Cursor live.
# Uso: SDDIA_LLM_INFER_COMMAND=SddIA/scripts/tools/kalma2-llm-infer-lab.sh
# Emite tokens por stdout distintos de sqlite-ack para validar el pipeline SSE.
set -euo pipefail
prompt="$(cat)"
# Respuesta mínima no-ack (marcador estable para asserts)
echo "[infer-lab] ok"
echo "echo:${prompt:0:120}"
