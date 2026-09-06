---
feature_name: email-triage-heuristic-cold-start
created: "2026-09-06"
process: feature
items:
  - crate-hash
  - handler-slice1
  - tests-slice1
  - entity-manager-process
  - entity-manager-norm
  - entity-manager-event
  - evolution-register
---

# Implementación — email-triage-heuristic-cold-start

## Touchpoints

| Path | Cambio |
|------|--------|
| `SddIA/user-preference-core/src/lib.rs` | `normalize_email_addr` + `canonical_subject_key_from_addr` |
| `SddIA/engine/execute-process/src/engine/handlers/email_triage.rs` | Orden spec §2; G5 mute; conjugación prompt |
| `SddIA/engine/execute-process/src/engine/capability_di_gate.rs` | Test DI `memory:pref-query` en Triaje-P |
| `email-triage-gateway` | EM update 1.1.0; uuid `9cb9a63a-…` |
| `email-triage-matrix` | EM update 1.1.0; uuid `3d8c7e09-…` |
| `email-triaged` | EM replacements; uuid `6a4b0e9a-…`; SemVer 1.1.0 |
| `SddIA/evolution/95441293-1049-4016-8112-a322919d34e8.md` | Alta evolution |

## Propuestas aplicadas

Slice 1 completo. Slice 2 no. YAML `outputs.decision_path` del proceso permanece descriptivo (`deterministic \| llm`); cuerpo y handler son SSOT de `preference` (forge de phases no muta outputs).
