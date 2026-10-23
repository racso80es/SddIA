---
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-05
uuid: "9635ef7a-00e5-4183-91a6-6ce68b5071c4"
title: "[ARQUITECTURA] Tracker lifecycle — sumidero file|linear"
format: markdown
version: "1.0.0"
status: pending
priority: alta
type: arquitectura
process: feature
dispatch: true
feature_name: tracker-lifecycle-sink
historia_ref: docs/todos/historias/HU-SDDIA-TRACKER-LIFECYCLE-DI-003.md
historia_document_id: HU-SDDIA-TRACKER-LIFECYCLE-DI-003
created: "2026-10-22"
author: tekton
updated: "2026-10-22"
cola_ejecucion: docs/todos/pending/
supersedes:
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-06
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-07
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-08
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-09
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-10
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-11
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-12
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-13
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-14
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-15
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-16
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-17
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-18
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-19
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-20
blocked_by:
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-01
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-02
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-03
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-04
unblocks: []
files:
  - SddIA/process/refine-hu.md
  - SddIA/engine/execute-process/src/engine/handlers/refine.rs
  - SddIA/library/codexes/codex-software-engineering/process/forge-pbi.md
  - SddIA/engine/execute-process/src/engine/handlers/forge_pbi.rs
  - SddIA/process/tracker-backlog-query.md
  - SddIA/engine/execute-process/src/engine/handlers/tracker_backlog_query.rs
  - SddIA/process/index.md
  - SddIA/process/tracker-sync-replay.md
  - SddIA/engine/execute-process/src/engine/handlers/tracker_sync_replay.rs
  - SddIA/process/tracker-linear-markdown-sync.md
  - SddIA/engine/execute-process/src/engine/handlers/tracker_linear_markdown_sync.rs
  - SddIA/engine/execute-process/src/engine/done_gate.rs
  - SddIA/engine/execute-process/src/engine/handlers/tracker_markdown_apply.rs
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - SddIA/engine/execute-process/src/engine/tracker_operations_gate.rs
  - SddIA/engine/execute-process/src/engine/capsules.rs
  - SddIA/norms/execution-contexts.md
  - SddIA/daemons/linear-bridge-watcher/src/main.rs
  - SddIA/library/codexes/codex-software-engineering/contracts/project-config-contract.md
  - SddIA/library/norms/features-documentation-pattern.md
  - SddIA/library/norms/todos-jurisdiction.md
  - SddIA/engine/execute-process/src/engine/project_binding.rs
  - .cursor/rules/task-closure-documental.mdc
  - SddIA/actions/materialize-fracture-pbi.md
  - SddIA/engine/execute-process/src/engine/materialize_fracture_pbi.rs
  - SddIA/actions/enrich-fracture-pbi-kaizen.md
  - SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs
  - SddIA/actions/materialize-kaizen-alert-doc.md
  - SddIA/engine/execute-process/src/engine/materialize_kaizen_alert_doc.rs
  - SddIA/actions/materialize-ci-chronic-failure-pbi.md
  - SddIA/engine/execute-process/src/engine/materialize_ci_chronic_failure_pbi.rs
  - .SddIA/project.md
---

# [ARQUITECTURA] Tracker lifecycle — sumidero file|linear

Historia: `HU-SDDIA-TRACKER-LIFECYCLE-DI-003` §7.5–§7.20.

Los PBI 01–04 están en `docs/todos/done/`. Este PBI absorbe los antiguos 05–20. Cada uno es una tarea. El orden de ejecución es el de la tabla: T13 y T14 abren el resto.

Ninguna tarea abre `tracker-operations` en `feature`, `bug-fix` o `refactorization`. Con provider `linear`, T05, T06 y T16–T19 no crean ficheros en `docs/todos/pending/` ni en `docs/todos/done/`. Con `file`, sí. `historias/` no es esa cola.

## Tareas

| ID | Antes | Hecho cuando |
|----|-------|----------------|
| T13 | 13 | Las tres normas dicen default `file`, sin `done_gate: both`, y el cierre partido. |
| T14 | 14 | El parser admite `file` \| `linear` y rechaza la contradicción. |
| T15 | 15 | La regla del operador copia ese cierre. |
| T05 | 05 | `refine-hu` usa el provider resuelto. |
| T06 | 06 | `forge-pbi` escribe un solo sumidero. |
| T07 | 07 | El backlog pide `tracker:lifecycle`. |
| T08 | 08 | El replay pide la capacidad y no corre en `file`. |
| T09 | 09 | El volcado a la descripción no fabrica backlog. |
| T10 | 10 | El cierre no escribe los dos sumideros. |
| T11 | 11 | El peaje compara el provider resuelto. |
| T12 | 12 | El watcher resuelve el binario y no arranca en `file`. |
| T16 | 16 | La fractura elige sumidero. |
| T17 | 17 | El enriquecido edita ese mismo sumidero. |
| T18 | 18 | La alerta Kaizen elige sumidero. |
| T19 | 19 | El fallo crónico de CI elige sumidero. |
| T20 | 20 | Core-self y Paciente 0 quedan en `linear` explícito. |

Dependencias internas: T14 después de T13. T15 después de T13. T05, T06, T09, T10, T11, T12, T16, T18, T19 y T20 después de T14. T17 después de T16. T10 también después de T13. T07 y T08 no esperan a T14.

## T13 — Tres normas del sumidero

`uuid` heredado `a35fba7b-80f5-43a4-87e5-fabc84bb86ef`.

| ID | Requisito |
|----|-----------|
| R-1 | `project-config-contract` bump. `tracker.provider`: `file` \| `linear`. Objeto o provider ausente ⇒ `file`. `linear` exige `team_key` y `state_map`. `both` no es valor de `done_gate`. |
| R-2 | `features-documentation-pattern`: `file` exige `pbi_archived: true` y el PBI en `done/` en la rama. `linear` exige el issue en `state_map.done` y prohíbe fichero nuevo en `pending/` y `done/`. La forja en `docs/features` y `docs/fixes` no cambia. `validacion.md` APTO sigue en ambos. |
| R-3 | `todos-jurisdiction`: `pending/` y `done/` son cola solo con `file`. Con `linear` el ancla es `tracker_ref`. Las acciones canónicas eligen sumidero. No nace una segunda acción. |

AC-1: el contrato no deja leer que el default es Linear ni que `both` es legal. AC-2: Done partido no pide el fichero cuando el provider es `linear`. AC-3: la jurisdicción nombra `materialize-fracture-pbi`, `enrich-fracture-pbi-kaizen`, `materialize-kaizen-alert-doc` y `materialize-ci-chronic-failure-pbi`.

Ficheros: `project-config-contract.md`, `features-documentation-pattern.md`, `todos-jurisdiction.md`.

## T14 — Parser file|linear

`uuid` heredado `a627c8c0-75cb-436f-a691-e51a2a4584e0`. Después de T13.

| ID | Requisito |
|----|-----------|
| R-1 | `provider` ∈ {`file`, `linear`}. Otro valor → `PROJECT_CONFIG_INVALID`. |
| R-2 | Sin objeto `tracker` o sin `provider`: efectivo `file`. No exige `team_key`. |
| R-3 | `linear` sin `team_key`, `done_gate: both`, o un `done_gate` que contradiga al provider → `PROJECT_CONFIG_INVALID`. Coherente (`linear`/`linear`, `file`/`git`) se acepta. |

AC-1: el manifiesto Core-self (`provider: linear`, `team_key: OSC`, `done_gate: linear`) sigue pasando. AC-2: sin `tracker` resuelve `file` y no exige team. AC-3: `jira`, `both` y `file` + `done_gate: linear` fallan.

Fichero: `SddIA/engine/execute-process/src/engine/project_binding.rs`.

## T15 — Regla de cierre del operador

`uuid` heredado `38468be1-a064-4cc2-b874-f9ca71c90d30`. Después de T13.

R-1: con `file`, Done pide `validacion.md` APTO, `pbi_archived: true` y el PBI en `docs/todos/done/` en la rama. R-2: con `linear`, Done pide el issue en `state_map.done` y `validacion.md` APTO, y no pide el fichero.

AC-1: no hay una definición única que exija el fichero para todo provider. AC-2: `done_gate: both` no es camino de cierre.

Fichero: `.cursor/rules/task-closure-documental.mdc`.

## T05 — Alta de HU

`uuid` heredado `c5c6aad7-bdb1-4f58-b80b-577bdab49983`. Después de T14.

R-1: `refine-hu.md` y `refine.rs` no contienen el literal de la tool. La llamada usa el provider resuelto. R-2: `linear` hace `create_issue` con la label `tracker.labels.hu` si falta `tracker_ref`, y no crea ficheros en `pending/` ni `done/`. R-3: `file` no hace `create_issue`. El refinado de `historias/` se mantiene.

AC-1: `rg linear-tracker-adapter` vacío en los dos ficheros. AC-2: manifiesto sin `tracker` no invoca la cápsula y actualiza `historias/`. AC-3: con `linear` el issue existe y `pending/` no gana un `.md` de esta ejecución.

Ficheros: `SddIA/process/refine-hu.md`, `handlers/refine.rs`.

## T06 — Alta de PBI

`uuid` heredado `e7c9b713-2d4e-45ab-9465-344cd12bc46c`. Después de T14.

R-1: proceso y handler sin el literal. R-2: `file` escribe el PBI en `docs/todos/pending/` y no hay `create_issue`. R-3: `linear` hace `create_issue` con labels `pbi` + tipo y `parent_ref` de la HU. Cero fichero nuevo en esos buckets. Sin `team_key` el handler no inventa un team: T14 ya invalidó el manifiesto.

AC-1: `rg` vacío en los dos ficheros. AC-2: los dos sumideros no se materializan en la misma ejecución.

Ficheros: `forge-pbi.md`, `handlers/forge_pbi.rs`.

## T07 — Backlog

`uuid` heredado `919e0f36-ce27-446c-855a-d354735c0542`.

R-1: los tres ficheros no contienen el literal. R-2: `linear` usa `list_issues` con el filtro de label vigente. R-3: `file` lista `pending/` y `done/` sin cápsula.

AC-1: `rg` vacío en los tres. AC-2: la fila `tracker-backlog-query` de `process/index.md` nombra `tracker:lifecycle`. AC-3: `file` no abre red.

Ficheros: `tracker-backlog-query.md`, `handlers/tracker_backlog_query.rs`, `SddIA/process/index.md`.

## T08 — Replay

`uuid` heredado `4f8dcdff-2977-47e5-bd14-eeb8618f3ef5`.

R-1: proceso y handler sin el literal; usan el provider resuelto. R-2: `file` no reintenta contra Linear. R-3: `linear` conserva el orden de replay vigente. Un fallo de schema sigue siendo dead-letter, no este replay.

AC-1: `rg` vacío en los dos ficheros. AC-2: un `Tracker_Sync_Failed` con `linear` sigue pudiendo reintentarse.

Ficheros: `tracker-sync-replay.md`, `handlers/tracker_sync_replay.rs`.

## T09 — Volcado markdown → Linear

`uuid` heredado `99ac13f0-ebaf-4aa2-9fa9-6631e8c050ba`. Después de T14.

R-1: proceso y handler sin el literal. R-2: `linear` escribe la descripción con el provider resuelto y no crea ficheros en `pending/` ni `done/`. R-3: `file` es no-op.

AC-1: `rg` vacío. AC-2: tras una ejecución `linear`, `pending/` y `done/` no ganan ficheros de este proceso.

Ficheros: `tracker-linear-markdown-sync.md`, `handlers/tracker_linear_markdown_sync.rs`.

## T10 — Cierre sin doble sumidero

`uuid` heredado `51268a96-d378-48ee-b558-29711b3b9739`. Después de T13 y T14.

R-1: `file` — `validacion` APTO y el PBI en `docs/todos/done/` en la rama. `phase_capsules` archiva desde `pending/`. R-2: `linear` — el gate comprueba `state_map.done` con `fetch_issue` del provider resuelto. No crea ni mueve ficheros de backlog. `pbi_archived` no exige ese path. R-3: los tres ficheros, en sus despachos, no contienen el literal.

AC-1: `tracker-markdown-apply` en `linear` no crea ficheros bajo `pending/` ni `done/`. AC-2: `file` termina en `done/` y no hay `fetch_issue`. AC-3: `rg` vacío en los tres.

Ficheros: `done_gate.rs`, `handlers/tracker_markdown_apply.rs`, `phase_capsules.rs`.

## T11 — Peaje por provider resuelto

`uuid` heredado `398d6479-cf4e-4494-8761-461e10fe4aad`. Después de T14.

R-1: desaparece `LINEAR_TRACKER_TOOL`. `capsules.rs` aplica el gate cuando `tool_name` es el provider resuelto de `tracker:lifecycle`. R-2: con `file`, `invoke_tool_for_process` no entra en ese gate. R-3: `execution-contexts.md` §2.10 cita `tracker:lifecycle` y mantiene el adaptador vigente como ejemplo.

AC-1: `rg LINEAR_TRACKER_TOOL` vacío. AC-2: un `create_issue` con `linear` sigue exigiendo contexto `tracker-operations`. AC-3: en `SddIA/norms/` el literal solo queda en ese ejemplo.

Ficheros: `tracker_operations_gate.rs`, `capsules.rs`, `execution-contexts.md`.

## T12 — Binario del watcher

`uuid` heredado `da93d4c5-a748-4919-a92c-5eb4f015fdf3`. Después de T14.

R-1: `resolve_adapter_bin` construye `SddIA/target/{profile}/{name}` con el `name` de la fila `provider`. R-2: el fuente del watcher no contiene el literal. R-3: con `file` el watcher sale sin poll y sin webhook. No se renombra el crate. No se toca `linear_bridge.rs`.

AC-1: `rg linear-tracker-adapter` vacío en `main.rs` del watcher. AC-2: con `linear` localiza el binario release o debug del `name` resuelto. AC-3: la ventana anti-eco no cambia.

Fichero: `SddIA/daemons/linear-bridge-watcher/src/main.rs`.

## T16 — Fractura

`uuid` heredado `40065401-0b52-4183-8217-2ffdc1f2a0f3`. Después de T13 y T14.

R-1: `file` crea o reabre el PBI en `docs/todos/pending/` y no hay `create_issue`. R-2: `linear` hace `create_issue` y `target_path` no apunta a un `.md` nuevo. La deduplicación por `fracture_hash` se conserva. R-3: los dos ficheros no contienen el literal.

AC-1: `file` deja el `.md` en `pending/` y no abre red. AC-2: `linear` no deja un `.md` nuevo y hay `issue_ref`. AC-3: el mismo hash no duplica el sumidero.

Ficheros: `materialize-fracture-pbi.md`, `materialize_fracture_pbi.rs`.

## T17 — Enriquecido de la fractura

`uuid` heredado `7571be3c-2a34-4403-8ae6-4da29999fcd6`. Después de T16.

R-1: `file` edita el `.md` de `pending/` ya materializado. R-2: `linear` edita el issue (`update_issue_description` o `create_comment`) y no crea `.md`. R-3: no nace una acción paralela.

AC-1: un solo sumidero editado. AC-2: `linear` sin `tracker_ref` no inventa un path bajo `docs/todos/`.

Ficheros: `enrich-fracture-pbi-kaizen.md`, `enrich_fracture_pbi_kaizen.rs`.

## T18 — Alerta Kaizen

`uuid` heredado `00fd5940-9aab-4c45-918b-e5c845c3e171`. Después de T13 y T14.

R-1: `file` deja el documento en `docs/todos/pending/`. R-2: `linear` hace `create_issue` con label `tracker.labels.kaizen` y cero fichero nuevo. R-3: la acción sigue siendo la canónica.

AC-1: los dos providers no materializan a la vez. AC-2: si hay despacho, el literal no queda: se usa el provider resuelto.

Ficheros: `materialize-kaizen-alert-doc.md`, `materialize_kaizen_alert_doc.rs`.

## T19 — Fallo crónico de CI

`uuid` heredado `9fff0083-f194-42fa-894d-8224a588b044`. Después de T13 y T14.

R-1: `file` escribe el PBI en `docs/todos/pending/`. R-2: `linear` hace `create_issue` y ningún `.md` nuevo. R-3: la acción sigue siendo la canónica. La detección del fallo no cambia.

AC-1: un solo sumidero por ejecución. AC-2: `file` no llama a la cápsula.

Ficheros: `materialize-ci-chronic-failure-pbi.md`, `materialize_ci_chronic_failure_pbi.rs`.

## T20 — Manifiestos Linear explícitos

`uuid` heredado `e1d4445d-20ad-400f-b23e-fc115e2cdd84`. Después de T14.

R-1: `.SddIA/project.md` de Core-self mantiene `provider: linear` y `team_key: OSC`. T14 lo acepta. R-2: si `/home/racso/Proyectos/SddIA_AP/.SddIA/project.md` existe, `tracker.provider` es `linear` con `team_key` y `state_map`. Si el checkout no está, la tarea no se cierra: no se fabrica el fichero dentro de este repo. R-3: ninguno de los dos omite `provider`.

AC-1: `project_binding` acepta Core-self. AC-2: Core-self no queda con provider efectivo `file`. AC-3: Paciente 0, cuando el fichero está en disco, valida como `linear`.

Fichero en este repo: `.SddIA/project.md`.
