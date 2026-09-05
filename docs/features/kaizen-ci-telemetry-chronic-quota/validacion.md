---
feature_name: kaizen-ci-telemetry-chronic-quota
created: "2026-09-05"
process: feature
phase: validate
agents: argos
branch: feat/kaizen-ci-telemetry-chronic-quota
branch_name: feat/kaizen-ci-telemetry-chronic-quota
persist_ref: docs/features/kaizen-ci-telemetry-chronic-quota
pbi_ref: docs/todos/done/[KAIZEN] Telemetría de CI — cuota crónica y degradación mapeada (CA8-CA9).md
document_id: PBI-KAIZEN-CI-TELEMETRY-CHRONIC-QUOTA
uuid: "166c91f9-7378-4766-b6fe-ff5e7eee382f"
global: APTO
pbi_archived: true
checks:
  CA8: APTO
  CA8-IDEM: APTO
  CA8-FORJA: APTO
  CA8-FILTRO-C: APTO
  CA8-CONTRACT: APTO
  CA9-NEG: APTO
  CA9: PENDIENTE-GATED
git_changes:
  - SddIA/actions/index.md
  - SddIA/actions/materialize-ci-chronic-failure-pbi.md
  - SddIA/agents/radamanto.thresholds.json
  - SddIA/core/eda-coverage.json
  - SddIA/core/event-domain-subscriptions.json
  - SddIA/engine/execute-process/src/engine/actions.rs
  - SddIA/engine/execute-process/src/engine/fractal_bus.rs
  - SddIA/engine/execute-process/src/engine/materialize_ci_chronic_failure_pbi.rs
  - SddIA/engine/execute-process/src/engine/mod.rs
  - SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/events/domain/ci-chronic-failure-detected.md
  - SddIA/events/domain/index.md
  - SddIA/events/index.md
  - SddIA/evolution/166c91f9-7378-4766-b6fe-ff5e7eee382f.md
  - SddIA/evolution/Evolution_log.md
  - docs/features/kaizen-ci-telemetry-chronic-quota/
  - docs/todos/done/[KAIZEN] Telemetría de CI — cuota crónica y degradación mapeada (CA8-CA9).md
---

# Validacion — kaizen-ci-telemetry-chronic-quota

`global: APTO`. Tests locales T5. PBI archivado en `docs/todos/done/` en esta rama. CA9 positivo gated (mapa `{}`); no bloquea el primer PR.

## Checks

| CA | Veredicto | Evidencia |
|----|-----------|-----------|
| CA8 | APTO | `ci_job_failed_quota_emits_chronic_then_skips`: 3.er `check_run_id` → un `CI_Chronic_Failure_Detected`; 4.º `alert_skipped`. Cero fractura/DIA/`stats.json`. Handler `ci_chronic_materialize_*` materializa e idempotencia pending/done. |
| CA8-IDEM | APTO | Sello `alerts` post-OK. `ci_job_failed_retries_emit_when_unsealed`. Handler `already_open_or_done`. |
| CA8-FORJA | APTO | EM create evento `c55ef8cc-41b8-42af-a524-c58b847039a8` y acción `a6eb7f0c-8b2f-4c7d-ae5e-6c1b589f3c92`. Umbral v1.3.0 DA-4. `radamanto.md` residual (UUID). `audit-eda-coverage --scan` `orphan_count: 0`. |
| CA8-FILTRO-C | APTO | `CONSUMER_SKIP_FORGE_ACTIONS` incluye `materialize-ci-chronic-failure-pbi`. |
| CA8-CONTRACT | APTO | `ci_job_failed_writes_ledger_not_stats` (`ok`/`kind`/`check_run_id`). `thresholds_110_process_intact` `version == "1.3.0"`. |
| CA9-NEG | APTO | `ci_job_failed_ca9_neg_empty_map_zero_degraded`: cero `Domain_Entity_Degraded`. |
| CA9 | PENDIENTE-GATED | Fuera del primer PR (L-MAP). |

## Tests

```text
cd SddIA && cargo test -p execute-process --lib -- ci_job_failed ci_chronic thresholds_110
# 8 passed
```
