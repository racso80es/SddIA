---
feature_name: kalma2-canal-telemetria-progreso
created: "2026-08-15"
process: feature
branch_name: feat/kalma2-canal-telemetria-progreso
persist_ref: docs/features/kalma2-canal-telemetria-progreso
uuid: c8f4a2e1-9d3b-4f67-a1c5-8e2b6d09f4a7
status: executed
agent: tekton
document_id: PBI-KALMA2-CANAL-TELEMETRIA-PROGRESO
items_applied:
  - T0-topologia-contrato
  - T1-emision-core
  - T2-bridge-sse
  - T3-wui-dual
  - T4-sweeper
  - T5-smokes-docs
---

# Ejecución — kalma2-canal-telemetria-progreso

## Smokes AC (evidencia Tekton)

| ID | Criterio | Evidencia | Resultado |
|----|----------|-----------|-----------|
| **AC1** | Bridge caído ⇒ execute-process OK | Emisión PTC best-effort sin HTTP push; tests IO failure no panic | PASS (unit) |
| **AC2** | Sin PTC en domain/DLT | `progress_trace` escribe solo bajo `eda_fractal.progress`; test `emit_writes_under_progress_leaf` | PASS (unit) |
| **AC3** | Sin PTC en peaje | Módulo aislado; sin llamadas `route-telemetry` / `write_fractal_event` telemetry | PASS (inspección) |
| **AC4** | `/api/status` intacto; progreso adicional | Routing nuevo antes de static (test `progress_stream_route_before_static` scoped a `dispatch`); UI dual-canal | PASS |
| **AC5** | Latencia WUI &lt;100 ms | AC interfaz — medición manual/lab UI (fuera gate Core) | DEFER (lab) |
| **AC6** | Poda progress post-PEC/TTL | `sweep_progress_leaf` + test `progress_sweep_purges_on_terminal_pec` | PASS (unit) |
| **AC7** | Rust sin panics touchpoints | `cargo test` perímetro + `cargo check` tres crates `Finished` | PASS |

## Comandos

```bash
cd SddIA && cargo test -p execute-process progress_trace
cd SddIA && cargo test -p kalma2-bridge
cd SddIA && cargo test -p sddia-daemon-runtime progress_sweep
cd SddIA && cargo check -p execute-process -p kalma2-bridge -p sddia-daemon-runtime
```

## Notas

- Norma `progress-trace-contract.md`: sello `entity-manager` create idempotente → `Domain_Entity_Created` `629e714d-f5f4-4598-8ebf-0659ae493ec1` (uuid `7d4e9f12-…` conservado; hash `sha256:45bd8b44…`).
- Evolution: `9451ac66-cfa9-4415-bc00-032c75b12a09`.
- Relay IDE: SSE `Read` no retorna EOF (`Ok(0)`) en idle; EventSource se cierra al veredicto; bridge no crea dirs en la hoja progress.
- Veredicto global: Argos **APTO** (`validacion.md`); PBI en `docs/todos/done/`.
