---
feature_name: eda-redb-07-cutover
created: "2026-10-21"
process: feature
branch_name: feat/eda-redb-07-cutover
persist_ref: docs/features/eda-redb-07-cutover
pbi_ref: docs/todos/pending/[ARQUITECTURA] EDA Redb 07 — Cutover a redb y baja de event-sweeper.md
document_id: PBI-EDA-REDB-07-CUTOVER
execution_id: "dd17d95f-33ac-4932-94f2-fad2125d6991"
---

# Objetivos — eda-redb-07-cutover

## Misión

Cutover del bus al store `redb` y baja de `event-sweeper`. Laudos L-REDB-02 y L-REDB-06. AC-12, CA-09 y CA-10.

## Alcance

- Omitir `SDDIA_EVENT_STORE` selecciona `redb`. `filesystem` y `dual` siguen siendo escapes explícitos.
- `cumulo.paths.json`: `event_bus`, `eda_bus` y `eda_fractal` nombran `.SddIA/runtime/events.redb` y las tablas `events`, `subscriber_delivery`, `dead_letter_index`, `progress_traces`, `dlt_audit_seals`. Las claves `*_subscriptions` no cambian.
- `.gitignore` ignora `.SddIA/runtime/*.redb` y el lock de `redb`. `/.events/` se queda para residuos.
- Baja de `event-sweeper` (UUID `3eafa012-2b71-47e5-b47e-467b59a3fd52`) vía `entity-manager`: ficha, fila de `daemons/index.md` y cobertura. El crate y los launchers salen en el mismo cambio.
- `probe_bus_topology` abre `events.redb` cuando el store escribe y comprueba los JSON de suscripción. No exige carpetas `.events/{telemetry,orchestration,domain,progress}`.
- Un `.events/pending/*.json` con store `redb` se registra como `System_Fracture_Detected` y no se reinyecta.

## Fuera de alcance

Borrar proofs o heartbeats, volver a suscribir IOTA y sustituir LanceDB.
