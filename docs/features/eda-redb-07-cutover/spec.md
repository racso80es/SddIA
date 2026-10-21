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

# Spec — eda-redb-07-cutover

| Pieza | Contrato |
| :--- | :--- |
| `mode_from_name` | vacío, ausente o desconocido → `Redb`. Solo `filesystem` y `dual` cambian el corte. |
| `is_filesystem_rel` | verdadero si hay `/` o el valor empieza por `.` y no termina en `.redb`. |
| `write_pending_event` | con store escribe `record_ecst`. Sin filesystem no crea el JSON; `target_path` queda `.events/pending/{id}.json`. |
| `probe_bus_topology` | ficheros de suscripción presentes. Con `dual` o `redb`, `events.redb` abre. Cero exigencia de hojas `eda_fractal`. |
| Watcher | en `redb`, un JSON bajo `pending/` emite `System_Fracture_Detected` y no llama a `route-domain-event`. El drenado del store sigue. |
| Baja | `entity-manager` `delete` de `daemon` quita la ficha, la fila del índice y reescribe el censo. |
