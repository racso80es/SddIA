---
feature_name: event-watcher-redb-queue-logs-metrics
created: "2026-10-30"
process: feature
pbi_ref: AUT-32
---

# Plan

1. Tipos `ClaimTickStats`, `log_claim_tick`, `store_mode_label` + contadores atómicos.
2. Instrumentar bucle `claim_store_pending` (conteos + logs spawn/done).
3. Log de fallo `reconcile_unstamped`.
4. Test unitario `store_mode_label`.
5. Documentar en `implementation.md` / `execution.md` y validar build `event-watcher`.
