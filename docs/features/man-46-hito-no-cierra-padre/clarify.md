---


feature_name: man-46-hito-no-cierra-padre
created: "2026-11-21"
updated: "2026-11-21"
process: feature
purpose: Estabilización de requisitos — un hito de plan no cierra el PBI padre ni el tracker (KZ-3)
branch_name: feat/man-46-hito-no-cierra-padre
persist_ref: docs/features/man-46-hito-no-cierra-padre
pbi_ref: MAN-46
parent_ref: MAN-43
historia_ref: MAN-43
document_id: PBI-74cd5db0
phase: mayeuta-stabilization
mayeuta_verdict: ok
norm_ref: features-documentation-pattern@1.3.2
tracker_channel: ".SddIA/state/tracker-outbound/MAN-46.jsonl"
refine_workspace: ".SddIA/workspaces/refine-item/0fbe6552-7538-4827-b3d4-ffcba4d702f0"
audit_ref: docs/audits/man-43-h1-milestone-razonamiento-20261111.md
kaizen_ref: KZ-3
agents: mayeuta
execution_id: "6db0b201-9b34-4fe7-9b7c-ca7757e8e84f"
---
# Clarificación — man-46-hito-no-cierra-padre

Transcript Mayeuta (2026-11-21). Fase **Estabilización de Requisitos** del proceso `feature`.
Canal tracker: **MAN-46** (padre **MAN-43**). Documento semilla **PBI-74cd5db0** (KZ-3). Provider Linear: sin copia nueva bajo `docs/todos/`.

## D0 — Semilla y canal tracker

| Hecho | Laudo |
|-------|-------|
| Tracker `pbi_ref` | **MAN-46** (provider Linear). Canal: `.SddIA/state/tracker-outbound/MAN-46.jsonl` (`forge-pbi` → `refine-item` → `tracker-stamp` → `lifecycle-phase-action` / `feature`). |
| Padre / origen | **MAN-43** — merge del slice H1 narrado como cierre del ciclo parental / tracker sin AC verdes ni refine de reducción de alcance. |
| Fuente vinculante | `docs/audits/man-43-h1-milestone-razonamiento-20261111.md` — sección **KZ-3 — Hito distinto de Done del padre**. Materializada en disco (MCP). |
| Documento refine | **PBI-74cd5db0** (`uuid` `d358497b-6c8b-45f3-bd4e-5475f051c6c7`). Workspace: `.SddIA/workspaces/refine-item/0fbe6552-7538-4827-b3d4-ffcba4d702f0/` (`verdict: refined`, `open_questions: []`). |
| Tipo / DoR | Kaizen (`dor_profile: kaizen`): Fricción / Propuesta / Métrica de mejora. |
| Semilla orquestación | Frontmatter PBI y auditoría KZ-3 declaran `process: feature`; ciclo runtime actual = **`feature`**. **L-PROCESS**. |
| Cuerpo Linear en disco | Canal jsonl solo operaciones; cuerpo DoR leído de `item.md` / `item.refined.md` (**L-TRACKER**). |
| Forja sellada en disco | `forge_key` declarado en PBI (`e4726ba8…`); JSON bajo `.SddIA/state/forge-pbi/` **no** verificado en esta pasada. **No bloquea** Mayeuta (**L-FORGE**). |
| Hermanos fuera de alcance | **MAN-44** (KZ-1 coherencia APTO/blocked), **MAN-45** (KZ-2 PPR vinculante), KZ-4 (deuda alcance MAN-43). |
| Modo | `branch_pr` / `feature`. |
| `execution_id` | **cbf032fc-8bbf-452e-b124-a5266b7d3fc4** |

## D1 — Matriz fricción × alcance estabilizado

| Afirmación (semilla / auditoría KZ-3) | Laudo |
|--------------------------------------|-------|
| Orden operativa «cerrar el ciclo» interpretada como Done del PBI padre y del tracker | **L-DOMAIN** / **L-PARENT-DONE** |
| Vocabulario `milestone: H1-only` / `h2_h6_deferred: true` convivió con `delivery_state: success` del feature parental | **L-SUCCESS** / **L-NARRATIVE** |
| Sin laudo de reducción de alcance (refine sellado) que autorice recortar L-CHILDREN (H1–H6, AC-1…14) antes del merge | **L-SCOPE-CUT** |
| `validacion.md` del padre narró «ciclo cerrado» con AC abiertos | **L-NARRATIVE** |
| Entrega parcial del hijo usada como señal de Done del padre / tracker | **L-CHILD** / **L-PARENT-DONE** |
| Merge de slice (hijo) puede mover o narrar el padre como realizado sin AC verdes | **L-SUCCESS** / **L-PARENT-DONE** |

## D2 — Laudos de estabilización (cerrados)

| ID | Decisión |
|----|----------|
| **L-DOMAIN** | Dominio = **norma de cierre parental vs hito**: un hito de plan no constituye Done del PBI padre ni del tracker. No es cambio de producto EventManager ni de alcance de MAN-43. |
| **L-SUCCESS** | `delivery_state: success` del proceso `feature` del **mismo** `persist_ref` solo si (a) los AC de `objectives.md` de ese `persist_ref` están verdes, **o** (b) el tracker tiene un **refine sellado** de reducción de alcance **anterior** al merge. |
| **L-CHILD** | Un hito de plan se entrega como **ítem hijo** (`historia_ref` = padre). El éxito del hijo **no** implica Done del padre. |
| **L-PARENT-DONE** | Tras el merge de un hijo con éxito, el padre permanece **fuera** de `state_map.done` mientras sus AC (o el alcance laudeado vigente) no estén cerrados. |
| **L-NARRATIVE** | Prohibido narrar «ciclo cerrado» (u homologable) en `validacion.md` del **padre** mientras existan AC abiertos del alcance parental vigente. Tokens de hito/diferimiento (`milestone: H*-only`, `*_deferred: true`) **no** habilitan éxito parental ni Done del tracker. |
| **L-SCOPE-CUT** | Recortar hijos/fases del plan parental (p. ej. H2–H6 / L-CHILDREN) exige **refine sellado previo** en el tracker; **prohibido** un rewrite de `validacion.md` en la misma pasada que el push como sustituto del laudo. |
| **L-COUNTERFACTUAL** | Con esta regla activa, el merge H1 de MAN-43 bajo las condiciones de la auditoría KZ-3 **no** habría cerrado el PBI padre ni el tracker; el contrafactual queda como métrica (KA-MILE-4), no como revert del merge histórico. |
| **L-NON-GOALS** | Fuera de alcance: KZ-1 / **MAN-44** (coherencia APTO/blocked / resoluciones); KZ-2 / **MAN-45** (PPR vinculante / atestación); KZ-4 (completar resto H1 y H2–H6 de MAN-43). No reabrir L-1…L-7 ni el *qué* de producto de MAN-43 salvo lo necesario para la norma hito ≠ Done del padre. |
| **L-PROCESS** | Ciclo documental/runtime = **`feature`** / `branch_pr`; tipo `kaizen` queda como metadato de forja. |
| **L-DOC-CANON** | Documento canónico = **PBI-74cd5db0**; refine `verdict: refined`. |
| **L-FORGE** | Ausencia o no verificación del JSON `forge-pbi` **no** bloquea Mayeuta; es gate de forja de código, no de estabilización. |
| **L-GIT** | Git solo vía `skill:git-manager` / `./sddia-run.sh --tool git-manager`. |
| **L-TRACKER** | Provider Linear: sin ficheros nuevos de backlog en `docs/todos/`; KM/TODOs solo vía `agent:cumulo` o evento `Kaizen_Alert_Required`. |
| **L-TRUTH** | No inventar éxito: fixtures/gates deben materializar el rechazo de cierre parental prematuro; si no hay evidencia, el check permanece NO_APTO. |

## D3 — Criterios de aceptación del ciclo (producto)

| ID | Criterio |
|----|----------|
| **AC-1 (KA-MILE-1)** | Fixture donde un hijo mergea con éxito y el padre aún tiene AC abiertos **no** mueve el padre a `state_map.done` ni permite `delivery_state: success` del feature padre. |
| **AC-2 (KA-MILE-2)** | Fixture con `milestone: H1-only` / `h2_h6_deferred` (o equivalente) y sin refine sellado de reducción de alcance hace que narrar «ciclo cerrado» en `validacion.md` del padre sea **rechazado** (no habilita éxito parental). |
| **AC-3 (KA-MILE-3)** | Fixture con refine sellado de reducción de alcance **anterior** al merge permite `delivery_state: success` del padre solo contra el alcance laudeado (no por rewrite ad hoc de `validacion.md` en la misma pasada). |
| **AC-4 (KA-MILE-4)** | Contrafactual documentado: con esta regla activa, el merge H1 de MAN-43 **no** habría cerrado el PBI padre ni el tracker bajo las condiciones de la auditoría KZ-3. |

## D4 — Preguntas abiertas → cerradas sin bloqueo

| # | Pregunta | Resolución |
|---|----------|------------|
| 1 | ¿Proceso `feature` vs `bug-fix`? | Ejecutar y documentar como **`feature`** (semilla + auditoría KZ-3) (**L-PROCESS**). |
| 2 | ¿Un hito puede cerrar el padre vía rewrite de `validacion`? | **No** — exige AC verdes o refine sellado previo (**L-SUCCESS** / **L-SCOPE-CUT** / **L-NARRATIVE**). |
| 3 | ¿El merge del hijo mueve el padre a `state_map.done`? | **No** (**L-CHILD** / **L-PARENT-DONE** / **AC-1**). |
| 4 | ¿Incluir coherencia APTO↔execution o PPR en este ítem? | **No** — MAN-44 / MAN-45 (**L-NON-GOALS**). |
| 5 | ¿Revertir el merge histórico de MAN-43? | **No** — contrafactual + norma futura (**L-COUNTERFACTUAL**). |
| 6 | ¿Falta JSON forge-pbi? | No bloquea estabilización (**L-FORGE**). |
| 7 | ¿Tokens `H1-only` / `*_deferred` habilitan éxito parental? | **No** (**L-NARRATIVE** / **AC-2**). |

## D5 — Riesgos (vigilancia, no bloqueo Mayeuta)

| Riesgo | Tratamiento |
|--------|-------------|
| Operador narra «ciclo cerrado» en el padre tras merge de hito | Rechazo narrativo + bloqueo de `delivery_state: success` (**L-NARRATIVE** / **L-SUCCESS**). |
| Recorte de H2–H6 vía rewrite en la misma pasada del push | Exigir refine sellado previo (**L-SCOPE-CUT** / **AC-3**). |
| Lifecycle del tracker mueve el padre a Done al cerrar el hijo | Padre fuera de `state_map.done` hasta AC/laudo (**L-PARENT-DONE** / **AC-1**). |
| Mezclar con gates de coherencia documental MAN-44 o PPR MAN-45 | Mantener fronteras de hermano (**L-NON-GOALS**). |

## D6 — Handoff Dedalo / Tekton

1. Consumir `clarify.md` + `objectives.md` en solo lectura; **no** reabrir L-SUCCESS…L-SCOPE-CUT.
2. Blueprint (`spec.md` / `plan.md`) acota touchpoints de cierre parental / lifecycle tracker / narración `validacion` + fixtures KA-MILE-*.
3. Mayeuta **no** diseña implementación ni emite `delegates_to` de código.
4. Hermanos MAN-44 / MAN-45 / deuda MAN-43 permanecen fuera del *qué* de este `persist_ref`.

## D7 — Veredicto

**ok** — Requisitos estabilizados. El *qué* queda cerrado: `delivery_state: success` del feature solo con AC verdes o refine sellado de reducción de alcance previo; hito = ítem hijo; padre fuera de `state_map.done`; prohibido narrar ciclo cerrado con AC abiertos; recorte de alcance exige refine previo (no rewrite ad hoc); ACs **1…4** / métricas KA-MILE-1…4; fuera de alcance KZ-1 / KZ-2 / KZ-4. Tracker **MAN-46** / documento **PBI-74cd5db0** / padre **MAN-43** / `execution_id` **cbf032fc-8bbf-452e-b124-a5266b7d3fc4**.
