---
feature_name: tracker-source-by-provider
created: "2026-10-25"
process: bug-fix
branch: fix/tracker-source-by-provider
persist_ref: docs/fixes/tracker-source-by-provider
tracker_ref: MAN-21
execution_id: "fd88acb8-0104-4101-875d-aef751fa47e6"
---

# Implementación

`tracker_lifecycle_port::resolve_target` lee `tracker.provider`. `invoke` despacha el verbo: cápsula remota si el provider es `linear`; `file-tracker-adapter` si es `file` o no hay clave.

La implementación de fichero incluye `historias/` en `fetch_issue` y `list_issues`, y devuelve `artifact_path`. El `fetch_issue` remoto pide `description`.

Fractura, Kaizen y CI crónico crean el ítem con `create_issue`. `refine` lee y sella con `fetch_issue` + `update_issue_description`. Con provider remoto no reescribe el markdown local. `tracker-description-sync` deja de escanear buckets: recibe cuerpo o lista por el verbo.
