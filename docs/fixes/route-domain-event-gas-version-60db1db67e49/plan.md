---
feature_name: route-domain-event-gas-version-60db1db67e49
created: "2026-09-09"
process: bug-fix
phases:
  - relay-serial-queue
  - dlt-gas-predicate
  - mayeuta-catchall-dlt-subtype
  - verify-unit
  - e2e-capsule-simulate0
  - genome-evolution-dcc
branch_name: fix/route-domain-event-gas-version-60db1db67e49
persist_ref: docs/fixes/route-domain-event-gas-version-60db1db67e49
execution_id: "468e7f10-99aa-4c91-8803-e06e229742a3"
---

# Plan — fractura `60db1db67e49`

## Fase 1 — Cola relay (CA1)

`.SddIA/services/iota-publish-relay/publish-queue.mjs` + test `publish-queue.test.mjs`. `server.mjs` serializa `publishImmutableData`. `npm test` en el relay.

## Fase 2 — Predicado gas (CA2)

`dlt_transient_gas_version_trace` en `route_domain_core.rs`. Guardia en `emit_dlt_batch_fracture`. Tests: traza PBI no escribe pending; `issues with transaction inputs` sin firma sí emite; `config-missing` y ENETUNREACH intactos.

## Fase 3 — Mayeuta (CA3/CA5/CA6/CA7)

`enrich_fracture_pbi_kaizen.rs`: subtipos DLT; catch-all sin `failed`. Tests `analyze_fracture_kaizen` (traza 60db1db, ENETUNREACH, `{acción} failed:` genérico). `colapsó` e2e intacto.

## Fase 4 — Unitario

```text
cd SddIA && cargo test -p execute-process --lib -- dlt_transient emit_dlt_batch_fracture analyze_fracture_kaizen enqueue_dlt_reanchor
cd .SddIA/services/iota-publish-relay && node --test publish-queue.test.mjs relay-error.test.mjs
```

## Fase 5 — E2E CA4

Cápsula `iota-immutable-publisher` `SIMULATE=0` (`./sddia-run.sh --tool`). Digest en `execution.md`. Prohibido curl POST.

## Fase 6 — Genoma + evolution + cierre

`entity-manager` update `enrich-fracture-pbi-kaizen`. `sddia-qa evolution-register`. `implementation.md` / `execution.md` / `validacion.md`. PBI → `docs/todos/done/`. `delivery-close-cycle`. Tras checks CI verdes: `accept-pr`.
