---
feature_name: aut-72-tracker-stamp-sweep-kaizen
created: "2026-11-04"
process: bug-fix
branch_name: Fix_Events_Multiple_03
sighting_branch: Fix_Events_Multiple_03
persist_ref: docs/fixes/aut-72-tracker-stamp-sweep-kaizen
pbi_ref: AUT-72
document_id: PBI-c66c2160
uuid: b42cf4d8-9801-4b44-8f1f-e8b9c9647aa2
parent: AUT-71
execution_id: "523a4752-c526-488d-aaa0-d9a12b1275c6"
---

# Objetivos — aut-72-tracker-stamp-sweep-kaizen

## Misión

Kaizen **AUT-72** (historia **AUT-71**): tras merge (evidencia PR #433), un fallo de `tekton.tracker-stamp` en `PullRequest_Merged` no debe purgarse el evento padre como `kaizen-finalized` hasta reconciliar Done (Linear + prueba `.SddIA/proofs/tracker-done/{tracker_ref}.json`). Los fallos reales permanecen en dead-letter vía `Tracker_Sync_Failed`; `tracker-sync-replay` sigue siendo el suscriptor de seguimiento.

## Alcance (manifiesto)

- Barrido EDA (`try_sweep_event`) y tests `aut72_*`.
- Idempotencia acotada del sello Done en `tracker_stamp` si el spec lo exige tras T1.
- Rama: `Fix_Events_Multiple_03`. Provider Linear: sin `docs/todos/`.

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
- Parada actual: planificación L0 (`spec.md` + `plan.md`); código en estímulo Tekton posterior.
