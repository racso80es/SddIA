---
feature_name: intercepcion-habitos-kalma2
created: "2026-09-12"
process: feature
items:
  - forge-action-em
  - genome-conscience
  - handler-tendon-dispatch
  - destilar-hint-hash
  - triage-key-candidates
  - tests-lab
branch_name: feat/intercepcion-habitos-kalma2
persist_ref: docs/features/intercepcion-habitos-kalma2
execution_id: "54b02c30-b579-4d0b-a7e0-272d3c3c6af0"
document_id: PBI-NUCLEO-INTERCEPCION-HABITOS-KALMA2
---

# Implementación — intercepcion-habitos-kalma2

| Path | Cambio |
|------|--------|
| `SddIA/actions/dispatch-aiua-intent.md` | EM update 1.1.0: tendón `delegar_habito` → emit canónico |
| `SddIA/actions/index.md` | Fila 1.1.0 |
| `SddIA/conscience/aiua_core.md` | 1.3.0; fila §6 |
| `SddIA/engine/execute-process/src/engine/aiua_intent.rs` | `HABIT_TENDON`, matriz, emit, tests |
| `SddIA/user-preference-core/src/lib.rs` | `canonical_subject_key_from_hint`; destilación; default mute |
| `SddIA/engine/execute-process/src/engine/handlers/email_triage.rs` | Candidatos de clave; test CA-8 |
| `SddIA/engine/execute-process/src/engine/handlers/aiua_stimulus.rs` | Overlay CA-9; mutex lab env |
| `SddIA/engine/execute-process/src/engine/handlers/user_preference.rs` | Test ingest hash hint |
