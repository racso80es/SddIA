---
document_id: PBI-TRACKER-PROVIDER-CAPSULE
uuid: "28851171-d695-4f36-bf75-bbe08d956ef2"
title: "[PBI] El ciclo documental persiste solo por la cápsula de tracker:lifecycle"
format: markdown
version: "1.3.0"
created: "2026-10-23"
status: pending
priority: alta
process: refactorization
dispatch: true
suggested_branch: refactor/tracker-provider-capsule
persist_ref: docs/features/tracker-provider-capsule
historia_ref: docs/todos/historias/HU-SDDIA-TRACKER-LIFECYCLE-DI-003.md
project_slug: core
tracker_ref: MAN-24
related:
  - SddIA/library/codexes/codex-software-engineering/process/forge-pbi.md
  - SddIA/process/refine-hu.md
  - SddIA/process/refine-pbi.md
  - SddIA/process/tracker-markdown-apply.md
  - SddIA/process/tracker-stamp.md
  - SddIA/process/tracker-backlog-query.md
  - SddIA/process/tracker-description-sync.md
  - SddIA/process/tracker-sync-replay.md
  - SddIA/engine/execute-process/src/engine/materialize_fracture_pbi.rs
  - SddIA/engine/execute-process/src/engine/tracker_lifecycle_port.rs
  - SddIA/library/codexes/codex-software-engineering/process/feature.md
  - SddIA/library/codexes/codex-software-engineering/process/bug-fix.md
  - SddIA/library/codexes/codex-software-engineering/process/refactorization.md
  - SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md
  - docs/tmp/forge-pbi-inyeccion-provider.md
---

# [PBI] El ciclo documental persiste solo por la cápsula de tracker:lifecycle

Refino de `docs/tmp/forge-pbi-inyeccion-provider.md`. Historia padre `MAN-14`.

El proceso llama un verbo de `tracker:lifecycle`. La cápsula inyectada persiste. El handler no elige `file` ni `linear` y no escribe el bucket.

## Tareas

### T1 — Contrato e inyección

- Estado: realizada. La cápsula `file-tracker-adapter` escribe el bucket. `linear` sigue en `linear-tracker-adapter`. Los handlers ya no llaman a `substrate()`.

- La capacidad sigue siendo `tracker:lifecycle`. No hay verbo nuevo.
- `create_issue` acepta `operation`, `title`, `description`, `labels` y `parent_ref` opaco. Devuelve `tracker_ref` opaco.
- Fuera del proceso: `team_key`, `state_name`, `project_id`, rutas de `docs/todos` y la elección de proveedor. Los lee la cápsula del manifiesto.
- El binding elige la cápsula: `linear` → `linear-tracker-adapter`; `file` → cápsula nueva del mismo contrato que escribe en el bucket de Cúmulo; otro proveedor → su cápsula, el mismo sobre.
- Quitar `tracker_lifecycle_port::substrate()` de los handlers. El motor deja de implementar el sumidero fichero.

### T2 — Mismo cuerpo

- Estado: realizada. `create_issue` recibe el markdown entero, cabecera incluida.

- El `description` de `create_issue` es el fichero entero, cabecera YAML incluida.
- Fuera solo los campos que el tracker añade y el fichero no tiene: `team_key`, `state_id`, `labelIds`.
- El título es el mismo en los dos soportes.

### T3 — `forge-pbi`

- Estado: realizada. Sin `fs::write`. `PBI_Forged` lleva `tracker_ref` y no publica `artifact_path`. El alta exige `parent_ref`.

- Arma título, cuerpo y etiquetas (`pbi`, y `fix` o `kaizen` según `process` / `tipo`) y hace una llamada.
- Sin `fs::write` en el handler. El Core con `provider: linear` no genera fichero en `docs/todos/pending/`.
- Si `create_issue` falla: `Tracker_Sync_Failed` y no hay alta local de compensación.
- **Sellado** emite `PBI_Forged` con `tracker_ref`. El evento deja de publicar `artifact_path`.
- El invocante pasa `parent_ref` opaco. El proceso no abre la ruta de la historia.

### T4 — `refine-hu`

- Estado: realizada. El sello del fichero lo escribe la cápsula de fichero. `create_issue` solo si falta `tracker_ref` y el manifiesto es linear.

- Alta y sello por el proveedor. El proceso no abre la ruta ni sella con `fs::write`.
- `create_issue` solo cuando falta `tracker_ref`, sin rama de sustrato.

### T5 — `refine-pbi`

- Estado: realizada. El sello llama a `update_issue_description` de la cápsula de fichero.

- El sello definitivo es una llamada al proveedor. Hoy el handler escribe el fichero y no llama `update_issue_description`.

### T6 — `tracker-markdown-apply`

- Estado: realizada. La rama local ya no sale de `substrate()`. Con linear la cápsula remota no escribe el bucket.

- Aplicar el cambio y cerrar la réplica es método del proveedor de fichero.
- Con `linear` la cápsula no escribe disco.

### T7 — `tracker-stamp`

- Estado: realizada. El sello resuelve el tracker por el manifiesto, sin `substrate()`.

- El sello de estado y el comentario siguen por la cápsula.
- `archive_cancelled_pbi` y la lectura del PBI por ruta pasan al proveedor.

### T8 — `tracker-backlog-query`

- Estado: realizada. El proveedor de fichero responde `list_issues`.

- Las dos ramas actuales (`list_issues` remoto y `backlog_items` local) pasan a ser `list_issues` de la cápsula inyectada.

### T9 — `tracker-description-sync`

- Estado: realizada. El proveedor de fichero carga el cuerpo. El proceso ya no responde «sumidero local».

- El cuerpo lo carga el proveedor. El proceso no abre el bucket ni sale por «sumidero local».

### T10 — `tracker-sync-replay`

- Estado: realizada. La reaplicación de fichero entra en la cápsula. Ya no se descarta por ser local.

- `fetch_issue` y la reaplicación van a la cápsula. Descartar por ser fichero deja de ser una rama del proceso.

### T11 — `materialize-fracture-pbi`

- Estado: realizada. El alta sin linear llama a `create_issue` de la cápsula de fichero.

- El alta pasa por `create_issue`. Deja de escribir el PBI en disco por su cuenta.

### T12 — Un commit por tarea en Ejecución

- Estado: realizada. Merge `6892409e` (PR #375). Commit de la tarea `669a0386`.
- En la fase **Ejecución** de `feature`, `bug-fix` y `refactorization`, Tekton deja un commit por tarea del `plan.md`, vía `git-manager`.
- El intent de esa fase quedó fijado en los tres procesos.
- Cada commit actualiza el documento de la fase (`execution.md` bajo `persist_ref`) con la tarea y el identificador del commit, de modo que quede registrado que la tarea fue forjada.
- El mismo commit actualiza el PBI de trabajo: la tarea queda marcada como forjada en el fichero y, por el mismo cuerpo, en la issue.
- **Snapshot final** de `delivery-close-cycle` no parte las tareas. Solo consolida lo que siga sin commit.

## Fuera de este corte

`feature`, `bug-fix`, `refactorization` y `delivery-close-cycle` no entran. Su documentación vive en `persist_ref` (`docs/features`, `docs/fixes`).
