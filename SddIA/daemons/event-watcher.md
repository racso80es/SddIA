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
hash_signature: sha256:2477e5bcb6b1a8aab3282a95d2851b9f9024aa5119fa0bf0e84ed2bbda2c49ef
jurisdiction: Aislada — Ceguera Lógica. Reclama pending (claim_store_pending) y fractal (claim_store_fractal) del store o ficheros .events/pending. En redb no enruta JSON residual (F-EDA-RESIDUAL-FILESYSTEM / F-EDA-RESIDUAL-FRACTAL). Side-channel latidos en .SddIA/daemons/state/heartbeats/ sin fila ECST. No abre el ECST en el claim.
name: event-watcher
source_sha256: sha256:b82b7486df46bfd56c55a24f1eea22499364fa5b29a5c4a1ac4dbf0ce3610c2f
telemetry_provided: true
telemetry_schema:
- uptime_seconds
- pid
- status
uuid: f995cc89-22a7-488d-9b25-ddb1e5e3a4a4
version: 1.2.1
---

# event-watcher

Centinela EDA. En `filesystem` drena `.events/pending/`. En `dual` o `redb` reclama entregas pending y familia fractal desde `events.redb` y delega route-* con path lógico sin exigir fichero. El tick dispara `daemon-heartbeat-audit` (`sweep`) y `system-vitality-probe`. Binario Rust en `SddIA/target/{release|debug}/event-watcher`; launcher `SddIA/daemons/event-watcher.sh`. Side-channel de latido cada 30s (no fila ECST en store).
