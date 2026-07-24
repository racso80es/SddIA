---
context:
- event-routing
- quality-assurance
- ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: sha256:018ec936627329a3e963461009b23fb892bd32ba39f54d51d6cfc6d9dc58ed7f
inputs:
- event_file_path: Ruta relativa al JSON de telemetría en ./.events/telemetry/
minteo_maximo: null
name: radamanto-batch
outputs:
- batch_result: Stats actualizados y acciones dominio emitidas (Self-Healing + Telemetry_Captured)
phases:
- delegates_to:
  - agent:radamanto
  intent: Acumular telemetría CLI, evaluar umbrales, emitir Domain_Entity_{Degraded|Restored|Deprecated} y Domain_Entity_Telemetry_Captured; sellar delivery_state.
  name: Consumo batch Radamanto
  requires_capability:
  - contract: fs.persist
    id: fs:persist
    version: '>=1.0.0'
porcentaje_de_exito: null
uuid: 2a3b4c5d-6e7f-4a8b-9c0d-1e2f3a4b5c6d
version: 1.1.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# radamanto-batch

Proceso laboratorio del agente **Radamanto**: sustituye `telemetry-batch-stub` (Fase 4). Consume `Raw_Execution_Finished`, actualiza acumulador en `.SddIA/radamanto/`, emite gobernanza Self-Healing y **siempre** (consumo OK no duplicado) emite `Domain_Entity_Telemetry_Captured` para ingesta vectorial.

```bash
SDDIA_LAB_ROUTE_SYNC=1 ./sddia-run.sh --process radamanto-batch \
  --inputs '{"event_file_path":".events/telemetry/<id>.json"}'
```

Con `SDDIA_LAB_ROUTE_SYNC=1`, la chispa domain se enruta de inmediato a `memory-evolution-ingest`.
