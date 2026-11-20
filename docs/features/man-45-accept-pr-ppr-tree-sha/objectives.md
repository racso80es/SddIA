---
feature_name: man-45-accept-pr-ppr-tree-sha
created: "2026-11-20"
updated: "2026-11-20"
process: feature
purpose: Objetivos estabilizados — accept-pr exige veredicto PPR del mismo tree_sha
branch_name: feat/man-45-accept-pr-ppr-tree-sha
persist_ref: docs/features/man-45-accept-pr-ppr-tree-sha
pbi_ref: MAN-45
parent_ref: MAN-43
historia_ref: MAN-43
document_id: PBI-75d12fd0
status: requisitos-estabilizados
mayeuta_verdict: ok
phase: mayeuta-stabilization
audit_ref: docs/audits/man-43-h1-milestone-razonamiento-20261111.md
kaizen_ref: KZ-2
norm_ref: features-documentation-pattern@1.3.2
tracker_channel: ".SddIA/state/tracker-outbound/MAN-45.jsonl"
refine_workspace: ".SddIA/workspaces/refine-item/2f723b31-7289-4392-a8e7-43348c05e83c"
agents: mayeuta
execution_id: "73910788-b7e7-4a61-9ac2-72fedd024b2a"
---
# Objetivos — man-45-accept-pr-ppr-tree-sha

## Misión

Impedir que `accept-pr` fusione un PR de delta `active` sin un veredicto PPR vinculante sobre el mismo árbol: atestación `qa-attestation/1.0` con `verdict: aprobado`, emitida por `pull-request-review`, con `tree_sha` igual al de la rama y `qa_profile: full` cuando el delta es `active`. Ni `docs-only` ni `skipped-ppr-dedupe` habilitan la fusión.

Tracker **MAN-45** (padre **MAN-43**, documento **PBI-75d12fd0**, kaizen **KZ-2**).

**Por qué:** en el cierre de MAN-43, PPR full quedó `blocked`/detached; el push usó atestación `docs-only` o JSON `full` no emitido por PPR; la Auditoría Genómica de `accept-pr` solo comprobó `PullRequest_Presented` y trató `skipped-ppr-dedupe` como QA suficiente, permitiendo fusionar sin aprobación Argos del `tree_sha` fusionado.

## Contexto estabilizado

| Hecho | Impacto |
|-------|---------|
| Tracker | **MAN-45** (canal `.SddIA/state/tracker-outbound/MAN-45.jsonl`); padre/historia **MAN-43** |
| Fuente vinculante | `docs/audits/man-43-h1-milestone-razonamiento-20261111.md` (KZ-2) |
| Documento refine | **PBI-75d12fd0** (`verdict: refined`; `open_questions: []`) |
| Clarificaciones | `clarify.md` (laudos L-ATTEST…L-DEDUPE; veredicto **ok**) |
| Hermanos | MAN-44 (KZ-1), MAN-46 (KZ-3); deuda alcance MAN-43 = KZ-4 — **fuera** |
| `execution_id` | `73910788-b7e7-4a61-9ac2-72fedd024b2a` |

## Alcance

| ID | Objetivo |
|----|----------|
| **C1** | Exigir atestación `qa-attestation/1.0` con `verdict: aprobado` antes de fusionar. |
| **C2** | Acreditar emisor = `pull-request-review` (`execution_id` de ese proceso); rechazar escritura directa. |
| **C3** | Exigir igualdad `tree_sha` atestación ↔ árbol de la rama a fusionar. |
| **C4** | Con delta `active`, exigir `qa_profile: full`; `docs-only` no habilita merge. |
| **C5** | Tratar `skipped-ppr-dedupe` como no-aprobación (no habilita fusión). |
| **C6** | Leer atestación/veredicto en la puerta pre-fusión; no bastar con `PullRequest_Presented`. |

## Objetivos medibles

| ID | Objetivo | Criterio de cierre |
|----|----------|-------------------|
| **O1** | Aborto sin PPR aprobado | Fixture KA-PPR-1: sin `qa-attestation/1.0` `aprobado` del `tree_sha` → `accept-pr` `abortado` |
| **O2** | Rechazo docs-only / full forjado | Fixture KA-PPR-2: delta `active` + `docs-only` (o JSON `full` no-PPR) → aborta |
| **O3** | Rechazo tree divergente | Fixture KA-PPR-3: `tree_sha` atestación ≠ rama → aborta |
| **O4** | Dedupe no es QA | Fixture KA-PPR-4: solo `skipped-ppr-dedupe` → no fusiona |
| **O5** | Contrafactual KZ-2 | Documentado que MAN-43 no habría pasado la aduana con la regla activa (KA-PPR-5) |

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| **AC-1 (KA-PPR-1)** | Sin atestación `qa-attestation/1.0` con `verdict: aprobado` sobre el `tree_sha` que se fusiona, `accept-pr` devuelve **`abortado`** (no fusiona). |
| **AC-2 (KA-PPR-2)** | Fixture con delta `active` y solo atestación `docs-only` (o JSON `full` no emitido por `pull-request-review`) hace que `accept-pr` **aborte**. |
| **AC-3 (KA-PPR-3)** | Fixture cuyo `tree_sha` de atestación **diverge** del árbol de la rama hace que `accept-pr` **aborte**. |
| **AC-4 (KA-PPR-4)** | Fixture que solo acredita `skipped-ppr-dedupe` (sin PPR `aprobado`) **no** habilita fusión. |
| **AC-5 (KA-PPR-5)** | Contrafactual documentado: con esta regla activa, el merge de MAN-43 bajo las condiciones de la auditoría KZ-2 **no** habría pasado la aduana. |

## Invariantes

- Unidad de contraste = árbol a fusionar (`tree_sha` de la rama) frente a atestación vinculante.
- Solo `pull-request-review` puede emitir la atestación que habilita merge.
- Delta `active` ⇒ `qa_profile: full`; `docs-only` no sustituye.
- `skipped-ppr-dedupe` ≠ aprobación.
- Presentación del PR (`PullRequest_Presented`) ≠ veredicto técnico pleno.
- Git solo vía `skill:git-manager`.
- KM / `docs/todos/`: solo `agent:cumulo` o evento `Kaizen_Alert_Required`.

## Fuera de alcance

- KZ-1 / **MAN-44** — coherencia `global: APTO` ↔ `execution`/`plan` / resoluciones no enumeradas.
- KZ-3 / **MAN-46** — hito distinto de Done del padre / tracker.
- KZ-4 — completar resto de H1 y H2–H6 de MAN-43.
- Reabrir L-1…L-7 ni el *qué* de producto EventManager de MAN-43.
- Revertir el merge histórico del PR #453.
- Semillas Kaizen bajo `docs/todos/` desde Tekton/Argos/Mayeuta.

## Dependencias

- Padre / origen causal: **MAN-43**; auditoría `docs/audits/man-43-h1-milestone-razonamiento-20261111.md`.
- Proceso de aduana: `accept-pr` (pre-fusión); emisor: `pull-request-review`.
- Artefacto vinculante: atestación `qa-attestation/1.0` (verdict, tree_sha, qa_profile, execution_id).
- Norma documental: `features-documentation-pattern` v1.3.2.
- Clarificaciones canónicas: `clarify.md` (laudos L-*).
- Refine: `.SddIA/workspaces/refine-item/2f723b31-7289-4392-a8e7-43348c05e83c/`.

## Ley aplicada

- Norma `features-documentation-pattern` v1.3.2 (un `.md` por fase + frontmatter).
- Proceso `feature`; Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
- Clarificaciones canónicas: `clarify.md` (laudos L-*).
