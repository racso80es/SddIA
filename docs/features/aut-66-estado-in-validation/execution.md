---
feature_name: aut-66-estado-in-validation
process: refactorization
created: "2026-11-03"
pbi_ref: AUT-66
execution_id: "impl-aut-66-20261103"
---

# Ejecución — AUT-66

## Verificación local

```bash
export CARGO_TARGET_DIR=/home/racso/Proyectos/SddIA/SddIA/target
cargo test -p execute-process \
  tracker_sync_replay::tests \
  tracker_stamp::tests::stamp_validation_started_in_lab \
  workspace_init::tests::maybe_emit_validation_started \
  actions::tests::emit_validation_started_writes_pending \
  -- --nocapture
```

## Criterio de aceptación

1. Tras Tekton `executed`, una sola instancia `Validation_Started` en pending (fail-soft).
2. `tracker-stamp` mueve a `in_validation` sin retroceder desde `in_review`/`done`.
3. `PullRequest_Presented` sigue avanzando a `in_review`.
