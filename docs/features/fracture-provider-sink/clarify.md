---
feature_name: fracture-provider-sink
created: "2026-10-30"
process: refactorization
purpose: contrastar AUT-30 con el código vigente
tracker_ref: AUT-30
---

# Clarificación — AUT-30

El issue (2026-10-27) describe un motor que escribía markdown siempre que faltaba slug. El código posterior ya distingue `file`, `linear`, provider desconocido y `Unconfigured`. El test `resolver_file_does_not_open_remote_and_linear_does_not_open_backlog` fija que un manifiesto sin objeto `tracker` es `Unconfigured`, no fichero.

Lo que seguía mal:

1. `invoke` trataba `Unconfigured` como el adaptador de fichero.
2. Sin slug, `tracker_config_for_inputs` devolvía `None` y no leía `.SddIA/project.md`.
3. El barrido de `pending/`/`done/` devolvía `already_open` antes de Linear. El alta remota mandaba solo la traza, la etiqueta `fix` y un título con el hash. `list_issues` no traía `description`.
4. Mayeuta y Cúmulo suscribían el mismo `System_Fracture_Detected`. Enrich solo comentaba si el input ya traía `tracker_ref`, y un tier cognitivo en redb iba a `AGENT_PHASE` en lugar de la acción.
5. `emit_fracture_for_denial` descartaba el error. `claim_store_pending` salía en silencio si `events.redb` no abría y mantenía el store abierto al lanzar el route.

Decisión: no reinterpretar «sin objeto tracker» como `file`. El auto-hospedado de este repo tiene `provider: linear`; sin slug debe usar ese proveedor.
