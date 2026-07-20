---
document_id: PBI-KALMA2-FULL-CYCLE-RUNTIME
uuid: 527007fa-7200-41ee-84bb-202737f4f983
title: "[FEATURE] kalma2-full-cycle — runtime de agentes y semántica de cierre"
format: markdown
version: "1.0.0"
created: "2026-07-20"
status: "done"
priority: alta
process: feature
feature_ref: docs/features/kalma2-full-cycle
validacion_ref: docs/features/kalma2-full-cycle/validacion.md
branch_name: feat/kalma2-full-cycle
archived_path: docs/todos/done/[FEATURE] kalma2-full-cycle — runtime de agentes y semántica de cierre (527007fa).md
execution_id_init: 956100c7-c03f-488b-af1e-2624f84bd0b0
evidence_event_id: e022814f-fc3a-441f-88c5-d60cb5e47e48
evidence_artifact: docs/fixes/7ad3175957d4/objectives.md
evidence_prompt_pbi: docs/todos/pending/[FIX] github-bridge-watcher — fractura sistémica (7ad3175957d4).md
depends_on:
  - docs/features/kalma2-process-dispatch
  - docs/features/kalma2-event-bus-integration
  - docs/features/kalma2-mayeuta-llm-router
related_pbi:
  - docs/todos/pending/[FIX] Validacion_interface_Kalma2.md
related:
  - SddIA/process/bug-fix.md
  - SddIA/engine/execute-process/src/engine/executor.rs
  - SddIA/engine/execute-process/src/engine/handlers/task_queue_manager.rs
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - interfaces/kalma2/app.js
  - docs/features/kalma2-process-dispatch/spec.md
inherited_debt:
  - L2-arranque-no-auto-PR
  - agent-phases-simulated-lab
  - R1-status-completed-semantico
progress:
  slice_A: forjado
  slice_B: forjado_hook_y_lab_wrapper
  slice_C: forjado
  argos: APTO
open_debt:
  - B2-process-agent-handoff-event-optional
---

# [FEATURE] kalma2-full-cycle — runtime de agentes y semántica de cierre

## Estado

**Done documental (Argos APTO)** en rama `feat/kalma2-full-cycle`. Slices A+B+C + wrapper producción Cursor CLI/SDK. Live requiere bóveda (`cursor-agent` o `CURSOR_API_KEY`).

## Síntoma (expectativa vs resultado)

| Expectativa del operador | Resultado observado |
|--------------------------|---------------------|
| Prompt Kalma2 «inicia fix sobre PBI adjunto» → ciclo `bug-fix` **gestionado por completo** (diseño → código → verificación → cierre) | Solo **arranque**: rama `fix/7ad3175957d4` + `docs/fixes/7ad3175957d4/objectives.md` |
| UI `completed` = fix resuelto | UI `completed` = PEC del orquestador lab tras init + fases `simulated` + skips L2 |
| PBI de fractura consumido y archivado | PBI sigue en `docs/todos/pending/`; sin `spec.md` / implementación / PR |

### Evidencia

| Campo | Valor |
|-------|--------|
| `event_id` | `e022814f-fc3a-441f-88c5-d60cb5e47e48` |
| Prompt | fix sobre `…/[FIX] github-bridge-watcher — fractura sistémica (7ad3175957d4).md` |
| Materialización | `objectives.md` únicamente |
| Status UI | `completed · e022814f…` + «PEC correlacionado» + `proceso=bug-fix status=success` |

## Diagnóstico estabilizado

No es fallo de emisión EDA ni alucinación de Mayeuta. La cadena Kalma2→TQM→hijo **cumple el laudo L2** y el perfil laboratorio de `bug-fix`.

### Causa 1 — Laudo L2 (`kalma2-process-dispatch`)

Con `correlation_id` (siempre desde Kalma2), TQM inyecta salvo `SDDIA_TQM_FULL_CYCLE=1`:

- `SDDIA_LAB_SKIP_PBI_ARCHIVE=1`
- `SDDIA_LAB_SKIP_DELIVERY_CLOSE=1`

Objetivo histórico: evitar dead-letter de cierre sin agentes IDE. Efecto: **arranque de ciclo, no auto-PR**.

### Causa 2 — Fases de agentes sin runtime físico

`bug-fix` v1.4.0: Diseño (`agent:dedalo`) → Ejecución (`agent:tekton`) → Verificación (`agent:argos`).

En `executor.rs`, delegados solo-`agent:` → `status: simulated` («agentes IDE; sin handler físico en laboratorio»).

El Sistema Nervioso despacha handlers nativos; **no despierta sesiones Dedalo/Tekton/Argos**. El PBI adjunto no se diseña ni forja.

### Causa 3 — Semántica UI (`completed`)

`project_status` prioriza PEC `success` → `completed`. El hijo emite PEC tras init + simulaciones → el operador interpreta cierre de negocio.

Relacionado: R1/R2 en `PBI-FIX-VALIDACION-INTERFACE-KALMA2`.

### Cadena observada

```text
POST /api/interact
  → Kalma2_Process_Requested
  → TQM → bug-fix
  → workspace-init (rama + objectives.md)     ← físico
  → Dedalo / Tekton / Argos                   ← simulated
  → archive / delivery-close                  ← skipped (L2)
  → PEC success → UI completed                ← engañoso
```

## Objetivo medible

Tras esta feature, un prompt Kalma2 de execute sobre un PBI en `docs/todos/pending/…`:

1. Arranca el ciclo (rama + `persist_ref`) — ya existe.
2. **Ejecuta** las fases de agentes (o handoff auditable equivalente) consumiendo el cuerpo del PBI.
3. Produce cascada mínima `bug-fix` (`spec` → `implementation`/`execution` → `validacion`).
4. Cierra documentalmente (PBI → `done/`, `pbi_archived: true`) y entrega (PR) cuando el runtime lo permita.
5. La UI **no** muestra `completed` de negocio mientras el ciclo esté en init/simulated/awaiting_agents.

## Rebanadas propuestas (priorización)

| Slice | Nombre | Qué entrega | Riesgo si se omite |
|-------|--------|-------------|-------------------|
| **A** | Semántica de status | Estados distintos: `initialized` / `awaiting_agents` / `completed` / `failed`; copy de acuse honesto; PEC lab ≠ cierre de negocio | Operador sigue creyendo el fix cerrado |
| **B** | Runtime de agentes post-init | Invocación Dedalo→Tekton→Argos (CLI Cursor / Agent SDK / cola IDE) tras `workspace-init`, con correlación al `event_id` | Sin B no hay gestión completa; solo honestidad |
| **C** | Consumo del PBI adjunto | Cargar cuerpo de `pbi_ref` en inputs hijo (`pbi_body` / handoff); `objectives.md` = misión del PBI, no eco del prompt | Agentes trabajan sobre semilla pobre |

**Orden recomendado:** A → B → C (C puede ir en el mismo PR que B).

### Nota crítica sobre `SDDIA_TQM_FULL_CYCLE`

Activar solo el flag **no** cumple el objetivo: habilita archive/delivery sobre un fix vacío (fases aún `simulated`). Requiere B antes o junto a derogar L2 en path Kalma2.

## Hipótesis tácticas (handoff Dedalo — no laudo)

| Vía | Descripción | Nota |
|-----|-------------|------|
| **A1** | Extender `project_status` + envelope hijo con `cycle_phase` / `agent_runtime` | Bajo coste; desbloquea UX |
| **B1** | Bridge de agentes vía `SDDIA_LLM_CLI_COMMAND` / Cursor Agent SDK por fase | Alineado a C3 del router; acopla a CLI local |
| **B2** | Evento de dominio `Process_Agent_Handoff_Requested` + daemon/cola IDE | Más EDA-puro; mayor superficie |
| **B3** | Derogar simulación silenciosa: fase `agent:` sin runtime → `failed` o `awaiting`, nunca PEC success de negocio | Compatibilidad lab vs producción |
| **C1** | En TQM/`workspace_init`, leer `pbi_ref` del FS y enriquecer `bug_summary`/`objectives` | Complemento de B |

## Fuera de alcance (salvo laudo Racso)

| Ítem | Motivo |
|------|--------|
| Re-forjar emisión `Kalma2_Process_Requested` / bridge write EDA | Ya APTO; no es la brecha |
| Culpar a Mayeuta / restaurar `sddia-client-bridge.py` | Diagnóstico fósil descartado |
| Remediación de `github-bridge-watcher` / otros fractura-PBI | PBIs de dominio aparte; este cierra el **mecanismo** de ciclo completo |
| IOTA / DLT del mismo evento | Canal paralelo (deuda aparte) |
| Autenticación Cerbero/Karma2 en Kalma2 | PoC; no bloquea el runtime de agentes |

## Criterios de aceptación (borrador Argos)

| ID | Criterio |
|----|----------|
| AC-A1 | Tras solo `workspace-init`, UI/status ≠ `completed` de negocio (p.ej. `initialized` o `awaiting_agents`) |
| AC-A2 | Mensaje operador distingue «ciclo arrancado» vs «bug-fix cerrado» |
| AC-B1 | Con runtime configurado, fases Dedalo/Tekton/Argos dejan de ser `simulated` silenciosas en path Kalma2 |
| AC-B2 | Aparecen artefactos mínimos bajo `persist_ref` (`spec.md` + implementación o equivalente auditable) |
| AC-B3 | PEC de `completed` de negocio exige fases agente ejecutadas o handoff explícito resuelto |
| AC-C1 | Prompt con path a PBI → hijo recibe contenido del PBI (no solo path en el texto) |
| AC-C2 | Al cierre completo: PBI en `docs/todos/done/` + `validacion.md` con `pbi_archived: true` (perfil no-lab o full-cycle) |

## Relación con otros PBI

| Documento | Relación |
|-----------|----------|
| `[FIX] Validacion_interface_Kalma2` | Auditoría del lazo UI↔EDA; R1/R2 apuntan aquí |
| `kalma2-process-dispatch` (done) | Entregó despacho TQM + L2; esta feature **evoluciona** L2 |
| Fracturas `*-watcher` / `event-sweeper` | Consumidores del mecanismo; no sustituyen este PBI |

## Mandato

Estabilizar y forjar (cuando se priorice) la feature `kalma2-full-cycle` vía proceso `feature`: cerrar el hueco entre **arranque EDA** y **gestión completa del ciclo de vida** solicitado desde Kalma2, con semántica de status honesta.

Prohibido: declarar APTO solo con `objectives.md` + UI `completed`.
