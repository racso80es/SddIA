---
feature_name: kalma2-canal-telemetria-progreso
branch: feat/kalma2-canal-telemetria-progreso
global: APTO
pbi_archived: true
created: "2026-08-15"
process: feature
document_id: PBI-KALMA2-CANAL-TELEMETRIA-PROGRESO
uuid: c8f4a2e1-9d3b-4f67-a1c5-8e2b6d09f4a7
checks:
  AC1_fire_and_forget: "APTO — emit_progress_trace traga IO; test io_failure_does_not_panic; sin HTTP push al bridge"
  AC2_no_domain: "APTO — escritura solo eda_fractal.progress; test emit_writes_under_progress_leaf_not_telemetry"
  AC3_no_peaje: "APTO — progress_trace.rs sin route-telemetry ni write_fractal_event telemetry"
  AC4_dual_canal: "APTO — GET /api/status intacto; SSE /api/progress/stream adicional; poll UI permanece"
  AC5_latencia_ui: "NO_MEDIDO — AC interfaz (I7); no gate Core; no tumba execute-process"
  AC6_poda: "APTO — sweep_progress_leaf + test progress_sweep_purges_on_terminal_pec"
  AC7_rust: "APTO — cargo test perímetro + cargo check execute-process/kalma2-bridge/sddia-daemon-runtime"
git_changes:
  - SddIA/core/cumulo.paths.json
  - SddIA/core/eda-coverage.json
  - SddIA/library/norms/progress-trace-contract.md
  - SddIA/library/norms/index.md
  - SddIA/engine/execute-process/src/engine/progress_trace.rs
  - SddIA/engine/execute-process/src/engine/executor.rs
  - SddIA/engine/execute-process/src/engine/fractal.rs
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - SddIA/sddia-daemon-runtime/src/lib.rs
  - SddIA/sddia-daemon-runtime/src/eda_sweep.rs
  - interfaces/kalma2/app.js
  - interfaces/kalma2/index.html
  - interfaces/kalma2/style.css
  - interfaces/kalma2/README.MD
  - SddIA/evolution/9451ac66-cfa9-4415-bc00-032c75b12a09.md
  - docs/features/kalma2-canal-telemetria-progreso/
  - docs/todos/done/[OPERATIVO] PBI: Canal Asíncrono de Telemetría de Progreso y Observabilidad Activa para Interfaces Externas (Kalma2).md
---

# Validación — kalma2-canal-telemetria-progreso

**Veredicto global: APTO**

Argos (relay IDE, 2026-08-15). Laudos Mayeuta D0–D7 y Dedalo L1–L11 vinculantes. AC5 no bloquea: I7 / L10.

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| AC1 | Bridge caído ⇒ execute-process OK | ✅ | Best-effort; `io_failure_does_not_panic`; sin push HTTP |
| AC2 | PTC fuera de domain/DLT | ✅ | `emit_writes_under_progress_leaf_not_telemetry` |
| AC3 | PTC fuera de peaje | ✅ | Módulo aislado; sin `route-telemetry` |
| AC4 | Dual-canal status + progreso | ✅ | Routing `dispatch`; `app.js` poll + EventSource |
| AC5 | Latencia WUI &lt;100 ms | ⚠️ | No medido en lab; **no** invariante Core |
| AC6 | Poda progress | ✅ | `progress_sweep_purges_on_terminal_pec` |
| AC7 | Rust perímetro | ✅ | tests 5+13+1; `cargo check` tres crates OK |
| L1 | Hoja `eda_fractal.progress` | ✅ | Cúmulo v1.6.2 |
| L2/L3 | PTC no-ECST + library_norm | ✅ | Norma + sello `Domain_Entity_Created` `629e714d-…` |
| L5 | `trace_id` ≠ `event_id` ECST | ✅ | Envelope PTC |
| L8 | No sustituir `/api/status` | ✅ | README + UI |
| I8 | Kaizen PEC no fusionado | ✅ | `adjacent_not_merged` |

## Comandos

```bash
cd SddIA && cargo test -p execute-process progress_trace
cd SddIA && cargo test -p kalma2-bridge
cd SddIA && cargo test -p sddia-daemon-runtime progress_sweep
cd SddIA && cargo check -p execute-process -p kalma2-bridge -p sddia-daemon-runtime
SDDIA_AGENT_RUNTIME_COMMAND= ./sddia-run.sh --process entity-manager \
  --inputs-file .tmp/em-progress-trace-contract.json
```

## Deudas / matices (no bloquean APTO)

| ID | Nota |
|----|------|
| N1 | AC5 sin medición empírica de p99 &lt;100 ms (alcance interfaz Kalma2). |
| N2 | `entity-manager create` idempotente (artefacto ya existía); sello EDA `629e714d-f5f4-4598-8ebf-0659ae493ec1`. UUID de norma conservado. |
| N3 | SSE live usa poll FS 400 ms (L7 admite poll acotado); no crate `notify`. |
| N4 | Kaizen PEC / 404 `GET /api/status` post-purge **fuera de alcance**. |

## Cierre documental

- PBI archivado en `docs/todos/done/` en esta rama (`pbi_archived: true`).
- Listo para `delivery-close-cycle` / PR único.
