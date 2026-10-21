---

feature_name: eda-redb-01-store
created: "2026-10-21"
process: feature
branch_name: feat/eda-redb-01-store
persist_ref: docs/features/eda-redb-01-store
pbi_ref: "docs/todos/pending/[ARQUITECTURA] EDA Redb 01 — Store transaccional sddia-event-store.md"
document_id: PBI-EDA-REDB-01-STORE
pbi_uuid: "3f8c1a62-7d4e-4b91-8a27-6c5e0d9f1b34"
status: in-progress
mayeuta_verdict: ok
execution_id: "82b4551a-3443-457b-8650-b3a0b6b6447a"
---
# Objetivos — eda-redb-01-store

## Misión

Materializar el crate síncrono `sddia-event-store`: seis tablas `redb` 2.6.4, `insert_event` atómico con N testigos `pending`, y testigos por `(event_id, subscriber_id)` que sobreviven al cierre del fichero.

## Punto objetivo

> **O-STORE:** `cargo test -p sddia-event-store` verde. Dos suscriptores del mismo `event_id` conservan ambos testigos tras reabrir el fichero. `SddIA/Cargo.toml` lista el miembro. No existe `SddIA/crates/`.

## Alcance

| Dentro | Fuera |
|--------|-------|
| Crate `SddIA/sddia-event-store` | Flag `SDDIA_EVENT_STORE` |
| Miembro del workspace | `execute-process` y daemons |
| Seis tablas y las operaciones mínimas del PBI | Borrado de `.events/` o `event-sweeper` |
| Tests AC-2, CA-01, CA-02 | Elección de suscriptores |
| Pin exacto `redb` 2.6.4 | Constitución, README, `cumulo.paths.json` |

## Objetivos medibles

| ID | Objetivo | CA |
|----|----------|----|
| O-AC2 | Dos `set_delivery` del mismo evento y distinto suscriptor no se pisan, ni tras reabrir | AC-2 |
| O-CA01 | Tests del crate verdes. Una clave de otra aridad no altera la tabla vecina | CA-01 |
| O-CA02 | `members` incluye `sddia-event-store`. Cero ruta `SddIA/crates/` | CA-02 |

## Ley aplicada

- L-REDB-01 y L-REDB-03 de `HU-SDDIA-EDA-REDB-001`.
- Git de inicialización solo vía `skill:git-manager` (`workspace-init`).
- Genoma (`events/`, `agents/`, `norms/`, `daemons/`) no se toca. Este PBI no forja entidades.
- Jerarquía: Acción, Agente, Skill, Tools. El crate es biblioteca, no cápsula.
