---
feature_name: gate-evolution-evol-stem-not-v4
created: "2026-10-27"
process: bug-fix
branch_name: fix/gate-evolution-evol-stem-not-v4
persist_ref: docs/fixes/gate-evolution-evol-stem-not-v4
---

# Implementación

## `SddIA/skills/sddia-evolution-register/src/lib.rs`

- Constante `REASON_ID_NOT_UUID_V4` (`EVOL_ID_NOT_UUID_V4`).
- `invalid_evolution_stem_findings`: escanea paths del diff antes del loop material.
- Helpers compartidos `is_excluded_evolution_md_fname`, `is_borrador_fname`.

## `SddIA/evolution/evolution_contract.md`

- Tabla §8: fila `EVOL_ID_NOT_UUID_V4`.
