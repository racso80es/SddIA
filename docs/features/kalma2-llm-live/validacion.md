---
feature_name: kalma2-llm-live
created: "2026-07-20"
process: feature
branch: feat/kalma2-llm-live
global: APTO
pbi_archived: true
checks:
  AC1_sse_tokens: APTO
  AC2_fracture_watchdog: APTO
  AC3_execute_not_chat: APTO
  AC4_cargo_without_py: APTO
  AC5_agent_phase_json: APTO
  AC6_infer_not_ack_lab: APTO
  AC7_agent_require_cli: APTO
  AC8_sqlite_keys: APTO
  AC9_closure_docs: APTO
  host_cursor_agent_live: DEUDA
git_changes:
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - SddIA/skills/mayeuta-llm/src/main.rs
  - SddIA/engine/execute-process/src/engine/handlers/kalma2.rs
  - SddIA/scripts/tools/kalma2-agent-runtime-cursor.py
  - SddIA/scripts/tools/kalma2-*-smoke.sh
  - interfaces/kalma2/
  - docs/features/kalma2-llm-live/
  - docs/todos/done/[FEATURE] kalma2-llm-live — ejecución real Cursor desde Kalma2 (f0f1b1ec).md
---

# Validación — kalma2-llm-live

## Veredicto

**APTO** (Argos lab) para circuito Kalma2 S+ Grade (SSE, execute, STREAM, prótesis SQLite, smokes S1–S5).  
**Deuda host:** `cursor-agent` ausente → chat/agent live real pendiente de instalación operador (`runbook-infer.md` / `runbook-agent.md`).

## Checks

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| AC1 | SSE tokens stdout Python | ✅ | smoke chat + bridge |
| AC2 | Colapso/watchdog → `System_Fracture_Detected` | ✅ | `kalma2-sse-fracture-smoke.sh` |
| AC3 | Execute ≠ texto libre | ✅ | `/api/execute` + mode execute |
| AC4 | Sin `.py` → `cargo build --release` Core | ✅ | estructurales (sin dep Cargo) |
| AC5 | AGENT_PHASE JSON intacto | ✅ | smoke MOCK + unit agent_runtime |
| AC6 | Infer ≠ sqlite-ack (lab) | ✅ | `kalma2-chat-infer-smoke.sh` |
| AC7 | REQUIRE_CLI → failed no soft | ✅ | `kalma2-agent-phase-smoke.sh` |
| AC8 | SQLite keys | ✅ | `kalma2-sqlite-smoke.sh` |
| AC9 | Cierre documental en rama | ✅ | este archivo + PBI → done |
| Live CLI | cursor-agent en host | ⏳ | bóveda apunta a CLI inexistente |

## Comandos

```bash
./SddIA/scripts/tools/kalma2-chat-infer-smoke.sh
./SddIA/scripts/tools/kalma2-agent-phase-smoke.sh
./SddIA/scripts/tools/kalma2-sqlite-smoke.sh
./SddIA/scripts/tools/kalma2-sse-fracture-smoke.sh
```

## Cierre documental

- PBI → `docs/todos/done/` · `pbi_archived: true`
- PR único en `feat/kalma2-llm-live`
