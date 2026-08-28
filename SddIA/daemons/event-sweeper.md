---
capabilities:
- eda-pending-sweep
- kaizen-dead-letter-alert
context: ecosystem-evolution
contract: daemons-contract v1.0.0
execution:
  entrypoint: SddIA/daemons/event-sweeper.sh
  heartbeat_interval_seconds: 30
  runtime: native-rust
hash_signature: sha256:a03673773e9b6fe243bcdb492532ce4544d7f91f1cebecf21207be2f14dbd9e5
jurisdiction: Aislada — Ceguera Lógica. Solo inyecta eventos físicos en el bus
name: event-sweeper
source_sha256: sha256:72eeed58868c4ed6362da2415ea62d8d82105c00a603c5dd04af8f26fddcf836
telemetry_provided: true
telemetry_schema:
- uptime_seconds
- pid
- status
uuid: 3eafa012-2b71-47e5-b47e-467b59a3fd52
version: 1.0.0
---


# event-sweeper

Recolector inerte del bus EDA V3+: escanea `.events/pending/` y purga padres con consenso de suscriptores. Emite alerta Kaizen en stderr ante testigos `dead-letter`. Binario Rust en `SddIA/target/{release|debug}/event-sweeper`; launcher `SddIA/daemons/event-sweeper.sh`. Emite `Daemon_Heartbeat` cada 30s.
