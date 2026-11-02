---
feature_name: aut-67-prepush-store-lock
created: "2026-11-02"
process: bug-fix
phase: design
agents: dedalo
base: main
branch_name: Fix_Events_Multiple_02
sighting_branch: Fix_Events_Multiple_02
persist_ref: docs/fixes/aut-67-prepush-store-lock
pbi_ref: AUT-67
document_id: PBI-3d736171
uuid: e46d8e0c-b9fa-4d18-a035-99e9cf66ac07
tracker_ref: AUT-67
historia_ref: AUT-20
execution_id: "e5c2418a-ce87-45ae-a522-2246568fd632"
status: dedalo_locked
---

# Spec — aut-67-prepush-store-lock

## 1. Misión

El pre-push de un PR ya abierto no debe tratar `StoreError::Locked` de `.SddIA/runtime/events.redb` como rechazo de `Local_QA_Requested`. Padre: AUT-20. Ítem: AUT-67 / `PBI-3d736171`.

## 2. Fricción verificada

`pre_push_gate.sh` (`handle_pre_push_pr_open`) inyecta `Local_QA_Requested` con `blocking: true` vía `route-domain-event` cuando no hay atestación vigente. Ese proceso abre el store. El `event-watcher` abre el mismo fichero en `reconcile_unstamped`, `claim_store_pending` y `claim_store_fractal`, y lanza hijos que lo reabren. redb admite un solo `Database`. Agotado `SDDIA_EVENT_STORE_OPEN_BUDGET_MS`, el hook imprime `BLOCKED — Local_QA_Requested failed` aunque `gate-evolution` haya devuelto `EVOL_OK`.

Caso: push de `Fix_Events_Multiple_02` hacia el PR 432. Poseedor: `SddIA/target/release/event-watcher`.

AUT-16, AUT-55 y AUT-50/52 cubren reintento de `open`, `Display` de `Locked` y fractura en `refine-item`. No cubren este choque.

## 3. Fuera de alcance

- Atestación QA con `tree_sha` distinto del HEAD. El gate que exige sello vigente se mantiene.
- Subir el default de `SDDIA_EVENT_STORE_OPEN_BUDGET_MS`.
- Cambiar el modelo exclusivo de redb por un segundo motor de store.

## 4. Laudos

| Ref | Decisión |
|-----|----------|
| L-DISTINGUISH | El hook distingue `STORE_LOCK_CONTENTION` de fallo de QA. |
| L-HOOK-RETRY | Ante lock en el `route-domain-event` de `Local_QA_Requested`, el hook reintenta con presupuesto propio hasta que el poseedor ceda el fichero. |
| L-WATCHER-YIELD | El tick no retiene `EventStore` mientras haya rutas hijas vivas. `claim_store_pending` ya hace `drop` antes del spawn; `reconcile_unstamped` y `claim_store_fractal` no pueden solapar ese `open` con los hijos. |
| L-EXPLICIT | Si el presupuesto del hook se agota, el mensaje nombra contención de store y no «QA no aprobada». |
| L-NO-BUDGET | Prohibido cambiar el default de `SDDIA_EVENT_STORE_OPEN_BUDGET_MS`. |

## 5. Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC1 | Con watcher activo y un poseedor que suelta `events.redb` dentro del presupuesto del hook, el pre-push de PR abierto no termina en `BLOCKED` por `StoreError::Locked`. |
| AC2 | Si nadie suelta el fichero, el hook sale con error explícito de contención, no con el texto de QA rechazada. |
| AC3 | Test de laboratorio sin red: contención simulada y cesión dentro del presupuesto. |
| AC4 | El default de `SDDIA_EVENT_STORE_OPEN_BUDGET_MS` no cambia. |
