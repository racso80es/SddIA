---
feature_name: process-stop-after-and-execution-registry
created: "2026-11-21"
updated: "2026-11-21"
process: feature
purpose: Objetivos estabilizados — stop_after contractual por phase_id y registro incremental con resume
branch_name: feat/process-stop-after-and-execution-registry
persist_ref: docs/features/process-stop-after-and-execution-registry
pbi_ref: MAN-34
document_id: PBI-PROCESS-STOP-AFTER-AND-EXECUTION-REGISTRY
pbi_uuid: "7f4e9a12-3c8b-4d5e-9a21-6b8c4e109f32"
status: requisitos-estabilizados
mayeuta_verdict: ok
phase: mayeuta-stabilization
norm_ref: features-documentation-pattern@1.3.2
agents: mayeuta
execution_id: "af40f4e7-f831-49fa-8112-6ddb0ff2d535"
supersedes:
  - INFORME-SDDIA-TQM-STOP-AFTER
---

# Objetivos — process-stop-after-and-execution-registry

## Misión

Hacer contractual y universal la parada de proceso tras fase(s) canónicas (`stop_after` por `phase_id`), con registro incremental write-ahead y reanudación determinista (`resume` / `execution_id`), erradicando el acoplamiento táctico por env `SDDIA_TQM_STOP_AFTER` y literales ES de fases Dedalo.

Tracker **MAN-34**. Documento **PBI-PROCESS-STOP-AFTER-AND-EXECUTION-REGISTRY** (`uuid` `7f4e9a12-3c8b-4d5e-9a21-6b8c4e109f32`).

**Por qué:** el motor detiene post-diseño con heurísticas de nombre humano y contaminación de entorno; no hay write-ahead ni resume, así que no se puede conmutar LLM (diseño caro → ejecución barata) ni recuperar progreso tras fallo sin rehacer fases `executed`.

## Contexto estabilizado

| Hecho | Impacto |
|-------|---------|
| Tracker | **MAN-34** |
| Clarificaciones | `clarify.md` (laudos L-PHASE-ID…L-WI; veredicto **ok**) |
| Antecedente | `docs/features/kalma2-agent-verdict-barrier/` — preservar `reason: stop_after` → `awaiting_agents` |
| Evidencia legacy | `executor.rs` (`stop_after_design` / `is_dedalo_design_phase`); `task_queue_manager.rs` (`child_env_for_kalma2` + env) |
| Catálogo hoy | `feature.md` sin `phases[].id`; contrato v1.4.0 sin `stop_after` lista |
| `execution_id` | `af40f4e7-f831-49fa-8112-6ddb0ff2d535` |

## Alcance

| ID | Objetivo |
|----|----------|
| **C1** | Exigir `id` snake_case inmutable por fase en contrato y catálogo. |
| **C2** | Definir `stop_after: list<string>` (default `[]`) y `resume: bool` en inputs base del contrato. |
| **C3** | Parada universal por `phase_id` con skip posteriores `reason: stop_after` y `cycle_phase: awaiting_agents`. |
| **C4** | Write-ahead de `phase_reports.json` + índice `active-executions.json` fuera de `persist_ref`. |
| **C5** | Resume determinista (skip `executed`; relanzar incompletas; barrera consumida no bloquea). |
| **C6** | Erradicar env y literales ES; deser legacy string→lista. |
| **C7** | `Work_Initiated` solo al entrar en primera fase efectiva `agent:tekton`. |

## Objetivos medibles

| ID | Objetivo | Criterio de cierre |
|----|----------|-------------------|
| **O1** | Contrato + catálogo con `id` | KA-STOP-1 / `verify-process-integrity` verde |
| **O2** | Cero legacy env/literales | KA-STOP-2 (grep fuente limpio) |
| **O3** | Parada universal | Fixture KA-STOP-3: stop tras `phase_id` → skipped + `awaiting_agents` |
| **O4** | Write-ahead + locus | KA-STOP-4: init/pre/post en disco; sin JSON runtime bajo `docs/` |
| **O5** | Cambio de LLM / continuación | KA-STOP-5: tras stop `design`, resume arranca en `execution` |
| **O6** | Recuperación mid-phase | KA-STOP-6: resume desde fase 3 sin rehacer 1–2 |
| **O7** | Invariante Work_Initiated | KA-STOP-7: no emisión en stop design; una vez en tekton |
| **O8** | Suite | KA-STOP-8: `cargo build --release` + `cargo test` + tests lista/legacy/resume |

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| **AC-1 (KA-STOP-1)** | `process-contract.md` exige `id` por fase y define `stop_after: list<string>` y `resume: bool`; catálogo con `id` pasa `verify-process-integrity`. |
| **AC-2 (KA-STOP-2)** | Cero referencias a `SDDIA_TQM_STOP_AFTER` e `is_dedalo_design_phase` (literales ES) en `execute-process`. |
| **AC-3 (KA-STOP-3)** | `stop_after: ["<phase_id_1>", …]` detiene tras `<phase_id_1>` `executed`; posteriores `skipped` (`reason: stop_after`); `cycle_phase: awaiting_agents`. |
| **AC-4 (KA-STOP-4)** | `phase_reports.json` init + pre (`in_progress`) + post; `active-executions.json` actualizado; sin estado runtime bajo `persist_ref`. |
| **AC-5 (KA-STOP-5)** | Tras `stop_after: ["design"]` en delivery, continuación salta `executed`, verifica artefactos de diseño si aplica, arranca en `execution`. |
| **AC-6 (KA-STOP-6)** | Resume con 1–2 `executed` y 3 `in_progress`/`failed` re-ejecuta desde 3. |
| **AC-7 (KA-STOP-7)** | Stop en `design` sin `Work_Initiated`; entrada a `execution` (`agent:tekton`) emite una sola vez. |
| **AC-8 (KA-STOP-8)** | `cargo build --release` y `cargo test` verdes; tests parada múltiple, deser legacy, resume. |

## Invariantes

- Unidad de parada/resume = `phases[].id` (`snake_case`), no `name`.
- `stop_after` es lista de inputs (no env); string legacy → lista.
- `reason: stop_after` ⇒ `cycle_phase: awaiting_agents` (preservar).
- Estado runtime solo en `workspace_path` y `.SddIA/state/` — nunca en `persist_ref`.
- Fases `executed` no se re-ejecutan en resume; incompletas sí.
- Barrera `stop_after` consumida no bloquea reanudación a la siguiente fase.
- `Work_Initiated` (delivery) solo en primera entrada efectiva `agent:tekton`.
- Genoma process vía `entity-manager` (DA-2).
- Git solo vía `skill:git-manager`.
- KM / `docs/todos/`: solo `agent:cumulo` o `Kaizen_Alert_Required`.

## Fuera de alcance

- Rediseño de agentes (Mayeuta/Dedalo/Tekton/Argos).
- Cambio de semántica termodinámica distinto de preservar `awaiting_agents` + `reason: stop_after`.
- Contaminar expedientes documentales (`docs/`) con JSON de estado runtime.
- NLP / heurísticas sobre nombres humanos de fase.
- Semillas Kaizen bajo `docs/todos/` desde Tekton/Argos/Mayeuta.

## Dependencias

- Contrato: `SddIA/process/process-contract.md`.
- Motor: `execute-process` (`executor.rs`, `handlers/task_queue_manager.rs`, `thermodynamic.rs`, `workspace_init.rs`).
- Catálogo delivery: `feature` / `bug-fix` / `refactorization` bajo domain roots.
- Antecedente barrera: `docs/features/kalma2-agent-verdict-barrier/`.
- Norma documental: `features-documentation-pattern` v1.3.2.
- Clarificaciones canónicas: `clarify.md` (laudos L-*).
- Gobernanza: `entity-manager` / DA-2 para mutación genómica.

## Ley aplicada

- Norma `features-documentation-pattern` v1.3.2 (un `.md` por fase + frontmatter).
- Proceso `feature`; Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
- Clarificaciones canónicas: `clarify.md` (laudos L-*).
