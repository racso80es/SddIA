---
feature_name: kalma2-llm-live
created: "2026-07-20"
process: feature
items:
  - kalma2-bridge-api-chat-sse
  - kalma2-bridge-api-execute
  - mayeuta-llm-stream
  - kalma2-handler-mode-deterministic
  - agent-runtime-dual-mode
  - ui-chat-forge
correlation_id: "00000000-0000-4000-8000-0000000000hc"
---

# Implementation — kalma2-llm-live

## Touchpoints (plan fases 1–5) — auditados presentes

| Artefacto | Cambio | Evidencia en árbol |
|-----------|--------|-------------------|
| `SddIA/interfaces/kalma2-bridge` | `POST /api/chat` SSE + watchdog + `System_Fracture_Detected`; `POST /api/execute`; `/api/interact` por `mode` | `main.rs` rutas `/api/chat` `/api/execute` + emit fractura |
| `SddIA/skills/mayeuta-llm` | Op `STREAM` (pipe stdout); `SDDIA_LLM_CHAT_COMMAND` ≻ `SDDIA_LLM_CLI_COMMAND` | `src/main.rs` `OP_STREAM` |
| `SddIA/engine/.../handlers/kalma2.rs` | `mode=execute\|chat` determinista (L-CI); CLASSIFY solo legado sin mode | ramas `mode == "execute"\|"chat"` |
| `SddIA/scripts/tools/kalma2-agent-runtime-cursor.py` | Dual-mode; `--trust` auto; oracle CLI; reject IDE_WATCH; wake | `run_chat_stream` / `resolve_*` |
| Smokes | S1–S5 + HOST-D live + HOST-B2 SSE | `kalma2-*-smoke.sh` |

## Notas

- Inserción SQLite: `composerData` + bubbles + headers. Oráculo = CLI (L-IDE).
- Inferencia: `SDDIA_LLM_INFER_COMMAND` / autodetection con `--trust`.
- Deuda host §9 + §11 absorbida — ver `validacion.md` / PBI v2.3.3.
