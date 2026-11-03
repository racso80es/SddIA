---




feature_name: aut-74-workspace-server-target-path
process: bug-fix
created: "2026-11-03"
pbi_ref: AUT-74
execution_id: "0d64f24c-1a3c-4165-a31e-fccae85c5ad3"
---
# Ejecución — AUT-74

Rama `fix/aut-74-workspace-server-target-path` · `persist_ref` `docs/fixes/aut-74-workspace-server-target-path`.

## Verificación

```bash
export CARGO_TARGET_DIR=/home/racso/Proyectos/SddIA/SddIA/target
cd SddIA
cargo test -p sddia-workspace-server
cargo test -p filesystem-manager
cargo test -p execute-process --lib resolve_tracker_ref_from_opaque
cargo test -p execute-process --lib work_initiated_stamps_with_opaque
cargo build -p execute-process -p sddia-workspace-server -p filesystem-manager
```

## Sello tracker (reparación ignición)

Tras parche opaco `pbi_ref`: `emit-work-initiated-event` + `tracker-stamp` → AUT-74 **In Progress** (2026-11-03T18:35:53Z).

## Evolution

`sddia-qa evolution-register` → `e57d1f05-b30d-4566-8193-090e2793e9b8` (`EVOL_OK`, `alta`).

## Entrega

PR https://github.com/racso80es/SddIA/pull/434 · `delivery-close-cycle` (Presented emitido; aduana evolution corregida en commit posterior).
