---
feature_name: linear-hu-a-01-contract
created: "2026-10-03"
process: feature
---

# Especificación — contrato tracker 1.3.0

## Artefactos

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/library/codexes/codex-software-engineering/contracts/project-config-contract.md` | v1.3.0 |
| `SddIA/engine/execute-process/src/engine/project_binding.rs` | Admite `1.3.0`; valida `todo`, labels opcionales, `done_gate` |

## Criterios (PBI AC-1)

- Manifiesto `1.3.0` con `state_map.todo`, labels extendidas y `done_gate: git` → binding OK.
- `done_gate: linear` o `both` → `PROJECT_CONFIG_INVALID` (HU-A).
- Manifiesto `1.2.0` sin campos nuevos → OK.
