---
feature_name: pr-merged-witness-store
branch: fix/pr-merged-witness-store
process: bug-fix
global: APTO
pbi_archived: false
tracker_ref: MAN-23
checks:
  AC-1: APTO
  AC-2: APTO
  AC-3: APTO
  AC-4: APTO
git_changes:
  - SddIA/engine/execute-process/src/engine/actions.rs
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/engine/execute-process/src/engine/accept_pr.rs
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - SddIA/engine/execute-process/src/engine/tracker_pbi_meta.rs
  - docs/fixes/pr-merged-witness-store/objectives.md
  - docs/fixes/pr-merged-witness-store/spec.md
  - docs/fixes/pr-merged-witness-store/plan.md
  - docs/fixes/pr-merged-witness-store/implementation.md
  - docs/fixes/pr-merged-witness-store/execution.md
  - docs/fixes/pr-merged-witness-store/validacion.md
---

# Validación

Proveedor `linear`. MAN-23 sigue en Backlog hasta el sello de merge. No hay fichero nuevo en `docs/todos/`.

| Criterio | Estado |
|----------|--------|
| AC-1 | APTO — `merged_alta_inserts_store_row_before_route` |
| AC-2 | APTO — `merged_alta_filesystem_writes_json_and_skips_store` |
| AC-3 | APTO — `promote_failure_keeps_delegation_status` |
| AC-4 | APTO — `objectives_tracker_ref_feeds_the_seal`, `presented_in_processed_still_yields_tracker_ref` |
