---
context:
- quality-assurance
- event-routing
contract: process-contract v1.4.0
hash_signature: sha256:pending-anchor-on-merge
inputs:
- event_file_path: Opcional; ruta relativa al JSON Daemon_Heartbeat en ./.events/telemetry/
- sweep: Opcional boolean; si true y sin event_file_path, audita staleness de todos los Centinelas con lock vivo
minteo_maximo: null
name: daemon-heartbeat-audit
outputs:
- audit_result: Estado de la auditoría (audited | sweep | skipped)
- fractures_emitted: Lista de sellos System_Fracture_Detected emitidos en eda_bus.pending
- daemon_name: Centinela auditado cuando aplica ingesta por evento
- suspend_reanchored: true si el sweep detectó host_suspend y reancló baseline
- skew_seconds: Delta wall−mono del sweep cuando aplica
- phagocyte: Resultado dry-run/apply de fagocitosis tras ignición sana
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
  intent: Cruzar locks vivos vs last_heartbeat; discriminación suspend/crash vía skew wall/mono; missed_cycles desde SSOT.
  name: Auditoría staleness
- delegates_to:
  - skill:bus-operator
  intent: Si missed_cycles >= umbral SSOT y no host_suspend, emitir System_Fracture_Detected (idempotente).
  name: Emisión fractura
- delegates_to:
  - agent:argos
  intent: Tras sweep sano, ledger fagocitosis; apply solo con SDDIA_PHAGOCYTE_APPLY=1.
  name: Fagocitosis residual
porcentaje_de_exito: null
uuid: f45bda9d-40d9-471e-82a1-b9404b5a0dfd
version: 1.1.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# daemon-heartbeat-audit

Triaje Argos (CEN-05). Fan-out suscriptor de `Daemon_Heartbeat`. Emite `System_Fracture_Detected` solo si no hay `host_suspend` y `missed_cycles >= missed_cycles_threshold` (SSOT).

**Macrófago empírico:** Argos exclusivo. Radamanto excluido (sin medición directa de PID).

```bash
./sddia-run.sh --process daemon-heartbeat-audit --inputs '{"sweep":true}'
```

## Umbrales SSOT

| Capa | Ruta |
|------|------|
| Core | `SddIA/daemons/heartbeat-audit.thresholds.json` (`cumulo.paths.json` → `argos.heartbeat_audit_thresholds`) |
| Overlay | `.SddIA/daemons/state/heartbeat-audit.thresholds.json` |

```json
{ "missed_cycles_threshold": 3, "suspend_skew_seconds": 120 }
```

## Discriminación suspend/crash

`skew = Δwall − Δmono` entre sweeps. Si `skew >= suspend_skew_seconds` → `host_suspend`: reancla `last_heartbeat_at`, `missed_cycles=0`, **sin** fractura.

Estado global en `heartbeat-audit.json`: `last_audit_wall_at`, `last_audit_mono_ms`. Por daemon: `classification` (`healthy` | `host_suspend` | `stale` | `recovered`).

## Fagocitosis

Tras sweep con `fractures_emitted: []` y todos los locks vivos con `missed_cycles=0`, invoca lógica `phagocyte-recovered-fracture-pbis` (ledger siempre; apply con env).

## Handler

`daemon_heartbeat.rs` / `audit_telemetry_file()` / `run_daemon_heartbeat_audit()`.

## Límites

* No arranca ni mata Centinelas.
* Una fractura por incidente hasta heartbeat válido.
* Sin evento `Anomaly_Detected` (reutiliza `Daemon_Heartbeat` + estado auditoría).
