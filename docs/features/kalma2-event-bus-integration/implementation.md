---
feature_name: kalma2-event-bus-integration
created: "2026-07-19"
process: feature
items:
  - T1-kalma2-degraded-correlation
  - T2-thermodynamic-pec-correlation
  - T3-bridge-api-status
  - T4-ui-poll
  - T5-docs-readme
---

# Implementation — kalma2-event-bus-integration

## Touchpoints

| # | Path | Cambio |
|---|------|--------|
| 1 | `SddIA/engine/execute-process/src/engine/handlers/kalma2.rs` | `degraded` en fallback/Triaje-C; `correlation_id ≡ event_id` en acuse execute |
| 2 | `SddIA/engine/execute-process/src/engine/thermodynamic.rs` | PEC emite con `payload.correlation_id` si viene en inputs; PEC también si solo hay correlation (sin workspace) |
| 3 | `SddIA/interfaces/kalma2-bridge/src/main.rs` | `GET /api/status` — lectura fractal domain/dead-letter/orchestration |
| 4 | `interfaces/kalma2/app.js` | Poll post-`emitted`; marca `[degradado]`; 404 intermedio = pending |
| 5 | `interfaces/kalma2/index.html` | `#status` aria-live |
| 6 | `interfaces/kalma2/style.css` | Colores por `data-kind` |
| 7 | `interfaces/kalma2/README.MD` | Contrato status + degraded |

## Notas de forja

- Dominio fractal se **purga** tras consenso (`route_domain_fractal_event` purge_after). Por eso la UI no aborta en HTTP 404 durante el sondeo.
- Prioridad de proyección status: PEC correlacionado > dead-letter > delivery_state dominio > pending.
- Bridge permanece sin write al bus.
