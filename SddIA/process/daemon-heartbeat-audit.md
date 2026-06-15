---
uuid: "f45bda9d-40d9-471e-82a1-b9404b5a0dfd"
name: "daemon-heartbeat-audit"
version: "1.0.0"
contract: "process-contract v1.4.0"
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
context:
  - "quality-assurance"
  - "event-routing"
hash_signature: sha256:dde581369442cc46609cd6878a474bc737ab5dce4847ac7bde8fe19f6992d1d6
inputs:
  - "event_file_path": "Opcional; ruta relativa al JSON Daemon_Heartbeat en ./.events/telemetry/"
  - "sweep": "Opcional boolean; si true y sin event_file_path, audita staleness de todos los Centinelas con lock vivo"
outputs:
  - "audit_result": "Estado de la auditoría (audited | sweep | skipped)"
  - "fractures_emitted": "Lista de sellos System_Fracture_Detected emitidos en eda_bus.pending"
  - "daemon_name": "Centinela auditado cuando aplica ingesta por evento"
phases:
  - name: "Ingesta heartbeat"
    intent: "Consumir Daemon_Heartbeat; actualizar registro `heartbeat-audit.json` bajo daemons_instance.state."
    delegates_to:
      - "agent:argos"
      - "skill:filesystem-manager"
  - name: "Auditoría staleness"
    intent: "Cruzar locks vivos vs last_heartbeat; calcular missed_cycles = floor(elapsed / heartbeat_interval_seconds)."
    delegates_to:
      - "agent:argos"
  - name: "Emisión fractura"
    intent: "Si missed_cycles >= 3 emitir System_Fracture_Detected en eda_bus.pending (idempotente por incidente)."
    delegates_to:
      - "skill:bus-operator"
minteo_maximo: null
porcentaje_de_exito: null
---

# daemon-heartbeat-audit

Triaje Argos (CEN-05). Fan-out suscriptor de `Daemon_Heartbeat` en bus fractal telemetry. Emite `System_Fracture_Detected` cuando un Centinela **crítico** (lock/PID vivo) omite **3 ciclos consecutivos** de latido.

```bash
# Sweep manual (sin evento)
python3 SddIA/scripts/qa/execute-process.py --process daemon-heartbeat-audit --inputs '{"sweep":true}'

# Fan-out vía route-telemetry (automático)
SDDIA_LAB_ROUTE_SYNC=1 python3 SddIA/scripts/qa/execute-process.py --process route-telemetry --inputs '{"event_file_path":".events/telemetry/<id>.json"}'
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
