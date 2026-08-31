---
context:
- ecosystem-evolution
- event-routing
contract: process-contract v1.4.0
hash_signature: "sha256:0aa0ee978b64c34f55d8384eb70476a6f734f7220cbcf0b936c238493f6f5e75"
inputs:
- event_file_path: Ruta relativa al JSON Domain_Entity_Telemetry_Captured en ./.events/domain/
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
porcentaje_de_exito: null
uuid: eb50d05d-c8d8-4cb7-a7ed-4d296971cbe2
version: 1.2.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/---

# memory-evolution-ingest

Suscriptor de `Domain_Entity_Telemetry_Captured` (fan-out `route-domain`). Indexa el snapshot de ejecución en el store evolution vía puerto `EvolutionStore` (`{paths.vectorStore}/lancedb/`; adapter `lancedb_evolution_repo`). No persiste JSON SSOT paralelo bajo `.SddIA/vector_store/evolution/`.

Runtime nativo: `memory_evolution_ingest_core` en `execute-process` (despacho fractal + invocación directa del proceso).

```bash
./sddia-run.sh --process memory-evolution-ingest \
  --inputs '{"event_file_path":".events/domain/<event_id>.json"}'
```

Idempotencia por `origin_stimulus.event_id`. Fail-soft hacia Radamanto (no tumba Self-Healing).
