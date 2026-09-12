---
feature_name: email-digest-preference-reply
created: "2026-09-12"
process: feature
branch: feat/email-digest-preference-reply
global: NO_APTO
pbi_archived: true
document_id: PBI-EMAIL-DIGEST-PREFERENCE-REPLY
execution_id: "b6930240-b8a7-440a-ab0d-f9f853b1855a"
checks:
  CA-1: APTO
  CA-2: APTO
  CA-3: APTO
  CA-4: APTO
  CA-5: APTO
  CA-6: APTO
  CA-7: APTO
  CA-8: APTO
  CA-9: APTO
  CA-10: APTO
  CA-11: APTO
  CA-CI: PENDIENTE-CI
git_changes:
  - SddIA/engine/execute-process/src/engine/handlers/email_noise_digest.rs
  - SddIA/engine/execute-process/src/engine/handlers/email_digest_preference_reply.rs
  - SddIA/engine/execute-process/src/engine/handlers/mod.rs
  - SddIA/engine/execute-process/src/engine/mod.rs
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/engine/execute-process/src/engine/user_preference_change_requested.rs
  - SddIA/library/codexes/codex-kalma2-assistant/process/email-digest-preference-reply.md
  - SddIA/library/codexes/codex-kalma2-assistant/process/email-noise-digest.md
  - SddIA/library/codexes/codex-kalma2-assistant/process/index.md
  - SddIA/library/codexes/codex-kalma2-assistant.md
  - SddIA/library/codexes/index.md
  - SddIA/core/event-domain-subscriptions.json
  - SddIA/core/eda-coverage.json
  - SddIA/evolution/7e4c1a90-2b6d-4f18-9c3a-5d8e0b1a2476.md
  - docs/features/email-digest-preference-reply/
  - docs/todos/done/[OPERATIVO] Réplica del digest de ruido → preferencias.md
---

# Validación — email-digest-preference-reply

Tests locales: `cargo test -p execute-process --lib -- email_noise_digest email_digest_preference_reply` → 15 passed (incluye `max_loop_triggers_p_exempt_c`).

CA-CI: `PENDIENTE-CI`. `global` no APTO hasta `run_id` verde.
