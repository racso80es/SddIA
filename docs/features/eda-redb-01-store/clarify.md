---
feature_name: eda-redb-01-store
created: "2026-10-21"
process: feature
purpose: Estabilización del store transaccional. Laudos L-REDB-01 y L-REDB-03.
version_clarify: "1.0.0"
execution_id: "2567be4f-95cf-44fe-8bf8-01a568046cce"
pbi_ref: "docs/todos/pending/[ARQUITECTURA] EDA Redb 01 — Store transaccional sddia-event-store.md"
document_id: PBI-EDA-REDB-01-STORE
pbi_uuid: "3f8c1a62-7d4e-4b91-8a27-6c5e0d9f1b34"
pbi_version: "1.0.0"
---

# Clarificación — eda-redb-01-store

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery + `SDDIA_LAB_ALLOW_DIRTY=1`. `execution_id` `2567be4f-95cf-44fe-8bf8-01a568046cce`. Rama `feat/eda-redb-01-store`. Mayeuta simulated; Dedalo…DCC phase-barrier. Relevo IDE.

Semilla: PBI `PBI-EDA-REDB-01-STORE` v1.0.0. Historia madre `HU-SDDIA-EDA-REDB-001` §3.1, §3.2, AC-2. El dirty de la HU y de los PBI 02–07 queda fuera de este ciclo.

## Decisiones

| ID | Laudo |
|----|-------|
| L-REDB-01 | Crate `SddIA/sddia-event-store`. Prohibido `SddIA/crates/`. |
| L-REDB-03 | Cada mutación es una write-tx. El beneficio es ACID por clave de testigo, no escritura paralela. |
| L-PIN | `redb = "=2.6.4"`. Última 2.x publicada. La 4.x no entra. |
| L-API | `EventStore` encapsula `redb::Database`. API síncrona. Cero tokio. |
| L-PATH | `open(path)` crea o reabre el fichero. No lee `cumulo.paths.json`. No crea el directorio padre. |
| L-TIME | `created_at` es i64 unix millis. `family_index` ordena por ese entero. |
| L-SUB | `insert_event` persiste los `subscriber_id` que ya trae el llamador. `set_delivery` no inventa pares. |
| L-REPLAY | Borra `dead_letter_index` y pone en `pending` solo el testigo de `failed_subscriber_id`, conservando su JSON. |
| L-READ | Las lecturas usan `begin_read`. Tabla aún no creada = vacío. |
| L-VAL | Valores JSON salvo `family_index`, cuyo valor es `()`. |

## Fuera

Flag `SDDIA_EVENT_STORE`. Llamadas desde `execute-process` o `sddia-daemon-runtime`. Borrado de `.events/` o de `event-sweeper`. Elección de suscriptores. Constitución, README y `cumulo.paths.json` (PBI 06 y 07).
