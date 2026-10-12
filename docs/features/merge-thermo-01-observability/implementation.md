---
feature_name: merge-thermo-01-observability
created: "2026-10-03"
process: feature
items:
  - execution_workspace_report.rs
  - executor.rs
  - residual_runner.rs
  - workspace_prune.rs
  - hook_common.sh
execution_id: "b2fc9bbf-2ee0-4806-95c4-f618a94b4e42"
---
# Implementación

| Componente | Ruta |
|------------|------|
| Informe por fase | `SddIA/engine/execute-process/src/engine/execution_workspace_report.rs` |
| Motor genérico | `SddIA/engine/execute-process/src/engine/executor.rs` |
| Motor residual (PRR, accept-pr, …) | `SddIA/engine/execute-process/src/engine/residual_runner.rs` |
| Purga workspaces | `SddIA/tools/sddia-qa/src/workspace_prune.rs` |
| Timings hook | `SddIA/scripts/qa/git-hooks/hook_common.sh`, `pre_push_gate.sh`, `post_merge_gate.sh` |

## Comandos

```bash
cd SddIA && cargo test -p execute-process execution_workspace_report
cd SddIA && cargo test -p sddia-qa workspace_prune
cargo run -p sddia-qa -- workspace-prune --empty --json
```
