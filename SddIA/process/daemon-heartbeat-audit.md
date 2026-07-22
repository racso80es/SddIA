---
context:
- quality-assurance
- event-routing
contract: process-contract v1.4.0
hash_signature: sha256:9ac827bb49dee0b4bc6d0f44e81ee1538ad1eda01f2027563d836865d9c4a849
inputs:
- event_file_path: Opcional; ruta relativa al JSON Daemon_Heartbeat en ./.events/telemetry/
- sweep: Opcional boolean; si true y sin event_file_path, audita staleness de todos los Centinelas con lock vivo
minteo_maximo: null
name: daemon-heartbeat-audit
outputs:
- audit_result: Estado de la auditoría (audited | sweep | skipped)
- fractures_emitted: Lista de sellos System_Fracture_Detected emitidos en eda_bus.pending
- daemon_name: Centinela auditado cuando aplica ingesta por evento
phases:
- delegates_to:
  - agent:argos
  intent: Consumir Daemon_Heartbeat; actualizar registro `heartbeat-audit.json` bajo daemons_instance.state.
  name: Ingesta heartbeat
  requires_capability:
  - contract: fs.persist
    id: fs:persist
    version: '>=1.0.0'
- delegates_to:
  - agent:argos
  intent: Cruzar locks vivos vs last_heartbeat; calcular missed_cycles = floor(elapsed / heartbeat_interval_seconds).
  name: Auditoría staleness
- delegates_to:
  - skill:bus-operator
  intent: Si missed_cycles >= 3 emitir System_Fracture_Detected en eda_bus.pending (idempotente por incidente).
  name: Emisión fractura
porcentaje_de_exito: null
uuid: f45bda9d-40d9-471e-82a1-b9404b5a0dfd
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# daemon-heartbeat-audit

Triaje Argos (CEN-05). Fan-out suscriptor de `Daemon_Heartbeat` en bus fractal telemetry. Emite `System_Fracture_Detected` cuando un Centinela **crítico** (lock/PID vivo) omite **3 ciclos consecutivos** de latido.

```bash
# Sweep manual (sin evento)
./sddia-run.sh --process daemon-heartbeat-audit --inputs '{"sweep":true}'

# Fan-out vía route-telemetry (automático)
SDDIA_LAB_ROUTE_SYNC=1 ./sddia-run.sh --process route-telemetry --inputs '{"event_file_path":".events/telemetry/<id>.json"}'
```

## Umbral

```text
missed_cycles = floor((now - last_heartbeat_at) / heartbeat_interval_seconds)
fractura si missed_cycles >= 3
```

Si nunca hubo heartbeat pero el lock está vivo, baseline = `started_at` del lock.

## Estado

`.SddIA/daemons/state/heartbeat-audit.json` — registro por `daemon_id` de `last_heartbeat_at`, `missed_cycles`, `fracture_event_id`.

## Handler laboratorio

`daemon_heartbeat_audit_core.audit_telemetry_file()` / `run_daemon_heartbeat_audit()`.

## Límites

* No arranca ni mata Centinelas (eso es `governance-daemon-manager` / `daemon-kill-switch`).
* Una fractura por incidente hasta que llegue un heartbeat válido (reset).
