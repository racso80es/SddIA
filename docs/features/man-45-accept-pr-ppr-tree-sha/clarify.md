---

feature_name: man-45-accept-pr-ppr-tree-sha
created: "2026-11-20"
updated: "2026-11-20"
process: feature
purpose: Estabilización de requisitos — accept-pr exige veredicto PPR del mismo tree_sha (KZ-2)
branch_name: feat/man-45-accept-pr-ppr-tree-sha
persist_ref: docs/features/man-45-accept-pr-ppr-tree-sha
pbi_ref: MAN-45
parent_ref: MAN-43
historia_ref: MAN-43
document_id: PBI-75d12fd0
phase: mayeuta-stabilization
mayeuta_verdict: ok
norm_ref: features-documentation-pattern@1.3.2
tracker_channel: ".SddIA/state/tracker-outbound/MAN-45.jsonl"
refine_workspace: ".SddIA/workspaces/refine-item/2f723b31-7289-4392-a8e7-43348c05e83c"
audit_ref: docs/audits/man-43-h1-milestone-razonamiento-20261111.md
kaizen_ref: KZ-2
agents: mayeuta
execution_id: "52da40f9-4dde-44a6-88d6-ce6efb0a1ec8"
---
# Clarificación — man-45-accept-pr-ppr-tree-sha

Transcript Mayeuta (2026-11-20). Fase **Estabilización de Requisitos** del proceso `feature`.
Canal tracker: **MAN-45** (padre **MAN-43**). Documento semilla **PBI-75d12fd0** (KZ-2). Provider Linear: sin copia nueva bajo `docs/todos/`.

## D0 — Semilla y canal tracker

| Hecho | Laudo |
|-------|-------|
| Tracker `pbi_ref` | **MAN-45** (provider Linear). Canal: `.SddIA/state/tracker-outbound/MAN-45.jsonl` (`forge-pbi` → `refine-item` → `tracker-stamp` → `lifecycle-phase-action` / `feature`). |
| Padre / origen | **MAN-43** — merge del epílogo (PR #453) sin PPR `aprobado` vinculante sobre el `tree_sha` fusionado. |
| Fuente vinculante | `docs/audits/man-43-h1-milestone-razonamiento-20261111.md` — sección **KZ-2 — Veredicto PPR vinculante**. Materializada en disco (MCP). |
| Documento refine | **PBI-75d12fd0** (`uuid` `7bdc3906-3664-4bbe-b783-f170887227d9`). Workspace: `.SddIA/workspaces/refine-item/2f723b31-7289-4392-a8e7-43348c05e83c/` (`verdict: refined`, `open_questions: []`). |
| Tipo / DoR | Kaizen (`dor_profile: kaizen`): Fricción / Propuesta / Métrica de mejora. |
| Semilla orquestación | Frontmatter PBI declara `process: bug-fix`; ciclo runtime actual = **`feature`**. **L-PROCESS**. |
| Cuerpo Linear en disco | Canal jsonl solo operaciones; cuerpo DoR leído de `item.md` / `item.refined.md` (**L-TRACKER**). |
| Forja sellada en disco | `forge_key` declarado en PBI (`143b3b25…`); JSON bajo `.SddIA/state/forge-pbi/` **no** verificado en esta pasada. **No bloquea** Mayeuta (**L-FORGE**). |
| Hermanos fuera de alcance | **MAN-44** (KZ-1 coherencia APTO/blocked), **MAN-46** (KZ-3 hito ≠ Done padre), KZ-4 (deuda alcance MAN-43). |
| Modo | `branch_pr` / `feature`. |
| `execution_id` | **73910788-b7e7-4a61-9ac2-72fedd024b2a** |

## D1 — Matriz fricción × alcance estabilizado

| Afirmación (semilla / auditoría KZ-2) | Laudo |
|--------------------------------------|-------|
| `pull-request-review` full quedó `blocked` o detached; no había aprobación Argos usable sobre el `tree_sha` de la rama | **L-PPR-BIND** |
| Push usó perfil `docs-only` o JSON `full` escrito fuera de `try_write_from_ppr` (no emitido por PPR) | **L-EMITTER** / **L-PROFILE** |
| Auditoría Genómica de `accept-pr` solo comprueba `PullRequest_Presented`; no lee veredicto Argos ni coherencia de `qa-attestation/1.0` | **L-GENOMIC** |
| `skipped-ppr-dedupe` se trató como paso de QA suficiente para habilitar merge | **L-DEDUPE** |
| Se pudo fusionar delta `active` sin PPR `aprobado` del mismo árbol | **L-TREE** / **L-ABORT** |
| La aduana acredita presentación del PR, no el veredicto técnico pleno | **L-DOMAIN** |

## D2 — Laudos de estabilización (cerrados)

| ID | Decisión |
|----|----------|
| **L-DOMAIN** | Dominio = **aduana pre-fusión de `accept-pr`**: exigir veredicto PPR vinculante vía atestación `qa-attestation/1.0` coherente con el árbol que se fusiona. No es cambio de producto EventManager ni de alcance de MAN-43. |
| **L-ATTEST** | `accept-pr` **rechaza** la fusión si no existe atestación `qa-attestation/1.0` con `verdict: aprobado`. |
| **L-EMITTER** | La atestación debe haber sido **emitida por** el proceso `pull-request-review` (p. ej. vía `try_write_from_ppr` / cadena canónica), con `execution_id` de ese proceso. Escritura directa / JSON forjado fuera de PPR **no** habilita fusión. |
| **L-TREE** | El `tree_sha` de la atestación debe **coincidir** con el de la rama (árbol) que se fusiona. Divergencia ⇒ aborto. |
| **L-PROFILE** | Cuando el delta es `active`, se exige `qa_profile: full`. El perfil `docs-only` **no** habilita merge de delta activo. |
| **L-DEDUPE** | `skipped-ppr-dedupe` **no** cuenta como aprobación ni como paso de QA que habilite fusión. |
| **L-GENOMIC** | La puerta pre-fusión de `accept-pr` debe **leer** el veredicto Argos / atestación `qa-attestation/1.0` y validar coherencia; **no** basta comprobar solo el evento `PullRequest_Presented`. |
| **L-ABORT** | Sin las condiciones L-ATTEST + L-EMITTER + L-TREE (+ L-PROFILE si delta `active`), `accept-pr` devuelve **`abortado`** (no fusiona). |
| **L-COUNTERFACTUAL** | Con esta regla activa, el merge de MAN-43 bajo las condiciones de la auditoría KZ-2 **no** habría pasado la aduana; el contrafactual queda como métrica (KA-PPR-5), no como revert del merge histórico. |
| **L-NON-GOALS** | Fuera de alcance: KZ-1 / **MAN-44** (coherencia APTO/blocked / resoluciones); KZ-3 / **MAN-46** (hito ≠ Done del padre / tracker); KZ-4 (completar resto H1 y H2–H6 de MAN-43). No reabrir L-1…L-7 ni el *qué* de producto de MAN-43 salvo lo necesario para la aduana PPR. |
| **L-PROCESS** | Ciclo documental/runtime = **`feature`** / `branch_pr`; el origen PBI `bug-fix` / tipo `kaizen` queda como metadato de forja, no como proceso de fase. |
| **L-DOC-CANON** | Documento canónico = **PBI-75d12fd0**; refine `verdict: refined`. |
| **L-FORGE** | Ausencia o no verificación del JSON `forge-pbi` **no** bloquea Mayeuta; es gate de forja de código, no de estabilización. |
| **L-GIT** | Git solo vía `skill:git-manager` / `./sddia-run.sh --tool git-manager`. |
| **L-TRACKER** | Provider Linear: sin ficheros nuevos de backlog en `docs/todos/`; KM/TODOs solo vía `agent:cumulo` o evento `Kaizen_Alert_Required`. |
| **L-TRUTH** | No inventar éxito: fixtures/gates deben materializar el aborto; si no hay evidencia, el check permanece NO_APTO. |

## D3 — Criterios de aceptación del ciclo (producto)

| ID | Criterio |
|----|----------|
| **AC-1 (KA-PPR-1)** | Sin atestación `qa-attestation/1.0` con `verdict: aprobado` sobre el `tree_sha` que se fusiona, `accept-pr` devuelve **`abortado`** (no fusiona). |
| **AC-2 (KA-PPR-2)** | Fixture con delta `active` y solo atestación `docs-only` (o JSON `full` no emitido por `pull-request-review`) hace que `accept-pr` **aborte**. |
| **AC-3 (KA-PPR-3)** | Fixture cuyo `tree_sha` de atestación **diverge** del árbol de la rama hace que `accept-pr` **aborte**. |
| **AC-4 (KA-PPR-4)** | Fixture que solo acredita `skipped-ppr-dedupe` (sin PPR `aprobado`) **no** habilita fusión. |
| **AC-5 (KA-PPR-5)** | Contrafactual documentado: con esta regla activa, el merge de MAN-43 bajo las condiciones de la auditoría KZ-2 **no** habría pasado la aduana. |

## D4 — Preguntas abiertas → cerradas sin bloqueo

| # | Pregunta | Resolución |
|---|----------|------------|
| 1 | ¿Proceso `bug-fix` (PBI) vs `feature` (runtime)? | Documentar y ejecutar como **`feature`** (**L-PROCESS**). |
| 2 | ¿Incluir coherencia APTO↔execution/plan en este ítem? | **No** — eso es KZ-1 / MAN-44 (**L-NON-GOALS**). |
| 3 | ¿`docs-only` puede fusionar delta activo? | **No** — exige `qa_profile: full` (**L-PROFILE**). |
| 4 | ¿`skipped-ppr-dedupe` habilita merge? | **No** (**L-DEDUPE**). |
| 5 | ¿Basta `PullRequest_Presented`? | **No** — hay que leer atestación/veredicto (**L-GENOMIC**). |
| 6 | ¿Revertir el merge histórico de MAN-43? | **No** — contrafactual + aduana futura (**L-COUNTERFACTUAL**). |
| 7 | ¿Falta JSON forge-pbi? | No bloquea estabilización (**L-FORGE**). |

## D5 — Riesgos (vigilancia, no bloqueo Mayeuta)

| Riesgo | Tratamiento |
|--------|-------------|
| Operador escribe JSON `full` a mano para bypass | Solo atestaciones emitidas por `pull-request-review` (**L-EMITTER**). |
| Reutilizar atestación de otro `tree_sha` | Contraste obligatorio árbol de rama ↔ atestación (**L-TREE**). |
| Contar dedupe como QA verde | Dedupe explícitamente no-aprobación (**L-DEDUPE** / **AC-4**). |
| Solo cubrir evento Presented y olvidar veredicto | Puerta genómica lee atestación (**L-GENOMIC**). |
| Mezclar con gates de coherencia documental MAN-44 | Mantener fronteras de hermano (**L-NON-GOALS**). |

## D6 — Handoff Dedalo / Tekton

1. Consumir `clarify.md` + `objectives.md` en solo lectura; **no** reabrir L-ATTEST…L-DEDUPE.
2. Blueprint (`spec.md` / `plan.md`) acota touchpoints de aduana en `accept-pr` (y cadena de emisión PPR / `qa-attestation`) + fixtures KA-PPR-*.
3. Mayeuta **no** diseña implementación ni emite `delegates_to` de código.
4. Hermanos MAN-44 / MAN-46 / deuda MAN-43 permanecen fuera del *qué* de este `persist_ref`.

## D7 — Veredicto

**ok** — Requisitos estabilizados. El *qué* queda cerrado: `accept-pr` exige `qa-attestation/1.0` con `verdict: aprobado` emitido por `pull-request-review`, `tree_sha` igual a la rama, `qa_profile: full` cuando delta es `active`; `docs-only` y `skipped-ppr-dedupe` no habilitan fusión; aborto sin PPR vinculante; ACs **1…5** / métricas KA-PPR-1…5; fuera de alcance KZ-1 / KZ-3 / KZ-4. Tracker **MAN-45** / documento **PBI-75d12fd0** / padre **MAN-43** / `execution_id` **73910788-b7e7-4a61-9ac2-72fedd024b2a**.
