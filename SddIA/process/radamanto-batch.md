---
uuid: "2a3b4c5d-6e7f-4a8b-9c0d-1e2f3a4b5c6d"
name: radamanto-batch
version: "1.1.0"
contract: process-contract v1.4.0
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
context:
- event-routing
- quality-assurance
- ecosystem-evolution
hash_signature: sha256:e7b2573427f71385c647f7a4469edd3c6bdb908cb15fef2ca81c4f80434e4108
inputs:
- event_file_path: Ruta relativa al JSON de telemetría en ./.events/telemetry/
outputs:
- batch_result: Stats actualizados y acciones dominio emitidas (Self-Healing + Telemetry_Captured)
phases:
- name: Consumo batch Radamanto
  intent: Acumular telemetría CLI, evaluar umbrales, emitir Domain_Entity_{Degraded|Restored|Deprecated} y Domain_Entity_Telemetry_Captured; sellar delivery_state.
  delegates_to:
  - agent:radamanto
minteo_maximo: null
porcentaje_de_exito: null
---

# radamanto-batch

Proceso laboratorio del agente **Radamanto**: sustituye `telemetry-batch-stub` (Fase 4). Consume `Raw_Execution_Finished`, actualiza acumulador en `.SddIA/radamanto/`, emite gobernanza Self-Healing y **siempre** (consumo OK no duplicado) emite `Domain_Entity_Telemetry_Captured` para ingesta vectorial.

```bash
SDDIA_LAB_ROUTE_SYNC=1 ./sddia-run.sh --process radamanto-batch \
  --inputs '{"event_file_path":".events/telemetry/<id>.json"}'
```

Con `SDDIA_LAB_ROUTE_SYNC=1`, la chispa domain se enruta de inmediato a `memory-evolution-ingest`.
