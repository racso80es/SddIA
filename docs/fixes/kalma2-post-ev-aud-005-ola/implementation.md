---
feature_name: kalma2-post-ev-aud-005-ola
created: "2026-08-13"
process: bug-fix
branch_name: fix/kalma2-post-ev-aud-005-ola
persist_ref: docs/fixes/kalma2-post-ev-aud-005-ola
pbi_ref: docs/todos/done/[OPERATIVO] Kalma2 — ola mejora post-auditoría EV-AUD-005 (K1–K5).md
document_id: b2e4c891-3f7a-4d2e-9c1b-8a5f6e0d2c47
items:
  - K4-suggested-branch
  - K5-single-flight
  - K3-watcher-async
  - K2-early-pec-awaiting-agents
  - K1-poll-ui
  - K6-bridge-latest-pec
---

# Implementación — ola Kalma2

| ID | Artefacto | Cambio |
|----|-----------|--------|
| K4 | `handlers/task_queue_manager.rs` | `extract_suggested_branch` + `slug_from_branch`; test frontmatter |
| K5 | `task_queue_manager.rs` | `try_acquire_single_flight` bajo `.SddIA/daemons/state/tqm-single-flight/`; hit → skip hijo |
| K3 | `daemons/event-watcher/src/main.rs` | `thread::spawn` + `MAX_IN_FLIGHT_ROUTES=16`; sin debug log |
| K2 | `thermodynamic.rs` | `emit_initialized_pec` → `awaiting_agents` |
| K1 | `interfaces/kalma2/app.js` | poll hasta `completed`/`failed`; timeout 30 min tras lifecycle |
| K6 | `kalma2-bridge/src/main.rs` | `find_pec_by_correlation` por `timestamp` máximo |

No se toca `kalma2.rs` (WIP era solo instrumentación debug).
