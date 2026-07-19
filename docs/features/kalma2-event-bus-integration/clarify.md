---
feature_name: kalma2-event-bus-integration
created: "2026-07-19"
process: feature
purpose: Estabilización Mayeuta del PBI de integración real Kalma2 ↔ motor de eventos
---

# Clarificación — kalma2-event-bus-integration

Transcript Mayeuta (2026-07-19). Semilla v0 «Por Refinar» (archivo mal nombrado `[[OPERATIVO]…`) → PBI v1.1.0 **Refinado**.

---

## D0 — Apertura formal

| Pregunta | Decisión |
|----------|----------|
| Proceso | `feature` v1.3.0 |
| `feature_name` | `kalma2-event-bus-integration` |
| Rama | `feat/kalma2-event-bus-integration` |
| `persist_ref` | `docs/features/kalma2-event-bus-integration` |
| `document_id` | `PBI-KALMA2-EVENT-BUS-INTEGRATION` |
| Init lab | `./sddia-run.sh --process feature` + `SDDIA_LAB_SKIP_PBI_ARCHIVE=1` + `SDDIA_LAB_SKIP_DELIVERY_CLOSE=1` → `execution_id` `f0637293-253a-4635-b365-2bba0cb59038` |
| Fase actual | Argos APTO — PBI en `done/` — pendiente PR (`delivery-close-cycle`) |
| Dependencias | `kalma2-bridge-rust` (APTO) · `kalma2-mayeuta-llm-router` (APTO) |
| Laudo Dedalo | Dual-signal (`delivery_state` + PEC/`correlation_id`) — ver `spec.md` L1–L4 |

---

## D1 — Origen del «mock» (incongruencia corregida)

| Borrador v0 | Hecho en `main` | Decisión |
|-------------|-----------------|----------|
| Hardcodeo en `interfaces/kalma2/app.js` | `app.js` solo `fetch("/api/interact")` + render `response` | **No purgar mock del frontend** — no existe |
| Bridge devolviendo mock por CLI fracturado | Bridge passthrough síncrono a `execute-process` | Aduana HTTP **fuera de sospecha primaria** |
| Texto «fricción arquitectónica» como simulación UI | `synthesize_mayeuta_response` en `handlers/mayeuta.rs` | Es **fallback determinista** compartido (paridad telegram) |

**Toll:** el síntoma visible es degradación de síntesis, no potestad del cliente de fingir inteligencia.

---

## D2 — Topología ya entregada (no re-forjar)

| Capacidad | Estado | Implicación |
|-----------|--------|-------------|
| `kalma2-bridge` GET estático + POST `/api/interact` | ✅ | No reescribir puente como emisor |
| `kalma2-interact` + Skill `mayeuta-llm` | ✅ | Chat vía CLI / fallback |
| Emisión `Kalma2_Process_Requested` + allowlist | ✅ | Execute path ya encola |
| Suscriptor / `dispatch_subscriber` rama Kalma2 | ✅ (O14 router) | Sistema Nervioso ya despacha |
| Feedback UI post-encolado | ❌ | **Brecha real de este PBI** |

---

## D3 — Contrato de payload (incongruencia corregida)

| Borrador v0 | Norma / código | Decisión |
|-------------|----------------|----------|
| UI debe enviar envelope `capsule-json-io.md` | Esa norma rige Skill/Tool stdin/stdout | UI conserva `{"prompt": string}`; el envelope v2 vive bajo el puente → orquestador |
| Puente materializa evento en `.SddIA/events/domain/` | SSOT `eda_fractal.domain` = `.events/domain` | Prohibido documentar `.SddIA/events/…` como destino |
| Puente «lanza evento sin opinar» | Ceguera espacial del bridge (O4 bridge-rust) | Emisor = `kalma2-interact`; bridge **no** escribe en bus |

---

## D4 — Dos caminos, un cliente

```text
POST /api/interact {prompt}
        │
        ▼
  kalma2-interact
        │
   ┌────┴────────────────────┐
   │ intent=chat             │ intent=execute + allowlist
   ▼                         ▼
 síntesis LLM / fallback   Kalma2_Process_Requested + acuse (event_id)
   │                         │
   └──── UI hoy: sync ───────┘
                             │
                             ▼
                    [BRECHA] UI no observa cierre
                    Process_Execution_Completed
                    (u otro contrato Dedalo)
```

| Camino | Alcance v1 |
|--------|------------|
| Chat degradado | Hacer visible/telemetrada la degradación; no confundir eco con resolución de proceso |
| Execute | Cerrar lazo: `pending` → resolución correlacionada por `event_id` |

---

## D5 — Señal de cierre y canal UI

| Opción | Pros | Contras | Laudo Mayeuta (provisional) |
|--------|------|---------|-----------------------------|
| Poll HTTP en bridge (`GET /api/status?event_id=`) que lea bus/orchestration | Encaja ceguera espacial si solo lee; sin WS | Bridge gana ruta de lectura (sigue sin business logic) | **Preferida v1** |
| UI lee filesystem `.events/` directo | Simple en lab | Filtra host; rompe encapsulación del cliente | Rechazada |
| WebSockets/SSE | UX baja latencia | Fuera de objetivos bridge previos; coste | Solo si Dedalo justifica |

Candidato de evento de cierre: `Process_Execution_Completed` (familia `orchestration`, ya emitido por peaje termodinámico). Dedalo debe confirmar si basta o hace falta proyección de dominio dedicada.

---

## D6 — Deudas heredadas (no olvidar)

Del PBI `kalma2-mayeuta-llm-router` (done):

| ID | Deuda | Relación con este PBI |
|----|-------|------------------------|
| D1 | Timeout CLI no implementado | Bloquea UI en chat si CLI cuelga — agrava percepción de «inerte» |
| D3 | `task-queue-manager` genérico | Puede retrasar resolución observable |
| D5 | E2E CLI Cursor real no en CI | Chat «real» depende de `.dev/.env` del operador |

Este PBI **no liquida** D1/D3/D5 salvo que Dedalo los declare bloqueantes del lazo.

---

## D7 — Higiene documental

| Ítem | Acción |
|------|--------|
| Nombre archivo PBI `[[OPERATIVO]…` | Renombrado a `[OPERATIVO]…` |
| Frontmatter PBI | Añadido (`document_id`, `uuid`, `feature_ref`, `depends_on`) |
| Semilla sin estructura SddIA | v1.1.0 alineada a patrón PBI operativo |

---

## Preguntas Dedalo — cerradas (L1–L4)

| # | Pregunta | Laudo |
|---|----------|-------|
| 1 | ¿PEC o `Kalma2_Process_Resolved`? | **L1** — Dual-signal: `delivery_state` dominio + `Process_Execution_Completed.payload.correlation_id`. **Sin** evento dominio nuevo. Requiere plumb PEC. |
| 2 | ¿Dónde status? | **L2** — `GET /api/status` en `kalma2-bridge` (solo lectura). |
| 3 | ¿`correlation_id` extra? | **L3** — `correlation_id ≡ event_id`; alias en `data` del acuse. |
| 4 | ¿`degraded`? | **L4** — Obligatorio `degraded: true` en fallback chat. |

Detalle normativo: `spec.md` §2–§3. Blueprint de forja: `plan.md` T1–T5.
