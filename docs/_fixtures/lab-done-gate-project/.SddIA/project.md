---
id: lab-done-gate
uuid: "a1b2c3d4-e5f6-4789-a012-3456789abcde"
git_remote: https://example.invalid/lab-done-gate.git
default_branch: main
delivery_mode: branch_pr
contract_version: "1.3.0"
codex_slug: codex-software-engineering
tracker:
  provider: linear
  team_key: LAB
  done_gate: linear
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

# lab-done-gate fixture
