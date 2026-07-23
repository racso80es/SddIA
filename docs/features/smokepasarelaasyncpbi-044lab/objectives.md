---
feature_name: smokepasarelaasyncpbi-044lab
created: "2026-07-23"
updated: "2026-07-23"
process: feature
branch_name: feat/smokepasarelaasyncpbi-044lab
persist_ref: docs/features/smokepasarelaasyncpbi-044lab
document_id: PBI-044-SMOKE-PASARELA-ASYNC-LAB
pbi_uuid: 8c71b50f-7067-472a-a149-40041920b054
pbi_ref: docs/todos/done/[ARQUITECTURA] PBI-044 — Pasarela asíncrona Kalma2 y desacople por bus de eventos.md
execution_id: e92ee44d-9992-4d1b-9384-b5aba5de1acc
correlation_id: e92ee44d-9992-4d1b-9384-b5aba5de1acc
prior_correlation_ids:
  - 33f4a9ee-290c-40af-8634-ae69c1445642
  - 54e86a6b-2bec-4010-8da8-ea50f2e86973
  - 6178f1d1-e1d7-4446-bc9b-fca16d79b872
  - 978397b0-c509-4678-a69c-3c69a4acaef7
  - 97af9687-41d5-4d6a-b094-bf2d4b678da8
  - ae3bba9e-ccd7-4d9a-a106-401c9897828f
  - e6bf6120-fb76-49c5-982d-b8e914e26174
phase: mayeuta-stabilization
agents: mayeuta
depends_on:
  - docs/features/kalma2-pasarela-asincrona-eda
  - docs/features/kalma2-event-bus-integration
  - docs/features/kalma2-process-dispatch
  - docs/features/kalma2-full-cycle
status: requirements_stable
reentry_note: "Reingreso cid e92ee44d post Argos NO_APTO FAIL_EVIDENCE_GAP (ciclo e6bf6120) — requisitos reafirmados; no relajar AC-L-*; T-GATE Unlock sigue siendo precondición de materialización; git-manager Rejected esta sesión"
---

# Objetivos — smokepasarelaasyncpbi-044lab

## Misión

Materializar en laboratorio evidencia física reproducible de los smokes/units de la **pasarela asíncrona Kalma2 (PBI-044)**: acuse HTTP no bloqueante (202/`accepted`/p99&lt;50 ms), correlación `correlation_id ≡ event_id`, proyección `GET /api/status`, ceguera espacial del bridge y regresión de suscripciones — **sin** reabrir el diseño H1+H2 ni rehabilitar peajes F4 del PR padre.

## Punto objetivo

> **O-SMOKE-044-LAB:** En entorno lab controlado, N≥10 disparos `POST /api/execute` responden 202/`accepted` con p99 RTT &lt; 50 ms; existe rastro correlacionado `Kalma2_Process_Requested` (o spawn equivalente); `GET /api/status` proyecta estado vivo para el cid; units bridge/handler verdes; audit estático cero writes EDA desde `kalma2-bridge` (camino execute); toda evidencia queda en `docs/features/smokepasarelaasyncpbi-044lab/` (execution/validacion), sin inventar éxito.

## Alcance

| Dentro | Fuera |
|--------|-------|
| Smokes L-S1 timing, L-S2 correlación, L-S3 status | Re-diseño spawn/202 / ECST nuevo |
| Units L-U1 / L-U2 (baseline padre) | H3 Telegram / chat SSE |
| Audit L-BLIND + L-REG suscripciones | waiting-for-shell Cursor Agent |
| Documentar método + artefactos lab en este `persist_ref` | Rehabilitación F4 / Cerbero PR #146 |
| Bugfix mínimo solo si smoke falla con causa (Q4) | PBI-043 DI · PPR#136 F3 |
| Rematerializar evidencia tras `FAIL_EVIDENCE_GAP` | Re-archivo PBI-044 (ya en `done/`) |
| T-GATE Unlock RBAC como precondición de ejecución | PEC e2e watcher+TQM como gate mínimo |
| | Relajar AC-L-* por bloqueo Shell/RBAC IDE |

## Objetivos medibles

| ID | Objetivo | Criterio (AC) |
|----|----------|---------------|
| **O1** | Timing no bloqueante | AC-L-S1: N≥10 → 202 + `accepted` + cid; p99 RTT &lt; 50 ms |
| **O2** | Correlación ECST | AC-L-S2: `event_id ≡ correlation_id` en dominio (o spawn correlacionado) |
| **O3** | Observabilidad status | AC-L-S3: `GET /api/status?event_id=` vivo; documentar techo lab si sin PEC |
| **O4** | Units baseline | AC-L-U: bridge + kalma2 handler verdes |
| **O5** | Ceguera espacial | AC-L-BLIND: cero writes EDA desde crate bridge (camino execute; stdout test) |
| **O6** | No regresión nervio | AC-L-REG: diff suscripciones/allowlist = 0 injustificado |
| **O7** | Evidencia auditable | AC-L-DOC: `execution.md` + fixtures/logs; ausencia física = NO_APTO |
| **O8** | Cierre lab | AC-DONE-LAB: `validacion.md` APTO en rama; sin mover PBI-044 |

## Flujo ontológico objetivo (qué, no cómo)

```text
Lab
  → Unlock runtime (T-GATE: shell-executor + git-manager no Rejected)
  → build bridge (+ execute-process si U2)
  → N× POST /api/execute (intención válida lab)
  → medir 202 / accepted / cid / RTT (sin await ciclo completo en el request)
  → verificar dominio Kalma2_Process_Requested (cid≡event_id) + audit no-write-bus
  → GET /api/status?event_id=<cid> (proyección viva)
  → cargo test bridge + kalma2
  → persistir evidencia en persist_ref → Argos
```

## No objetivos

- Reimplementar o rediseñar la pasarela async.
- Inventar evento de intención paralelo o writes EDA desde bridge.
- Absorber F4 RBAC, Shell IDE async, Telegram H3 o DI residual.
- Declarar APTO sin evidencia física capturable.
- Escribir semillas en `docs/todos/` (solo Cumulo / Kaizen_Alert).
- Sustituir units/audits por lectura estática narrativa.

## Invariantes

- `SddIA/core/cumulo.paths.json` = SSOT de paths.
- Contrato padre R1–R5 / `correlation_id ≡ event_id` intacto.
- Git vía `skill:git-manager` (preferente `./sddia-run.sh --tool git-manager`).
- Semillas Kaizen/TODOs solo agent:cumulo o evento `Kaizen_Alert_Required`.
- Bloqueo de runtime ≠ cambio de requisito: `FAIL_EVIDENCE_GAP` exige rematerializar, no bajar el piso.

## Ley aplicada

- `.cursorrules` §4–§5 (cápsulas JSON; agnosticismo Core)
- Ceguera espacial kalma2-bridge (laudos PBI-044)
- `features-documentation-pattern` v1.2.1 + `task-closure-documental` (lab: sin re-archivo PBI padre)
- Proceso `feature` — fase Estabilización → Dedalo consume este cuerpo como `refined_requirements`
- Clarificaciones D0–D14 y laudos Q1–Q6 en `clarify.md`

## Artefactos de referencia

- Padre: `docs/features/kalma2-pasarela-asincrona-eda/` (spec/plan/execution smokes S1–S3)
- Este lab (cascada previa): `spec.md`, `plan.md` v1.1.0 (+T-GATE), `execution.md` (`blocked`), `validacion.md` (`NO_APTO` / `FAIL_EVIDENCE_GAP`)
- Bridge: `SddIA/interfaces/kalma2-bridge/`
- Handler: `SddIA/engine/execute-process/src/engine/handlers/kalma2.rs`
- UI: `interfaces/kalma2/app.js`
- Fixture: `docs/features/smokepasarelaasyncpbi-044lab/_smoke-timing-execute.json`
- PBI padre: `docs/todos/done/[ARQUITECTURA] PBI-044 — Pasarela asíncrona Kalma2 y desacople por bus de eventos.md`
