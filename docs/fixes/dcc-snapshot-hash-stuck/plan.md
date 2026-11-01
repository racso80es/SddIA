---
feature_name: dcc-snapshot-hash-stuck
created: "2026-10-31"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/dcc-snapshot-hash-stuck
tracker_ref: AUT-22
phases:
  - id: T1
    title: Guardia HEAD contra branch_name antes del commit
  - id: T2
    title: Fallo con stdout si HEAD o el ref no avanzan
  - id: T3
    title: Firma F-DCC-SNAPSHOT-HASH-STUCK y fila de corpus
---

# Plan — AUT-22

Un commit por tarea vía `skill:git-manager`. Rama `Fix_Events_Multiple`. Sin mutar genoma indexado (`events`, `process`, `norms`, códices).

| Id | Tarea | Hecho |
|----|--------|--------|
| T1 | En Snapshot final, `branch_list` y predicado de alineación. Si la rama actual no es `branch_name`, `SNAPSHOT_BRANCH_MISMATCH` y return. Tests del predicado. | C1 ✅ |
| T2 | Conservar stdout del commit. Comparar `HEAD` y `refs/heads/{branch_name}` con el hash previo. Si no avanzan, `SNAPSHOT_DIRTY_SKIPPED` con esa salida. Tests del predicado. | C2 ✅ |
| T3 | `F-DCC-SNAPSHOT-HASH-STUCK` en `SddIA/core/fracture-signatures.json`. Fila `51dc7bdfd2fe` en `fracture_corpus_regression`. Nota en `SddIA/evolution/`. | C3 ✅ |

Orden: T1 → T2 → T3.

Verificación posterior (Argos, no estas tareas): `validacion.md` `global: APTO` en este ancla; AUT-22 en Done. Provider `linear`: no crear fichero en `docs/todos/`.
