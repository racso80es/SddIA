---
feature_name: eda-redb-01-store
created: "2026-10-21"
process: feature
phases:
  - modelo
  - write-tx
  - lecturas
  - tests
branch_name: feat/eda-redb-01-store
persist_ref: docs/features/eda-redb-01-store
pbi_ref: "docs/todos/pending/[ARQUITECTURA] EDA Redb 01 — Store transaccional sddia-event-store.md"
document_id: PBI-EDA-REDB-01-STORE
uuid: "3f8c1a62-7d4e-4b91-8a27-6c5e0d9f1b34"
execution_id: "2567be4f-95cf-44fe-8bf8-01a568046cce"
---

# Plan — eda-redb-01-store

Init: `./sddia-run.sh --process feature` + relevo IDE + skips archive/delivery. `execution_id` `2567be4f-95cf-44fe-8bf8-01a568046cce`.

## modelo

Tipos `Family`, `DeliveryStatus`, `StoredEvent`, `DeliveryWitness`, `DeadLetter`, `DltSeal`, `StoreError`. Constantes `TableDefinition` con aridad distinta por tabla.

## write-tx

`EventStore::open`. `insert_event`, `set_delivery`, `seal_dlt`, `append_progress`, `dead_letter`, `replay`. Un `commit` por método.

## lecturas

`get_event`, `get_delivery`, `get_seal`, `get_dead_letter`, `list_family`, `list_progress`. Tabla ausente = colección vacía.

## tests

`cargo test -p sddia-event-store`. AC-2 con reapertura. Vecinos de clave distinta. Miembro del workspace y ausencia de `SddIA/crates/`. Lectores concurrentes y dos escritores de suscriptor distinto.

Cierre de entrega (`delivery-close-cycle`) queda fuera de este relevo: la fase quedó en phase-barrier y no hay orden de subida.
