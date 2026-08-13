---
feature_name: kalma2-post-ev-aud-005-ola
created: "2026-08-13"
process: bug-fix
branch_name: fix/kalma2-post-ev-aud-005-ola
persist_ref: docs/fixes/kalma2-post-ev-aud-005-ola
pbi_ref: docs/todos/done/[OPERATIVO] Kalma2 — ola mejora post-auditoría EV-AUD-005 (K1–K5).md
document_id: b2e4c891-3f7a-4d2e-9c1b-8a5f6e0d2c47
items_applied:
  - K4-suggested-branch
  - K5-single-flight
  - K3-watcher-async
  - K2-early-pec-awaiting-agents
  - K1-poll-ui
  - K6-bridge-latest-pec
---

# Ejecución — ola Kalma2

## Rama

`fix/kalma2-post-ev-aud-005-ola` desde `main` (no cherry-pick de EV-AUD-005). Stash origen: `via-b-kalma2-ola-wip-post-ev-aud-005` (sin `evolution-contract-index-v11`, sin debug).

## Tests (host 2026-08-13)

```bash
cd SddIA && env -u CARGO_TARGET_DIR cargo test -p execute-process --lib handlers::task_queue_manager
# 6 passed (incl. suggested_branch_from_pbi_frontmatter, single_flight_second_acquire_hits_while_guard_lives)

cd SddIA && env -u CARGO_TARGET_DIR cargo test -p execute-process --lib emit_initialized_pec_writes_orchestration
# 1 passed — cycle_phase=awaiting_agents

cd SddIA && env -u CARGO_TARGET_DIR cargo test -p kalma2-bridge find_pec_by_correlation_prefers_latest
# 1 passed
```

```bash
cd SddIA && env -u CARGO_TARGET_DIR cargo build -p execute-process -p event-watcher -p kalma2-bridge
# Finished dev profile
```

## Smoke runtime

Kalma2 HTTP `127.0.0.1:8765` → 200 (binarios vivos pre-rebuild). No se re-forja el PBI EV-AUD-005 (ya `done/` / PR #170). Centinelas no reiniciados en esta sesión (evitar interferir PPR #170). Operador: reiniciar `event-watcher` + `kalma2-bridge` post-merge para activar K3/K6 en vivo.
