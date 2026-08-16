---
feature_name: process-creator-full-contract-forge
created: "2026-08-16"
process: bug-fix
items:
  - create-persists-process-phases
  - hash-written-phases-integrity
  - entity-manager-payload-propagation
  - evolution-audit-fixture
  - fail-closed-index
---

# Implementation — EV-AUD-003

## Touchpoints

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/engine/execute-process/src/forges/factory.rs` | CREATE serializa payload contractual; `refresh_process_hash` sella `sha256_phases_integrity` sobre fases **escritas**; índice con aliases/contexto; delete+error si índice falla |
| `SddIA/engine/execute-process/src/engine/entity_manager.rs` | Seed `process` propaga `workspace_template`, inputs/outputs, `phase_invocations` |
| `SddIA/process/process-creator.md` | Sin mutación (contrato ya exigía el payload) |

## Fuera de esta entrega

- Ola heartbeat PR #177.
- D7 jurisdicción (cerrado).
