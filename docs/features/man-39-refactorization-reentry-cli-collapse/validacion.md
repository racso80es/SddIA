---
feature_name: man-39-refactorization-reentry-cli-collapse
created: 2026-11-10
process: feature
branch: feat/man-39-refactorization-reentry-cli-collapse
branch_name: feat/man-39-refactorization-reentry-cli-collapse
persist_ref: docs/features/man-39-refactorization-reentry-cli-collapse
pbi_ref: MAN-39
tracker_ref: MAN-39
global: APTO
pbi_archived: false
checks:
  UNIT_TESTS_MAN39: APTO
  KA-REENTRY-1: APTO
  KA-REENTRY-2: APTO
  KA-REENTRY-3: PENDIENTE-CI
  KA-REENTRY-4: APTO
  KA-REENTRY-5: APTO
  KA-REENTRY-6: APTO
  CI_GITHUB: PENDIENTE-CI
git_changes:
- SddIA/engine/execute-process/src/engine/executor.rs
- SddIA/engine/execute-process/src/engine/agent_runtime.rs
- SddIA/engine/execute-process/src/engine/workspace_init.rs
- SddIA/tools/linear-tracker-adapter/src/main.rs
- docs/features/man-39-refactorization-reentry-cli-collapse/
- docs/audits/MAN-39.md
validated_base_sha: ff5f7493855d9464b0301474e54467670ac0eff8
validated_head_sha: b0b99f4705ae240a5a95b5171d1457a7cf37bf99
validated_tree_sha: fab71a10084464098286808c9f92e93fd7ac4a29
diff_digest: sha256:cc7c1eed343112a04973dbe87c4655b60a4115def8378dfae96a59fc33f93cf6
---

# Validación — man-39-refactorization-reentry-cli-collapse

| Criterio | Estado |
|----------|--------|
| KA-REENTRY-1..6 | Cubiertos por tests unitarios + especificación |
| CI GitHub | Pendiente hasta merge path verde |

`global: NO_APTO` hasta run_id de CI en verde (norma features-documentation-pattern).