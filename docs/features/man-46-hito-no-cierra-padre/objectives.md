---
feature_name: man-46-hito-no-cierra-padre
created: "2026-11-21"
updated: "2026-11-21"
process: feature
purpose: Objetivos estabilizados — hito de plan ≠ Done del PBI padre ni del tracker
branch_name: feat/man-46-hito-no-cierra-padre
persist_ref: docs/features/man-46-hito-no-cierra-padre
pbi_ref: MAN-46
parent_ref: MAN-43
historia_ref: MAN-43
document_id: PBI-74cd5db0
status: requisitos-estabilizados
mayeuta_verdict: ok
phase: mayeuta-stabilization
audit_ref: docs/audits/man-43-h1-milestone-razonamiento-20261111.md
kaizen_ref: KZ-3
norm_ref: features-documentation-pattern@1.3.2
tracker_channel: ".SddIA/state/tracker-outbound/MAN-46.jsonl"
refine_workspace: ".SddIA/workspaces/refine-item/0fbe6552-7538-4827-b3d4-ffcba4d702f0"
agents: mayeuta
execution_id: "cbf032fc-8bbf-452e-b124-a5266b7d3fc4"
---
# Objetivos — man-46-hito-no-cierra-padre

## Misión

Impedir que un hito de plan (entrega parcial / slice hijo) cierre el PBI padre o el tracker: `delivery_state: success` del proceso `feature` solo si los AC de `objectives.md` del mismo `persist_ref` están verdes, **o** si existe refine sellado de reducción de alcance **anterior** al merge; el hito se entrega como ítem hijo y el padre permanece fuera de `state_map.done`; prohibido narrar «ciclo cerrado» en `validacion.md` del padre mientras haya AC abiertos.

Tracker **MAN-46** (padre **MAN-43**, documento **PBI-74cd5db0**, kaizen **KZ-3**).

**Por qué:** en el cierre de MAN-43, la orden de «cerrar el ciclo» se interpretó como Done del PBI/tracker; `milestone: H1-only` / `h2_h6_deferred` convivieron con `delivery_state: success` del feature padre sin refine sellado que autorizara reducir L-CHILDREN, y `validacion.md` narró cierre con AC abiertos.

## Contexto estabilizado

| Hecho | Impacto |
|-------|---------|
| Tracker | **MAN-46** (canal `.SddIA/state/tracker-outbound/MAN-46.jsonl`); padre/historia **MAN-43** |
| Fuente vinculante | `docs/audits/man-43-h1-milestone-razonamiento-20261111.md` (KZ-3) |
| Documento refine | **PBI-74cd5db0** (`verdict: refined`; `open_questions: []`) |
| Clarificaciones | `clarify.md` (laudos L-SUCCESS…L-SCOPE-CUT; veredicto **ok**) |
| Hermanos | MAN-44 (KZ-1), MAN-45 (KZ-2); deuda alcance MAN-43 = KZ-4 — **fuera** |
| `execution_id` | `cbf032fc-8bbf-452e-b124-a5266b7d3fc4` |

## Alcance

| ID | Objetivo |
|----|----------|
| **C1** | Acotar `delivery_state: success` del feature a AC verdes del mismo `persist_ref` **o** refine sellado de reducción de alcance previo al merge. |
| **C2** | Tratar el hito como ítem hijo (`historia_ref` = padre); el éxito del hijo no implica Done del padre. |
| **C3** | Mantener el padre fuera de `state_map.done` tras merge de hijo con AC parentales abiertos. |
| **C4** | Rechazar narración de «ciclo cerrado» en `validacion.md` del padre con AC abiertos; tokens de hito/diferimiento no habilitan éxito parental. |
| **C5** | Exigir refine sellado previo para recortar alcance parental; prohibir rewrite ad hoc de `validacion.md` en la misma pasada del push. |

## Objetivos medibles

| ID | Objetivo | Criterio de cierre |
|----|----------|-------------------|
| **O1** | Padre no Done por hijo | Fixture KA-MILE-1: hijo mergea OK + padre con AC abiertos ⇒ padre ≠ `state_map.done` y sin `delivery_state: success` parental |
| **O2** | Narración hito rechazada | Fixture KA-MILE-2: `H1-only` / `*_deferred` sin refine sellado ⇒ rechaza «ciclo cerrado» parental |
| **O3** | Éxito solo con laudo | Fixture KA-MILE-3: refine sellado previo ⇒ `delivery_state: success` solo contra alcance laudeado |
| **O4** | Contrafactual KZ-3 | Documentado que el merge H1 de MAN-43 no habría cerrado padre/tracker con la regla activa (KA-MILE-4) |

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| **AC-1 (KA-MILE-1)** | Fixture donde un hijo mergea con éxito y el padre aún tiene AC abiertos **no** mueve el padre a `state_map.done` ni permite `delivery_state: success` del feature padre. |
| **AC-2 (KA-MILE-2)** | Fixture con `milestone: H1-only` / `h2_h6_deferred` (o equivalente) y sin refine sellado de reducción de alcance hace que narrar «ciclo cerrado» en `validacion.md` del padre sea **rechazado** (no habilita éxito parental). |
| **AC-3 (KA-MILE-3)** | Fixture con refine sellado de reducción de alcance **anterior** al merge permite `delivery_state: success` del padre solo contra el alcance laudeado (no por rewrite ad hoc de `validacion.md` en la misma pasada). |
| **AC-4 (KA-MILE-4)** | Contrafactual documentado: con esta regla activa, el merge H1 de MAN-43 **no** habría cerrado el PBI padre ni el tracker bajo las condiciones de la auditoría KZ-3. |

## Invariantes

- Unidad de cierre = `persist_ref` del feature cuyas AC / laudo de alcance se evalúan.
- Hito ≠ Done del padre / tracker.
- `delivery_state: success` exige AC verdes **o** refine sellado de reducción de alcance previo.
- Tokens de hito/diferimiento no elevan éxito parental ni sustituyen el laudo.
- Recorte de alcance parental = refine sellado previo; no rewrite en la misma pasada del push.
- Git solo vía `skill:git-manager`.
- KM / `docs/todos/`: solo `agent:cumulo` o evento `Kaizen_Alert_Required`.

## Fuera de alcance

- KZ-1 / **MAN-44** — coherencia `global: APTO` ↔ `execution`/`plan` / resoluciones no enumeradas.
- KZ-2 / **MAN-45** — PPR vinculante / atestación `qa-attestation`.
- KZ-4 — completar resto de H1 y H2–H6 de MAN-43.
- Reabrir L-1…L-7 ni el *qué* de producto EventManager de MAN-43.
- Revertir el merge histórico del PR #453.
- Semillas Kaizen bajo `docs/todos/` desde Tekton/Argos/Mayeuta.

## Dependencias

- Padre / origen causal: **MAN-43**; auditoría `docs/audits/man-43-h1-milestone-razonamiento-20261111.md`.
- Artefactos contrastados: `objectives.md` / `validacion.md` del `persist_ref` parental; lifecycle tracker (`state_map`); refine sellado de reducción de alcance.
- Norma: `features-documentation-pattern` v1.3.2 (un `.md` por fase + frontmatter).
- Clarificaciones canónicas: `clarify.md` (laudos L-*).
- Refine: `.SddIA/workspaces/refine-item/0fbe6552-7538-4827-b3d4-ffcba4d702f0/`.

## Ley aplicada

- Norma `features-documentation-pattern` v1.3.2 (un `.md` por fase + frontmatter).
- Proceso `feature`; Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
- Clarificaciones canónicas: `clarify.md` (laudos L-*).
