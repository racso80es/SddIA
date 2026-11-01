---
feature_name: dcc-presented-seal-contract
created: "2026-11-01"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/dcc-presented-seal-contract
tracker_ref: AUT-48
execution_id: "9e85f016-c9be-46de-af20-92455e7817b8"
phases:
  - id: T1
    title: Alinear el sello en delivery-close-cycle.md
  - id: T2
    title: Nota evolution AUT-48
---

# Plan — AUT-48

Rama `Fix_Events_Multiple` @ `31ec7ac4`. Un commit por tarea. El motor, con git activo, habría creado `fix/dcc-presented-seal-contract`. No se hace ese checkout.

## T1 — Genoma

`./sddia-run.sh --process entity-manager` con `entity_class: process`, `entity_name: delivery-close-cycle`, `lifecycle_operation: update`.

En `SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md`:

- Versión `1.6.1`.
- En la tabla de Sello Presentación ECST, la fila `pr_url` describe el origen real (`state`, `inputs`, `gh pr view` acotado). Sin la frase «opcional en payload ECST v1.1».
- Bajo la tabla, una línea: ausencia de URL → `blocked` / `F-DCC-PR-URL-MISSING`; no hay `PullRequest_Presented`.

No editar `phase_capsules.rs` ni `fracture-signatures.json`.

## T2 — Evolution

Nota nueva en `SddIA/evolution/` con UUID, vinculada a AUT-48 y a `delivery-close-cycle.md`. Registrar la fila en `Evolution_log.md` para que `gate-evolution` no devuelva `EVOL_NOT_INDEXED`.

## Hecho que este plan no repite

`blocked` del sello, recuperación de URL y la firma `186c785103d8` (AUT-31, `docs/fixes/dcc-pr-url-presented-gate/`).
