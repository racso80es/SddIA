---
feature_name: kalma2-post-ev-aud-005-ola
created: "2026-08-13"
process: bug-fix
branch_name: fix/kalma2-post-ev-aud-005-ola
persist_ref: docs/fixes/kalma2-post-ev-aud-005-ola
pbi_ref: docs/todos/done/[OPERATIVO] Kalma2 — ola mejora post-auditoría EV-AUD-005 (K1–K5).md
document_id: b2e4c891-3f7a-4d2e-9c1b-8a5f6e0d2c47
phases:
  - k4-suggested-branch
  - k5-single-flight
  - k3-watcher-async
  - k2-early-pec
  - k1-poll-ui
  - k6-bridge-latest-pec
---

# Plan — K4 → K5 → K3 → K2 → K1 → K6

Orden del procedimiento de retoma. Rama desde `main` (no mezclar commit EV-AUD-005). Podar debug `agent_debug_log` / `.cursor/debug-8d1dd3.log`. Tests unitarios por ID. Cierre: persist_ref propio + PBI `done/` + `delivery-close-cycle`.
