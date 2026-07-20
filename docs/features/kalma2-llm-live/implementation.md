---
feature_name: kalma2-llm-live
created: "2026-07-20"
process: feature
---

# Implementation — kalma2-llm-live

## Cambios

| Artefacto | Cambio |
|-----------|--------|
| `kalma2-bridge` | `POST /api/chat` SSE + watchdog + `System_Fracture_Detected`; `POST /api/execute`; `/api/interact` alias por `mode` |
| `mayeuta-llm` | Operación `STREAM` (pipe stdout subproceso); `SDDIA_LLM_CHAT_COMMAND` precede a `SDDIA_LLM_CLI_COMMAND` |
| `handlers/kalma2.rs` | `mode=execute\|chat` determinista (L-CI); legado CLASSIFY solo sin mode |
| `kalma2-agent-runtime-cursor.py` | Dual-mode `CHAT_STREAM` + `AGENT_PHASE`; prótesis SQLite `state.vscdb` (`cursorDiskKV` + `composer.composerHeaders`) |

## Notas

- Inserción SQLite: `composerData` + bubbles user/assistant + índice `composer.composerHeaders` + fila `composerHeaders`.
- Inferencia: `SDDIA_LLM_INFER_COMMAND` / `SDDIA_AGENT_RUNTIME_CLI` (nunca reentra en el `.py` prótesis).
- Escribir en la DB live de Cursor con el IDE abierto puede contender por WAL; preferir lab con `SDDIA_CURSOR_VSCDB` de copia o cerrar Cursor.
- Genoma skill `.md` / índices: pendiente sync vía `entity-manager` si se formaliza operación STREAM en contrato.
