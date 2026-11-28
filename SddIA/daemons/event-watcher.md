---
capabilities:
- eda-bus-watch
- route-domain-event-delegate
context: ecosystem-evolution
contract: daemons-contract v1.0.0
execution:
  entrypoint: SddIA/daemons/event-watcher.sh
  heartbeat_interval_seconds: 30
  runtime: native-rust
hash_signature: "sha256:d34f832bc1315f4dd5d4b601b8fb490d9ebe6ef687d32a1eaddddc4a0661169e"
jurisdiction: Aislada — Ceguera Lógica. Reclama pending y fractal vía EventManagerPort. Side-channel latidos en .SddIA/daemons/state/heartbeats/ sin fila ECST. No abre el ECST en el claim.
name: event-watcher
source_sha256: sha256:2c5d89976b9227cfcc73501788c6daeb0adb220e4099d8ae1d47ccae2f8783f4
telemetry_provided: true
telemetry_schema:
- uptime_seconds
- pid
- status
uuid: f995cc89-22a7-488d-9b25-ddb1e5e3a4a4
version: 1.2.1
---

# event-watcher

## Propósito

Centinela EDA. En modo puerto (`EventManagerPort`) reclama entregas pending y familia fractal desde `events.redb` y delega route-* con path lógico sin exigir fichero. El tick dispara `daemon-heartbeat-audit` (`sweep`) y `system-vitality-probe`. Binario Rust en `SddIA/target/{release|debug}/event-watcher`; launcher `SddIA/daemons/event-watcher.sh`. Side-channel de latido cada 30s (no fila ECST en store).

## Ciclo de ejecución

Latido/tick vía entrypoint en frontmatter; binario nativo en `SddIA/target/release/<daemon>`.
