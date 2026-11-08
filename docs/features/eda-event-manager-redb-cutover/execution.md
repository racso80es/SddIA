---




feature_name: eda-event-manager-redb-cutover
persist_ref: docs/features/eda-event-manager-redb-cutover
pbi_ref: MAN-33
execution_id: "6b4f2856-aa17-4a58-b317-c97a6e6c9d49"
---
# Ejecución — MAN-33

## Verificación local (2026-11-07)

```text
export TMPDIR=<repo>/.tmp/cargo-tmp
unset CARGO_TARGET_DIR
cd SddIA
cargo build -p event-watcher -p execute-process -p sddia-qa
cargo build --workspace
cargo test -p sddia-core event_manager
cargo test -p execute-process --test memory_evolution_ingest
sddia-qa verify-tools-index
sddia-qa verify-process-integrity
sddia-qa verify-compiled-capsules
```

Resultado: **ok** (workspace ~9 min; smokes verify ok). `run-iota-ci-smoke --simulate` falló en lab por `tekton.tracker-stamp` (entorno tracker); pendiente reproducir en CI.

Modo datos: **cold-cut** documentado en `implementation.md`.

## Pendiente pre-merge

- Migrar `event-bus-audit`, `repo-retention-purger`, kalma2 + `lancedb_thought_repo` al puerto.
- Retirar `sddia-event-store` del workspace y búsqueda residual AC-3.
- Paridad tests publish→claim→ack en Redb tempdir (AC-4).
