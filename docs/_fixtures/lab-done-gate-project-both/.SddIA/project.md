---
id: lab-done-gate-both
uuid: "b2c3d4e5-f6a7-4890-b123-456789abcdef"
git_remote: https://example.invalid/lab-done-gate-both.git
default_branch: main
delivery_mode: branch_pr
contract_version: "1.3.0"
codex_slug: codex-software-engineering
tracker:
  provider: linear
  team_key: LAB
  done_gate: both
  state_map:
    backlog: Backlog
    todo: Todo
    in_progress: In Progress
    in_review: In Review
    done: Done
    cancelled: Cancelled
  labels:
    hu: hu
    pbi: pbi
    editable: sddia-editable
docs_layout:
  features: docs/features
  fixes: docs/fixes
  todos_pending: docs/todos/pending
  todos_done: docs/todos/done
---

# lab-done-gate-both fixture
