---
feature_name: event-manager-port-cutover
created: "2026-11-08"
process: refactorization
pbi_ref: MAN-35
document_id: PBI-176f9f2f
phase: T8-cierre
---
# Execution — MAN-35

## Tajos ejecutados

| Fase | Commit | Verificación |
|------|--------|--------------|
| T0–T3 | `18fa2048` … `ba60dd80` | `execute-process` motor puerto |
| T4–T6 | `bc23aeec` | fractal_bus + event-bus-audit |
| T7 | `ad9b200f` | `eda_port` productivo |
| T8 | (rama) | `sddia-qa gate-event-manager-port`; CI + docs AC-9 |

## Gate CI

- `SddIA/target/debug/sddia-qa gate-event-manager-port --json`
- Tests: `sddia-core` (event_manager), `sddia-infrastructure-redb-event`, `sddia-event-manager-host`, `event-bus-audit`, `execute-process --lib`

## Comandos locales (T8)

```bash
cd SddIA
cargo build -p sddia-qa
./target/debug/sddia-qa gate-event-manager-port
cargo test -p sddia-core event_manager --lib
cargo test -p event-bus-audit
cargo test -p execute-process --lib
```
