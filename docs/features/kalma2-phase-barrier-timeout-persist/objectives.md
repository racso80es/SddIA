---
feature_name: kalma2-phase-barrier-timeout-persist
created: "2026-08-14"
process: refactorization
branch_name: refactor/kalma2-phase-barrier-timeout-persist
persist_ref: docs/features/kalma2-phase-barrier-timeout-persist
pbi_ref: docs/todos/done/[REFACTOR] Kalma2 — serialización de fases, timeout y rama refactor (KALMA2-AUD-4b9de6).md
document_id: 1de0bdd1-6144-4e45-8efa-92db0f399147
source_audit: "auditoría runtime Kalma2 WUI 2026-08-14 (Tekton)"
source_correlation_id: 4b9de6b3-c400-49c8-86f2-55f08ec64ce4
findings:
  - KALMA2-AUD-4b9de6-001
  - KALMA2-AUD-4b9de6-002
  - KALMA2-AUD-4b9de6-003
  - KALMA2-AUD-4b9de6-004
  - KALMA2-AUD-4b9de6-005
phase: blueprint
execution_id: d630a6cf-1767-4751-a2b9-b1f4210a01fb
---

# Objetivos — kalma2-phase-barrier-timeout-persist

## Objetivo

Hacer que un despacho Kalma2 `refactorization` (y homólogos `feature`/`bug-fix`) **no avance a Verificación** si Ejecución no terminó `executed`; que timeout de agente sea **terminal de fase** (no soft); que `suggested_branch` `refactor/…` sobreviva a `workspace-init`; que `persist_ref` llegue al payload de agente y al handoff máquina.

## Alcance

1. **Barrera de fase (orquestador):** no ejecutar Verificación / cierre si la fase agente previa ∈ {`failed`, `blocked`, `awaiting_agents`, `awaiting`}. `simulated` no dispara barrera.
2. **Timeout de Ejecución:** `"timeout"` fuera de allowlist soft. Default 600 s; override `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS_EJECUCION`. Timeout = `failed` + no disparar Argos.
3. **Rama `refactor/`:** `workspace-init` y TQM conservan prefijo; default de proceso `refactorization` = `refactor/{slug}`.
4. **`persist_ref` end-to-end:** TQM resuelve (FM PBI o Cúmulo `paths.featurePath`/`fixPath`) y lo inyecta al hijo. Runtime no envía `persist_ref` vacío si el hijo lo tiene. Handoff bajo ese path.
5. **Tests** de los cuatro vectores. Skip lab de cierre documental también para `refactorization`.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-TIMEOUT | Timeout de `cursor-agent` en Ejecución no lanza Argos en el mismo hijo. |
| AC-SKIP | Verificación `skipped` si Ejecución ∈ {`failed`, `awaiting_agents`}. |
| AC-BRANCH | `suggested_branch: refactor/foo` → worktree `refactor/foo`. |
| AC-PERSIST | `AGENT_PHASE.persist_ref` no vacío; existe `{persist_ref}/_agent_handoff.md`. |
| AC-TESTS | Unit/smoke: TQM `build_child_inputs` + `workspace-init` + timeout runtime. |
| AC-PR | PR único; `validacion.md` APTO; PBI en `docs/todos/done/` en la rama. |

## Ley aplicada

- Cúmulo: `paths.featurePath`, `paths.fixPath`; genoma `directories.norms` solo vía `entity-manager`.
- `features-documentation-pattern` v1.2.x — cierre documental en rama.
- `external-ai-constraints` DA-2/DA-3: no mutar process Core a mano; engine + scripts no son genoma.
- Prohibido timeout-as-only-fix. Prohibido mezclar Evolution.

## Dependencias

Ninguna PBI bloqueante. Evolution `7bb37ff1-…` es ciclo **ajeno**.
