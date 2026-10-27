---
feature_name: kaizen-fracture-pbi-core-slug
created: "2026-10-27"
process: refactorization
persist_ref: docs/features/kaizen-fracture-pbi-core-slug
branch_name: refactor/kaizen-fracture-pbi-core-slug
---

# Objetivos — kaizen-fracture-pbi-core-slug

Linear: **AUT-24** (kaizen, hijo de **AUT-5**). Descripción del issue = cuerpo forjado vía `forge-pbi`.

## Misión

Con `tracker.provider: linear` en Core-self, `materialize-fracture-pbi` debe abrir el backlog en Linear aunque `System_Fracture_Detected` no traiga `project_slug`, sin escribir markdown en `docs/todos/pending/`.

## Causa raíz (auditoría 2026-10-27)

1. Los emisores de fractura (DCC, `route-domain-event` / merkle, vitalidad, heartbeat) no incluyen `project_slug` en el payload.
2. `tracker_config_for_inputs` sin slug → `None` → rama fichero vía `tracker_lifecycle_port` (`Unconfigured` = `file_tracker_adapter`).
3. El ledger en `pending/` devuelve `already_open` y impide `create_issue` aunque no exista issue en Linear.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-1 | Fractura sin `project_slug` en repo con índice `core` + manifiesto `linear` → `create_issue`, sin `.md` nuevo en `pending/`. |
| AC-2 | Repo de test sin `.SddIA/projects/core.md` → materialización en fichero (paridad actual). |
| AC-3 | Con `linear`, un `.md` huérfano en `pending/` no bloquea alta remota (dedupe remoto, no ledger local). |
| AC-4 | `create_fracture_issue` usa `build_pbi_body` y labels `pbi` + `fix`. |
| AC-5 | Tests en `materialize_fracture_pbi` + actualización de `SddIA/actions/materialize-fracture-pbi.md` vía `entity-manager` y registro evolution. |

## Fuera de alcance (corte 2)

- Inyectar `project_slug` en todos los emisores ECST.
- `materialize-ci-chronic-failure-pbi` / kaizen alert doc.
- Regresión `R<n>` cuando el canónico solo está en issue Linear `Done` (requiere list + estado).
