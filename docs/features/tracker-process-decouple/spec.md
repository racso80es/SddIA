---
feature_name: tracker-process-decouple
process: refactorization
created: "2026-10-23"
branch_name: refactor/tracker-process-decouple
persist_ref: docs/features/tracker-process-decouple
---

# Especificación

Un módulo, `tracker_lifecycle_port`, resuelve el layout (`pending`, `done`, historias como hermano de `pending`) desde Cúmulo. Si el fichero de Cúmulo no está, el layout por defecto es el de `paths.todos`.

Los procesos de ciclo nombran `tracker:lifecycle` y referencias opacas. El volcado de descripción se llama `tracker-description-sync` (uuid `a3b4c5d6-e7f8-4890-a123-456789abcd01`). El despacho sigue aceptando el nombre anterior.

El archivo de PBI en el cierre y el movimiento a cerrado de `tracker-markdown-apply` usan `done` del puerto, no una ruta escrita en el handler.
