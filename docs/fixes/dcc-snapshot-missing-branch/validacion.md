---
fix_name: dcc-snapshot-missing-branch
branch_name: fix/dcc-snapshot-missing-branch
global: APTO
pbi_ref: docs/todos/done/[FIX] delivery-close-cycle — fractura sistémica (969f05933a46).md
fracture_hash: 969f05933a46
---

Causa: `Snapshot final` invocaba `get_last_commit` con `ref` = nombre de rama inexistente localmente (`git rev-parse` ambiguo).

Remedio: `checkout` con `create_if_not_exists: true` antes del snapshot.
