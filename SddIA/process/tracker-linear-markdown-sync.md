---
context:
- tracker-operations
contract: process-contract v1.4.0
hash_signature: sha256:4b86a5b79e6fe76f8467511d110a50cd51df209011c6e2431193691f91277690
inputs:
- sync_all: Volcar todos los .md con tracker_ref en pending/done/Realizado (default true)
minteo_maximo: null
name: tracker-linear-markdown-sync
outputs:
- synced_count: Número de issues actualizados
- synced: Detalle por archivo
phases:
- intent: update_issue_description con el provider resuelto de tracker:lifecycle. Con file es no-op y no crea ficheros de backlog.
  name: Volcado descripción Linear
porcentaje_de_exito: null
uuid: a3b4c5d6-e7f8-4890-a123-456789abcd01
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# tracker-linear-markdown-sync

Sincroniza la **descripción** del issue Linear con el contenido íntegro del markdown local (`tracker_ref` en frontmatter). No crea issues. Redacta líneas de secretos conocidos antes del envío.

Inputs adicionales (no declarados en frontmatter; el handler los acepta): `markdown_path` + `issue_ref` para volcado puntual sin escaneo.
