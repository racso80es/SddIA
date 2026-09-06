---
feature_name: lab-paciente0-dlt-telemetry-mvp
created: "2026-09-06"
process: feature
branch: feat/lab-paciente0-dlt-telemetry-mvp
global: NO_APTO
pbi_archived: false
document_id: PBI-LAB-PACIENTE0-DLT-TELEMETRY-MVP
execution_id: "17ed4fb6-e729-4dbe-9813-cf9985aa9bce"
evolution_id: "ad46c2d6-30fc-451e-8e74-5b19f4f2602e"
checks:
  CA-1: APTO
  CA-2: APTO
  CA-3: APTO
  CA-4: APTO
  CA-5: APTO
  CA-6: PENDIENTE-CI
git_changes:
  - SddIA/core/event-domain-subscriptions.json
  - SddIA/core/event-subscriptions.json
  - SddIA/core/eda-coverage.json
  - SddIA/events/domain/domain-entity-telemetry-captured.md
  - SddIA/engine/execute-process/src/engine/dlt_telemetry_anchor.rs
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/engine/execute-process/src/engine/route_fractal_core.rs
  - SddIA/engine/execute-process/src/engine/mod.rs
  - SddIA/engine/execute-process/src/engine/handlers/email_triage.rs
  - SddIA/engine/execute-process/tests/dlt_telemetry.rs
  - SddIA/evolution/ad46c2d6-30fc-451e-8e74-5b19f4f2602e.md
  - docs/features/lab-paciente0-dlt-telemetry-mvp/
  - docs/todos/pending/PBI-LAB-PACIENTE0-DLT-TELEMETRY-MVP.md
---

# Validación — lab-paciente0-dlt-telemetry-mvp

Plano A locales: tests `dlt_telemetry` (lib 8 + integration 2) + CA-1 email-triage + regresión PR `failed` sin bóveda.

CA-6 `PENDIENTE-CI`: `global` no es APTO hasta `run_id` verde del PR. PBI permanece en `pending/` hasta entonces.

Plano B (Testnet físico Paciente 0) fuera de este gate.
