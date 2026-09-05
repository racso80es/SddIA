---
context:
- quality-assurance
- event-routing
contract: process-contract v1.4.0
hash_signature: sha256:f5cb7744f3bf4d32c6013f8fd2376b1c9fbb1841d18a90578c40783018188a3b
inputs: []
minteo_maximo: null
name: event-bus-audit
outputs:
- audit_summary: Conteos por estado y familia del bus
- anomalies: Lista de anomalías detectadas
- report_path: Ruta del informe Markdown en workspace
- kaizen_event_id: UUID del evento Kaizen emitido (si aplica)
phases:
- delegates_to:
  - tool:event-bus-audit
  intent: Escanear estados DLT y familias fractales; validar ECST; generar informe y Kaizen si procede
  name: Auditoría empírica del bus
  requires_capability:
  - contract: qa.probe
    id: qa:probe
    version: '>=1.0.0'
porcentaje_de_exito: null
uuid: 8d577a50-055a-40b9-b7e2-93e2d2415796
version: 1.0.2
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# event-bus-audit

Proceso on-demand de auditoría empírica del bus EDA. Inspecciona `./.events/` (pending, processing, processed, dead-letter y familias fractales telemetry/orchestration/domain), valida coherencia ECST, detecta anomalías (staleness, huérfanos, tipos desconocidos) y genera informe. Emite `Kaizen_Alert_Required` en `eda_bus.pending` solo si `circuit_alert` o hay pending estancados que no sean `System_Fracture_Detected`. El volumen histórico de dead-letter no dispara Kaizen.

```bash
# Auditoría completa con emisión Kaizen (default)
./sddia-run.sh --process event-bus-audit --inputs '{}'

# Solo informe, sin Kaizen
./sddia-run.sh --process event-bus-audit --inputs '{"emit_kaizen_alert":false}'

# Umbral de staleness personalizado (48h)
./sddia-run.sh --process event-bus-audit --inputs '{"stale_threshold_hours":48}'
```

## Handler

Cápsula Rust `SddIA/tools/event-bus-audit/` invocada vía `tool:event-bus-audit` en fase única.
