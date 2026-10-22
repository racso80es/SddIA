---
feature_name: tracker-lifecycle-contract
created: "2026-10-22"
process: feature
items:
  - taxonomy-1.0.9
  - bindings-1.7.0
  - schema-emit-v2
  - validator-test
branch_name: feat/tracker-lifecycle-contract
persist_ref: docs/features/tracker-lifecycle-contract
execution_id: "3e0a8c0f-1b97-4b9c-a467-b149494b3e28"
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-01
---

# Implementation — tracker-lifecycle-contract

## Touchpoints

| Path | Cambio |
|------|--------|
| `SddIA/library/norms/capability-taxonomy.md` | v1.0.9. Término `tracker:lifecycle` / `tracker.lifecycle` / 1.0.0. Hash `sha256:4cf09806f229ac1d4ca0c3131bb054a2db06c715bce28d549310a1d111e1ceab`. |
| `SddIA/library/norms/index.md` | Fila de la taxonomía a `1.0.9`. |
| `SddIA/core/capability-bindings.md` | v1.7.0. Una fila hacia `tool:linear-tracker-adapter`. |
| `SddIA/library/norms/capability-contracts/tracker.lifecycle.schema.json` | Required `success` + `exitCode`. `additionalProperties`. |
| `SddIA/engine/execute-process/src/engine/capability_di_output_validator.rs` | Test del cuerpo `emit_v2` y del filtro A. |

`norm-creator` no se usa: en update no preserva `catalog`. El sello es `emit-domain-mutation` sobre la norma ya escrita.
