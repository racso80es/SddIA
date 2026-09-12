---
feature_name: email-digest-preference-reply
created: "2026-09-12"
process: feature
items:
  - digest-tokens-markup
  - reply-handler-eda
  - entity-manager-genome
  - exempt-e2e
---

# Implementación — email-digest-preference-reply

## Touchpoints

| Path | Cambio |
|------|--------|
| `email_noise_digest.rs` | Tokens 8/12 hex, mensaje numerado, `reply_markup`, `tokens` en estado (skip/vacío preservan) |
| `email_digest_preference_reply.rs` | Handler nativo: gate `dpref:`, correlación, emit activate max/mute |
| `handlers/mod.rs` / `engine/mod.rs` | Dispatch `email-digest-preference-reply` |
| `route_domain_core.rs` | Allowlist `event_file_path` |
| `event-domain-subscriptions.json` | `TelegramCallback_Received` → proceso réplica |
| `user_preference_change_requested.rs` | Fallback UUID si crypto-broker ausente (lab tmp) |
| process `email-digest-preference-reply` | EM create domain kalma2; uuid `c77f96ad-…` |
| process `email-noise-digest` | EM update 1.0.1 markup en fase notificación |
| `codex-kalma2-assistant` | EM update 1.0.2 membership |

## Propuestas aplicadas

L1–L4. Cero diff en `telegram-watcher`. Cero Clase ECST nueva. Cero NL/hábitos.
