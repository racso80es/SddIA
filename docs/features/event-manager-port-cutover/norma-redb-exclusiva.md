---

feature_name: event-manager-port-cutover
created: "2026-11-08"
process: refactorization
phase: T0-norma-redb-exclusiva
pbi_ref: MAN-35
parent_ref: MAN-33
document_id: PBI-176f9f2f
status: materializado
norm_ref: features-documentation-pattern@1.3.1
execution_id: "8b39f170-38b9-4aea-961b-31c4c63512b7"
---
# T0 — Norma Redb exclusiva

## Runtime SSOT

| Concepto | Valor |
|----------|-------|
| Store productivo | `.SddIA/runtime/events.redb` (`cumulo.paths` → `event_bus` / `eda_bus.store`) |
| Modos prohibidos en runtime | `filesystem`, `dual`, env `SDDIA_EVENT_STORE` en consumidores |
| Identificadores lógicos | tokens `.events/*` / nombres de tabla — **no** cola ni SSOT |

## Delimitación ECST vs no-evento (**L-NON-EVENT-FS**)

| Dominio | SSOT |
|---------|------|
| ECST / entregas / DLQ / progreso / DLT | Redb vía `EventManagerPort` |
| Proofs | `.SddIA/proofs` (`eda_instance.proofs`) |
| Customización instancia / no-cola | `.SddIA/events` (`eda_instance.customization`) |
| Reanchor DLT | `.SddIA/dlt/reanchor-queue` |
| Preferencias / vector store | `.SddIA/vector_store/**` |

## Reglas

1. Un JSON físico conflictivo bajo `.events/*` **nunca** prevalece sobre Redb.
2. Consumidores no abren `events.redb` ni importan `sddia_event_store`.
3. Solo el adapter productivo conoce el path físico y las tablas.

## AC-9 — Retirada legacy (T7)

- Prohibido `EventStore::open` en productores, audit, purger y bridges (solo tests legacy / crate `sddia-event-store`).
- `StoreMode` / `SDDIA_EVENT_STORE` no gobiernan runtime productivo.
- `fs_event_adapter` fuera de bindings productivos (rol `test` / lab).
