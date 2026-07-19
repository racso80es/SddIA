---
feature_name: kalma2-event-bus-integration
created: "2026-07-19"
process: feature
branch_name: feat/kalma2-event-bus-integration
persist_ref: docs/features/kalma2-event-bus-integration
phases: "T0-auditoria,T1-degraded,T2-correlation-pec,T3-status-bridge,T4-ui-poll,T5-smokes"
uuid: 7047f38b-333f-4b85-bab1-1a6ff6992009
status: validacion_apto
---

# Blueprint — kalma2-event-bus-integration

## Estrategia

Cerrar el lazo UI↔EDA **sin** nuevo evento de dominio y **sin** romper ceguera espacial del bridge. Cadena mínima:

```text
T0  Auditoría cerrada (documental) — mock ≠ app.js
T1  Handler: degraded + correlation_id alias en acuse
T2  Peaje: correlation_id → Process_Execution_Completed
T3  Bridge: GET /api/status (lectura fractal)
T4  UI: pending + poll + marca degradado
T5  Smokes + implementation/execution + handoff Argos
```

Laudos vinculantes: **L1–L4** en `spec.md`.

## Fases

### T0 — Auditoría aduana (cerrada en Mayeuta)

- [x] Confirmar passthrough `app.js` → `/api/interact`
- [x] Localizar fallback en `handlers/mayeuta.rs`
- [x] Confirmar emisión `Kalma2_Process_Requested` + inyección `correlation_id` en `route_domain_core`
- Entregable: O1 satisfecho en `objectives.md` / `clarify.md`

### T1 — Handler kalma2 (`degraded` + alias)

- [x] En `handlers/kalma2.rs`: si respuesta por `synthesize_mayeuta_response` → `data.degraded = true`
- [x] En acuse execute: `data.correlation_id = event_id`
- [x] Tests unitarios `kalma2` verdes + aserciones nuevas
- Gate: `cargo test -p execute-process kalma2` ✅

### T2 — Plumb PEC

- [x] `thermodynamic.rs`: copiar `correlation_id` al payload PEC
- [x] PEC también si solo hay `correlation_id` (sin workspace)
- [x] Smoke `workspace-smoke` + correlation → PEC observable
- Gate: smoke S5 ✅

### T3 — Bridge `GET /api/status`

- [x] Ruta antes de `serve_static`
- [x] Paths vía `cumulo.paths.json` / `eda_fractal`
- [x] Proyección `pending|routed|completed|failed`
- [x] Tests project_status + uuid
- Gate: smokes S3/S4/S7 ✅

### T4 — UI poll

- [x] `app.js`: ramificar `emitted` / `degraded`; poll; 404→pending
- [x] `index.html` / `style.css`: `#status`
- [x] README contrato
- Gate: listo para `./start-sddia.sh` manual

### T5 — Smokes y cierre documental Tekton

- [x] Smokes S1–S7 en `execution.md`
- [x] `implementation.md` + `execution.md`
- [x] Handoff Argos → `validacion.md` APTO + PBI en `done/`

## Gates agregados

| Hito | Gate | Estado |
|------|------|--------|
| Fin T1 | tests kalma2 + `degraded`/`correlation_id` | ✅ |
| Fin T2 | PEC con `correlation_id` | ✅ |
| Fin T3 | `/api/status` 200/404 | ✅ |
| Fin T4 | UI pending→cierre | ✅ código |
| Fin T5 | AC1–AC7; Argos | ✅ APTO |

## No hacer

- Inventar `Kalma2_Process_Resolved`
- WebSockets
- Mutar allowlist / mayeuta-llm motor
- Escribir eventos desde bridge
