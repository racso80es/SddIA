---
context:
- tracker-operations
contract: process-contract v1.4.0
hash_signature: "sha256:8b1aa7e9b1134da18a9acf003cb009dbe4fc266f84e469c903c5d84ecfd0edda"
inputs:
- sync_all: Volcar todos los .md con tracker_ref en pending/done/Realizado (default true)
minteo_maximo: null
name: tracker-linear-markdown-sync
outputs:
- synced_count: Número de issues actualizados
- synced: Detalle por archivo
phases:
- delegates_to:
  - tool:linear-tracker-adapter
  intent: update_issue_description con cuerpo completo del markdown (secretos redactados).
  name: Volcado descripción Linear
porcentaje_de_exito: null
uuid: a3b4c5d6-e7f8-4890-a123-456789abcd01
version: 1.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# tracker-linear-markdown-sync

Sincroniza la **descripción** del issue Linear con el contenido íntegro del markdown local (`tracker_ref` en frontmatter). No crea issues. Redacta líneas de secretos conocidos antes del envío.

Inputs adicionales (no declarados en frontmatter; el handler los acepta): `markdown_path` + `issue_ref` para volcado puntual sin escaneo.
