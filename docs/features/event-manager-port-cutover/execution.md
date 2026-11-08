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
| T0–T3 | `18fa2048` | `cargo build -p execute-process` |
| T1 | `1e99a3bb` | contract suite mock + Redb |
| T4 | `2ec050ed` | DLT helpers + suite 1.2 |
| T5 | (pendiente push) | `cargo build -p kalma2-bridge` |
| T6 | (pendiente push) | `cargo build -p event-bus-audit` + purger |
| T7 | (pendiente push) | productores sin `EventStore::open` |
| T8 | (pendiente push) | tests paquetes núcleo |

## Gate CI

`PENDIENTE-CI` — pendiente push PR `refactor/event-manager-port-cutover`.

## Comandos locales (T8)

```bash
cd SddIA
cargo test -p sddia-core
cargo test -p sddia-infrastructure-redb-event
cargo test -p sddia-event-manager-host
cargo test -p execute-process --lib
cargo test -p kalma2-bridge progress_stream
```
