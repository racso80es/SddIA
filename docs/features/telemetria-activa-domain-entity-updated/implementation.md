---
feature_name: telemetria-activa-domain-entity-updated
created: "2026-07-19"
process: feature
items:
  - T0.1 domain-entity-telemetry-captured
  - T0.2 memory-evolution-ingest
  - T1 event-domain-subscriptions
  - T2 radamanto_batch_core emit
  - T3 memory_evolution_ingest_core + adapter
---

# Implementation — telemetria-activa-domain-entity-updated

## Touchpoints aplicados

| ID | Artefacto | Cambio |
|----|-----------|--------|
| G1 | `SddIA/events/domain/domain-entity-telemetry-captured.md` | Forjado vía `entity-manager` — UUID `54a49fa7-8d45-4376-9aa1-deeebeb301ea` |
| G2 | `SddIA/process/memory-evolution-ingest.md` | Forjado vía `entity-manager` — UUID `eb50d05d-c8d8-4cb7-a7ed-4d296971cbe2` (stub creator; runtime en handler nativo) |
| G3 | `SddIA/core/event-domain-subscriptions.json` | Clave `Domain_Entity_Telemetry_Captured` → `memory-evolution-ingest` |
| R1 | `radamanto_batch_core.rs` | `emit_telemetry_captured_failsoft` en caminos success |
| R2 | `memory_evolution_ingest_core.rs` + `engine/mod.rs` + `route_fractal_core.rs` | Handler + dispatch fractal |
| R3 | `lancedb_evolution_repo` | Persistencia JSON durable bajo `.SddIA/vector_store/evolution/` |
| R4 | `thermodynamic.rs` | Exención peaje para `memory-evolution-ingest` |

## Decisiones de implementación

- **Plan B** confirmado en runtime: no se mutó `Domain_Entity_Updated`.
- Store v1 = archivos JSON idempotentes (`stim-<hash16>.json` por `origin_stimulus.event_id`); bindings LanceDB nativos = deuda.
- Embeddings: `null` (metadata-first).
- `Vector_Memory_Indexed`: no emitido en v1 (deuda opcional; AC3 cubierto por store).
- Proceso genoma stub: despacho early en `run_process` / `dispatch_fractal_subscriber` (paridad radamanto/daemon-heartbeat).

## Deuda explícita

1. Bindings LanceDB reales en adapter (hoy JSON durable).
2. Emisión opcional `Vector_Memory_Indexed` post-ingest.
3. `agent-creator` update destruye cuerpo rico (incidente al sellar radamanto; restaurado UUID `4d5e6f7a-…` a mano bajo feature activa).

## Sync pre-cierre

README, evolution `ccabb2a1-…`, índices events/process/agents, `radamanto.instructions.json` R4.5 — ver `execution.md`.
