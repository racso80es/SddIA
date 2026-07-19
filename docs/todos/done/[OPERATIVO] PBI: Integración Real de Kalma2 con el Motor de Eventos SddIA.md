---
document_id: PBI-KALMA2-EVENT-BUS-INTEGRATION
title: "[OPERATIVO] PBI: Integración Real de Kalma2 con el Motor de Eventos SddIA"
format: markdown
version: "1.4.0"
created: "2026-07-19"
status: done
priority: alta
process: feature
branch_name: feat/kalma2-event-bus-integration
feature_ref: docs/features/kalma2-event-bus-integration
validacion_ref: docs/features/kalma2-event-bus-integration/validacion.md
uuid: 7047f38b-333f-4b85-bab1-1a6ff6992009
closed: "2026-07-19"
depends_on:
  - docs/features/kalma2-bridge-rust
  - docs/features/kalma2-mayeuta-llm-router
---

# PBI — Integración Real de Kalma2 con el Motor de Eventos SddIA

| Campo | Valor |
|-------|-------|
| **ID** | `PBI-KALMA2-EVENT-BUS-INTEGRATION` |
| **Estatus** | ✅ Done (Argos APTO) — pendiente PR/merge |
| **Feature** | [`docs/features/kalma2-event-bus-integration/`](../../features/kalma2-event-bus-integration/) |
| **Validación** | [`validacion.md`](../../features/kalma2-event-bus-integration/validacion.md) |
| **Rama** | `feat/kalma2-event-bus-integration` |
| **Depende de** | `kalma2-bridge-rust` ✅ · `kalma2-mayeuta-llm-router` ✅ |

---

## 0. Estado

**v1.4.0 Done documental** (2026-07-19). Argos `global: APTO`, `pbi_archived: true`. Pendiente `delivery-close-cycle` / merge en `main`.

---

## 1. Especificación (Spec)

### Anomalía real (tras auditoría)

El cliente Kalma2 **ya no simula** en el frontend. `interfaces/kalma2/app.js` hace `POST /api/interact` y renderiza `response` del envelope. El puente `kalma2-bridge` es aduana inerte hacia `execute-process` → `kalma2-interact`.

Lo que el operador percibe como «texto predeterminado» (`[Tormentosa/Aiúa] Recibo el estímulo…`) es el **fallback determinista** `synthesize_mayeuta_response` en `SddIA/engine/execute-process/src/engine/handlers/mayeuta.rs`, activado cuando:

- falta `SDDIA_LLM_CLI_COMMAND`, o
- la Skill `mayeuta-llm` falla / no resuelve síntesis.

En paralelo, el enrutamiento asíncrono a procesos (`Kalma2_Process_Requested`) **ya existe** (feature `kalma2-mayeuta-llm-router`), pero el lazo UI↔EDA queda **abierto**:

1. La UI solo consume la respuesta **síncrona** del POST (síntesis o acuse de encolado).
2. No hay correlación ni sondeo del veredicto del proceso despachado (`Process_Execution_Completed` u evento de dominio de cierre).
3. El operador no distingue empíricamente «chat degradado a eco» vs «proceso encolado y en curso» vs «proceso resuelto».

### Verdad objetiva (estado deseado)

Kalma2 permanece **terminal sensorial periférica**. Tras un estímulo:

| Camino | Comportamiento objetivo |
|--------|-------------------------|
| **Chat** (`intent=chat`) | Síntesis vía `mayeuta-llm` (CLI Cursor); fallback determinista solo como degradación explícita y telemetrada, nunca como ilusión de inteligencia. |
| **Execute** (`intent=execute` + allowlist) | Emisión `Kalma2_Process_Requested` (ya forjada) + acuse inmediato con `event_id`; UI entra en estado `pending` y cierra el lazo al correlacionar la resolución del proceso en el bus fractal. |

Invariante: **`kalma2-bridge` no interpreta ni emite eventos** (ceguera espacial intacta). Emisión y orquestación viven en genoma (`kalma2-interact` + Sistema Nervioso).

---

## 2. Clarificación (Clarify)

Resumen vinculante — detalle en `docs/features/kalma2-event-bus-integration/clarify.md`.

| ID | Decisión |
|----|----------|
| **D1** | Origen del «mock» = fallback Mayeuta (`mayeuta.rs`), **no** `app.js` ni el bridge. |
| **D2** | Aduana HTTP ya operativa; no reescribir el puente salvo endpoints de **consulta** de estado (si Dedalo lo exige). |
| **D3** | Payload UI = `{"prompt": string}` (contrato actual). `capsule-json-io` aplica a cápsulas Skill/Tool, no al JSON del browser. |
| **D4** | Bus fractal SSOT: `eda_fractal.domain` → `.events/domain/` (no `.SddIA/events/domain/`). |
| **D5** | `Kalma2_Process_Requested` + suscriptor ya existen; alcance = **cierre del lazo de feedback UI**, no re-forjar emisión. |
| **D6** | Dual-signal L1: `delivery_state` dominio + `Process_Execution_Completed.payload.correlation_id` (`correlation_id ≡ event_id`). Sin evento dominio nuevo. |
| **D7** | Canal UI L2: `GET /api/status` en bridge (poll); WebSockets fuera. |
| **D8** | Deudas heredadas relevantes: D1 timeout CLI, D3 triaje `task-queue-manager`, D5 E2E CLI real (PBI router). |

---

## 3. Plan de Acción (Dedalo — `plan.md`)

| Fase | Acción |
|------|--------|
| T0 | Auditoría aduana (cerrada) |
| T1 | Handler: `degraded` + `correlation_id` alias |
| T2 | Peaje: `correlation_id` → `Process_Execution_Completed` |
| T3 | Bridge: `GET /api/status` |
| T4 | UI: pending + poll + marca degradado |
| T5 | Smokes + `implementation.md` / `execution.md` → Argos |

---

## 4. Fuera de alcance (v1)

- Reescribir `kalma2-bridge` como emisor EDA (viola ceguera espacial).
- WebSockets / SSE obligatorios.
- Conversación con historial (Estado Cero por POST).
- Sustituir allowlist o ampliar procesos despachables.
- Sustituir `Process_Execution_Completed` por un evento de dominio nuevo **sin** laudo Dedalo.
