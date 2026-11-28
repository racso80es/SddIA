---
context: quality-assurance
contract: process-contract v1.4.0
hash_signature: "sha256:53c6fcafebe88740a46c870aa8d97a5ce26e7059a8c25b385131e4c19b524564"
inputs:
- description: Boolean; default true — escribe ecosystem-health.json.
  name: persist
- description: Boolean; default false — recompila map-snapshot antes de fusionar (solo CLI/seed).
  name: compile_map
name: query-ecosystem-health
outputs:
- description: Matriz de salud fusionada
  name: rows
- description: Ruta relativa del artefacto persistido
  name: ecosystem_health_path
phases:
- delegates_to:
  - agent:argos
  intent: Cruzar map-snapshot con heartbeat-audit, stats y revoked_entities; emitir Read Model.
  name: Fusionar salud
  requires_capability:
  - contract: fs.persist
    id: fs:persist
    version: '>=1.0.0'
  id: fusionar_salud
uuid: 2b337302-e794-46b8-ad4e-f65bafd21c94
version: 1.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# query-ecosystem-health

## Propósito

Producir un read model JSON de salud del ecosistema fusionando `compile-ecosystem-map-snapshot`, auditoría de heartbeats, estadísticas EDA y entidades revocadas. Entradas opcionales `persist` (default true) y `compile_map` (recompila snapshot antes de fusionar).

## Orquestación

1. Fase `Fusionar salud` delega en `agent:argos` con capacidad `fs:persist`.
2. Cruza map-snapshot con `heartbeat-audit`, stats de store y `revoked_entities`.
3. Salidas: `rows` (matriz de salud) y `ecosystem_health_path` (ruta relativa del artefacto persistido, típicamente `ecosystem-health.json`).

## Consumo

Proceso QA consumible desde CLI/Kalma2 para paneles Espejo sin invocar procesos de dominio individuales.
