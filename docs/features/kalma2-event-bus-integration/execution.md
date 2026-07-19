---
feature_name: kalma2-event-bus-integration
created: "2026-07-19"
process: feature
items_applied:
  - T1
  - T2
  - T3
  - T4
  - T5
---

# Execution — kalma2-event-bus-integration

## Comandos

```bash
cd SddIA && CARGO_TARGET_DIR=target cargo build -p execute-process -p kalma2-bridge
cargo test -p execute-process kalma2
cargo test -p kalma2-bridge
```

## Smokes (2026-07-19)

| ID | Escenario | Resultado |
|----|-----------|-----------|
| S1 | `kalma2-interact` chat sin CLI | `degraded:true` ✅ |
| S2 | prompt fix → emit | `emitted` + `event_id == correlation_id` + archivo `.events/domain/{id}.json` ✅ |
| S3 | `GET /api/status?event_id=` (dominio fresco) | `status:pending` HTTP 200 ✅ |
| S4 | PEC sintético con `correlation_id` | `status:completed` ✅ |
| S5 | `workspace-smoke` + `correlation_id` | PEC en orchestration con campo ✅ |
| S6 | `POST /api/interact` vía bridge | `degraded:true` ✅ |
| S7 | UUID desconocido | HTTP 404 ✅ |

## Tests unitarios

- `execute-process` filtro `kalma2`: 6 ok (incl. degraded + correlation alias)
- `kalma2-bridge`: 5 ok (uuid, project_status completed/routed, resolve bin)

## Semántica observada

Durante S3→S4 el watcher puede mover el dominio a dead-letter si suscriptores fallan en lab; PEC correlacionado sigue ganando proyección `completed` (L1).
