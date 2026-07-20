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
| `SddIA/scripts/tools/kalma2-agent-runtime-cursor.py` | Dual-mode `CHAT_STREAM` + `AGENT_PHASE`; SQLite Foso; `REQUIRE_CLI` | `run_chat_stream` / `run_agent_phase` |
| `SddIA/scripts/tools/kalma2-agent-runtime-cursor.sh` | Wrapper producción `SDDIA_AGENT_RUNTIME_COMMAND` | exec → `.py` |
| `interfaces/kalma2/` | Botones Chat / Forjar → `/api/chat` / `/api/execute` | `index.html` + `app.js` |
| `.dev/.env.example` | Timeout SSE, infer, agent runtime, SQLite | documentado |
| Smokes S1–S5 | infer / agent-phase / sqlite / sse-fracture | `SddIA/scripts/tools/kalma2-*-smoke.sh` |

## Notas

- Inserción SQLite: `composerData` + bubbles + `composer.composerHeaders` (+ fila `composerHeaders` best-effort).
- Inferencia: `SDDIA_LLM_INFER_COMMAND` / `SDDIA_AGENT_RUNTIME_CLI` — nunca reentra el `.py` prótesis.
- Genoma skill `.md` / índices STREAM: sync formal vía `entity-manager` si se exige contrato genómico (no mutación manual).
- Deuda host §9: A/B cerrados en PBI; **C** (agent live E2E) / **D** opc / **E** cierre — fuera de forge de código lab.

## Delta esta corrida Tekton (`…hc`)

Sin mutación de código nueva: árbol ya cumple spec/plan. Entrega = sellado documental + auditoría.
