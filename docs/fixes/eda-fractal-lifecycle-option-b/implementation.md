---
feature_name: eda-fractal-lifecycle-option-b
created: "2026-07-16"
process: bug-fix
items:
  - route-fractal-purge-after-domain
  - stamp-delivery-state
  - telegram-ack-first
  - sweeper-fractal
---

# Implementation

| Path | Cambio |
|------|--------|
| `engine/.../route_fractal_core.rs` | `purge_after=true` domain; stamp; skip already-delivered |
| `daemons/telegram-watcher/src/main.rs` | ACK-first + seen + state contrato |
| `daemons/event-watcher/src/main.rs` | skip fractal-terminal / eco |
| `sddia-daemon-runtime/src/eda_sweep.rs` | sweep fractal dirs |
