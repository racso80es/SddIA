---
capabilities:
- eda-pending-sweep
- kaizen-dead-letter-alert
- vitality-probe-sweep
context: ecosystem-evolution
contract: daemons-contract v1.0.0
execution:
  entrypoint: SddIA/daemons/event-sweeper.sh
  heartbeat_interval_seconds: 30
  runtime: native-rust
hash_signature: sha256:bda8bb014c269960a8af696f07736a7318cd86dcbb97f21ba7ecf70cb9dc3754
jurisdiction: Aislada — Ceguera Lógica. Solo inyecta eventos físicos en el bus
name: event-sweeper
source_sha256: sha256:9c0a1d4001196f754ac562c1cec4b7036e292fd3624a5504274777d8eaab7ff8
telemetry_provided: true
telemetry_schema:
- uptime_seconds
- pid
- status
uuid: 3eafa012-2b71-47e5-b47e-467b59a3fd52
version: 1.0.0
---

# event-sweeper

Recolector inerte del bus EDA V3+: escanea `.events/pending/` y purga padres con consenso de suscriptores. Emite alerta Kaizen en stderr ante testigos `dead-letter`. Invoca `daemon-heartbeat-audit` (tick 30s) y `system-vitality-probe` (`SDDIA_VITALITY_PROBE_SECONDS`, default 300, piso 30; capability `vitality-probe-sweep`). Binario Rust en `SddIA/target/{release|debug}/event-sweeper`; launcher `SddIA/daemons/event-sweeper.sh`. Emite `Daemon_Heartbeat` cada 30s.
