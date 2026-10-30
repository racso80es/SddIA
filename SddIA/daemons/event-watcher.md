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
hash_signature: sha256:9092012a71130071f2fd2525e4ba673d2c37e367d805d5264720d4b576a87a44
jurisdiction: Aislada — Ceguera Lógica. Reclama ids pending del store o ficheros .events/pending y delega el despacho. No abre el ECST. El tick dispara daemon-heartbeat-audit y system-vitality-probe.
name: event-watcher
source_sha256: sha256:e45f47fddf146419f012f4a67250b692f6013ff8aa01f70bc605818966daa93c
telemetry_provided: true
telemetry_schema:
- uptime_seconds
- pid
- status
uuid: f995cc89-22a7-488d-9b25-ddb1e5e3a4a4
version: 1.2.0
---

# event-watcher

Centinela EDA. En filesystem drena `.events/pending/`. En dual o redb reclama pares pending del store y delega `route-domain-event` sin abrir el ECST. El tick dispara `daemon-heartbeat-audit` (`sweep`) y `system-vitality-probe`. Binario Rust en `SddIA/target/{release|debug}/event-watcher`; launcher `SddIA/daemons/event-watcher.sh`. Emite `Daemon_Heartbeat` cada 30s.
