---

feature_name: man-38-redb-port-lock
process: bug-fix
phase: implementation
tracker_ref: MAN-38
persist_ref: docs/fixes/man-38-redb-port-lock
branch_name: fix/man-38-redb-port-lock
execution_id: "be63c70f-633c-452e-aaf1-8ffa645fd54b"
---
# Implementación — man-38-redb-port-lock

## Touchpoints (T1–T3)

| Área | Cambio |
|------|--------|
| `sddia-event-manager-host/src/lib.rs` | `with_productive_port`, telemetría `open_contention`, test CA-6 |
| `execute-process/.../eda_port.rs` | Productivo scoped; mock intacto; `with_event_port`; `bootstrap_*` no-op; `probe_productive_handshake` |
| `execute-process/src/main.rs`, `engine/mod.rs` | Sin `bootstrap_composition_root` al arrancar |
| `event_store_query.rs`, `eda_bus.rs`, `eda_bus_topology.rs` | Consultas vía `with_event_port` |
| `handlers/system_vitality.rs` | Handshake scoped en probe topología |
| `daemons/event-watcher/src/main.rs` | Sin Arc de proceso; claim/ACK scoped en tick e hilos |
| T4 | `sddia-daemon-runtime`, `kalma2-bridge/eda_productive`, `event-bus-audit`, `repo-retention-purger`, `sddia-qa/iota_ci_smoke`, `lancedb_thought_repo`, `actions` tests |

## Laudos

- C1–C2: composition lazy + sin cache productiva en thread-local.
- C3: watcher no retiene Redb durante `invoke_route_process`.
- C5: `port-lifetime.md` + telemetría host.

## Verificación mínima

```bash
cd SddIA && cargo test -p sddia-event-manager-host --lib
cd SddIA && cargo test -p execute-process scoped_tests --lib -- --test-threads=1
cd SddIA && cargo test -p event-watcher -- --test-threads=1
```
