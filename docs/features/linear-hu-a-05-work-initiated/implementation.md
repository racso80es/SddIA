---
feature_name: linear-hu-a-05-work-initiated
created: "2026-10-04"
process: feature
---

# Implementación

Emisión movida a `SddIA/engine/execute-process/src/engine/workspace_init.rs` (`maybe_emit_work_initiated_on_tekton_entry`) invocada desde `executor.rs` y `residual_runner.rs`.

Verificación: `cargo test -p execute-process work_initiated --lib`.
