---
feature_name: kalma2-pasarela-asincrona-eda
created: "2026-07-22"
process: feature
branch_name: feat/kalma2-pasarela-asincrona-eda
persist_ref: docs/features/kalma2-pasarela-asincrona-eda
document_id: PBI-044-KALMA2-PASARELA-ASINCRONA-EDA
pbi_uuid: 8c71b50f-7067-472a-a149-40041920b054
pbi_ref: docs/todos/pending/[ARQUITECTURA] PBI-044 — Pasarela asíncrona Kalma2 y desacople por bus de eventos.md
depends_on:
  - docs/features/kalma2-event-bus-integration
  - docs/features/kalma2-process-dispatch
  - docs/features/kalma2-full-cycle
  - docs/features/kaizen-kalma2-feature-cycle-observability
status: blueprint_locked
---

# Objetivos — kalma2-pasarela-asincrona-eda

## Misión

Desacoplar el socket HTTP de `kalma2-bridge` (`POST /api/interact|execute`, `mode=execute`) del runtime completo de `execute-process kalma2-interact`, de modo que el operador reciba acuse inmediato y siga el veredicto por correlación EDA (`GET /api/status` / opcional Telegram), **sin** convertir el bridge en emisor del bus ni inventar un segundo evento de intención.

## Punto objetivo

> **O-PASARELA-ASYNC:** Dada una intención execute válida, el bridge responde en p99 &lt; 50 ms con `success: true`, `status: "accepted"` y `correlation_id` (HTTP preferente **202**), despacha en fondo el emisor genómico, y el ciclo existente (`Kalma2_Process_Requested` → nervio → TQM → `Process_Execution_Completed`) progresa con `correlation_id ≡ event_id`, cero writes EDA desde el bridge.

## Alcance (H1+H2 = Done mínimo PBI)

| Dentro | Fuera |
|--------|-------|
| Ingesta HTTP no bloqueante (R1) | waiting-for-shell / Shell async Cursor Agent (Q5) |
| Ceguera espacial: spawn/detach o cápsula ingest ciega; veto bridge-write-EDA (R2/Q1) | IOTA DLT / firma protocolo |
| Reutilizar ECST `Kalma2_Process_Requested` (R3) | systemd daemons |
| Consumo nervioso intacto (R4) | Reescribir emisión / allowlist |
| Terminal PEC + `GET /api/status` (R5) | Chat SSE / `mode=chat` (AC-R5) |
| Contrato acuse UI + smoke timing + audit no-write-bus | PBI-043 DI residual |
| Regresión Kalma2 execute post-desacople | PPR#136 F3 git-manager KM |
| | R6 Telegram (H3) — opcional, no bloquea Done (Q4) |

## Objetivos medibles

| ID | Objetivo | Criterio (AC) |
|----|----------|---------------|
| **O1** | Acuse no bloqueante | AC-R1: respuesta `accepted` + `correlation_id` en &lt; 50 ms p99; sin esperar Argos/cascada en el mismo request |
| **O2** | Ceguera espacial | AC-R2: rastro durable `Kalma2_Process_Requested` o spawn correlacionado; audit/grep = cero writes EDA desde `kalma2-bridge` |
| **O3** | ECST canónico | R3: solo `Kalma2_Process_Requested`; prohibido `Kalma2_Interaction_Requested` |
| **O4** | Nervio intacto | AC-R3: TQM/ciclo sin regresión process-dispatch; orphan/DL no empeora baseline |
| **O5** | Observabilidad terminal | AC-R4: `GET /api/status?event_id=` con UUID del acuse proyecta hasta `completed`/`failed` vía PEC en `eda_fractal.orchestration` |
| **O6** | Correlación | Q3: bridge preasigna UUID → inputs → emisión; `correlation_id ≡ event_id` |
| **O7** | Contrato HTTP | Q2: preferir **202 Accepted**; cuerpo con `success`/`status`/`correlation_id` |
| **O8** | Documentación / Done | clarify+objectives (+spec/plan Dedalo); cierre con AC-DONE-PBI (`validacion.md` APTO, PBI en `done/`, mismo PR) |

## Flujo ontológico objetivo (qué, no cómo)

```text
Cliente HTTP
  → POST execute (intención)
  → bridge: valida mínimo + 202 + correlation_id (libera socket)
  → [fondo] execute-process kalma2-interact (genoma)
       → Kalma2_Process_Requested (eda_bus / fractal según contrato vigente)
  → event-watcher → route-domain-event → TQM / ciclo hijo
  → Process_Execution_Completed (eda_fractal.orchestration)
  → Cliente: GET /api/status (Telegram R6 solo si H3)
```

## No objetivos

- Convertir `kalma2-bridge` en escritor de `.events/**`.
- Inventar evento de intención paralelo.
- Reabrir arquitectura de despacho/emisión ya sellada (salvo bugs).
- Absorber chat SSE, DI residual, F3 PPR#136 o Shell IDE.
- Exigir R6/Telegram para declarar Done.

## Invariantes

- `SddIA/core/cumulo.paths.json` = SSOT de paths (`eda_bus.pending`, `eda_fractal.orchestration`, …).
- Bridge = aduana inerte; emisión = genoma.
- `correlation_id ≡ event_id`.
- Git vía `skill:git-manager`; semillas Kaizen/TODOs solo Cumulo o `Kaizen_Alert_Required`.

## Ley aplicada

- `.cursorrules` §4–§5 (cápsulas JSON; agnosticismo Core)
- Ceguera espacial kalma2-bridge (laudo event-bus-integration / process-dispatch O5)
- `features-documentation-pattern` v1.2.1 + `task-closure-documental`
- Proceso `feature` — fase Estabilización → Dedalo consume este cuerpo como `refined_requirements`
- Clarificaciones D0–D8 y laudos Q1–Q5 en `clarify.md`

## Artefactos de referencia

- Bridge: `SddIA/interfaces/kalma2-bridge/`
- Cúmulo: `SddIA/core/cumulo.paths.json`
- Suscripciones: `SddIA/core/event-domain-subscriptions.json`
- PBI: `docs/todos/pending/[ARQUITECTURA] PBI-044 — Pasarela asíncrona Kalma2 y desacople por bus de eventos.md`
