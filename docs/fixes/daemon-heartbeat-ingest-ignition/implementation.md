---
feature_name: daemon-heartbeat-ingest-ignition
created: "2026-07-23"
process: bug-fix
---

# Implementation — daemon-heartbeat-ingest-ignition

| Cambio | Path | Notas |
|--------|------|-------|
| Gate ingest | `start-sddia.sh` | `_ingest_telemetry_heartbeats` + sweep `{"sweep":true}` |
| Schema tool | `SddIA/library/norms/capability-contracts/di.binding.schema.json` | pattern `skill\|action\|tool` |
| Validador DI | `phase_capsules.rs` | valida envelope sddia-io (`success`/`exitCode`) antes de unwrap |
| Orquestador | `SddIA/target/debug/execute-process` | rebuild con `CARGO_TARGET_DIR` real |

Sin mutación de genoma de Centinelas (keepalive intacto). Fairness de `event-watcher` diferida (Kaizen).
