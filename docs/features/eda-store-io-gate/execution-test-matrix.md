---
feature_name: eda-store-io-gate
artifact: test-matrix
created: "2026-10-30"
items_applied:
  - tests/common/mod.rs (lab_repo, roundtrip_publish_load)
  - tests/eda_store_io_matrix_pending.rs
  - tests/eda_store_io_matrix_fractal.rs
---

# Ejecución — matriz de tests I/O

## Comandos

```bash
cd SddIA
cargo test -p execute-process --test eda_store_io_matrix_pending -- --test-threads=1
cargo test -p execute-process --test eda_store_io_matrix_fractal -- --test-threads=1
```

## Cobertura

| Ola | Tests | Modos |
|-----|-------|-------|
| Pending | Todos los `event_type` en unión `event-subscriptions.json` + `event-domain-subscriptions.json` | redb, filesystem; dual estratificado (6 tipos) |
| Fractal telemetry | Registro + `System_Vitality_Probed` | redb; filesystem (2 tipos) |
| Fractal orchestration | Registro + `TQM_Dispatch_Discarded`, `Local_QA_Requested` | redb |
| Fractal domain | Muestra 4 tipos | redb |
| Route smoke | `route_telemetry_event` sin fichero, store redb | redb |

Fixture copia catálogos core desde `find_repo_root()`; no muta genoma.
