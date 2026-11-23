---
id: aut-117-execution
tracker_ref: AUT-117
process: feature
persist_ref: docs/features/observabilidad-dlq-alertas-kaizen
---

# Evidencia de Ejecución — AUT-117

## Comandos de Verificación Previstos
- `RUSTFLAGS="-Dwarnings" cargo check --workspace`
- `RUSTFLAGS="-Dwarnings" env -u SDDIA_EXECUTE_PROCESS_BIN cargo test -p execute-process --lib dlq_kaizen_alert`
- `RUSTFLAGS="-Dwarnings" cargo test -p sddia-daemon-runtime`
- `RUSTFLAGS="-Dwarnings" cargo test -p event-bus-audit`
- `cargo run -p sddia-qa -- gate-event-manager-port`
- `cargo run -p sddia-qa -- gate-eda-unit-no-real-backend`
- `cargo run -p sddia-qa -- gate-capsule-kind`
- `cargo run -p sddia-qa -- gate-no-env-bypass`
- `cargo run -p sddia-qa -- audit-eda-coverage --scan`
- `./SddIA/target/debug/sddia-qa gate-evolution --range --sync-base --json`
