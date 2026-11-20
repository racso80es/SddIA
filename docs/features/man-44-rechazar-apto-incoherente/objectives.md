---


feature_name: man-44-rechazar-apto-incoherente
created: "2026-11-20"
updated: "2026-11-20"
process: feature
purpose: Objetivos estabilizados — rechazar global APTO incoherente (blocked / fases abiertas / resolución no enumerada)
branch_name: feat/man-44-rechazar-apto-incoherente
persist_ref: docs/features/man-44-rechazar-apto-incoherente
pbi_ref: MAN-44
parent_ref: MAN-43
historia_ref: MAN-43
document_id: PBI-8205539f
status: requisitos-estabilizados
mayeuta_verdict: ok
phase: mayeuta-stabilization
audit_ref: docs/audits/man-43-h1-milestone-razonamiento-20261111.md
kaizen_ref: KZ-1
norm_ref: features-documentation-pattern@1.3.1
tracker_channel: ".SddIA/state/tracker-outbound/MAN-44.jsonl"
refine_workspace: ".SddIA/workspaces/refine-item/41124dbc-febe-4ad8-a955-4e58560423f5"
agents: mayeuta
execution_id: "a9098555-c861-40cc-8b13-c6871caf57e9"
---
# Objetivos — man-44-rechazar-apto-incoherente

## Misión

Impedir que la aduana de entrega (`delivery-close-cycle` / `accept-pr`) acepte un `validacion.md` con `global: APTO` cuando el mismo `persist_ref` contradice la verdad de ejecución o de plan: `execution` en `blocked` (o no cerrada), fases/hijos de `plan.md` sin evidencia de cierre, o elevación mediante resolución no enumerada en el patrón documental.

Tracker **MAN-44** (padre **MAN-43**, documento **PBI-8205539f**, kaizen **KZ-1**).

**Por qué:** en el cierre de MAN-43, `global: APTO` escrito a mano convivió con `execution.md` `verdict: blocked`, plan con H2–H6 abiertos y resolución ad hoc `PASS_H1_MILESTONE`, permitiendo el merge del epílogo (PR #453) sin coherencia pre-sello/pre-fusión.

## Contexto estabilizado

| Hecho | Impacto |
|-------|---------|
| Tracker | **MAN-44** (canal `.SddIA/state/tracker-outbound/MAN-44.jsonl`); padre/historia **MAN-43** |
| Fuente vinculante | `docs/audits/man-43-h1-milestone-razonamiento-20261111.md` (KZ-1) |
| Documento refine | **PBI-8205539f** (`verdict: refined`; `open_questions: []`) |
| Clarificaciones | `clarify.md` (laudos L-COHERENCE…L-RESOLUTION; veredicto **ok**) |
| Hermanos | MAN-45 (KZ-2), MAN-46 (KZ-3); deuda alcance MAN-43 = KZ-4 — **fuera** |
| `execution_id` | `e049f4db-4e0e-41d0-b651-e9b9de645f71` |

## Alcance

| ID | Objetivo |
|----|----------|
| **C1** | Contraste pre-sello y pre-fusión `validacion.md` ↔ `execution.md` ↔ fases de `plan.md` del mismo `persist_ref`. |
| **C2** | Invalidar `global: APTO` si `execution.verdict` es `blocked` (o estado de no-cierre equivalente). |
| **C3** | Invalidar `global: APTO` si queda fase/hijo del plan sin evidencia de cierre. |
| **C4** | Impedir que resoluciones no enumeradas en `features-documentation-pattern` eleven APTO. |
| **C5** | Rechazo como fricción accionable (`friction_id`); **sin** `System_Fracture_Detected`. |

## Objetivos medibles

| ID | Objetivo | Criterio de cierre |
|----|----------|-------------------|
| **O1** | Aborto por blocked | Fixture KA-CLOSE-1: `accept-pr` aborta ante `execution` blocked + `validacion` APTO |
| **O2** | Rechazo por fase abierta | Fixture KA-CLOSE-2: pre-sello/pre-fusión rechaza APTO con fricción (sin fractura) |
| **O3** | Resolución no canónica | Fixture KA-CLOSE-3: `PASS_H1_MILESTONE` (u homologable) no eleva APTO ni completa sello de merge |
| **O4** | Contrafactual KZ-1 | Documentado que MAN-43 no habría mergeado el epílogo con la regla activa (KA-CLOSE-4) |
| **O5** | Doble puerta | Mismo contraste en pre-sello (`delivery-close-cycle`) y pre-fusión (`accept-pr`) |

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| **AC-1 (KA-CLOSE-1)** | Fixture con `execution.md` en `verdict: blocked` y `validacion.md` con `global: APTO` hace que `accept-pr` **aborte** (no fusiona). |
| **AC-2 (KA-CLOSE-2)** | Fixture con fase de `plan.md` sin evidencia de cierre y `validacion.md` APTO hace que el pre-sello / pre-fusión **rechace** el APTO con fricción accionable (sin `System_Fracture_Detected`). |
| **AC-3 (KA-CLOSE-3)** | Fixture cuya `resolution` no está en el patrón documental (p. ej. `PASS_H1_MILESTONE`) **no** permite elevar `global` a APTO ni completar el sello de merge del epílogo. |
| **AC-4 (KA-CLOSE-4)** | Contrafactual documentado: con esta regla activa, el epílogo de MAN-43 **no** habría mergeado bajo las condiciones de la auditoría KZ-1. |
| **AC-5 (Puertas)** | El contraste de coherencia se ejecuta en pre-sello y en pre-fusión. |

## Invariantes

- Un único `persist_ref` es la unidad de contraste (no atestaciones ajenas al árbol documental de la tarea).
- `global: APTO` exige coherencia con ejecución no bloqueada y plan sin fases abiertas.
- Resoluciones ad hoc no sustituyen el patrón documental.
- Rechazo = fricción; nunca `System_Fracture_Detected` por esta causa.
- Git solo vía `skill:git-manager`.
- KM / `docs/todos/`: solo `agent:cumulo` o evento `Kaizen_Alert_Required`.

## Fuera de alcance

- KZ-2 / **MAN-45** — PPR vinculante / atestación `qa-attestation`.
- KZ-3 / **MAN-46** — hito distinto de Done del padre / tracker.
- KZ-4 — completar resto de H1 y H2–H6 de MAN-43.
- Reabrir L-1…L-7 ni el *qué* de producto EventManager de MAN-43.
- Revertir el merge histórico del PR #453.
- Semillas Kaizen bajo `docs/todos/` desde Tekton/Argos/Mayeuta.

## Dependencias

- Padre / origen causal: **MAN-43**; auditoría `docs/audits/man-43-h1-milestone-razonamiento-20261111.md`.
- Procesos de aduana: `delivery-close-cycle` (pre-sello), `accept-pr` (pre-fusión).
- Artefactos contrastados: `validacion.md`, `execution.md`, `plan.md` del mismo `persist_ref`.
- Norma: `features-documentation-pattern` v1.3.1 (resoluciones/campos canónicos de `validacion.md`).
- Clarificaciones canónicas: `clarify.md` (laudos L-*).
- Refine: `.SddIA/workspaces/refine-item/41124dbc-febe-4ad8-a955-4e58560423f5/`.

## Ley aplicada

- Norma `features-documentation-pattern` v1.3.1 (un `.md` por fase + frontmatter).
- Proceso `feature`; Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
- Clarificaciones canónicas: `clarify.md` (laudos L-*).
