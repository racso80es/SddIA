---
feature_name: event-sweeper-heartbeat-fracture-f4befd66c513
created: "2026-07-19"
process: bug-fix
branch: fix/event-sweeper-heartbeat-fracture-f4befd66c513
global: APTO
pbi_archived: true
pbi_ref: docs/todos/done/[FIX] event-sweeper — fractura sistémica (f4befd66c513).md
checks:
  - id: CA1
    result: APTO
    evidence: "test baseline_prefers_newer_started_at_on_cold_start — missed < 3"
  - id: CA2
    result: APTO
    evidence: "test baseline_prefers_newer_heartbeat_in_steady_state"
  - id: CA3
    result: APTO
    evidence: "cargo test -p execute-process daemon_heartbeat → 3 ok"
  - id: CA4
    result: APTO
    evidence: "cargo build -p execute-process OK"
  - id: CA5
    result: APTO
    evidence: "PBI en docs/todos/done/; este validacion.md pbi_archived true"
git_changes:
  - SddIA/engine/execute-process/src/engine/handlers/daemon_heartbeat.rs
  - docs/fixes/event-sweeper-heartbeat-fracture-f4befd66c513/
  - docs/todos/done/[FIX] event-sweeper — fractura sistémica (f4befd66c513).md
---

# Validación — event-sweeper-heartbeat-fracture-f4befd66c513

**global: APTO**

Causa raíz: carrera post-arranque en `daemon-heartbeat-audit` (baseline obsoleta). Keepalive de `event-sweeper` no tocado.
