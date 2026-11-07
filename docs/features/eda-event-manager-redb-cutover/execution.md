---
feature_name: eda-event-manager-redb-cutover
persist_ref: docs/features/eda-event-manager-redb-cutover
pbi_ref: MAN-33
---

# Ejecución — MAN-33

## Verificación local (2026-11-07)

```text
cargo check -p execute-process
cargo check -p event-watcher -p sddia-daemon-runtime
cargo test -p sddia-core -p sddia-event-manager-host -p sddia-infrastructure-redb-event
cargo build -p execute-process -p event-watcher --release
```

Modo datos: **cold-cut** documentado en `implementation.md`.

## Pendiente pre-merge

- Migrar tools + kalma2 + lancedb_thought_repo al puerto.
- Retirar `sddia-event-store` del workspace y búsqueda residual AC-3.
- Paridad tests publish→claim→ack en Redb tempdir (AC-4).
