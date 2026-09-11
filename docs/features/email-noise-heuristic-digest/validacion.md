---
feature_name: email-noise-heuristic-digest
created: "2026-09-11"
process: feature
branch: feat/email-noise-heuristic-digest
global: PENDIENTE-CI
pbi_archived: true
document_id: PBI-EMAIL-NOISE-HEURISTIC-DIGEST
execution_id: "8fa709ed-6c4a-4b71-bac1-a2b69ecb0bf3"
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
  CA-CI: PENDIENTE-CI
git_changes:
  - SddIA/engine/execute-process/src/engine/handlers/email_noise_digest.rs
  - SddIA/engine/execute-process/src/engine/handlers/mod.rs
  - SddIA/engine/execute-process/src/engine/mod.rs
  - SddIA/engine/execute-process/src/forges/factory.rs
  - SddIA/engine/execute-process/src/engine/entity_manager.rs
  - SddIA/library/codexes/codex-kalma2-assistant/process/email-noise-digest.md
  - SddIA/library/codexes/codex-kalma2-assistant/process/index.md
  - SddIA/library/codexes/codex-kalma2-assistant.md
  - SddIA/library/codexes/index.md
  - SddIA/core/eda-coverage.json
  - SddIA/evolution/cf1ddf69-3dc6-4576-8245-e47c9536b000.md
  - SddIA/evolution/Evolution_log.md
  - docs/features/email-noise-heuristic-digest/
  - docs/todos/done/[OPERATIVO] Digest heurístico de ruido de correo (Cuarentena asíncrona).md
---

# Validación — email-noise-heuristic-digest

Tests locales: `cargo test -p execute-process --lib -- email_noise_digest` → 7 passed.

CA-CI permanece `PENDIENTE-CI` hasta `run_id` verde del PR. `global` no es APTO hasta entonces.
