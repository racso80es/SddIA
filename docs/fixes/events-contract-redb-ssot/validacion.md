---
feature_name: events-contract-redb-ssot
created: "2026-10-29"
process: bug-fix
branch: fix/events-contract-redb-ssot
global: APTO
tracker_ref: AUT-45
pbi_archived: false
checks:
  AC-CONTRACT: APTO
  AC-STORE-IDEM: APTO
git_changes:
  - SddIA/events/events-contract.md
  - SddIA/sddia-event-store/src/gate.rs
  - SddIA/sddia-event-store/src/lib.rs
  - SddIA/engine/execute-process/src/engine/eda_bus_topology.rs
  - SddIA/engine/execute-process/tests/event_store_writers.rs
  - docs/fixes/events-contract-redb-ssot/
---

# Validación — AUT-45

- `terminal_witness_exists_redb_without_fs_witness` — OK (`--test-threads=1`).
- Criterio AUT-45: contrato redb + idempotencia store. Pendiente merge PR → Done Linear (`done_gate: linear`).
