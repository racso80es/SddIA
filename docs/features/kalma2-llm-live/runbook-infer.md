---
feature_name: kalma2-llm-live
created: "2026-07-20"
process: feature
purpose: Runbook S1/S2 — inferencia live Kalma2 chat (L-INF)
---

# Runbook — Inferencia live (S1/S2)

## Objetivo

Chat SSE con `backend=cli` (≠ `sqlite-ack`, ≠ `*_MOCK`).

## Instalación Cursor Agent CLI (operador)

```bash
curl https://cursor.com/install -fsS | bash
export PATH="$HOME/.local/bin:$PATH"
command -v cursor-agent || command -v agent
agent --version   # o cursor-agent --version
```

Autenticación si el CLI lo exige: `agent login`.

## Bóveda (`.dev/.env` — no versionar secretos)

```bash
# Preferente (comillas: espacios/flags; --trust para no-interactivo)
SDDIA_LLM_INFER_COMMAND="/home/racso/.local/bin/cursor-agent --print --mode ask --trust"
# Alternativa (fases agent)
# SDDIA_AGENT_RUNTIME_CLI="/home/racso/.local/bin/cursor-agent --print --trust"

# Chat inyector (prótesis SQLite + stream)
SDDIA_LLM_CLI_COMMAND=python3 SddIA/scripts/tools/kalma2-agent-runtime-cursor.py

# Demo live: sin mock
# unset SDDIA_LLM_CHAT_MOCK SDDIA_AGENT_RUNTIME_MOCK
SDDIA_LLM_REQUIRE_INFER=1
```

Auth: `agent login` (o `CURSOR_API_KEY`). Host no-interactivo: añadir `--trust` (o `-f`/`--yolo`) tras `Workspace Trust Required`.

Reiniciar `kalma2-bridge` / `start-sddia` tras cambiar bóveda.

## Lab de cableado (sin Cursor CLI)

```bash
./SddIA/scripts/tools/kalma2-chat-infer-smoke.sh
```

Usa `kalma2-llm-infer-lab.sh` → prueba pipeline ≠ `sqlite-ack`. **No** sustituye live.

## Criterios

| Check | OK |
|-------|-----|
| Meta SSE `[kalma2-meta] {"backend":"cli",...}` | sí |
| Cuerpo sin `sqlite-ack` | sí |
| `*_MOCK` unset en demo | sí |
| `SDDIA_LLM_REQUIRE_INFER=1` sin CLI → exit ≠ 0 | sí |
