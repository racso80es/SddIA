---
feature_name: refine-item-mayeuta
created: "2026-10-27"
process: refactorization
phase: Verificación
branch: refactor/refine-item-mayeuta
global: NO_APTO
tracker_ref: AUT-19
document_id: PBI-f9627b0b
pr_url: https://github.com/racso80es/SddIA/pull/404
pbi_archived: false
execution_id: "51caf6b7-c386-447b-a380-edf5e8c97d73"
approval_status: integrity_failure
checks:
  AC-1-yaml-header: NO_EVIDENCIA_CAPSULA
  AC-2-needs-clarification: NO_EVIDENCIA_CAPSULA
  AC-3-dor-refined: NO_EVIDENCIA_CAPSULA
  AC-4-hu-create-issue: NO_EVIDENCIA_CAPSULA
  AC-5-ide-handoff: NO_EVIDENCIA_CAPSULA
  AC-6-timeout: NO_EVIDENCIA_CAPSULA
  AC-7-conflict: NO_EVIDENCIA_CAPSULA
  AC-8-process-renamed: NO_EVIDENCIA_CAPSULA
  AC-9-ecst-source-process: NO_EVIDENCIA_CAPSULA
  cascade-verificacion: SKIPPED
  capsules-qa: BLOCKED
  git-evidence: BLOCKED
  CI-GITHUB: PENDIENTE-CI
git_changes:
  - SddIA/engine/execute-process/src/engine/handlers/refine_item/
  - SddIA/library/codexes/codex-software-engineering.md
  - SddIA/library/codexes/codex-software-engineering/process/refine-item.md
  - SddIA/library/norms/backlog-item-refinement.md
  - SddIA/events/domain/pbi-refined.md
  - SddIA/events/domain/hu-refined.md
  - docs/refactors/refine-item-mayeuta/
  - SddIA/evolution/74585763-bb9f-4835-8cb3-bbeeb3794a77.md
---

# Validación — AUT-19 (Argos)

## Veredicto

`approval_status: integrity_failure` · `global: NO_APTO`

Causa: sin stdout de cápsulas en este turno; L-ARGOS-SYNTHESIS prohíbe sustituir tests/QA/git por inferencia LLM. Cascada oficial `phase_reports.json` (execution_id `51caf6b7-…`): Estabilización=`blocked`; Verificación=`skipped` (`prior_agent_phase_not_executed`). Norma `features-documentation-pattern`: `CI-GITHUB: PENDIENTE-CI` impide `global: APTO`.

## Evidencia disponible (insuficiente)

| Fuente | Hallazgo |
|--------|----------|
| Filesystem (Read/Glob) | Existen `handlers/refine_item/*`, `refine-item.md`, norma DoR, códice 1.1.0, alias `refine-pbi`/`refine-hu` ausentes, evolution + cascada documental bajo `persist_ref` |
| Cápsulas (`sddia-run` / `sddia-qa` / `cargo test`) | **No ejecutables**: Shell IDE Rejected (×3); MCP `sddia-workspace-server` ausente del catálogo |
| `git-manager` | Sin acuse JSON este turno |
| Workspace | `.SddIA/workspaces/refactorization/51caf6b7-…/phase_reports.json` no sella Verificación |

APTO previo en este archivo **no re-corroborado**; se retira hasta evidencia determinista.

## correction_blueprint_md

```yaml
name: reabrir-verificacion-argos-aut19
intent: Restaurar peaje de cápsulas y re-sellar Argos con evidencia física
delegates_to:
  - tool:shell-executor   # habilitar Shell/`./sddia-run.sh` en runtime
  - tool:git-manager      # status/diff rama refactor/refine-item-mayeuta
  - action:execute-process # cargo/tests vía peaje; sddia-qa si aplica
  - agent:argos           # reinyectar Verificación con process/phase explícitos
```

Desbloqueo: aprobar MCP workspace + Shell → cápsulas QA/tests + `git-manager` → run CI con `run_id`/URL → reinyectar `refactorization` / `Verificación` / `agent:argos`.
