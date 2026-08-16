---
feature_name: process-creator-full-contract-forge
created: "2026-08-16"
process: bug-fix
items:
  - create-persists-process-phases
  - persist-workspace-inputs-outputs
  - ev-aud-003-unit-test
---

# Implementation — EV-AUD-003

## Touchpoints

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/engine/execute-process/src/forges/factory.rs` | CREATE serializa `process_phases` reales + `workspace_template` / inputs / outputs / aliases / `phase_invocations`; aborta si YAML leído ≠ fases pedidas |
| Genoma `process-creator.md` | Sin mutación (contrato ya exigía el payload) |
| Jurisdicción D7 | Intacta |

## Pendiente (mismos ACs del PBI)

- Fixture `evolution-audit` sin laudo de excepción.
- Alineación explícita `entity-manager` si el envelope no propaga `process_phases`.
