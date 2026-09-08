---
feature_name: pref-store-lancedb-migration
created: "2026-09-08"
process: feature
branch_name: feat/pref-store-lancedb-migration
persist_ref: docs/features/pref-store-lancedb-migration
execution_id: "338b68e4-de96-4303-93a7-d0c0e40acb9c"
items_applied:
  - L1-domain-trait
  - L2-lancedb-adapter
  - L3-migrate-parity
  - L4-execute-process-wire
  - L5-evolution-docs
---

# Ejecución — pref-store-lancedb-migration

## Init

`SDDIA_AGENT_RELAY_IDE=1 SDDIA_LAB_SKIP_PBI_ARCHIVE=1 SDDIA_LAB_SKIP_DELIVERY_CLOSE=1 SDDIA_LAB_ALLOW_DIRTY=1 ./sddia-run.sh --process feature --inputs-file .tmp/feature-pref-store-lancedb-migration.json`

`execution_id` `338b68e4-de96-4303-93a7-d0c0e40acb9c`. workspace-init **executed**. Mayeuta…DCC simulated / phase-barrier. Relevo IDE. Commit planificación `7748cee`.

## L1 Dominio

`UserPreference.embedding`, trait `UserPreferenceStore`, `JsonUserPreferenceStore`, `get_active`, `list_head_revisions`, `finalize_preference_ids`, `QuerySpec` vectorial ignorado por JSON.

## L2–L3 Adaptador

Crate `sddia-infrastructure-lancedb-preferences`. Ficha uuid `4c0103c3-5683-4219-aafd-1370c2b63c69`. Migración `migrate_json_to_lancedb_at`. Tests tempfile: reopen, idempotencia, KNN ordenado, tombstone, dim, paridad filtro.

## L4 Wiring

`put_revision_durable` / `purge_preference_durable` en ingest. Lectura: si tabla existe → LanceDB (error → bloque vacío); si no → cápsula/JSON.

## Tests

```text
cd SddIA && cargo test -p user-preference-core --lib
# 9 passed
cd SddIA && cargo test -p sddia-infrastructure-lancedb-preferences --lib
# 7 passed
cd SddIA && cargo test -p execute-process --lib -- email_triage user_preference
# 30 passed
```

## Evolution

Cápsula `sddia-evolution-register` → `84233af2-b3c3-40e8-9b28-0aef17c87c4c` (`EVOL_OK`, `alta`). Persistencia host del `detail`/`index`.
