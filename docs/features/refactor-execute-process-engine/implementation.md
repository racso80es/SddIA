---
feature_name: refactor-execute-process-engine
created: "2026-05-20"
process: feature
items:
  - execute-process-dynamic-interpreter
  - execute-process-core-module
  - execute-process-capsules-registry
  - execute-action-eda-handlers
  - emit-pr-presented-action-catalog
  - event-subscriptions-iota-restore
  - ola-c-cli-shims
---

# Implementación — touchpoints

| Componente | Cambio |
|------------|--------|
| `SddIA/scripts/qa/execute-process.py` | CLI delgado; delega en core + cápsulas; shim `--action` → `execute-action.py` |
| `SddIA/scripts/qa/execute_process_core.py` | Resolución proceso/aliases, validación inputs, warnings deprecación |
| `SddIA/scripts/qa/execute_process_capsules.py` | Bucle de fases, `workspace-init`, `CAPSULE_ACTION_REGISTRY`, forja skill/event, entity-manager |
| `SddIA/scripts/qa/execute-action.py` | Handlers `emit-pr-merged-event`, `emit-pr-presented-event`, `emit-domain-mutation` |
| `SddIA/actions/emit-pr-presented-event.md` | Contrato acción + fila en `SddIA/actions/index.md` |
| `SddIA/core/event-subscriptions.json` | Restauración IOTA en `PullRequest_Presented` y `Domain_Entity_Created/Updated` |
| `docs/todos/[ARQUITECTURA] Deuda Ola C — …` | Registro deuda retirada shims CLI |

## Commits relevantes

| Ref | Descripción |
|-----|-------------|
| `31bae72` | Intérprete dinámico agnóstico (squash base PR #9) |
| `775114c` | Registry acciones EDA (en historial feature) |
| `d2c9559` | Merge PR #9 → `main` |
| `18d80ea` | Fix suscripciones IOTA |
| `9b9d611` | Manifiestos de cierre (este hito documental) |
