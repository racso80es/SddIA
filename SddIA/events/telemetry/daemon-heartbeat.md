---
uuid: "9c5190ac-ac8a-46b6-b61d-67d45ff7caf1"
name: "daemon-heartbeat"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "telemetry"
event_type: "Daemon_Heartbeat"
context: "system-operations"
capabilities:
  - "daemon_heartbeat"
hash_signature: "sha256:0ab6667f40161abe203828f48ec778f765f30e226707f5e4dd4c5c96e5cf9c72"
---

# Event: Daemon_Heartbeat

Telemetría vital periódica emitida por Centinelas periféricos.

## Payload ECST

### REQUIRED
- `daemon_name`
- `daemon_uuid`
- `pid`
- `uptime_seconds`
- `status`

### OPTIONAL
- `last_stimulus_at`

### FORBIDDEN
- *(ninguno)*

## Emisores autorizados

- `event-watcher`
- `event-sweeper`
- `telegram-watcher`
- `github-bridge-watcher`
- `email-watcher`
- `iota-publish-relay`
- `kalma2-bridge`

## Suscripciones

Ver `SddIA/core/event-telemetry-subscriptions.json` → clave `Daemon_Heartbeat` → fan-out `daemon-heartbeat-audit` (Argos).
