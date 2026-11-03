---
feature_name: aut-74-workspace-server-target-path
process: bug-fix
created: "2026-11-03"
pbi_ref: AUT-74
execution_id: "6c811ed9-8c0d-45d5-8801-4386b54821df"
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
