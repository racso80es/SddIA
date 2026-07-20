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
  host_cursor_agent_live: APTO
  host_d_sqlite_live: APTO
  debt_l_ide_oracle_cli: APTO
  debt_ecst_no_reforge: APTO
  debt_secrets_not_versioned: APTO
  host_b2_sse_live: APTO
git_changes:
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - SddIA/skills/mayeuta-llm/src/main.rs
  - SddIA/engine/execute-process/src/engine/handlers/kalma2.rs
  - SddIA/scripts/tools/kalma2-agent-runtime-cursor.py
  - SddIA/scripts/tools/kalma2-*-smoke.sh
  - interfaces/kalma2/
  - docs/features/kalma2-llm-live/
  - .dev/.env.example
  - docs/todos/done/[FEATURE] kalma2-llm-live — ejecución real Cursor desde Kalma2 (f0f1b1ec).md
---

# Validación — kalma2-llm-live

## Veredicto

**APTO** — lab + host A–D + deuda §11 (L-IDE oráculo CLI, ECST no-reforge, secrets).

## Checks

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| AC1–AC9 | Lab S+ | ✅ | smokes S1–S5 |
| HOST-A…C | CLI auth + chat + AGENT_PHASE | ✅ | previos |
| HOST-D | SQLite backup L-WAL host | ✅ | `kalma2-sqlite-live-smoke.sh` |
| HOST-B2 | SSE `/api/chat` live | ✅ | `kalma2-chat-sse-live-smoke.sh` |
| DEBT-L-IDE | Oráculo CLI; reject IDE_WATCH; wake | ✅ | exit 4 + `kalma2-wake ok` |
| DEBT-ECST | No reforge; full-cycle APTO | ✅ | `kalma2-full-cycle/validacion.md` |
| DEBT-SECRETS | `.env` no trackeado | ✅ | gitignore + `.env.example` |

## Comandos

```bash
./SddIA/scripts/tools/kalma2-chat-infer-smoke.sh
./SddIA/scripts/tools/kalma2-agent-phase-smoke.sh
./SddIA/scripts/tools/kalma2-sqlite-smoke.sh
./SddIA/scripts/tools/kalma2-sse-fracture-smoke.sh
./SddIA/scripts/tools/kalma2-sqlite-live-smoke.sh
./SddIA/scripts/tools/kalma2-chat-sse-live-smoke.sh
```

## Cierre documental

- PBI → `docs/todos/done/` · `pbi_archived: true` · v2.3.3
- PR único `#123` — merge pendiente operador
