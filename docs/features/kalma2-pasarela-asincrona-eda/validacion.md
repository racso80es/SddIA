---
feature_name: kalma2-pasarela-asincrona-eda
created: "2026-07-22"
updated: "2026-07-22"
process: feature
phase: Verificación
agent: argos
branch: feat/kalma2-pasarela-asincrona-eda
persist_ref: docs/features/kalma2-pasarela-asincrona-eda
document_id: PBI-044-KALMA2-PASARELA-ASINCRONA-EDA
pbi_uuid: 8c71b50f-7067-472a-a149-40041920b054
pbi_ref: docs/todos/done/[ARQUITECTURA] PBI-044 — Pasarela asíncrona Kalma2 y desacople por bus de eventos.md
correlation_id: 6178f1d1-e1d7-4446-bc9b-fca16d79b872
execution_id: 002f4e1b-0155-4874-95cd-8e6953ed0f70
global: APTO
pbi_archived: true
approval_status: approved
verdict: aprobado
git_manager_invoked: true
tekton_status: evidence_apto
scope: "H1+H2 Done mínimo PBI-044 (R1–R5; R6/H3 defer)"
checks:
  DOC_CASCADE: APTO
  AC_R1_ACCEPT_CONTRACT_STATIC: APTO
  AC_R1_TIMING_P99_SMOKE: APTO
  AC_R2_SPATIAL_BLINDNESS_STATIC: APTO
  AC_R2_SPAWN_CORRELATION_RUNTIME: APTO
  AC_R3_NERVE_REGRESSION: APTO
  AC_R4_STATUS_TERMINAL_SMOKE: APTO
  AC_R5_CHAT_OUT_OF_DONE: APTO
  AC_R6_H3_DEFER: APTO
  R3_ECST_CANONICAL: APTO
  U1_CARGO_TEST_BRIDGE: APTO
  U2_CARGO_TEST_KALMA2: APTO
  GIT_EVIDENCE_VIA_GIT_MANAGER: APTO
  AC_DONE_PBI: APTO
git_changes:
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - SddIA/interfaces/kalma2-bridge/Cargo.toml
  - SddIA/Cargo.lock
  - SddIA/engine/execute-process/src/engine/handlers/kalma2.rs
  - interfaces/kalma2/app.js
  - docs/features/kalma2-pasarela-asincrona-eda/
  - docs/todos/done/[ARQUITECTURA] PBI-044 — Pasarela asíncrona Kalma2 y desacople por bus de eventos.md
  - docs/todos/pending/[ARQUITECTURA] PBI-043 — Pasarela asíncrona de Cursor y desacople por bus de eventos.md
---

# Validación — kalma2-pasarela-asincrona-eda (Argos · Verificación)

## Veredicto

**APTO / approved** — H1+H2 evidentes (estático + `cargo test` + smokes HTTP S1–S3 + git-manager). R6/H3 defer documentado.

## Ingesta

| Input | Resolución |
|-------|------------|
| `persist_ref` | `docs/features/kalma2-pasarela-asincrona-eda` |
| `branch` | `feat/kalma2-pasarela-asincrona-eda` |
| `pbi_ref` | archivado en `docs/todos/done/` · `document_id: PBI-044-KALMA2-PASARELA-ASINCRONA-EDA` |
| `execution_id` | `002f4e1b-0155-4874-95cd-8e6953ed0f70` |
| Smoke cid | `6178f1d1-e1d7-4446-bc9b-fca16d79b872` |

## Checks

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| DOC_CASCADE | Cascada bajo persist_ref | **APTO** | clarify→objectives→spec→plan→implementation→execution→validacion |
| AC-R1 contrato | 202 + `accepted` + cid | **APTO** | `accept_execute` / `reply_accept_result`; 12/12 bodies |
| AC-R1 timing | p99 &lt; 50 ms | **APTO** | N=12; p99 RTT **4.5 ms**; bridge `duration_ms` p99=3 |
| AC-R2 ceguera | Bridge sin write fractal | **APTO** | audit prod `!write_fractal_event`; spawn+reaper |
| AC-R2 runtime | `Kalma2_Process_Requested` cid≡event_id | **APTO** | `.events/domain/6178f1d1-….json` |
| R3 ECST | Solo evento canónico | **APTO** | cero `Kalma2_Interaction_Requested` |
| AC-R3 nervio | Sin regresión suscripciones | **APTO** | diff `main` subscriptions = 0 líneas |
| AC-R4 status | poll status con cid | **APTO** | GET 200 `pending` + domain.found (PEC e2e diferido sin watcher; unit PEC ok) |
| AC-R5 chat | fuera Done | **APTO** | chat sync intacto |
| AC-R6 H3 | Telegram defer | **APTO** | documentado |
| U1 / U2 | cargo tests | **APTO** | bridge 9/9; kalma2 10/10 |
| GIT | git-manager | **APTO** | `success: true` status |
| AC-DONE-PBI | PBI done + `pbi_archived` | **APTO** | este veredicto + move en rama |

## Git (`skill:git-manager`)

```text
printf '%s' '{"operation_type":"status","repository_path":"/home/racso/Proyectos/SddIA","operation_payload_json":{}}' \
  | ./sddia-run.sh --tool git-manager
→ success: true
```

## Cierre documental

| Campo | Valor |
|-------|--------|
| `global` | `APTO` |
| `pbi_archived` | `true` |
| H3/R6 | defer; no bloquea Done H1+H2 |
| Siguiente | `delivery-close-cycle` → PR único |
