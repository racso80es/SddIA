---
context: ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: "sha256:e00473f267780ea7726ae5a90f4678fa090e5925541f5656c86ae71d8903fc7a"
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
uuid: e7f09165-c445-49ae-965d-41abb4738679
version: 1.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# compile-ecosystem-map-snapshot

Precompila inventario esperado (tools/skills/daemons) en map-snapshot.json para el Espejo de Consciencia.
