---
feature_name: eda-redb-03-watcher-vitality
created: "2026-10-21"
process: feature
document_id: PBI-EDA-REDB-03-WATCHER-VITALITY
uuid: "9b4e7c25-8a1d-4e63-b5f0-2d8c6a1e4f97"
branch: feat/eda-redb-03-watcher-vitality
branch_name: feat/eda-redb-03-watcher-vitality
persist_ref: docs/features/eda-redb-03-watcher-vitality
pbi_ref: "docs/todos/pending/[ARQUITECTURA] EDA Redb 03 — Watcher y sondas de vitalidad.md"
global: APTO
pbi_archived: false
---

# Validación — eda-redb-03-watcher-vitality

| Criterio | Resultado | Prueba |
| --- | --- | --- |
| CA-06 | APTO | `filesystem_pending_is_routed_without_opening_the_store` |
| AC-7 | APTO | `one_tick_fires_both_probes_without_the_sweeper` |
| Reclamo sin ECST | APTO | `redb_claim_delegates_the_id_without_reading_the_ecst` |
| AC-4 | APTO | `tier_none_does_not_spawn_and_cognitive_receives_ecst_and_workspace` |
| Pares pending | APTO | `dual_keeps_two_subscriber_rows` incluye `list_pending_deliveries` |

`event-sweeper` no se arranca en AC-7. El binario y su unidad siguen en el árbol.
