---
document_id: 1de0bdd1-6144-4e45-8efa-92db0f399147
title: Kalma2 — serialización de fases, timeout y rama refactor
type: refactorization
status: done
priority: high
created: "2026-08-14"
suggested_branch: refactor/kalma2-phase-barrier-timeout-persist
source_audit: "auditoría runtime Kalma2 WUI 2026-08-14 (Tekton)"
source_correlation_id: 4b9de6b3-c400-49c8-86f2-55f08ec64ce4
source_pbi: docs/todos/pending/[REFACTOR] Evolution — migrar históricos y extraer borradores (EV-AUD-002-007).md
persist_ref_suggested: docs/features/kalma2-phase-barrier-timeout-persist
findings:
  - KALMA2-AUD-4b9de6-001
  - KALMA2-AUD-4b9de6-002
  - KALMA2-AUD-4b9de6-003
  - KALMA2-AUD-4b9de6-004
  - KALMA2-AUD-4b9de6-005
related_wip:
  - SddIA/scripts/tools/kalma2-agent-runtime-cursor.py
  - SddIA/engine/execute-process/src/engine/executor.rs
  - SddIA/engine/execute-process/src/engine/phase_terminal.rs
  - SddIA/engine/execute-process/src/engine/workspace_init.rs
  - SddIA/engine/execute-process/src/engine/handlers/task_queue_manager.rs
  - SddIA/engine/execute-process/src/engine/agent_runtime.rs
  - SddIA/norms/git-operations.md
---

# Kalma2 — serialización de fases, timeout y rama refactor

## Problema

Ciclo Kalma2 `4b9de6b3-…` (`refactorization` sobre EV-AUD-002/007): acuse HTTP **correcto** (`202`), despacho TQM **ocurrió**, Mayeuta/Dedalo **cerraron**. Ejecución Tekton se **cortó a mitad de mutación**; Argos verificó un snapshot incompleto (`global: NO_APTO`); el bus selló `tekton.task-queue-manager: failed` → dead-letter. WUI proyectó el fallo con fidelidad. La calidad «parcial con error de cierre» es **deuda de runtime Kalma2/orquestador**, no del PBI Evolution.

Causas medidas:

| ID | Hallazgo | Evidencia |
|----|----------|-----------|
| **001** | Timeout CLI 600 s mata `cursor-agent`; se trata como *soft* → `awaiting_agents` | Tekton 11:07 → Argos 11:17; `TimeoutExpired` → `"timeout"` en allowlist soft |
| **002** | Executor recorre **todas** las fases; `awaiting_agents` es **neutral** en `phase_terminal` | Verificación arranca sin Ejecución `executed` |
| **003** | `workspace-init` reescribe `refactor/` → `feat/` | Worktree `feat/evolution-history-normalization` ≠ L-BRANCH |
| **004** | TQM no inyecta `persist_ref` al hijo | Prompt agente `persist_ref:` vacío; `_agent_handoff.md` ausente |
| **005** | Blueprint T0–T8 no cabe en 600 s; primer `StrReplace` a medias | `mod migrate_evolution_history` sin `*.rs`; sin `implementation.md` |

## Objetivo

Hacer que un despacho Kalma2 `refactorization` (y homólogos `feature`/`bug-fix`) **no avance a Verificación** si Ejecución no terminó `executed`; que timeout de agente sea **terminal de fase** (no soft); que `suggested_branch` `refactor/…` sobreviva a `workspace-init`; que `persist_ref` llegue al payload de agente y al handoff máquina.

## Alcance

1. **Barrera de fase (orquestador):** no ejecutar Verificación / cierre si la fase agente previa no es `executed`. Timeout o kill de CLI → `failed` (no `awaiting_agents`), salvo allowlist explícita de *config* (CLI ausente, 401, `not found`) **sin** incluir `"timeout"`.
2. **Timeout de Ejecución:** `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS` diferenciable por fase (Ejecución ≥ presupuesto de forja) **o** política única: timeout = `failed` + no disparar Argos. Documentar default y override.
3. **Rama `refactor/`:** `workspace-init` conserva prefijo `refactor/` (simetría `feat/` / `fix/`). Alinear `git-operations.md` si la norma aún omite el prefijo.
4. **`persist_ref` end-to-end:** TQM resuelve y pasa `persist_ref` al hijo (Cúmulo `paths.featurePath` / `fixPath` según proceso+rama). Runtime Cursor escribe `_agent_handoff.md` bajo ese path. Prompt HARD OVERRIDE no envía `persist_ref` vacío si el hijo lo tiene.
5. **Tests:** timeout → fase `failed` y proceso hijo `success=false` **sin** reporte de Verificación `executed`; `refactor/x` no se reescribe a `feat/x`; hijo TQM tiene `persist_ref` no vacío.

## Fuera de alcance

- Completar EV-AUD-002/007 (`7bb37ff1-…`); retoma de ese PBI es ciclo aparte.
- Activar `SDDIA_TQM_FULL_CYCLE` / quitar `SDDIA_LAB_SKIP_*` por defecto (cierre documental sigue gated a Argos APTO).
- Mutar genoma de procesos `refactorization.md` / `feature.md` salvo que la barrera exija contrato de fase (entonces vía `entity-manager`).
- UX WUI: el mensaje dead-letter fue proyección correcta.

## Criterios de aceptación

- Timeout de `cursor-agent` en fase Ejecución **no** lanza Argos en el mismo hijo.
- `phase_reports` de Verificación ausente o `skipped` si Ejecución ∈ {`failed`, `awaiting_agents` por timeout}.
- PBI con `suggested_branch: refactor/foo` → worktree/HEAD `refactor/foo` (no `feat/foo`).
- Payload `AGENT_PHASE` incluye `persist_ref` no vacío; existe `{persist_ref}/_agent_handoff.md` tras Mayeuta o Dedalo OK.
- Unit/smoke: TQM `build_child_inputs` + `workspace-init` + normalización timeout en `kalma2-agent-runtime-cursor.py`.
- PR único; `validacion.md` APTO; PBI archivado en la misma rama.

## Restricción

Prohibido subir el timeout como único remedio sin barrera de fase. Prohibido mezclar este PR con la migración Evolution.
