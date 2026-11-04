---
feature_name: aut-72-tracker-stamp-sweep-kaizen
created: "2026-11-04"
process: bug-fix
phase: blueprint
agents: dedalo
phases: T0-T5
branch_name: Fix_Events_Multiple_03
sighting_branch: Fix_Events_Multiple_03
persist_ref: docs/fixes/aut-72-tracker-stamp-sweep-kaizen
pbi_ref: AUT-72
document_id: PBI-c66c2160
uuid: b42cf4d8-9801-4b44-8f1f-e8b9c9647aa2
parent: AUT-71
execution_id: "523a4752-c526-488d-aaa0-d9a12b1275c6"
---

# Plan — aut-72-tracker-stamp-sweep-kaizen

Parada actual (**L0**): `objectives.md`, `clarify.md`, `spec.md`, `plan.md`. **Commit de planificación** antes de mutar código. Rama: `Fix_Events_Multiple_03`.

## T0 — Sellar planificación (esta entrega)

| Paso | Acción |
| --- | --- |
| T0.1 | Frontmatter coherente (`branch_name`, `pbi_ref`, `document_id`, `uuid`). |
| T0.2 | Commit: `docs(AUT-72): cascada planificación sweep tracker-stamp` |
| T0.3 | Anotar en `execution.md` init `523a4752-…` + relevo Dedalo IDE. |

## T1 — Guard de barrido (`eda_bus_topology`)

1. Extraer helper p. ej. `tracker_stamp_done_reconciled(repo, event_type, payload, dead_names) -> bool`.
2. En rama dead-letter de `try_sweep_event`, **antes** de `finalize_kaizen_terminal`, si el evento exige Done y stamp unreconciled → retornar `tracker-stamp-unreconciled` (mirror `fracture-unmaterialized`).
3. Resolver `tracker_ref` desde payload (`tracker_ref`, `pbi_ref`, `item_ref` — mismo orden que stamp).

## T2 — Idempotencia prueba Done (opcional acotada)

1. En `stamp_done_with_hu_roll_up`: si `fetch_issue` reporta estado Done y proof ausente, escribir proof local sin re-mutar Linear (solo si transición remota idempotente confirmada).
2. No emitir `Tracker_Sync_Failed` en reintentos benignos.

## T3 — Watcher (solo si hace falta tras T1)

1. Verificar que watcher no anuncie «Kaizen terminalizado» cuando el sweep devuelve `tracker-stamp-unreconciled`.
2. Sin cambio si T1 ya evita `kaizen-finalized`.

## T4 — Tests

1. `aut72_tracker_stamp_dead_letter_not_kaizen_finalized` — pending sobrevive.
2. Variante con proof presente — permite purge cuando resto terminales (regresión).
3. `cargo test -p execute-process aut72 -- --nocapture`

## T5 — Cascada bug-fix / cierre

1. `implementation.md` + `execution.md` por tarea (T1→T4).
2. Argos → `validacion.md` `global: APTO`, `branch: Fix_Events_Multiple_03`.
3. `delivery-close-cycle` con `source_process: bug-fix`, `persist_ref`, `branch_name`.
4. Linear AUT-72 → Done tras merge del PR único.

## Orden

T0 → T1 → T4 (tests) → T2 si T1 no basta para AC idempotencia → T3 → T5.
