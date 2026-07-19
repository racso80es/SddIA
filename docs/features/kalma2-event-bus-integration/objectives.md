---
feature_name: kalma2-event-bus-integration
created: "2026-07-19"
process: feature
branch_name: feat/kalma2-event-bus-integration
persist_ref: docs/features/kalma2-event-bus-integration
document_id: PBI-KALMA2-EVENT-BUS-INTEGRATION
uuid: 7047f38b-333f-4b85-bab1-1a6ff6992009
status: validacion_apto
pbi_ref: docs/todos/done/[OPERATIVO] PBI: Integración Real de Kalma2 con el Motor de Eventos SddIA.md
depends_on:
  - docs/features/kalma2-bridge-rust
  - docs/features/kalma2-mayeuta-llm-router
execution_id_init: f0637293-253a-4635-b365-2bba0cb59038
---

# Objetivos — kalma2-event-bus-integration

## Misión

Cerrar el lazo sensorial Kalma2 ↔ motor de eventos SddIA: el cliente permanece terminal periférica; el operador distingue chat (síntesis real o degradación marcada) de execute (encolado EDA + resolución correlacionada), sin reintroducir simulación en UI ni romper la ceguera espacial de `kalma2-bridge`.

## Punto objetivo

> **O-LAZO-KALMA2:** Tras `Kalma2_Process_Requested`, la UI observa el cierre del proceso vía correlación (`event_id`) y solo entonces renderiza el veredicto; el eco determinista Mayeuta deja de confundirse con inteligencia ni con fin de proceso.

## Alcance

| Dentro | Fuera |
|--------|-------|
| Correlación acuse → resolución en bus fractal | Reescribir bridge como emisor EDA |
| Estado `pending` + canal de consulta (poll v1) | WebSockets/SSE obligatorios |
| Etiquetado/telemetría de degradación chat | Historial conversacional |
| Validación S+ UI↔`.events/`↔proceso | Ampliar allowlist de procesos |
| Documentación Dedalo (spec/plan) sobre señal de cierre | Liquidar deudas D1/D3/D5 del router salvo bloqueo |

## Objetivos medibles

| ID | Objetivo | Criterio |
|----|----------|----------|
| **O1** | Diagnóstico aduana cerrado | Confirmado: sin mock en `app.js`; fallback = `synthesize_mayeuta_response` |
| **O2** | Acuse correlacionable | Respuesta `emitted:true` expone `event_id` usable por el cliente |
| **O3** | Canal de resolución | Mecanismo documentado+implementado para consultar estado pending/completed/failed sin leer FS desde el browser |
| **O4** | Bucle UI | Indicador `pending` hasta resolución; render final = payload real del ecosistema |
| **O5** | Chat sin ilusión | Degradación a eco determinista visible (campo/flag o mensaje) cuando aplica |
| **O6** | Invariantes | Bridge sin business logic ni write al bus; paths vía Cúmulo / fractal `.events/` |
| **O7** | Validación S+ | Prompt execute crea evento en `.events/domain/`, proceso avanza, UI muestra cierre correlacionado |

## No objetivos

- Mutar genoma de emisión `Kalma2_Process_Requested` salvo defecto bloqueante descubierto en Dedalo/Tekton.
- Sustituir `mayeuta-llm` / motor CLI.
- Autenticación, Cerbero nuevo, TLS, despliegue remoto.

## Ley aplicada

- `.cursorrules` §4 (cápsulas, JSON), §5 (agnosticismo Core)
- Ceguera espacial `kalma2-bridge` (`docs/features/kalma2-bridge-rust/objectives.md` O4)
- Async EDA C2 (`docs/features/kalma2-mayeuta-llm-router/` — no despacho síncrono de ciclos largos)
- `SddIA/core/cumulo.paths.json` + `eda_fractal` (`.events/domain`, `.events/orchestration`)
- `features-documentation-pattern` v1.2.1 / proceso `feature` v1.3.0
- Clarificaciones D1–D7 en `clarify.md`
