---
feature_name: kalma2-canal-telemetria-progreso
created: "2026-08-15"
process: feature
branch_name: feat/kalma2-canal-telemetria-progreso
persist_ref: docs/features/kalma2-canal-telemetria-progreso
uuid: c8f4a2e1-9d3b-4f67-a1c5-8e2b6d09f4a7
status: implemented
agent: tekton
document_id: PBI-KALMA2-CANAL-TELEMETRIA-PROGRESO
---

# Implementación — kalma2-canal-telemetria-progreso

## Touchpoints (T0–T4)

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/core/cumulo.paths.json` | `eda_fractal.progress` = `./.events/progress` (v1.6.2) |
| `SddIA/library/norms/progress-trace-contract.md` | Norma PTC + fila índice (Raw Kernel; entity-manager bloqueado) |
| `SddIA/sddia-daemon-runtime/src/lib.rs` | `BusTopology.progress`, `load_bus_topology`, `ensure_fractal_dirs` |
| `SddIA/sddia-daemon-runtime/src/eda_sweep.rs` | Poda hoja `progress` (PEC terminal + TTL 24h) |
| `SddIA/engine/execute-process/src/engine/fractal.rs` | `load_progress_dir` / tupla 4 vías fractal |
| `SddIA/engine/execute-process/src/engine/progress_trace.rs` | `emit_progress_trace` best-effort |
| `SddIA/engine/execute-process/src/engine/executor.rs` | Ganchos inicio/fin fase con `correlation_id` |
| `SddIA/interfaces/kalma2-bridge/src/main.rs` | `GET /api/progress/stream` (replay + poll SSE) |
| `interfaces/kalma2/{app.js,index.html,style.css,README.MD}` | Dual-canal UI + consola cromática |
| `SddIA/evolution/9451ac66-cfa9-4415-bc00-032c75b12a09.md` | Hito topología ↔ feature uuid |

## Invariantes respetados

- Sin mutación `eda_fractal.telemetry` / `route-telemetry`.
- `GET /api/status` sin cambio semántico.
- PTC fuera de dominio y peaje ECST.
- Sin escritura bajo `docs/todos/`.

## Gates pendientes Argos

- `validacion.md` APTO + PBI `done/` en mismo PR (DoD single-PR).
