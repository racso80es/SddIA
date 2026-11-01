---
feature_name: aut-50-backlog-alta
created: "2026-11-01"
process: bug-fix
base: main
scope: forge-pbi-padre-list-issues-forge-hu-refine-lock
branch_name: Fix_Events_Multiple_02
persist_ref: docs/fixes/aut-50-backlog-alta
pbi_ref: AUT-50
tracker_ref: AUT-50
execution_id: "82abfda6-47a2-41f6-855f-bddbc8546bda"
---

# Spec — aut-50-backlog-alta (AUT-50)

## Hecho

Cuatro fricciones del camino de alta en equipo AUT / proyecto `core` (padre AUT-18; reproducción 2026-10-30 con AUT-49):

| ID | Defecto | Evidencia / ancla |
|----|---------|-------------------|
| F1 | `forge-pbi` sin padre usable responde `success: true`, `create_issue_invoked: false`, fase «Registro en tracker» `skipped` (`parent_ref ausente`); queda `document_id` local y ningún issue Linear | `handlers/forge_pbi.rs` → `tracker_registration_phase` solo lee `parent_ref`; el contrato publicado `forge-pbi.md` v1.0.2 declara `historia_ref` como padre. El gate de fallo exige `tracker_required && invoked && tracker_ref.is_none()` — el skip por padre ausente no entra y cae en `success_envelope` |
| F2 | `list_issues` con `project_id` del manifiesto → GraphQL 400 (`$projectId: String!` donde Linear exige `ID`); sin `project_id` + `first` default 50 + `children { nodes { identifier } }` → `Query too complex` (~23050 > 10000). Solo 200 con `first: 20` y sin `project_id` | `linear-tracker-adapter` `build_list_issues_query` / `run_list_issues`. El puente y `tracker-backlog-query` pasan `team_key` + `project_id` → quedan ciegos |
| F3 | No hay proceso del códice invocable para HU raíz (`labels: hu`, sin padre). `refine-hu`/`refine-pbi` → `PROCESS_RENAMED`. Evolution AUT-19 (`74585763…`) citó «create_issue HU sin tracker_ref» en slice 2, pero el camino no es invocable | Índice códice SE: solo `forge-pbi` (exige padre) + `refine-item` (no crea). Adapter ya admite `create_issue` sin `parent_ref` |
| F4 | Con runtime + watcher activos, `refine-item` colapsa ~243 s con `Database already open. Cannot acquire lock.`; refinamiento local sin sello Linear verificable y sin `System_Fracture_Detected` nuevo | AUT-16 (`docs/fixes/eda-redb-lock`) añadió reintento/`StoreError::Locked` (presupuesto default 2000 ms). Un poseedor de larga duración (execute-process / route-domain-event) agota el presupuesto. `refine_item::run` emite fractura en fallo Mayeuta, no en lock del store al sellar; `forge-pbi` sí trata lock post-create (AUT-29) |

`plan.md` sí: T1–T4 (+ blueprint `forge-hu`). Fuera: AUT-49, jail purger, `SddIA/target`.

## Corte

### C1 — forge-pbi: padre usable y éxito honesto

1. Resolver padre: `parent_ref` si no vacío; si no, `historia_ref` (equivalente de contrato). Ambos presentes → gana `parent_ref`.
2. Con tracker configurado en manifiesto y **sin** padre usable → envelope `success: false`, `exit_code: 1`, error estable `PARENT_REF_REQUIRED`, `create_issue_invoked: false`, fase «Registro en tracker» `failed` (no `skipped` silencioso). Seguir emitiendo `Tracker_Sync_Failed` con ese código.
3. Con tracker configurado: `success: true` **solo** si hay `tracker_ref` (alta nueva, reuso idempotente o ledger). Ampliar el gate actual para cubrir `!invoked` por padre ausente y skip no-idempotente sin ref.
4. Sin tracker en manifiesto: conservar alta local sin cápsula (`create_issue_invoked: false`) — no es el falso positivo AUT.
5. Tests: (a) solo `historia_ref` tipo `AUT-*` + lab → `create_issue` con ese padre y `tracker_ref`; (b) tracker + sin padre → `success: false` / `PARENT_REF_REQUIRED`; (c) actualizar/retirar `ac_0b2_hu_without_tracker_ref_still_forges` (hoy espera éxito sin ref con `historia_ref` ruta de fichero — contradice el contrato).

### C2 — list_issues: tipo ID y techo de complejidad

1. En `build_list_issues_query`, tipar `$projectId: ID!` (filtro `project: { id: { eq: $projectId } }`).
2. Default `first`: **20** (antes 50); `min(100)` se mantiene para override explícito.
3. Selección: quitar `children { nodes { identifier } }` del query por defecto (el puente/backlog no lo requieren; `parent { identifier }` se conserva). Opt-in futuro solo si hace falta — fuera de este fix salvo test.
4. Tests unitarios del string GraphQL + lab/mock: `team_key` + `project_id` UUID construye variables tipadas `ID`; con labels `hu` y `first` default la query no incluye children.
5. Verificación empírica (Argos/lab): `list_issues` con `team_key=AUT` + `project_id` del manifiesto `core` → HTTP 200 e ítems del proyecto. `tracker-backlog-query` hereda el arreglo sin cambio de contrato de proceso.

### C3 — forge-hu: alta de HU raíz (blueprint)

1. Nuevo proceso del códice `codex-software-engineering`: **`forge-hu`** (process-creator / entity-manager según ley genómica), indexado y ejecutable vía `./sddia-run.sh --process forge-hu`.
2. Inputs mínimos: `raw_idea`, `project_slug`; sin `parent_ref` / `historia_ref` de parentesco.
3. Handler nativo (espejo reducido de `forge_pbi`): `create_issue` con `labels: [label_hu]` del manifiesto, **sin** `parent_ref` en el request; team/project/state backlog como forge-pbi.
4. `success: true` solo con `tracker_ref` cuando hay tracker configurado; si no, fallo explícito (misma honestidad que C1).
5. Evento de dominio: emitir alta observable (`Domain_Entity_Created` o clase `HU_Forged` si se mintea con event-creator). No reabrir `refine-hu`.
6. Documentar en contrato del proceso el hueco cerrado de AUT-19 slice 2. Cobertura EDA / índice códice en el mismo PR.

### C4 — refine-item / events.redb: accionable + fractura

1. Clasificar `StoreError::Locked` / literales `Database already open` / `Cannot acquire lock` como código estable **`STORE_LOCK_CONTENTION`** en la ruta de materialización que usa refine-item.
2. Ante ese código en sellado/emisión de `PBI_Refined`/`HU_Refined`: emitir `System_Fracture_Detected` (mismo patrón que `emit_mayeuta_fracture`) con `source_process: refine-item`, detalle accionable (reintento, `SDDIA_EVENT_STORE_OPEN_BUDGET_MS`, reducir `route-domain-event` concurrente).
3. Envelope: `success: false` + error `STORE_LOCK_CONTENTION` **salvo** el caso espejo AUT-29: si el `update_issue_description` remoto ya aplicó el sello y solo falla la emisión al store → `success: true` con `refined_event_pending: true` y `tracker_ref` (sello Linear verificable por `fetch_issue` / description).
4. Presupuesto de open: subir default de `EventStore::open` a **10000 ms** (override por env intacto). No rediseñar motor multi-handle en este fix.
5. Corregir `StoreError` `Display`: `Invalid`/`NotFound`/`Conflict` no deben renderizar como `StoreError::Locked` (bug actual en `error.rs` que ensucia clasificación).
6. Lab: flag existente o análogo `SDDIA_LAB_SIMULATE_STORE_LOCK` sobre la emisión de refine → fractura observable o pending según corte 3.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| AUT50-CA1 | `forge-pbi` sin padre usable + tracker → `success: false`, `PARENT_REF_REQUIRED` |
| AUT50-CA2 | `forge-pbi` con solo `historia_ref` usable → aceptado como padre; issue creada / `tracker_ref` presente para `success: true` |
| AUT50-CA3 | `list_issues` query declara `$projectId: ID!`; default `first=20`; sin `children` en selección default |
| AUT50-CA4 | `list_issues` AUT + `project_id` manifiesto → 200 y lista del proyecto (lab o evidencia live) |
| AUT50-CA5 | `forge-hu` invocable crea HU (`labels` hu, sin padre) y devuelve `tracker_ref` |
| AUT50-CA6 | Runtime+watcher: `refine-item` completa **o** falla `STORE_LOCK_CONTENTION` con `System_Fracture_Detected` observable |
| AUT50-CA7 | Refine exitoso: description Linear / evento `PBI_Refined`\|`HU_Refined` verificable (o `refined_event_pending` con description sellada) |
| AUT50-CA8 | Nota en `SddIA/evolution/` vinculada a AUT-50; sin sembrar `docs/todos/` desde este fix |

## Fuera de alcance

- Rehacer kaizen AUT-49 / retención de disco.
- Ampliar jail de `ephemeral-cache-purger`.
- Tocar `SddIA/target`.
- Motor redb multi-conexión / poseedor único del store a largo plazo (solo presupuesto + fail-soft + fractura).
- Restaurar procesos `refine-hu` / `refine-pbi` como ejecutables.

## Touchpoints (Tekton)

| Módulo | Acción |
|--------|--------|
| `SddIA/engine/execute-process/src/engine/handlers/forge_pbi.rs` (+ tests) | C1 |
| `SddIA/library/codexes/codex-software-engineering/process/forge-pbi.md` | Alinear inputs/`parent_ref`↔`historia_ref` si hace falta (versión menor) |
| `SddIA/tools/linear-tracker-adapter/src/main.rs` (+ tests) | C2 |
| Códex SE `forge-hu` + handler nativo + índice + EDA | C3 (blueprint) |
| `refine_item/run.rs`, materialize/store path, `sddia-event-store` open budget + `error.rs` Display | C4 |
| `SddIA/evolution/<uuid>.md` | AUT50-CA8 |
