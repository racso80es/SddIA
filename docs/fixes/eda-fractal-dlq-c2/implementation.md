---
feature_name: eda-fractal-dlq-c2
created: "2026-07-16"
process: bug-fix
---

# Implementation

| Archivo | Cambio |
|---------|--------|
| `SddIA/core/cumulo.paths.json` | `eda_fractal.dead_letter` → `./.events/dead-letter` |
| `SddIA/sddia-daemon-runtime/src/lib.rs` | `BusTopology.dead_letter`; carga fractal>bus; `ensure_*` mkdir |
| `SddIA/sddia-daemon-runtime/src/eda_sweep.rs` | all-ok unlink; terminal-with-failure → rename DLQ |
| `SddIA/engine/execute-process/src/engine/fractal_bus.rs` | helpers DLQ + resolución SSOT |
| `SddIA/engine/execute-process/src/engine/route_fractal_core.rs` | `purge_after` + failed → move DLQ |
| `SddIA/daemons/event-watcher/src/main.rs` | terminal incluye `failed` |
| `SddIA/daemons/event-sweeper/src/main.rs` | report `dead_lettered` |
