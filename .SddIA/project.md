---
id: sddia
uuid: "081eef9b-1b69-42d0-9bb7-c118fd568a02"
git_remote: https://github.com/racso80es/SddIA.git
default_branch: main
delivery_mode: branch_pr
contract_version: "1.3.0"
codex_slug: codex-software-engineering
docs_layout:
  features: docs/features
  fixes: docs/fixes
  todos_pending: docs/todos/pending
  todos_done: docs/todos/done
env_ref: .SddIA/.dev/.env
tracker:
  provider: linear
  team_key: OSC
  project_id: P-OSC-1
  state_map:
    backlog: Backlog
    in_progress: "In Progress"
    in_review: "In Review"
    done: Done
    cancelled: Canceled
  labels:
    hu: hu
    pbi: pbi
---

# Manifiesto — SddIA

Proyecto forja del ecosistema. Tracker Linear acotado al proyecto **P-OSC-1** (`tracker.project_id`). Token en bóveda `env_ref` (`LINEAR_API_TOKEN`). No usar `LINEAR_PROJECT` en `.env` para configuración: el SSOT es este manifiesto.
