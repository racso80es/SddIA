---
feature_name: pull-request-automation-dlt
process: feature
created: "2026-05-23"
persist_ref: docs/features/pull-request-automation-dlt
branch_name: feat/pull-request-automation-dlt
related_todo: docs/todos/done/Activacion_Validacion_PR_DLT.md
related_todo_pdf: docs/todos/pending/SddIA_PBI_TODO_Activacion_Validacion_PR_v2.0.0.pdf
---

# Objetivos — Oráculo Sensor DLT y Activación Validación PR

## Meta

Cerrar la **ceguera transaccional** cuando un agente autónomo remoto (Jules) crea un PR en GitHub: forzar que **toda detección pase por IOTA Rebased Testnet** antes de materializar `PullRequest_Presented` en el bus local, despertando simétricamente la aduana `pull-request-review` sin exponer claves privadas al contexto de la IA obrera.

## Objetivos medibles

| ID | Objetivo | Criterio |
|----|----------|----------|
| **O1** | **Demonio sensor H1** | `github_bridge_watcher.py` operativo — polling/webhook agnóstico al autor del PR |
| **O2** | **Puente firma aislada H2** | En `pull_request.opened`: validación GitHub REST + firma vía wallet local + `iota-immutable-publisher` en Testnet |
| **O3** | **Materialización idempotente H3** | Suscriptor local escribe ECST en `eda_bus.pending` con `event_id` = digest DLT; reintentos no duplican |
| **O4** | **Smoke E2E desacoplado H4** | `SDDIA_LAB_SIMULATE_REMOTE_PR=1` → evento en bus → 7 fases `pull-request-review` con exit code binario |
| **O5** | **Contrato ECST** | Payload alineado al PBI (`repository`, `branch`, `pr_url`, `origin_agent`, `dlt_anchor_address`, `signer_identity_rbac`, `delivery_state`) |
| **O6** | **Contención Filtro B** | Fallback dead-letter tras 3 reintentos IOTA; validación ciega pre-firma (Filtro A) |
| **O7** | **Validación Argos** | `validacion.md` con `global: APTO`, laudo empírico y trazabilidad DLT |

## Hitos del PBI (trazabilidad)

| Hito | Objetivo técnico | Validación estricta |
|------|------------------|---------------------|
| H1 | Diseño demonio `sddia-github-bridge` | Listener local Python interconectado con API repositorio |
| H2 | Puente firma + inyección DLT | Solo demonio accede a `.SddIA/.dev/wallet.key` |
| H3 | Materialización idempotente en bus | `transaction_id` como `event_id` raíz |
| H4 | Prueba humo E2E desacoplada | Simulación Jules sin privilegios locales → aduana automatizada |

## No objetivos (esta feature)

- Sustituir `emit-pr-presented-event` en flujo local Cursor (`delivery-close-cycle`).
- Desplegar webhook productivo permanente en GitHub (solo lab/polling/tunnel).
- Reabrir aduana `pull-request-review` — ya entregada en `pull-request-review-redesign`.
- Exponer seed/privkey a entornos de ejecución de LLM.

## Precedencia y ley aplicada

- PBI MD: `docs/todos/done/Activacion_Validacion_PR_DLT.md` (v2.0.0)
- PBI PDF: `docs/todos/pending/SddIA_PBI_TODO_Activacion_Validacion_PR_v2.0.0.pdf`
- Evento: `SddIA/events/pull-request-presented.md` v1.1.0
- Aduana: `SddIA/process/pull-request-review.md` v2
- Orquestación local PR: `docs/features/pr-presented-orchestration/`
- Tool DLT: `SddIA/tools/iota-immutable-publisher.md`
- Suscripciones: `SddIA/core/event-subscriptions.json`
- Proceso: `feature` v1.3.0 · norma `features-documentation-pattern` v1.2.0

## Estado

| Fase feature | Estado |
|--------------|--------|
| Inicialización | ✅ rama `feat/pull-request-automation-dlt` |
| Clarificación | ✅ `clarify.md` |
| PBI ampliado | ✅ `Activacion_Validacion_PR_DLT.md` |
| Especificación | ✅ `spec.md` |
| Planificación | ✅ `plan.md` |
| Implementación | ✅ H1–H5 (Tekton) |
| Validación | ✅ `validacion.md` APTO |
