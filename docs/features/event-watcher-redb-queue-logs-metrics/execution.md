---
feature_name: event-watcher-redb-queue-logs-metrics
created: "2026-10-30"
process: feature
branch_name: feat/event-watcher-redb-queue-logs-metrics
persist_ref: docs/features/event-watcher-redb-queue-logs-metrics
pbi_ref: AUT-32
execution_id: "d126f8ae-c6c7-4507-8b11-cfc1199daa3a"
---

# Ejecución — AUT-32

## Init

`SDDIA_AGENT_RELAY_IDE=1` + skips archive/DCC + `SDDIA_LAB_ALLOW_DIRTY=1` → `execution_id` `d126f8ae-c6c7-4507-8b11-cfc1199daa3a`.

## Tekton

- [x] Instrumentación `claim_store_pending` + logs de ruta (commit en rama).
- [ ] Argos / DCC (pendiente relevo o runtime agente).

## Verificación prevista

```bash
cd SddIA && cargo test -p event-watcher
cd SddIA && cargo build -p event-watcher
```
