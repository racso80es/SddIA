---
feature_name: eda-fractal-dlq-c2
created: "2026-07-16"
process: bug-fix
branch: fix/eda-fractal-dlq-c2
global: APTO
pbi_archived: true
checks:
  - id: CA1
    result: APTO
    evidence: "Código: fractal_event_all_ok → safe_remove_path (sin cambio de contrato B)"
  - id: CA2
    result: APTO
    evidence: "Sweeper --once: 2 domain failed → .events/dead-letter/; domain=0"
  - id: CA3
    result: APTO
    evidence: "move_fractal_to_dead_letter / move_fractal_event_to_dead_letter: create_dir_all"
  - id: CA4
    result: APTO
    evidence: "cumulo eda_fractal.dead_letter + load_bus_topology / load_fractal_dead_letter_dir"
git_changes:
  - SddIA/core/cumulo.paths.json
  - SddIA/sddia-daemon-runtime/src/lib.rs
  - SddIA/sddia-daemon-runtime/src/eda_sweep.rs
  - SddIA/engine/execute-process/src/engine/fractal_bus.rs
  - SddIA/engine/execute-process/src/engine/route_fractal_core.rs
  - SddIA/daemons/event-watcher/src/main.rs
  - SddIA/daemons/event-sweeper/src/main.rs
---

# Validación — eda-fractal-dlq-c2

**global: APTO** (lab/smoke local). `pbi_archived` se pondrá `true` al mover PBI a `done/` pre-merge.
