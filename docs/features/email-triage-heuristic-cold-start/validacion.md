---
feature_name: email-triage-heuristic-cold-start
created: "2026-09-06"
process: feature
branch: feat/email-triage-heuristic-cold-start
global: NO_APTO
pbi_archived: false
checks:
  CA1: APTO
  CA2: APTO
  CA3: APTO
  CA5: APTO
  CA6: APTO
  CA7: APTO
  CA9: APTO
  CA10: APTO
  CA11: APTO
  CA4: N/A
  CA8: N/A
  CA-CI: PENDIENTE-CI
git_changes:
  - SddIA/user-preference-core/src/lib.rs
  - SddIA/engine/execute-process/src/engine/handlers/email_triage.rs
  - SddIA/engine/execute-process/src/engine/capability_di_gate.rs
  - SddIA/library/codexes/codex-kalma2-assistant/process/email-triage-gateway.md
  - SddIA/library/codexes/codex-kalma2-assistant/process/index.md
  - SddIA/library/norms/email-triage-matrix.md
  - SddIA/library/norms/index.md
  - SddIA/events/domain/email-triaged.md
  - SddIA/core/eda-coverage.json
  - SddIA/evolution/95441293-1049-4016-8112-a322919d34e8.md
  - SddIA/evolution/Evolution_log.md
  - docs/features/email-triage-heuristic-cold-start/
---

# Validación — email-triage-heuristic-cold-start

Slice 1: tests locales verdes (`email_triage` 24, `canonical_subject` 1, DI 1). CA4/CA8 N/A (Slice 2).

`global: APTO` queda bloqueado hasta `run_id` verde de GitHub Actions (CA-CI). PBI permanece en `pending/` hasta ese sello.
