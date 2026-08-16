---
feature_name: process-creator-full-contract-forge
created: "2026-08-16"
process: bug-fix
items_applied:
  - factory-create-yaml-phases
  - hash-on-written-phases
  - entity-manager-propagation
  - cargo-test-forges-factory
  - cargo-test-entity-manager-payload
  - fail-closed-index-write
---

# Execution — EV-AUD-003

1. CREATE persiste `process_phases` reales; hash vía `refresh_process_hash` = mismo algoritmo que `verify-process-integrity`.
2. `entity-manager` reenvía workspace/inputs/outputs/invocations al forge.
3. Tests: `CARGO_TARGET_DIR=SddIA/target cargo test -p execute-process --lib forges::factory -- --test-threads=1` → 12 ok; `process_creator_inputs` → 2 ok.
4. Fixture `ev_aud_003_evolution_audit_fixture_recreates_without_stub` + integrity sin parche.
5. PBI `4f7ff349-…` archivado en esta rama. PR #178.
