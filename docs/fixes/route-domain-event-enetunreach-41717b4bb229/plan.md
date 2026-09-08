---
feature_name: route-domain-event-enetunreach-41717b4bb229
created: "2026-09-08"
process: bug-fix
phases:
  - dlt-transient-predicate
  - mayeuta-dlt-bucket
  - verify-unit
  - e2e-capsule-simulate0
  - evolution-dcc
branch_name: fix/route-domain-event-enetunreach-41717b4bb229
persist_ref: docs/fixes/route-domain-event-enetunreach-41717b4bb229
execution_id: "7065b7fe-c55c-43cd-876c-78dfca1f99b1"
---

# Plan — fractura `41717b4bb229`

## Fase 1 — Predicado (CA1/CA2)

`dlt_transient_network_trace` en `route_domain_core.rs`. Guardia al inicio de `emit_dlt_batch_fracture`: match → return (no pending).

Tests: traza PBI → pending sin `System_Fracture_Detected`; `enqueue_dlt_reanchor` con esa causa escribe `{uuid}.json`; `config-missing` sigue emitiendo (`emit_dlt_batch_fracture_publish_error_friction`).

## Fase 2 — Cubo Mayeuta (CA3)

`is_dlt_publish_error_trace` + propuesta `process_fix`. Catch-all `failed` exige `!dlt_publish`.

Tests: traza PBI → `process_fix`, sección sin «Ajustar instrucción operador»; `analyze_fracture_kaizen_recursion_verdict` / e2e `colapsó` intactos.

## Fase 3 — Verificación unitaria

```text
cd SddIA && cargo test -p execute-process --lib -- dlt_transient emit_dlt_batch_fracture analyze_fracture_kaizen enqueue_dlt_reanchor
```

## Fase 4 — E2E CA4

Cápsula `iota-immutable-publisher` `SIMULATE=0` (`./sddia-run.sh --tool …`). Si `863d1511` sigue sin `merkle_anchored`, re-encolar y dejar que el drain del siguiente `route-domain-event` ancle; si no resoluble, lote isomorfo. Documentar digest en `execution.md`. Prohibido curl POST.

## Fase 5 — Evolution + cierre

`sddia-qa evolution-register` (`alta`/`modificacion`). `implementation.md` / `execution.md` / `validacion.md`. PBI → `docs/todos/done/`. CA-CI `PENDIENTE-CI` hasta `run_id` verde. `delivery-close-cycle`. Tras checks verdes: `accept-pr`.
