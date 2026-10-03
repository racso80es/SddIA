---
feature_name: tracker-genoma-forja-backfill
created: "2026-10-02"
process: refactorization
branch_name: fix/linear-tracker-adapter-hash
persist_ref: docs/features/tracker-genoma-forja-backfill
pbi_ref: docs/todos/done/[DEUDA] Tracker — backfill forja entity-manager.md
document_id: PBI-DEUDA-TRACKER-GENOMA-FORJA
---

# Objetivos — backfill forja genoma Tracker

Re-sellar entidades de la HU Tracker Linear vía `entity-manager` (`hash_refresh_only` o sello ECST) para que `eda-coverage.json` refleje hashes canónicos y eventos `Domain_Entity_*`, sin cambiar comportamiento de producto.

## Alcance sellado

Procesos tracker, acciones emit-*, eventos domain, norm `execution-contexts` (uuid Cerbero en `SddIA/norms/`), matriz EDA coherente con `orphan_count: 0` tras OSC-9.
