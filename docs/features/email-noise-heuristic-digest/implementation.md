---
feature_name: email-noise-heuristic-digest
created: "2026-09-11"
process: feature
items:
  - handler-aggregation
  - tests-cursor-telegram
  - entity-manager-process
  - entity-manager-codex
  - evolution-register
---

# Implementación — email-noise-heuristic-digest

## Touchpoints

| Path | Cambio |
|------|--------|
| `SddIA/engine/execute-process/src/engine/handlers/email_noise_digest.rs` | Handler nativo: filtro C cerrado, clamp, Telegram `parse_mode: null` |
| `SddIA/engine/execute-process/src/engine/handlers/mod.rs` | `pub mod email_noise_digest` |
| `SddIA/engine/execute-process/src/engine/mod.rs` | Dispatch `email-noise-digest` |
| `SddIA/engine/execute-process/src/forges/factory.rs` | Update de códice vía `markdown_body_replacements` |
| `SddIA/engine/execute-process/src/engine/entity_manager.rs` | Forward replacements en clase `codex` |
| `email-noise-digest` | EM create domain kalma2; uuid `fc11c0d6-…` |
| `codex-kalma2-assistant` | EM update 1.0.1 membership; uuid `c43544f3-…` |
| `SddIA/evolution/cf1ddf69-3dc6-4576-8245-e47c9536b000.md` | Alta evolution |

## Propuestas aplicadas

L1–L3. Sin timer de instancia. Sin suscripción de dominio. Sin réplica (PBI hijo).
