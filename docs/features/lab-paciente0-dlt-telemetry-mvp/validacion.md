---
feature_name: lab-paciente0-dlt-telemetry-mvp
created: "2026-09-06"
process: feature
branch: feat/lab-paciente0-dlt-telemetry-mvp
global: APTO
pbi_archived: true
document_id: PBI-LAB-PACIENTE0-DLT-TELEMETRY-MVP
execution_id: "17ed4fb6-e729-4dbe-9813-cf9985aa9bce"
evolution_id: "ad46c2d6-30fc-451e-8e74-5b19f4f2602e"
pr_url: https://github.com/racso80es/SddIA/pull/264
ci_run_id: "34015318153"
ci_run_url: https://github.com/racso80es/SddIA/actions/runs/34015318153
ci_head_sha: "4f3c821f1161aa992c07526041575f8f806fd87a"
checks:
  CA-1: APTO
  CA-2: APTO
  CA-3: APTO
  CA-4: APTO
  CA-5: APTO
  CA-6: APTO
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
  - docs/todos/done/PBI-LAB-PACIENTE0-DLT-TELEMETRY-MVP.md
---

# Validación — lab-paciente0-dlt-telemetry-mvp

Plano A locales: tests `dlt_telemetry` (lib 8 + integration 2) + CA-1 email-triage + regresión PR `failed` sin bóveda.

CA-6 APTO: GitHub Actions `success` en `headSha=4f3c821f1161aa992c07526041575f8f806fd87a`.

- push: [34015316079](https://github.com/racso80es/SddIA/actions/runs/34015316079)
- pull_request: [34015318153](https://github.com/racso80es/SddIA/actions/runs/34015318153)

PBI en `docs/todos/done/PBI-LAB-PACIENTE0-DLT-TELEMETRY-MVP.md`. PR https://github.com/racso80es/SddIA/pull/264.

Plano B (Testnet físico Paciente 0) permanece como aceptación de instancia, fuera del gate CI.
