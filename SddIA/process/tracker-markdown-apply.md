---
context:
- tracker-operations
- filesystem-ops
- source-control
contract: process-contract v1.4.0
hash_signature: "sha256:80125c4bf6325b71178206e9103334913a9c754f11f4b63d690245700adf0b41"
inputs:
- event_uri: Ruta relativa al JSON ECST Tracker_Issue_Changed en pending
minteo_maximo: null
name: tracker-markdown-apply
outputs:
- applied: boolean
- tracker_ref: Identificador opaco del ítem actualizado
phases:
- delegates_to:
  - skill:git-manager
  intent: Commit en rama tracker-sync/{project_slug}; prohibido default_branch.
  name: Persistencia tracker-sync
  id: persistencia_tracker_sync
porcentaje_de_exito: null
uuid: b7c8d9e0-f123-4567-89ab-cdef01234567
version: 2.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# tracker-markdown-apply

## Propósito

Suscriptor de `Tracker_Issue_Changed`. Handler nativo (sin LLM). La réplica al sumidero la hace la implementación de `tracker:lifecycle`. Fail-soft con `Tracker_Sync_Failed` si divergencia.

## Operación

Invocación vía `./sddia-run.sh --process tracker-markdown-apply`; handler/fases en frontmatter YAML.
