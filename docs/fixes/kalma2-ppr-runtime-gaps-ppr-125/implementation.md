---
feature_name: kalma2-ppr-runtime-gaps-ppr-125
created: "2026-07-22"
process: bug-fix
branch_name: fix/kalma2-ppr-runtime-gaps-ppr-125
persist_ref: docs/fixes/kalma2-ppr-runtime-gaps-ppr-125
agents: tekton
uuid: 0a24332e-e120-480a-87eb-ec9854d27aaa
---

# Implementation — Kalma2 PPR runtime gaps

| # | Cambio | Estado |
|---|--------|--------|
| 1 | `engine/pull_request_review.rs` — Prep / Triaje técnico / Handoff | done |
| 2 | `residual_runner.rs` — wire proceso `pull-request-review` | done |
| 3 | `route_domain_core.rs` — `branch_name` alias | done |
| 4 | `agent_runtime.rs` — coalesce + test | done |
| 5 | `kalma2-agent-runtime-cursor.py` — branch + KM rules | done |
| 6 | Tests unitarios G4/handoff | done |
