---
context:
- tracker-operations
contract: process-contract v1.4.0
hash_signature: sha256:3a22a119a059ae4d84a1a2e960fe042708d10bb5f4e8b42f30b8dd3bdb2a2e81
inputs:
- sync_all: Volcar los ítems con tracker_ref que devuelve el proveedor (default true)
- tracker_ref: Identificador opaco de tracker:lifecycle
- body: Cuerpo ya cargado por el proveedor
minteo_maximo: null
name: tracker-description-sync
outputs:
- synced_count: Número de ítems actualizados
- synced: Detalle por ítem
phases:
- intent: update_issue_description vía tracker:lifecycle. Recibe la referencia opaca y el cuerpo ya cargado por el proveedor. No escanea buckets.
  name: Volcado de descripción
  id: volcado_de_descripcion
porcentaje_de_exito: null
uuid: a3b4c5d6-e7f8-4890-a123-456789abcd01
version: 1.0.3
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# tracker-description-sync

`update_issue_description` de `tracker:lifecycle` con `tracker_ref` opaco y el cuerpo que el proveedor ya cargó. No crea ítems. Redacta líneas de secretos conocidos antes del envío.
