---
document_id: PBI-KAIZEN-TRACKER-PROCESS-DECOUPLE
uuid: "34a1e5b7-721d-455b-861a-de926e7f591a"
title: "[KAIZEN] Desacoplamiento del ciclo documental — inversión de dependencias y ceguera espacial"
format: markdown
version: "1.0.0"
created: "2026-10-23"
status: pending
priority: alta
process: refactorization
dispatch: true
suggested_branch: refactor/tracker-process-decouple
persist_ref: docs/features/tracker-process-decouple
source_audit: docs/audits/linear-tracker-adapter-integracion-20261013.md
related:
  - SddIA/process/refine-hu.md
  - SddIA/process/refine-pbi.md
  - SddIA/library/codexes/codex-software-engineering/process/forge-pbi.md
  - SddIA/process/tracker-linear-markdown-sync.md
  - SddIA/process/tracker-markdown-apply.md
  - SddIA/process/tracker-stamp.md
  - SddIA/process/tracker-sync-replay.md
  - SddIA/process/tracker-backlog-query.md
  - SddIA/library/codexes/codex-software-engineering/contracts/project-config-contract.md
  - SddIA/library/norms/todos-jurisdiction.md
  - SddIA/library/norms/features-documentation-pattern.md
  - docs/todos/historias/HU-SDDIA-TRACKER-LIFECYCLE-DI-003.md
---

# [KAIZEN] Desacoplamiento del ciclo documental — inversión de dependencias y ceguera espacial

## 0. Hechos que esta ficha no puede contradecir

Verificado en el árbol el 2026-10-23. Una forja que los ignore rehace el corte sobre una arquitectura que no existe.

1. El único proveedor de `tracker:lifecycle` es `tool:linear-tracker-adapter` (`SddIA/tools/linear-tracker-adapter.md`). No hay adaptador de sistema de archivos ni adaptador GitHub. `tool:github-raw-fetcher` provee `asset:fetch`, otro contrato. Ningún proceso del ciclo nombra GitHub como tracker: no hay purga GitHub que hacer en estos YAML.
2. Con provider efectivo `file` el motor escribe en `paths.todos` / `docs_layout`. No pasa por `create_issue`. La frase «el adaptador inyectado materializa `create_issue` en disco» es el objetivo de este Kaizen, no el runtime actual.
3. `project-config-contract` v1.4.0 fija `tracker.provider` ∈ {`file`, `linear`} y exige `docs_layout.todos_pending` y `docs_layout.todos_done`. Esos campos son la costura de inyección y el layout del sumidero fichero. Sacarlos del manifiesto sin bump de contrato invalida los proyectos.
4. `todos-jurisdiction` v1.1.1 es la norma que posee los buckets. Un proceso que repite `docs/todos/...` se acopla. La norma que los define no es el mismo antipatrón: Cúmulo sigue siendo SSOT de rutas (`cumulo.paths.json`).
5. `docs/features/` y `docs/fixes/` (o `docs_layout.features` / `docs_layout.fixes`) son ingeniería de forja. `persist_ref` permanece. Este corte no los mueve al tracker.
6. `forge-pbi` v1.0.1 ya llama a la fase `Registro en tracker`. El nombre `Registro Linear` sigue en `refine-hu` v1.0.2, no en `forge-pbi`.
7. `refine-pbi` v1.0.1 no nombra Linear. Su acoplamiento es espacial (`todos_pending`, `done/`) y no declara `tracker-operations` ni `tracker:lifecycle`.
8. El input `sync_all` de `tracker-linear-markdown-sync` cita el bucket `Realizado`. `todos-jurisdiction` no lo reconoce. Es fósil de texto, no un tercer sumidero legal.
9. HU-003 ya prohibió el literal de despacho `linear-tracker-adapter` dentro de los procesos. Este Kaizen no reabre ese barrido. Abre el que HU-003 dejó: el nombre del producto y la ruta física dentro del proceso.

## 1. Problema

El proceso del Core nombra al producto y al directorio. La capacidad `tracker:lifecycle` queda como comentario dentro de un condicional (`provider efectivo es linear` / `Con file no hay create_issue`) y la entidad de gestión (HU, PBI, Kaizen, Fix) se identifica por un path de repositorio.

Eso invierte la dependencia al revés: el proceso conoce el sustrato; el puerto solo se usa en una rama.

## 2. Corte

### 2.1 Inversión en los procesos

Cada fase de ciclo de vida invoca `tracker:lifecycle` (`requires_capability` donde el contrato de proceso lo exige). El YAML no contiene el literal `linear`, `Linear`, `file` ni `GitHub`, ni la rama «si el provider es …».

La rama de sustrato sale del proceso y entra en el resolvedor: con provider de fichero no hay cápsula remota y la operación la cumple la implementación de fichero; con provider remoto la cumple la tool de la fila de binding. Hasta que exista esa implementación de fichero detrás del mismo contrato, borrar el condicional del YAML deja el proceso en falso: el motor seguiría bifurcando en handlers. El criterio de hecho es el handler, no la frase del `intent`.

Vocabulario de salida, común:

| Campo | Definición en el proceso |
|---|---|
| `tracker_ref` | Identificador opaco del ítem devuelto por el proveedor resuelto de `tracker:lifecycle`. |
| `hu_ref` / `pbi_ref` | Referencia opaca de la entidad en el gestor activo. No es una ruta. |
| Fase de alta | `Registro en Tracker`. |
| Fase de sello | Nombre de dominio (`Sello de ciclo`, `Replay de sincronización`, `Consulta de backlog`), sin producto. |

`create_issue`, `update_issue_state`, `update_issue_description`, `fetch_issue`, `list_issues`, `create_comment` siguen siendo los verbos del contrato `tracker.lifecycle` 1.0.0. No se renombran en este corte.

### 2.2 Ceguera espacial en los procesos

Un proceso no declara `docs/todos/`, `historias/`, `todos_pending`, `pending/`, `done/` ni `Realizado` como lugar de la entidad de gestión.

La copia de trabajo de Mayeuta y Argos vive solo en `workspace_template` (`.SddIA/workspaces/{process_name}/{execution_id}/`) durante la ejecución. La lectura del ítem y la persistencia definitiva las hace `tracker:lifecycle`.

Sigue en el proceso, porque no es backlog:

- `persist_ref` hacia `docs/features/` o `docs/fixes/`.
- `event_file_path` de un ECST ya emitido (ruta de bus, no de entidad de gestión).
- `workspace_template`.

### 2.3 Qué no se mueve de sitio

| Artefacto | Se queda |
|---|---|
| `todos-jurisdiction` | Dueña de los buckets del sumidero fichero. Se enmienda solo si una frase ordena a un proceso nombrar el path. No se disuelve la norma dentro de un adaptador. |
| `project-config-contract` | Conserva `tracker.provider` como clave de inyección y `docs_layout.todos_*` como layout de ese sumidero. La prosa que dice «Linear Project», el valor de `done_gate` llamado `linear` y el ejemplo `LINEAR_*` se reescriben al bump: el id de provider es el de la fila de binding; el proceso no lo lee. Borrar el enum sin fila `file` en bindings deja el manifiesto sin sumidero. |
| `features-documentation-pattern` | El cierre en `done/` es regla del provider fichero, escrita en la norma, no en el proceso. |
| Identidad del adaptador | `SddIA/tools/linear-tracker-adapter.md`, la fila `provider` del binding y los registros de `SddIA/evolution/` no se rebautizan. |
| Eventos ya versionados | `tracker_ref` pasa a «identificador opaco» en el genoma de evento solo en el mismo PR de forja, vía `entity-manager`. No es edición directa de este PBI. |

## 3. Inventario verificado (texto vigente → destino)

| Entidad | Acoplamiento presente | Destino del corte |
|---|---|---|
| `refine-hu` 1.0.2 | `hu_ref` «bajo historias/»; `tracker_ref` «Issue Linear»; fase `Registro Linear`; intent con `docs/todos/historias/` y rama `linear`/`file`; cuerpo «en historias/». | Refs opacas; fase `Registro en Tracker`; un solo `create_issue` vía contrato; cuerpo sin path. |
| `refine-pbi` 1.0.1 | `pbi_ref` «bajo todos_pending»; intent «sin tocar … done/»; sin puerto tracker. | `pbi_ref` opaco; sello sobre la copia de workspace; persistencia vía `tracker:lifecycle`. Alta de contexto `tracker-operations` en el mismo bump: hoy Cerbero rechazaría el cruce si se añade una fase `tool:` sin ese contexto. |
| `forge-pbi` 1.0.1 | Fase ya agnóstica. Siguen `historia_ref` «Ruta HU en historias/», `artifact_path` bajo `docs_layout.todos_pending`, intent con rama `linear`/`file`. | `historia_ref` = `tracker_ref` padre opaco; el path de alta lo devuelve el proveedor, no el output del proceso. |
| `tracker-linear-markdown-sync` 1.0.1 | Nombre de proceso, fase `Volcado descripción Linear`, escaneo `pending/done/Realizado`, cuerpo «issue Linear». | El proceso deja de escanear buckets. `update_issue_description` recibe la referencia opaca y el cuerpo ya cargado por el proveedor. El renombre del proceso es parte de la forja (nuevo `{name}.md` por `entity-manager`, no alias manual). `Realizado` no se conserva. |
| `tracker-markdown-apply` 1.0.0 | No dice `Linear` en el YAML. Output `markdown_path` y cuerpo «markdown local»; delega en `skill:git-manager`. | Deja de ser el proceso que elige el bucket. La réplica a fichero, si el provider es fichero, queda dentro de esa implementación. La rama git `tracker-sync/{project_slug}` no es una ruta de backlog; se decide en la forja si sigue siendo efecto del proceso o del adaptador. |
| `tracker-stamp` 1.1.0 | Fase `Sello Linear`; intent «Con provider file no hay cápsula»; cuerpo «→ Linear». Ya declara `requires_capability`. | Fase `Sello de ciclo`. El intent invoca el contrato. El «no hay cápsula» lo resuelve el binding, no el texto. |
| `tracker-sync-replay` 1.0.1 | `issue_ref` «Identificador Linear»; fase `Replay Linear`; intent «Con file no hay replay». | `issue_ref` opaco; fase `Replay de sincronización`; un solo `fetch_issue` + reaplicación. |
| `tracker-backlog-query` 1.0.1 | Intent «si el provider es linear» / «Con file lista pending y done»; cuerpo «backlog Linear». Fase `Consulta backlog` ya es agnóstica. | Un solo `list_issues`. El listado en fichero lo hace el proveedor de fichero. |
| `process/index.md` | Blurbs con «Linear», «historias/», «Sello Linear». | Se regenera con el índice del proceso, no a mano. |
| Clases ECST citadas en §0.9 | `tracker_ref` glosado como «identificador Linear»; `Tracker_Issue_Changed` / `Tracker_Sync_Failed` narran el producto. | Glosa = identificador opaco. El nombre de la clase no cambia en este corte. |
| `project-config-contract` 1.4.0 | Enum `file`\|`linear`, «Linear Project», `done_gate: linear`, secreto `LINEAR_*`. | Bump de contrato en la forja. El proceso no gana un condicional nuevo mientras el enum siga. |
| `todos-jurisdiction` 1.1.1 | Habla de `pending/` y `historias/` como ley de buckets, y de provider `file`/`linear`. | Sigue siendo la ley del sumidero fichero. No se copia esa ley dentro de `refine-hu` ni de `forge-pbi`. |
| `features-documentation-pattern` | Cierre `docs/todos/done/` vs `state_map.done` según provider. | Norma de cierre, fuera del YAML de proceso. No es hallazgo de este inventario de procesos. |

## 4. Criterios de aceptación

| ID | Criterio |
|---|---|
| KZ-CA1 | En los ocho procesos de la tabla §3, el frontmatter y el cuerpo no contienen `Linear`, `linear`, `file` como nombre de provider, ni `docs/todos`, `historias/`, `todos_pending`, `pending/`, `done/`, `Realizado`. |
| KZ-CA2 | `refine-hu` declara `tracker_ref` como identificador opaco de `tracker:lifecycle`, `hu_ref` como referencia opaca, y la fase de alta se llama `Registro en Tracker` con un único `create_issue`. |
| KZ-CA3 | Ningún handler de esos procesos bifurca por el literal `linear` o por un path `docs/todos` propio. La bifurcación, si queda, está en el resolvedor del provider o en la implementación de fichero del mismo contrato. Un test de handler cubre provider fichero y provider remoto. |
| KZ-CA4 | `persist_ref`, `docs_layout.features` y `docs_layout.fixes` no se eliminan. Un PBI forjado sigue apuntando su carpeta de forja en el repositorio. |
| KZ-CA5 | `todos-jurisdiction` y `cumulo.paths.json` siguen definiendo los buckets. Los procesos no los reescriben. |
| KZ-CA6 | No aparece un `github-tracker-adapter` ni un segundo id de capacidad. Los verbos siguen siendo los seis de `tracker.lifecycle` 1.0.0. |
| KZ-CA7 | Mutación de genoma solo con topología `docs/features/tracker-process-decouple/`, `entity-manager` y registro en `SddIA/evolution/`. |

## 5. Fuera de alcance

- Rebautizar `linear-tracker-adapter` o reescribir `SddIA/evolution/`.
- Cambiar el workflow de forja (`spec.md` … `validacion.md`).
- Tratar `event_file_path` del bus como entidad de gestión.
- Inventar un provider GitHub.
