---
context: ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: sha256:91e1248c355fadc7fadd8e26c47fd75422a98ba44425c46b6cec116bf6e4355f
inputs:
- 'event_uri: Token lógico EventUri resuelto por EventManagerPort (eda://{family}/{event_id})'
minteo_maximo: null
name: memory-evolution-ingest
outputs:
- ingest_result: Registro EvolutionEvent persistido o skip idempotente
phases:
- delegates_to:
  - agent:cumulo
  intent: Leer evento domain Telemetry_Captured; capturar EvolutionEvent; persistir vía puerto EvolutionStore en {paths.vectorStore}/lancedb/; sellar delivery_state.
  name: Ingesta evolution
  requires_capability:
  - contract: fs.persist
    id: fs:persist
    version: '>=1.0.0'
  id: ingesta_evolution
porcentaje_de_exito: null
uuid: eb50d05d-c8d8-4cb7-a7ed-4d296971cbe2
version: 2.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# memory-evolution-ingest

Suscriptor de `Domain_Entity_Telemetry_Captured` (fan-out `route-domain`). Indexa el snapshot de ejecución en el store evolution vía puerto DI `EvolutionIngestPort` → `tool:lancedb-memory-adapter` (`{paths.lancedbStore}`; adapter `lancedb_evolution_repo`). No persiste JSON SSOT paralelo bajo `.SddIA/vector_store/evolution/`.

Runtime nativo: `memory_evolution_ingest_core` en `execute-process` (despacho fractal + invocación directa del proceso).

```bash
./sddia-run.sh --process memory-evolution-ingest \
  --inputs '{"event_uri":"eda://domain/<event_id>"}'
```

Idempotencia por `origin_stimulus.event_id`. Fail-soft hacia Radamanto (no tumba Self-Healing).
