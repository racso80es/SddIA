---
feature_name: kaizen-ci-telemetry-chronic-quota
created: "2026-09-05"
process: feature
base: main
scope: mvp-b1-b2-ca9neg
branch_name: feat/kaizen-ci-telemetry-chronic-quota
persist_ref: docs/features/kaizen-ci-telemetry-chronic-quota
pbi_ref: docs/todos/done/[KAIZEN] Telemetría de CI — cuota crónica y degradación mapeada (CA8-CA9).md
document_id: PBI-KAIZEN-CI-TELEMETRY-CHRONIC-QUOTA
uuid: "166c91f9-7378-4766-b6fe-ff5e7eee382f"
execution_id: "18aec32c-f457-4330-819c-2366b959cf57"
---

# Especificación — kaizen-ci-telemetry-chronic-quota

## 1. Contratos

| Pieza | Contrato |
|-------|----------|
| Umbral | `radamanto.thresholds.json` v1.3.0 `ci_failures.per_job_limit` (default 3), `job_entity_map: {}` |
| Runtime cuota | `cfg["thresholds"]["ci_failures"]` — **no** `cfg["ci_failures"]` (ruta ledger) |
| Ledger | `{ "failures": [...], "alerts": { "<job_name>": { emitted_at, count_at_alert, event_type } } }` |
| Clase | `CI_Chronic_Failure_Detected` / `event_family: domain` / `context: quality-assurance` / emisor `radamanto` |
| Destino | `eda_fractal.domain` → `./.events/domain/{event_id}.json` |
| Fan-out | `event-domain-subscriptions.json` → Cúmulo / `materialize-ci-chronic-failure-pbi` |
| Acción | `ecosystem-evolution`; handler nativo; `CONSUMER_SKIP_FORGE_ACTIONS` |

## 2. Payload `CI_Chronic_Failure_Detected`

REQUIRED: `job_name`, `workflow_name`, `failure_count`, `quota_limit`, `sample_check_run_id`, `sample_html_url`, `repository`, `head_sha`.

OPTIONAL: `run_id`, `step_name`.

FORBIDDEN: `entity_id`, `asset_id`, `review_id`, `process_name`.

## 3. Motor Radamanto (`process_ci_job_failed`)

1. Dedup `check_run_id`. Si duplicado y sello ausente y `count >= limit`: reintentar emisión (fallo previo de `write_fractal_event`).
2. Append fila si no duplicado.
3. `count` = filas con ese `job_name`. `limit` = `per_job_limit` (fallback 3).
4. `count < limit` → `{ ok, kind, check_run_id, status: "accumulated" }`.
5. Sello presente → `status: "alert_skipped"` (fallo nuevo) o `skipped: duplicate_check_run_id`.
6. Sin sello y cuota cruzada: mapa vacío o par inválido/inexistente → `CI_Chronic_Failure_Detected`. Par válido en genoma → `Domain_Entity_Degraded` (`reason: ci_failure_quota_exceeded`, `success_rate: 0.0`, `recovery_attempt: 0`). No `governance_payload`.
7. Persistencia única post-decisión. Sello solo si emisión OK.

Cero `stats.json`. Return conserva `ok` + `kind` + `check_run_id`.

## 4. Materialización Cúmulo

`document_id = PBI-KAIZEN-CI-CHRONIC-{SLUG}`. Archivo `docs/todos/pending/[KAIZEN] CI crónica — {slug}.md`. Idempotencia: `document_id` en `pending/` o `done/` → `{ success: true, status: "already_open_or_done" }`.

## 5. Tests

| ID | Dónde | Qué |
|----|-------|-----|
| T-ACC | `radamanto_batch_core` | 1–2 fallos: cero domain; `status=accumulated`; stats intacto |
| T-CA8 | `radamanto_batch_core` | 3.er fallo: un `CI_Chronic_Failure_Detected`; 4.º: skip; sello en ledger |
| T-CA9-NEG | `radamanto_batch_core` | mapa `{}`: cero `Domain_Entity_Degraded` |
| T-MAT | `materialize_ci_chronic_failure_pbi` | crea + idempotente pending/done |
| T-VER | `radamanto_batch_core` | `thresholds` version `1.3.0` + bloque `ci_failures` |

## 6. Evolution

`sddia-qa evolution-register` id `166c91f9-7378-4766-b6fe-ff5e7eee382f`. `gate-evolution --range` si el diff toca `directories.evolution`.
