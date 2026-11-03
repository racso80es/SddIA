---
feature_name: aut-69-unificar-etiqueta-fix-bug
created: "2026-11-03"
process: refactorization
branch_name: refactor/man-26-ciclo-vida-software
persist_ref: docs/features/aut-69-unificar-etiqueta-fix-bug
pbi_ref: AUT-69
execution_id: "plan-aut-69-20261103"
---

# Implementación — aut-69-unificar-etiqueta-fix-bug

## T1 — Migración Linear

Script puntual GraphQL (`issueUpdate` con `labelIds`): por cada issue AUT con label `Fix`, se sustituye el id de `Fix` (`a786a2eb-af85-4cf0-b9b5-0c8d4a8cadb9`) por el id de `Bug` (`6154e3e4-42d8-403e-9795-7779cfbc9df8`), conservando el resto de labels.

Resultado: **26** issues migrados, **0** errores. Tras la pasada, **0** issues con `Fix`.

## T2 — Manifiesto

`.SddIA/project.md`: `tracker.labels.fix` pasa de `Fix` a `Bug`. Clave `fix` sin cambios.

## T3 — Borrar label

`issueLabelDelete` sobre la label de equipo `Fix`. `fix_label_deleted=true`.

## T4 — Verificación

- Issues AUT con label `Fix`: 0.
- Label `Fix` ausente en el equipo AUT.
- Manifiesto: `fix: Bug`.

Labels visibles en equipo AUT tras el corte: Bug, Deuda, Feature, HU, Improvement, Kaizen, Pbi, Spike (Linear puede reflejar labels de workspace en el listado del equipo).

## Fuera de diff de genoma

No se modificó `linear-tracker-adapter` ni procesos. Solo instancia (manifiesto) y estado remoto Linear.
