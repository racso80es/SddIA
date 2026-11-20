---


feature_name: man-44-rechazar-apto-incoherente
created: "2026-11-20"
updated: "2026-11-20"
process: feature
purpose: Estabilización de requisitos — rechazar global APTO si execution está blocked o el plan tiene fases/hijos abiertos (KZ-1)
branch_name: feat/man-44-rechazar-apto-incoherente
persist_ref: docs/features/man-44-rechazar-apto-incoherente
pbi_ref: MAN-44
parent_ref: MAN-43
historia_ref: MAN-43
document_id: PBI-8205539f
phase: mayeuta-stabilization
mayeuta_verdict: ok
norm_ref: features-documentation-pattern@1.3.1
tracker_channel: ".SddIA/state/tracker-outbound/MAN-44.jsonl"
refine_workspace: ".SddIA/workspaces/refine-item/41124dbc-febe-4ad8-a955-4e58560423f5"
audit_ref: docs/audits/man-43-h1-milestone-razonamiento-20261111.md
kaizen_ref: KZ-1
agents: mayeuta
execution_id: "a9098555-c861-40cc-8b13-c6871caf57e9"
---
# Clarificación — man-44-rechazar-apto-incoherente

Transcript Mayeuta (2026-11-20). Fase **Estabilización de Requisitos** del proceso `feature`.
Canal tracker: **MAN-44** (padre **MAN-43**). Documento semilla **PBI-8205539f** (KZ-1). Provider Linear: sin copia nueva bajo `docs/todos/`.

## D0 — Semilla y canal tracker

| Hecho | Laudo |
|-------|-------|
| Tracker `pbi_ref` | **MAN-44** (provider Linear). Canal: `.SddIA/state/tracker-outbound/MAN-44.jsonl` (`forge-pbi` → `refine-item` → `tracker-stamp` → `lifecycle-phase-action` / `feature`). |
| Padre / origen | **MAN-43** — epílogo mergeado con `validacion.md` `global: APTO` incoherente frente a `execution.md` `blocked` y plan con hijos abiertos (PR #453). |
| Fuente vinculante | `docs/audits/man-43-h1-milestone-razonamiento-20261111.md` — sección **KZ-1 — Coherencia de cierre**. Materializada en disco (MCP). |
| Documento refine | **PBI-8205539f** (`uuid` `bc1a0e6e-177c-4aa5-af7a-77965d92cd5b`). Workspace: `.SddIA/workspaces/refine-item/41124dbc-febe-4ad8-a955-4e58560423f5/` (`verdict: refined`, `open_questions: []`). |
| Tipo / DoR | Kaizen (`dor_profile: kaizen`): Fricción / Propuesta / Métrica de mejora. |
| Semilla orquestación | Frontmatter PBI declara `process: bug-fix`; ciclo runtime actual = **`feature`**. **L-PROCESS**. |
| Cuerpo Linear en disco | Canal jsonl solo operaciones; cuerpo DoR leído de `item.md` / `item.refined.md` (**L-TRACKER**). |
| Forja sellada en disco | `forge_key` declarado en PBI (`e413b26e…`); JSON bajo `.SddIA/state/forge-pbi/` **no** verificado en esta pasada. **No bloquea** Mayeuta (**L-FORGE**). |
| Hermanos fuera de alcance | **MAN-45** (KZ-2 PPR vinculante), **MAN-46** (KZ-3 hito ≠ Done padre), KZ-4 (deuda alcance MAN-43). |
| Modo | `branch_pr` / `feature`. |
| `execution_id` | **e049f4db-4e0e-41d0-b651-e9b9de645f71** |

## D1 — Matriz fricción × alcance estabilizado

| Afirmación (semilla / auditoría KZ-1) | Laudo |
|--------------------------------------|-------|
| `delivery-close-cycle` / `accept-pr` aceptan `global: APTO` escrito a mano sin contrastar el mismo `persist_ref` | **L-COHERENCE** |
| Coexistencia `validacion` APTO + `execution.verdict: blocked` / `status: partial` | **L-BLOCKED** |
| `plan.md` con fases/hijos sin evidencia de cierre y aun así APTO parental | **L-OPEN-PHASES** |
| Resolución ad hoc `PASS_H1_MILESTONE` elevó `global` a APTO | **L-RESOLUTION** |
| Dictamen Argos `NO_APTO` descartado del working tree | Contexto causal; este ciclo **no** exige PPR vinculante (eso es MAN-45) |
| Rechazo = fricción accionable; **no** `System_Fracture_Detected` | **L-FRICTION** |
| Momentos de contraste: pre-sello de diff y pre-fusión | **L-MOMENTS** |

## D2 — Laudos de estabilización (cerrados)

| ID | Decisión |
|----|----------|
| **L-DOMAIN** | Dominio = **aduana de coherencia documental de cierre** sobre el mismo `persist_ref`: `validacion.md` ↔ `execution.md` ↔ fases/`plan.md`. No es cambio de producto EventManager ni de alcance de MAN-43. |
| **L-COHERENCE** | Antes del **sello de diff** (`delivery-close-cycle`) y antes de la **fusión** (`accept-pr`), el motor **debe** contrastar `validacion.md` con `execution.md` y con las fases declaradas en `plan.md` del mismo `persist_ref`. Un `global: APTO` que falle el contraste es **inválido**. |
| **L-BLOCKED** | Si `execution.md` declara `verdict: blocked` (o estado equivalente de bloqueo / entrega parcial que el frontmatter canónico marque como no cerrada), `global: APTO` es **inválido**. |
| **L-OPEN-PHASES** | Si `plan.md` declara una fase/hijo sin evidencia de cierre en el mismo `persist_ref` (p. ej. sin registro de cierre en `execution.md` / artefacto de fase correspondiente), `global: APTO` del epílogo es **inválido**. «Cierre» = evidencia de fase completada en el árbol documental; no basta CI verde ni rewrite manual de `validacion.md`. |
| **L-RESOLUTION** | Solo resoluciones **enumeradas** por la norma activa `features-documentation-pattern` (y campos canónicos de `validacion.md` que esa norma reconoce) pueden sostener `global: APTO`. Tokens ad hoc (p. ej. `PASS_H1_MILESTONE`, `milestone: H1-only`, `h2_h6_deferred: true` usados como elevación) **no** elevan APTO ni habilitan sello/fusión del epílogo. |
| **L-FRICTION** | El rechazo emite **fricción accionable** (`friction_id` nuevo, Dedalo/Tekton eligen el identificador concreto). **Prohibido** emitir `System_Fracture_Detected` por esta incoherencia. |
| **L-MOMENTS** | Dos puertas obligatorias: (1) pre-sello de diff en el ciclo de entrega; (2) pre-fusión en `accept-pr`. Fallar en cualquiera aborta el avance (no fusiona / no sella como APTO coherente). |
| **L-COUNTERFACTUAL** | Con esta regla activa, el epílogo de MAN-43 bajo las condiciones de la auditoría KZ-1 **no** habría mergeado; el contrafactual queda como métrica de cierre (KA-CLOSE-4), no como revert del merge histórico. |
| **L-NON-GOALS** | Fuera de alcance: KZ-2 / **MAN-45** (PPR vinculante / atestación); KZ-3 / **MAN-46** (hito ≠ Done del padre / tracker); KZ-4 (completar resto H1 y H2–H6 de MAN-43). No reabrir L-1…L-7 ni el *qué* de producto de MAN-43 salvo lo necesario para la aduana de coherencia. |
| **L-PROCESS** | Ciclo documental/runtime = **`feature`** / `branch_pr`; el origen PBI `bug-fix` / tipo `kaizen` queda como metadato de forja, no como proceso de fase. |
| **L-DOC-CANON** | Documento canónico = **PBI-8205539f**; refine `verdict: refined`. |
| **L-FORGE** | Ausencia o no verificación del JSON `forge-pbi` **no** bloquea Mayeuta; es gate de forja de código, no de estabilización. |
| **L-GIT** | Git solo vía `skill:git-manager` / `./sddia-run.sh --tool git-manager`. |
| **L-TRACKER** | Provider Linear: sin ficheros nuevos de backlog en `docs/todos/`; KM/TODOs solo vía `agent:cumulo` o evento `Kaizen_Alert_Required`. |
| **L-TRUTH** | No inventar éxito: fixtures/gates deben materializar el rechazo; si no hay evidencia, el check permanece NO_APTO. |

## D3 — Criterios de aceptación del ciclo (producto)

| ID | Criterio |
|----|----------|
| **AC-1 (KA-CLOSE-1)** | Fixture con `execution.md` en `verdict: blocked` y `validacion.md` con `global: APTO` hace que `accept-pr` **aborte** (no fusiona). |
| **AC-2 (KA-CLOSE-2)** | Fixture con fase de `plan.md` sin evidencia de cierre y `validacion.md` APTO hace que el pre-sello / pre-fusión **rechace** el APTO con fricción accionable (sin `System_Fracture_Detected`). |
| **AC-3 (KA-CLOSE-3)** | Fixture cuya `resolution` (u homologable) no está enumerada en el patrón documental (p. ej. `PASS_H1_MILESTONE`) **no** permite elevar `global` a APTO ni completar el sello de merge del epílogo. |
| **AC-4 (KA-CLOSE-4)** | Contrafactual documentado: con esta regla activa, el epílogo de MAN-43 **no** habría mergeado bajo las condiciones de la auditoría KZ-1. |
| **AC-5 (Puertas)** | El contraste de coherencia se ejecuta en **pre-sello** y en **pre-fusión**; ambas puertas aplican el mismo *qué* (L-COHERENCE / L-MOMENTS). |

## D4 — Preguntas abiertas → cerradas sin bloqueo

| # | Pregunta | Resolución |
|---|----------|------------|
| 1 | ¿Proceso `bug-fix` (PBI) vs `feature` (runtime)? | Documentar y ejecutar como **`feature`** (**L-PROCESS**). |
| 2 | ¿Incluir PPR / atestación Argos en este ítem? | **No** — eso es KZ-2 / MAN-45 (**L-NON-GOALS**). |
| 3 | ¿Un hito puede cerrar el padre vía rewrite de `validacion`? | Fuera de este ciclo (MAN-46); aquí solo se invalida APTO incoherente (**L-RESOLUTION** / **L-OPEN-PHASES**). |
| 4 | ¿Revertir el merge histórico de MAN-43? | **No** — contrafactual + aduana futura (**L-COUNTERFACTUAL**). |
| 5 | ¿`System_Fracture_Detected` ante APTO falso? | **No** — solo fricción accionable (**L-FRICTION**). |
| 6 | ¿Falta JSON forge-pbi? | No bloquea estabilización (**L-FORGE**). |
| 7 | ¿Qué resoluciones son válidas? | Solo las que el patrón documental activo enumera/reconoce; ad hoc no elevan APTO (**L-RESOLUTION**). |

## D5 — Riesgos (vigilancia, no bloqueo Mayeuta)

| Riesgo | Tratamiento |
|--------|-------------|
| Operador reescribe `validacion.md` a APTO tras Argos NO_APTO | Puertas pre-sello/pre-fusión contrastan execution/plan (**L-COHERENCE**). |
| «Hito» narrado como Done del padre | Invalidar APTO si quedan fases abiertas; Done de padre = MAN-46 (**L-OPEN-PHASES** / **L-NON-GOALS**). |
| Fricción confusa con fractura sistémica | Contrato explícito: friction_id, cero `System_Fracture_Detected` (**L-FRICTION**). |
| Solo cubrir `accept-pr` y olvidar sello | Ambas puertas obligatorias (**L-MOMENTS** / **AC-5**). |

## D6 — Handoff Dedalo / Tekton

1. Consumir `clarify.md` + `objectives.md` en solo lectura; **no** reabrir L-COHERENCE…L-RESOLUTION.
2. Blueprint (`spec.md` / `plan.md`) acota touchpoints de aduana en `delivery-close-cycle` y `accept-pr` + fixtures KA-CLOSE-*.
3. Mayeuta **no** diseña implementación ni emite `delegates_to` de código.
4. Hermanos MAN-45 / MAN-46 / deuda MAN-43 permanecen fuera del *qué* de este `persist_ref`.

## D7 — Veredicto

**ok** — Requisitos estabilizados. El *qué* queda cerrado: coherencia pre-sello/pre-fusión `validacion`↔`execution`↔`plan`; APTO inválido si blocked o fases abiertas; resoluciones no enumeradas no elevan APTO; rechazo = fricción sin fractura; ACs **1…5** / métricas KA-CLOSE-1…4; fuera de alcance KZ-2…KZ-4. Tracker **MAN-44** / documento **PBI-8205539f** / padre **MAN-43** / `execution_id` **e049f4db-4e0e-41d0-b651-e9b9de645f71**.
