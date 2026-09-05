---
feature_name: kaizen-ci-telemetry-chronic-quota
created: "2026-09-05"
process: feature
branch_name: feat/kaizen-ci-telemetry-chronic-quota
persist_ref: docs/features/kaizen-ci-telemetry-chronic-quota
pbi_ref: docs/todos/done/[KAIZEN] Telemetría de CI — cuota crónica y degradación mapeada (CA8-CA9).md
document_id: PBI-KAIZEN-CI-TELEMETRY-CHRONIC-QUOTA
uuid: "166c91f9-7378-4766-b6fe-ff5e7eee382f"
execution_id: "18aec32c-f457-4330-819c-2366b959cf57"
---

# Objetivos — kaizen-ci-telemetry-chronic-quota

## Misión

Residual CA8/CA9 de `PBI-KAIZEN-CI-TELEMETRY-OBSERVABILITY`. El ledger `.SddIA/radamanto/ci_failures.json` acumula `CI_Job_Failed` y no actúa. Al superar `per_job_limit` filas distintas (`check_run_id`) para un `job_name` sin mapa: Radamanto emite `CI_Chronic_Failure_Detected` y Cúmulo materializa un PBI Kaizen. Mapa vacío en el MVP (CA9-NEG). Cero DIA, cero Kintsugi, cero `stats.json`.

## Alcance

| Dentro | Fuera |
|--------|-------|
| `radamanto.thresholds.json` v1.3.0 bloque `ci_failures` (parche DA-4, no EM) | `github-bridge-watcher` / CA1–CA7 |
| Evaluación de cuota en `process_ci_job_failed` | `Kaizen_Alert_Required` / `PENDING_AUDIT_DOC_*` |
| Clase `CI_Chronic_Failure_Detected` + acción `materialize-ci-chronic-failure-pbi` vía EM | Pares reales job→entidad (L-MAP = `{}`) |
| Lookup CA9-NEG (mapa vacío → CA8) | Compactación del ledger / L-RESET |
| Tests unidad ledger + handler | `gh pr checks` / `gh run rerun` |

## Ley aplicada

- DA-2: Clase evento y acción vía `entity-manager`. `radamanto.md` no se actualiza (agent-creator update regenera UUID).
- JSON companion `radamanto.thresholds.json`: parche DA-4, sin creator.
- DA-4: topología `docs/features/kaizen-ci-telemetry-chronic-quota` activa.
- DA-5/DA-6: estímulo = ledger local. Tekton no vigila CI.
- Trinidad: `CI_Chronic_Failure_Detected` → `./.events/domain/`. Cuota en `cfg["thresholds"]["ci_failures"]`, no en `cfg["ci_failures"]` (ruta del ledger).

## Criterios

CA8 · CA8-IDEM · CA8-FORJA · CA8-FILTRO-C · CA8-CONTRACT · CA9-NEG. CA9 positivo gated (fuera del primer PR).
