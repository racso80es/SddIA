---
feature_name: inyeccion-dependencias-capacidades
created: "2026-07-21"
process: feature
branch_name: feat/inyeccion-dependencias-capacidades
persist_ref: docs/features/inyeccion-dependencias-capacidades
items:
  - M2-codice-lengua
  - M1-metadatos-contratos-piloto
  - M3-aduana-temprana
---

# Implementation — DI por capacidades (MVP)

## Touchpoints aplicados

| Ítem | Path | Estado |
|------|------|--------|
| Códice de la Lengua | `SddIA/library/norms/capability-taxonomy.md` | forjado `entity-manager` + catálogo `doc:closure` |
| Contrato piloto | `SddIA/library/norms/capability-contracts/doc.closure.schema.json` | creado |
| Cúmulo | `SddIA/core/cumulo.paths.json` v1.5.2 | `capability_taxonomy` + `capability_contracts` |
| Contratos ED | `process-contract` / `actions-contract` / `skills-contract` | § Metadatos Activos |
| Piloto consumidor | `SddIA/process/feature.md` fase cierre | `requires_capability` |
| Piloto proveedor | `SddIA/skills/filesystem-manager.md` | `provides` |
| Aduana Temprana | `engine/execute-process/.../capability_di_gate.rs` | + cableado `executor` / `residual_runner` |

## Notas

- Opt-out lab: `SDDIA_LAB_SKIP_CAPABILITY_DI=1`
- EDA §2.6 / Library_Codex DI / Cerbero schema: diferidos (`spec.md` §7)
