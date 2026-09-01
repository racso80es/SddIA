---
feature_name: kaizen-ci-telemetry-observability
created: "2026-09-01"
process: feature
branch_name: feat/kaizen-ci-telemetry-observability
persist_ref: docs/features/kaizen-ci-telemetry-observability
pbi_ref: docs/todos/pending/[KAIZEN] Telemetría de CI - Captura remota de colapsos y asimilación local.md
document_id: PBI-KAIZEN-CI-TELEMETRY-OBSERVABILITY
uuid: "f8661783-55b7-4419-b659-e96369c02410"
execution_id: "88cff2d5-39c5-41e6-8ca8-2a68049c4344"
---

# Objetivos — kaizen-ci-telemetry-observability

## Misión

Cruzar el umbral: un job de GitHub Actions con `conclusion=failure` en un PR abierto se convierte en telemetría local `CI_Job_Failed` (`./.events/telemetry/`) sin que Tekton vigile CI (DA-6) y sin side-channel de comentarios en el PR.

## Alcance

| Dentro (MVP) | Fuera |
|--------------|-------|
| Sensor Check Runs en `github-bridge-watcher` | `if: failure()` + `gh pr comment` |
| Idempotencia `BridgeState.processed_check_run_ids` | Reacciones GitHub |
| Clase ECST `ci-job-failed.md` vía entity-manager | `radamanto.thresholds.json` |
| Excepción Trinidad: centinela como emisor de ruido periférico | `Domain_Entity_Degraded` / Cerbero |
| Acumulador `.SddIA/radamanto/ci_failures.json` | `Kaizen_Alert_Required` (CA8) |
| Fixture lab + tests | Asimilación de `push` a `main` sin PR |

## Ley aplicada

- DA-2: Clase de evento solo `entity-manager`. Daemon + `SddIA/core/` + `SddIA/engine/` no están en la tabla.
- DA-4: topología `docs/features/kaizen-ci-telemetry-observability` activa.
- DA-5/DA-6: centinela en su tick; Tekton no hace `gh pr checks` / `gh run rerun`.
- Trinidad: telemetría ≠ dominio. No reusar `Raw_Execution_Finished` ni `PullRequest_Presented`.
- `events-contract.md` §6 y prosa de `telemetry/index.md`: parche T-CONTRACT post-forja de Clase (sin contract-creator).

## Criterios (PBI CA1–CA7)

CA1 cruce umbral · CA2 idempotencia · CA3 Trinidad · CA4 `cancelled` ≠ fallo · CA5 forja · CA6 no revocación · CA7 lab.
