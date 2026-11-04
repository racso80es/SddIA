---
feature_name: aut-72-tracker-stamp-sweep-kaizen
created: "2026-11-04"
purpose: decision
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

# Clarificaciones — AUT-72

## Decisiones

1. Rama de trabajo única: `Fix_Events_Multiple_03`. El init canónico abrió `fix/aut-72-tracker-stamp-sweep-kaizen`; la cascada documental y la ejeción Tekton anclan la rama de sighting, no la rama efímera del lab.
2. Problema acotado al barrido EDA (`try_sweep_event`) cuando hay testigos en `dead_letter_subscribers` y el suscriptor `tekton.tracker-stamp` falló en eventos que exigen sello **Done** (`PullRequest_Merged`, `Delivery_Committed` en `trunk_direct`).
3. **No** se reescribe la política fail-soft de Linear: `Tracker_Sync_Failed` + `tracker-sync-replay` permanecen. El fix impide que el padre se purgue como `kaizen-finalized` mientras el sello Done no esté reconciliado (prueba local o stamp exitoso).
4. Idempotencia Done: `done_gate::write_tracker_done_proof` solo corre tras transición remota exitosa hoy; el plan incluye sellar prueba local cuando el issue ya está en Done en Linear pero el handler reintenta (idempotencia explícita en stamp), sin enmascarar un fallo real.
5. Paridad de prueba: test `aut72_*` en `eda_bus_topology` (mismo patrón que `aut70_fracture_dead_letter_not_kaizen_finalized`).
6. Cierre Linear (`state_map.done`) fuera de esta parada Dedalo. Sin ficheros nuevos en `docs/todos/`.

## Evidencia de partida

- Refino AUT-72 (`execution_id` `030d8a50-3dc1-43a2-aa20-16b97ad96770`).
- Sighting merge PR #433 (`docs/fixes/aut-70-fractura-project-id/validacion.md` → `pr_url` #433): `PullRequest_Merged` con `tekton.tracker-stamp` en dead-letter y barrido `kaizen-finalized` prematuro.
