---
uuid: "8d577a50-055a-40b9-b7e2-93e2d2415796"
name: "event-bus-audit"
version: "1.0.0"
contract: "process-contract v1.4.0"
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
context:
  - "quality-assurance"
  - "event-routing"
hash_signature: sha256:a236607c025cd3a9d3e01ef3371ea0b683071de59e1b8ffb24617261cfa9b7ce
inputs: []
outputs:
  - "audit_summary": "Conteos por estado y familia del bus"
  - "anomalies": "Lista de anomalías detectadas"
  - "report_path": "Ruta del informe Markdown en workspace"
  - "kaizen_event_id": "UUID del evento Kaizen emitido (si aplica)"
phases:
  - name: "Auditoría empírica del bus"
    intent: "Escanear estados DLT y familias fractales; validar ECST; generar informe y Kaizen si procede"
    delegates_to:
      - "tool:event-bus-audit"
minteo_maximo: null
porcentaje_de_exito: null
---

# event-bus-audit

Proceso on-demand de auditoría empírica del bus EDA. Inspecciona `./.events/` (pending, processing, processed, dead-letter y familias fractales telemetry/orchestration/domain), valida coherencia ECST, detecta anomalías (staleness, huérfanos, tipos desconocidos) y genera informe. Emite `Kaizen_Alert_Required` en `eda_bus.pending` cuando hay dead-letters o anomalías estructurales.

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
