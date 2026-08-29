---
feature_name: telemetria-cognitiva-llm-kalma2
created: "2026-08-29"
process: feature
branch_name: feat/telemetria-cognitiva-llm-kalma2
persist_ref: docs/features/telemetria-cognitiva-llm-kalma2
phases: "T0-contrato-topologia,T1-emision-aduana,T1b-stream-inbox,T2-radamanto,T3-bridge,T4-wui,T5-smokes-docs"
uuid: a1535038-8db5-4351-8a81-cfa5586b8c5b
status: dedalo_locked
agent: dedalo
document_id: PBI-TELEMETRY-LLM-COGNITIVE-METRICS-KALMA2
execution_id: "0cdb618e-51e4-461b-9d14-469a5363257b"
---

# Blueprint — telemetria-cognitiva-llm-kalma2

## Estrategia

Cerrar el hueco real (Peaje sin receipt + STREAM fuera de Aduana) sobre el fan-out **ya vivo**, sin tercer schema ni ECST desde el bridge.

```text
T0   Contrato evento + schema default + Cúmulo inbox + evolution
T1   Acumulador state + mapeo thermodynamic.rs + mayeuta JSON
T1b  STREAM → sidecar inbox (mismo receipt)
T2   radamanto-batch: bloque cognitive + rate N1/N2
T3   Bridge: GET /api/telemetry/stream + GET /api/telemetry/cognitive
T4   WUI widget
T5   Smokes AC1–AC8 + implementation/execution → Argos
```

Laudos: **L1–L14** en `spec.md`. Genoma: `entity-manager` (evento, thresholds, skill). Código motor/bridge/WUI: Tekton directo (no DA-2).

**Parada de este commit:** T0–T5 documentados; **sin** materializar código. Siguiente estímulo = ejecución Tekton.

## Fases

### T0 — Contrato y topología

- [ ] `entity-manager` / evento: `raw-execution-finished.md` — campos receipt L2; suscriptores reales (I1); quitar “Fase 3.C”
- [ ] `DEFAULT_TELEMETRY_SCHEMA` en `fractal_bus.rs`: mantener tokens; documentar opcionales (L14: no breach por opcionales)
- [ ] `cumulo.paths.json`: `radamanto.cognitive_inbox` = `.SddIA/radamanto/inbox`
- [ ] Evolution: hito ↔ uuid `a1535038-8db5-4351-8a81-cfa5586b8c5b`
- Gate: códice coherente; sin `cognitive_metrics`

**Delegación:** forja evento/thresholds/skill vía runtime; no bisturí raw en `SddIA/events/` ni `SddIA/agents/`.

### T1 — Emisión Aduana (camino A)

- [ ] Extraer receipt de stdout cápsula (`telemetry_receipt` o mapear `thermodynamic_cost` L3) → `state.telemetry_receipts`
- [ ] `thermodynamic.rs`: L4/L5 al payload REF + `capsule_id` si existe
- [ ] `mayeuta-llm` SYNTHESIZE/CLASSIFY: parse usage best-effort; DD-2
- [ ] Tests: mapeo coste; proceso sin cápsula LLM = REF sin receipt (compliance L14)
- Gate: `cargo test -p execute-process` filtros peaje/receipt; `cargo test -p mayeuta-llm` si aplica

### T1b — STREAM inbox (camino B)

- [ ] Fin de STREAM: escribir inbox Cúmulo (L6); nunca `./.events/telemetry/`
- [ ] DD-2 si prótesis muda; exit 0 si el stream de negocio ok
- [ ] Test: inbox JSON = mismo schema
- Gate: chat SSE no contaminado (receipt **no** va por stdout STREAM)

### T2 — Radamanto

- [ ] `entity-manager`: `radamanto.thresholds.json` bloque `cognitive` (L10)
- [ ] `radamanto_batch_core.rs`: agregar receipt de REF + drenar inbox (L7)
- [ ] N1 `quota_alert`; N2 Degraded `cognitive_critical_quota` (L11)
- [ ] Tests: entidades intactas; N1 no emite dominio; N2 sí
- Gate: tests batch existentes verdes + nuevos cognitivos

### T3 — Bridge

- [ ] `GET /api/telemetry/stream` replay+watch telemetry∪inbox (L8)
- [ ] `GET /api/telemetry/cognitive` snapshot stats (L9)
- [ ] Rutas **antes** de `serve_static`; tests estilo `progress_stream_route_before_static`
- Gate: `cargo test -p kalma2-bridge`

### T4 — WUI

- [ ] `index.html` / `app.js` / CSS mínimo: pulso + badge N1
- [ ] EventSource stream + GET inicial
- [ ] No romper progress/status/chat
- Gate: verificación browser o smoke estático si no hay servidor

### T5 — Cierre documental de ejecución

- [ ] `implementation.md` + `execution.md` (Tekton)
- [ ] `validacion.md` Argos; archivo PBI en rama
- Gate: patrón v1.2.1

## Orden y riesgos

| Riesgo | Mitigación |
|--------|------------|
| Compliance rompe cápsulas no-LLM | L14: opcionales no-breach |
| STREAM rompe chat | Receipt fuera de stdout |
| Bridge escribe bus | L6 prohibido |
| N1 revoca | L11 |
| `window[]` crece | Cap acotado (p. ej. 120 muestras) en T2 |

## Fuera de este commit

Código T0–T5, PR, merge.
