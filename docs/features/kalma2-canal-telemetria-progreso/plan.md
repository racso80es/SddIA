---
feature_name: kalma2-canal-telemetria-progreso
created: "2026-08-15"
process: feature
branch_name: feat/kalma2-canal-telemetria-progreso
persist_ref: docs/features/kalma2-canal-telemetria-progreso
phases: "T0-topologia-contrato,T1-emision-core,T2-bridge-sse,T3-wui-dual,T4-sweeper,T5-smokes-docs"
uuid: c8f4a2e1-9d3b-4f67-a1c5-8e2b6d09f4a7
status: dedalo_locked
agent: dedalo
document_id: PBI-KALMA2-CANAL-TELEMETRIA-PROGRESO
---

# Blueprint — kalma2-canal-telemetria-progreso

## Estrategia

Materializar canal **efímero distinto** (opción C / laudo `C-ephemeral-progress-leaf`) sin contaminar peaje ni dominio, y sin invalidar status=veredicto.

```text
T0  Topología Cúmulo + norma PTC (library_norms)
T1  Emisión fire-and-forget en execute-process
T2  Bridge: GET /api/progress/stream (replay + watch)
T3  WUI: consola cromática + dual-canal (status poll intacto)
T4  Sweeper poda hoja progress
T5  Smokes AC1–AC7 + implementation/execution → Argos
```

Laudos vinculantes: **L1–L11** en `spec.md`.

## Fases

### T0 — Topología y contrato PTC

- [ ] Añadir `eda_fractal.progress` = `./.events/progress` en `cumulo.paths.json` (forja vía cadena entidad/norma; sin bypass raw destructivo)
- [ ] Materializar `SddIA/library/norms/progress-trace-contract.md` (envelope §3 spec; **fuera** de `capability_contracts`)
- [ ] Resolver hoja en helpers de topología compartidos (bridge/daemon) sin hardcode host
- [ ] Evolution log: hito topología ↔ `uuid` feature
- Gate: paths resolubles; contrato legible; sin tocar `telemetry` peaje

**Delegación conceptual:** forja genoma vía runtime `entity-manager` / procesos norma; Tekton no inventa paths fuera de Cúmulo.

### T1 — Emisión Core

- [ ] `emit_progress_trace` best-effort (tragar IO; sin panic)
- [ ] Ganchos inicio/fin de fase en `executor` si `correlation_id` presente
- [ ] Mapa `phase.name` → enum UI (`spec`…`closure`)
- [ ] Tests unitarios: con/sin `correlation_id`; fallo FS no tumba fase
- Gate: `cargo test -p execute-process` (filtros progreso) verdes

### T2 — Bridge SSE progreso

- [ ] Ruta `GET /api/progress/stream` antes de static
- [ ] Replay + watch subdir `{progress}/{correlation_id}/`
- [ ] Frames `event: progress`; 400 UUID inválido
- [ ] Tests proyección/routing; **no** reusar handler SSE chat
- Gate: curl/lab SSE recibe PTC de fixture FS

### T3 — WUI dual-canal

- [ ] `EventSource` a `/api/progress/stream` tras `emitted`
- [ ] Conservar poll `/api/status` hasta veredicto/timeout
- [ ] Consola cromática + badge `source_agent`; auto-scroll
- [ ] README: dualidad veredicto vs progreso (no «sustituye poll»)
- Gate: lab UI muestra trazas mientras status sigue pending→terminal

### T4 — Sweeper

- [ ] Poda por PEC terminal correlacionado y/o TTL
- [ ] Aislamiento: cero writes/deletes en `eda_fractal.telemetry` peaje desde esta lógica
- Gate: smoke huérfano + post-PEC vacío

### T5 — Smokes y cierre documental Tekton

- [ ] Smokes AC1–AC7 en `execution.md`
- [ ] `implementation.md` + `execution.md`
- [ ] Handoff Argos → `validacion.md` + PBI `done/` en la **misma** rama (DoD single-PR)

## Gates agregados

| Hito | Gate |
|------|------|
| Fin T0 | `progress` en Cúmulo + norma PTC |
| Fin T1 | emisión no bloqueante + tests |
| Fin T2 | SSE distinto de chat |
| Fin T3 | dual-canal UI |
| Fin T4 | poda sin colisión peaje |
| Fin T5 | AC1–AC7; Argos |

## No hacer

- Escribir bajo `.SddIA/events/` o `capability_contracts`
- Reusar `eda_fractal.telemetry` / `route-telemetry` / compliance registry
- Sustituir `/api/status` por el stream
- Multiplexar progreso en SSE de chat
- Absorber Kaizen PEC suscriptores
- Declarar PTC como Clase ECST / cuarta familia
- Tekton/Argos escribiendo semillas en `docs/todos/`

## Orden de commits sugerido

1. docs (`spec`/`plan` ya en cascada) + T0 topología/norma + evolution
2. T1 execute-process + tests
3. T2 bridge + tests
4. T3 WUI + README
5. T4 sweeper
6. T5 implementation/execution → validacion/PBI archive
