---
feature_name: kalma2-phase-barrier-timeout-persist
created: "2026-08-14"
purpose: Estabilización KALMA2-AUD-4b9de6 — barrera de fase, timeout terminal y rama refactor
branch_name: refactor/kalma2-phase-barrier-timeout-persist
persist_ref: docs/features/kalma2-phase-barrier-timeout-persist
pbi_ref: docs/todos/done/[REFACTOR] Kalma2 — serialización de fases, timeout y rama refactor (KALMA2-AUD-4b9de6).md
document_id: 1de0bdd1-6144-4e45-8efa-92db0f399147
phase: mayeuta-stabilization
agents: mayeuta
source_audit: "auditoría runtime Kalma2 WUI 2026-08-14 (Tekton)"
source_correlation_id: 4b9de6b3-c400-49c8-86f2-55f08ec64ce4
findings:
  - KALMA2-AUD-4b9de6-001
  - KALMA2-AUD-4b9de6-002
  - KALMA2-AUD-4b9de6-003
  - KALMA2-AUD-4b9de6-004
  - KALMA2-AUD-4b9de6-005
execution_id: d630a6cf-1767-4751-a2b9-b1f4210a01fb
---

# Clarificación — kalma2-phase-barrier-timeout-persist

Transcript Mayeuta. El PBI ya es semilla termodinámica; este artefacto fija laudos, no blueprint.

## D0 — Semilla

Ciclo Kalma2 `4b9de6b3-…` (`refactorization` sobre EV-AUD-002/007): acuse HTTP 202, TQM despachó, Mayeuta/Dedalo cerraron. Tekton se cortó a mitad de mutación (timeout CLI 600 s). Argos verificó snapshot incompleto (`global: NO_APTO`). El bus selló `tekton.task-queue-manager: failed`. WUI proyectó el fallo con fidelidad.

La calidad «parcial con error de cierre» es **deuda de runtime Kalma2/orquestador**, no del PBI Evolution.

Este ciclo **no** se orquesta vía Kalma2 (el runtime es el objeto). Ejecución = Tekton en IDE sobre rama `refactor/…` desde `main` limpio.

## D1 — Misión (qué)

Un despacho Kalma2 `refactorization` (y homólogos `feature`/`bug-fix`) **no avanza a Verificación ni cierre** si la fase agente previa no terminó `executed`. Timeout o kill de CLI es **terminal de fase** (`failed`), no *soft* `awaiting_agents`. Prefijo `refactor/` sobrevive a `workspace-init`. `persist_ref` llega al payload `AGENT_PHASE` y a `{persist_ref}/_agent_handoff.md`.

## D2 — Laudos

| Ref | Pregunta | Laudo |
|-----|----------|-------|
| **L-BARRIER** | ¿Qué se salta si la fase agente no es `executed`? | Verificación + Cierre documental + Cierre de entrega, **y** fases agente posteriores (serialización). `simulated` (sin runtime) **no** dispara barrera (lab/CI). Disparadores: `failed`, `blocked`, `awaiting_agents`, `awaiting`. |
| **L-TIMEOUT** | ¿Timeout soft o terminal? | **Terminal.** Quitar `"timeout"` de la allowlist soft. Soft solo config: CLI ausente, `not found`, `no instalado`, `401`, `auth`, `api_key`. |
| **L-TIMEOUT-ENV** | ¿Timeout único o por fase? | Default `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS=600`. Override Ejecución: `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS_EJECUCION`. **Prohibido** subir el default como único remedio. |
| **L-PREFIX** | ¿`refactor/` vs `feat/`? | Conservar prefijo de trabajo si ya está (`feat/` / `feature/` / `fix/` / `refactor/`). Default de `refactorization` = `refactor/{slug}`, no `feat/{slug}`. |
| **L-PERSIST** | ¿De dónde sale `persist_ref`? | TQM: `persist_ref_suggested` (o `persist_ref`) del FM del PBI; si falta, Cúmulo `paths.featurePath`/`fixPath` + slug. Runtime: no emitir `persist_ref` vacío si el hijo o `inputs` lo tienen. |
| **L-SCOPE** | ¿Mutar genoma de process? | **No.** Barrera en `executor` (runtime). `git-operations.md` (norma Core) solo vía `entity-manager` si se alinea el ejemplo de prefijos. |
| **L-EVO** | ¿Mezclar Evolution? | **Prohibido.** PBI `7bb37ff1-…` es ciclo aparte. |
| **L-FULL** | ¿`SDDIA_TQM_FULL_CYCLE`? | Fuera de alcance. Skip lab de cierre documental se extiende a `refactorization` (simetría `feature`/`bug-fix`). |

## D3 — Criterios observables

| ID | Criterio |
|----|----------|
| AC-TIMEOUT | Timeout de `cursor-agent` en Ejecución **no** lanza Argos en el mismo hijo. |
| AC-SKIP | `phase_reports` de Verificación `skipped` (o ausente) si Ejecución ∈ {`failed`, `awaiting_agents`}. |
| AC-BRANCH | PBI `suggested_branch: refactor/foo` → HEAD `refactor/foo` (no `feat/foo`). |
| AC-PERSIST | Payload `AGENT_PHASE` con `persist_ref` no vacío; `{persist_ref}/_agent_handoff.md` tras Mayeuta o Dedalo OK. |
| AC-TESTS | Unit: TQM `build_child_inputs` + `workspace-init` + normalización timeout en runtime Cursor. |
| AC-PR | PR único; `validacion.md` APTO; PBI archivado en la misma rama. |

## D4 — Fuera de alcance

- Completar EV-AUD-002/007.
- Activar full-cycle TQM / quitar `SDDIA_LAB_SKIP_*` por defecto.
- Mutar `refactorization.md` / `feature.md` salvo contrato de fase (entonces `entity-manager`).
- UX WUI (proyección dead-letter correcta).
