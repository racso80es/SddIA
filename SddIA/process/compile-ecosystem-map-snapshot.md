---
context: ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: "sha256:d25cb92a290e77a4d264ca2d78e53ca9741eac0aa50fbf2e34162d0c67ad1c44"
inputs:
- description: Opcional; reservado para recompilación explícita.
  name: force
name: compile-ecosystem-map-snapshot
outputs:
- description: Ruta relativa del artefacto map-snapshot.json
  name: map_snapshot_path
- description: Cuerpo JSON del snapshot compilado
  name: snapshot
phases:
- delegates_to:
  - agent:cumulo
  intent: Leer índices tools/skills/daemons vía Cúmulo y persistir map-snapshot.json bajo observability.
  name: Compilar mapa
  requires_capability:
  - contract: fs.persist
    id: fs:persist
    version: '>=1.0.0'
  id: compilar_mapa
uuid: e7f09165-c445-49ae-965d-41abb4738679
version: 1.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# compile-ecosystem-map-snapshot

Precompila inventario esperado (tools/skills/daemons) en map-snapshot.json para el Espejo de Consciencia.

## Propósito

Precompila inventario esperado (tools/skills/daemons) en map-snapshot.json para el Espejo de Consciencia.

## Operación

Ejecución conforme al contrato de familia y topología Cúmulo; delegación en cápsulas autorizadas sin rutas cableadas.
