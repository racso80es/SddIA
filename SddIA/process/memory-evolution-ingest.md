---
uuid: "eb50d05d-c8d8-4cb7-a7ed-4d296971cbe2"
name: "memory-evolution-ingest"
version: "1.1.0"
contract: "process-contract v1.4.0"
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
context:
  - "ecosystem-evolution"
  - "event-routing"
hash_signature: "sha256:b6751f5f70d23638899528703fe3db0e72ab5be3e5727c7b39953417c68e0877"
inputs:
  - "event_file_path": "Ruta relativa al JSON Domain_Entity_Telemetry_Captured en ./.events/domain/"
outputs:
  - "ingest_result": "Registro EvolutionEvent persistido o skip idempotente"
phases:
  - name: "Ingesta evolution"
    intent: "Leer evento domain Telemetry_Captured; capturar EvolutionEvent; persistir en .SddIA/vector_store/evolution/; sellar delivery_state."
    delegates_to:
      - "agent:cumulo"
minteo_maximo: null
porcentaje_de_exito: null
---

# memory-evolution-ingest

Suscriptor de `Domain_Entity_Telemetry_Captured` (fan-out `route-domain`). Indexa el snapshot de ejecución en el store evolution (JSON durable bajo `.SddIA/vector_store/evolution/`; adapter `lancedb_evolution_repo`).

Runtime nativo: `memory_evolution_ingest_core` en `execute-process` (despacho fractal + invocación directa del proceso).

```bash
./sddia-run.sh --process memory-evolution-ingest \
  --inputs '{"event_file_path":".events/domain/<event_id>.json"}'
```

Idempotencia por `origin_stimulus.event_id`. Fail-soft hacia Radamanto (no tumba Self-Healing).
