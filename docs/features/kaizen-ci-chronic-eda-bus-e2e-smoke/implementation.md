---
feature_name: kaizen-ci-chronic-eda-bus-e2e-smoke
created: "2026-10-21"
process: feature
items:
  - exclude-event-bus-audit
branch_name: feat/kaizen-ci-chronic-eda-bus-e2e-smoke
persist_ref: docs/features/kaizen-ci-chronic-eda-bus-e2e-smoke
execution_id: "0db56ca9-0fe4-4e81-96cd-88f5c52fcdd4"
document_id: PBI-KAIZEN-CI-CHRONIC-EDA-BUS-E2E-SMOKE
---

# Implementación — kaizen-ci-chronic-eda-bus-e2e-smoke

| Path | Cambio |
|------|--------|
| `SddIA/scripts/qa/build-wasi-capsules.sh` | `event-bus-audit` entra en `EXCLUDE`. Comentario: CLI nativo del store `redb`; `redb` 2.6.4 activa `feature(wasi_ext)` en `target_os=wasi`. |

`SddIA/sddia-event-store/Cargo.toml`, el workflow, `SddIA/tools/event-bus-audit/` y el ledger Radamanto no se tocaron.
