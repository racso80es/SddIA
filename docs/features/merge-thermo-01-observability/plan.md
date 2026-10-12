---
feature_name: merge-thermo-01-observability
created: "2026-10-03"
process: feature
phases:
  - motor-informe-fase
  - workspace-prune
  - hook-timings-jsonl
  - baseline-validacion
execution_id: "b2fc9bbf-2ee0-4806-95c4-f618a94b4e42"
---
# Plan

1. Módulo `execution_workspace_report.rs` + cableado en `executor` / `residual_runner`.
2. Subcomando `workspace-prune` en `sddia-qa`.
3. Helpers `hook_timing_*` en `hook_common.sh`; flush en gates.
4. Baseline documentado en `validacion.md`.
