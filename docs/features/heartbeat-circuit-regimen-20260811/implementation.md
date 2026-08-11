---
feature_name: heartbeat-circuit-regimen-20260811
created: "2026-08-11"
process: refactorization
---

# Implementation — Circuito Daemon_Heartbeat

| Pieza | Cambio |
|-------|--------|
| `sddia-daemon-runtime` | `emit_heartbeat` escribe side-channel (obligatorio) + ECST telemetry (best-effort) |
| `daemon_heartbeat.rs` | `ingest_regime` (side-channel + último HB telemetry) antes de `audit_staleness` |
| `event-sweeper` | Invoca `daemon-heartbeat-audit` sweep cada 30s |
| `event-watcher` | Roots: telemetry → pending → domain → orchestration; HB primero dentro de telemetry |
| 4 centinelas | Crash-Only (`HEARTBEAT_EMIT_FAIL_BUDGET=5`) en worker keepalive |

## Smoke (2026-08-11)

Poison `heartbeat-audit.json` (`missed_cycles=99`, `last_heartbeat` 2026-08-10) + side-channels frescos → `./sddia-run.sh --process daemon-heartbeat-audit --inputs '{"sweep":true}'` → `missed_cycles=0`, `fractures_emitted=[]`.

## Operativa

Reiniciar centinelas (`./start-sddia.sh` o equivalente) para cargar binarios con side-channel. Genoma `daemon-heartbeat-audit.md`: bump documental vía `entity-manager` (no mutado en esta entrega de código).
