---



feature_name: process-stop-after-and-execution-registry
created: "2026-11-21"
updated: "2026-11-21"
process: feature
purpose: Estabilización de requisitos — stop_after contractual por phase_id y registro incremental con resume
branch_name: feat/process-stop-after-and-execution-registry
persist_ref: docs/features/process-stop-after-and-execution-registry
pbi_ref: MAN-34
document_id: PBI-PROCESS-STOP-AFTER-AND-EXECUTION-REGISTRY
pbi_uuid: "7f4e9a12-3c8b-4d5e-9a21-6b8c4e109f32"
phase: mayeuta-stabilization
mayeuta_verdict: ok
norm_ref: features-documentation-pattern@1.3.2
agents: mayeuta
supersedes:
  - INFORME-SDDIA-TQM-STOP-AFTER
related:
  - docs/features/kalma2-agent-verdict-barrier/
  - SddIA/process/process-contract.md
execution_id: "1e7ca2f5-f505-46a6-a0be-c7a82d0b58a6"
---
# Clarificación — process-stop-after-and-execution-registry

Transcript Mayeuta (2026-11-21). Fase **Estabilización de Requisitos** del proceso `feature`.
Semilla **PBI-PROCESS-STOP-AFTER-AND-EXECUTION-REGISTRY** (`uuid` `7f4e9a12-3c8b-4d5e-9a21-6b8c4e109f32`). Tracker **MAN-34**. Antecedente preservado: barrera `reason: stop_after` / `cycle_phase: awaiting_agents` de `kalma2-agent-verdict-barrier` (L-HALT / L-PEC).

## D0 — Semilla y evidencia en fuente

| Hecho | Laudo |
|-------|-------|
| Tracker | **MAN-34** (provider Linear / canal auditoría `docs/audits/MAN-34.md`). Sin copia nueva bajo `docs/todos/` (**L-TRACKER**). |
| Documento | **PBI-PROCESS-STOP-AFTER-AND-EXECUTION-REGISTRY**; tipo kaizen; `refined_by: mayeuta`. |
| Parada legacy verificada | `executor.rs`: `stop_after_design()` lee `SDDIA_TQM_STOP_AFTER`; `is_dedalo_design_phase()` compara literales ES (`Diseño del fix` / `Diseño de Blueprint` / `Diseño de refactor`). Barrera con `reason: stop_after` tras `executed`. |
| Propagación TQM | `task_queue_manager.rs`: `child_env_for_kalma2(..., stop_after)` inyecta `SDDIA_TQM_STOP_AFTER` si el padre no lo tiene; input `stop_after` tratado como **string** único. |
| Termodinámica | `thermodynamic.rs`: `reason == "stop_after"` ⇒ `cycle_phase: awaiting_agents` (preservar; no redefinir). |
| `phase_reports.json` | Escrito al **cierre** del bucle en `run_generic` (`persist_phase_reports_json`); no hay write-ahead pre/post fase ni índice `active-executions.json`. |
| Fases sin `id` | `feature.md` (y hermanos delivery) declaran `phases[].name` humano; **sin** `id` snake_case. `process-contract.md` v1.4.0 no exige `id` aún. |
| `Work_Initiated` hoy | `workspace_init` puede emitir en Fase 1; `maybe_emit_work_initiated_on_tekton_entry` solo propaga sello previo. Semilla exige emisión **solo** al entrar en primera fase efectiva `agent:tekton` (**L-WI**). |
| Informe legacy | `INFORME-SDDIA-TQM-STOP-AFTER` queda **superseded** (**L-SUPERSEDE**). |
| `execution_id` | **af40f4e7-f831-49fa-8112-6ddb0ff2d535** |
| Modo | `feature` / `branch_pr`. |

## D1 — Matriz fricción × alcance estabilizado

| Afirmación (semilla) | Laudo |
|----------------------|-------|
| Parada acoplada a env + nombres ES de fase Dedalo | **L-PHASE-ID** / **L-STOP-LIST** / **L-ERADICATE** |
| Binario conoce catálogo concreto (`is_dedalo_design_phase`) | **L-UNIVERSAL** |
| Contaminación padre→hijo vía env `SDDIA_TQM_STOP_AFTER` | **L-NO-ENV** |
| Sin write-ahead; caída pierde trazabilidad | **L-WRITEAHEAD** / **L-RUNTIME-LOCUS** |
| Sin `resume` / índice de ejecuciones activas | **L-RESUME** / **L-INDEX** |
| Halt design no debe emitir `Work_Initiated` / mover a In Progress de trabajo efectivo | **L-WI** |
| Mutar genoma `process-contract` / procesos vía forja directa | **L-GENOME** (entity-manager / DA-2) |

## D2 — Laudos de estabilización (cerrados)

| ID | Decisión |
|----|----------|
| **L-PHASE-ID** | Toda fase en `phases[]` del frontmatter de proceso declara `id` (slug `snake_case` **inmutable**). El motor evalúa `stop_after` y `resume` contra `id`, nunca contra `name` humano. |
| **L-STOP-LIST** | Input opcional universal `stop_after: list<string>` (default `[]`). Tras completar una fase cuyo `id` ∈ lista con `status: executed`, el orquestador marca posteriores `skipped` (`reason: stop_after`) y deja `cycle_phase: awaiting_agents`. Lista puede contener **varios** `phase_id` (parada múltiple / reentrada). |
| **L-LEGACY-DESER** | Compatibilidad de deserialización: string único legacy (p. ej. `"design"`) → `["design"]`. El valor semántico post-migración es el **`id`** canónico (`design`, `execution`, …), no el nombre ES. |
| **L-UNIVERSAL** | Parada aplicable a **cualquier** proceso del catálogo con `id`s; prohibido hardcodear nombres Dedalo/delivery en el binario. |
| **L-NO-ENV** | Erradicar `SDDIA_TQM_STOP_AFTER` y la inyección en `child_env_for_kalma2`. El contrato viaja por **inputs** del proceso / envelope, no por entorno. |
| **L-ERADICATE** | Cero referencias a `SDDIA_TQM_STOP_AFTER`, `stop_after_design`, `is_dedalo_design_phase` (literales ES) en fuente de `execute-process`. |
| **L-RESUME** | Inputs `resume: bool` + `execution_id` (o resolución por índice). Fases `executed` no se re-ejecutan (verificar artefactos obligatorios cuando aplique); `in_progress` / `failed` / `blocked` se relanzan; barrera `stop_after` **ya consumida** no bloquea el avance a la siguiente fase en reanudación. |
| **L-WRITEAHEAD** | Persistencia atómica de `{workspace_path}/phase_reports.json` en init / pre-fase (`in_progress`) / post-fase (`executed`\|`blocked`\|`failed`\|`skipped`) / barrera-cierre. |
| **L-INDEX** | Índice `.SddIA/state/active-executions.json` actualizado en los mismos hitos. |
| **L-RUNTIME-LOCUS** | Prohibido escribir JSON de estado de runtime bajo `persist_ref` (`docs/`). Runtime solo bajo `workspace_path` y `.SddIA/state/`. |
| **L-THERMO** | Preservar semántica: `reason: stop_after` ⇒ `awaiting_agents`; skipped por stop_after es neutro para terminalidad de éxito del ciclo (como L-PEC del antecedente). **No** rediseñar peaje termodinámico más allá de eso. |
| **L-WI** | En `feature` / `bug-fix` / `refactorization`, `Work_Initiated` se emite **solo** al entrar en la primera fase efectiva `agent:tekton` (`execution`). Parada en `design` deja el ítem en Todo (sin Work_Initiated). Esto **ajusta** el texto actual de `feature.md` § ignición AUT-65 (Fase 1) — el cambio de genoma va con **L-GENOME**. `lifecycle_phase_action` / forge_started queda para Dedalo+Tekton según blueprint; Mayeuta no fija columnas Linear más allá del invariante de emisión. |
| **L-GENOME** | Cambios a `process-contract.md` y `SddIA/process/*.md` / domain roots **solo** vía `entity-manager` / proceso autorizado (DA-2). |
| **L-SUPERSEDE** | `INFORME-SDDIA-TQM-STOP-AFTER` superseded por este `persist_ref`. |
| **L-ANTECEDENT** | Se preserva el *qué* de barrera del PBI `kalma2-agent-verdict-barrier` (reason/cycle_phase); se **erradica** el *cómo* env+literales ES. |
| **L-PROCESS** | Ciclo documental/runtime = **`feature`**; tipo semilla kaizen es metadato de forja. |
| **L-DOC-CANON** | Norma documental: `features-documentation-pattern` v1.3.2 — un `.md` por fase + frontmatter. |
| **L-GIT** | Git solo vía `skill:git-manager` / `./sddia-run.sh --tool git-manager`. |
| **L-TRACKER** | Provider Linear: sin ficheros nuevos de backlog en `docs/todos/`; KM/TODOs solo `agent:cumulo` o `Kaizen_Alert_Required`. |
| **L-TRUTH** | No inventar éxito: gates KA-STOP-* requieren materialización; sin evidencia → NO_APTO. |
| **L-NON-GOALS** | Fuera: rediseño de agentes; cambio de semántica termodinámica distinto de preservar `awaiting_agents`+`stop_after`; contaminar expedientes documentales con estado runtime; NLP de nombres de fase. |

## D3 — Criterios de aceptación (métricas semilla)

| ID | Criterio |
|----|----------|
| **AC-1 (KA-STOP-1)** | `process-contract.md` exige `id` por fase y define `stop_after: list<string>` y `resume: bool` en inputs base; todos los procesos del catálogo tienen `id` y pasan `verify-process-integrity`. |
| **AC-2 (KA-STOP-2)** | Cero referencias a `SDDIA_TQM_STOP_AFTER` e `is_dedalo_design_phase` (literales ES) en fuente de `execute-process`. |
| **AC-3 (KA-STOP-3)** | Invocar cualquier proceso con `stop_after: ["<phase_id_1>", …]` detiene tras completar `<phase_id_1>` (`executed`), marca posteriores `skipped` (`reason: stop_after`) y devuelve `cycle_phase: awaiting_agents`. |
| **AC-4 (KA-STOP-4)** | Durante ejecución, `phase_reports.json` se crea al init y se actualiza en disco pre (`in_progress`) y post (`executed`/`blocked`/`failed`) de cada fase; se actualiza `active-executions.json`; ningún `.json` de estado runtime bajo `persist_ref`. |
| **AC-5 (KA-STOP-5)** | Tras `stop_after: ["design"]` en `feature`/`bug-fix`/`refactorization`, continuación sobre el mismo `persist_ref` o `execution_id` salta fases `executed`, verifica artefactos de diseño cuando apliquen, y arranca en `execution`. |
| **AC-6 (KA-STOP-6)** | Con fases 1–2 `executed` y fase 3 `in_progress`/`failed`, el resume re-ejecuta desde la fase 3 sin rehacer 1–2. |
| **AC-7 (KA-STOP-7)** | Parada en `design` no emite `Work_Initiated`; al entrar en `execution` (`agent:tekton`) se emite una sola vez. |
| **AC-8 (KA-STOP-8)** | `cargo build --release` y `cargo test` en verde, con tests de parada múltiple, deserialización legacy string→lista y resume. |

## D4 — Preguntas abiertas → cerradas sin bloqueo

| # | Pregunta | Resolución |
|---|----------|------------|
| 1 | ¿Parar por `name` ES o por `id`? | Solo por **`id`** (**L-PHASE-ID**). |
| 2 | ¿Mantener env `SDDIA_TQM_STOP_AFTER` como alias? | **No** — erradicar (**L-NO-ENV** / **L-ERADICATE**); solo deser string→lista en inputs. |
| 3 | ¿`stop_after` string o lista? | Lista; string legacy se normaliza a lista de un elemento (**L-LEGACY-DESER**). |
| 4 | ¿Dónde vive el registro? | `workspace_path/phase_reports.json` + `.SddIA/state/active-executions.json`; nunca bajo `docs/` (**L-RUNTIME-LOCUS**). |
| 5 | ¿Resume re-ejecuta `executed`? | **No**; sí relanza `in_progress`/`failed`/`blocked` (**L-RESUME**). |
| 6 | ¿`Work_Initiated` en Fase 1 o Tekton? | Solo Tekton (`execution`) (**L-WI**); genoma delivery se actualiza vía entity-manager. |
| 7 | ¿Mutar process YAML a mano? | **No** — entity-manager / DA-2 (**L-GENOME**). |
| 8 | ¿Cambiar derive_cycle_phase? | Solo preservar `stop_after`→`awaiting_agents` (**L-THERMO**). |

## D5 — Riesgos (vigilancia, no bloqueo Mayeuta)

| Riesgo | Tratamiento |
|--------|-------------|
| Catálogo sin `id` tras contrato | Gate `verify-process-integrity` + AC-1. |
| Operador reintroduce env | Tests KA-STOP-2; ban en code review / suite. |
| Resume sin verificación de artefactos de diseño | AC-5 exige verificación cuando aplique. |
| Escribir estado en `persist_ref` | **L-RUNTIME-LOCUS** + AC-4. |
| Doble `Work_Initiated` (init + tekton) | Emisión única en entry Tekton (**L-WI** / AC-7). |
| Contaminar semántica PEC | **L-THERMO** / **L-ANTECEDENT**. |

## D6 — Handoff Dedalo / Tekton

1. Consumir `clarify.md` + `objectives.md` en solo lectura; **no** reabrir L-PHASE-ID…L-WI.
2. Blueprint (`spec.md` / `plan.md`) acota: contrato `process-contract`, migración `id` en catálogo delivery+resto, motor `executor`/`TQM`/`workspace_init`, índice active-executions, tests KA-STOP-*.
3. Mayeuta **no** diseña implementación ni emite `delegates_to` de código.
4. Genoma vía `entity-manager` (**L-GENOME**).
5. Antecedente `kalma2-agent-verdict-barrier` es contexto; este ítem **sustituye** el mecanismo env/literales.

## D7 — Veredicto

**ok** — Requisitos estabilizados. El *qué* queda cerrado: `phase_id` canónico; `stop_after: list<string>` contractual; resume/`execution_id` con write-ahead bajo `.SddIA/`+workspace; erradicación env+literales ES; `Work_Initiated` solo al entrar en `execution` (tekton); ACs **KA-STOP-1…8**; fuera de alcance rediseño de agentes y contaminación documental con estado runtime. Tracker **MAN-34** / documento **PBI-PROCESS-STOP-AFTER-AND-EXECUTION-REGISTRY** / `execution_id` **af40f4e7-f831-49fa-8112-6ddb0ff2d535**.
