---
feature_name: lab-paciente0-dlt-telemetry-mvp
created: "2026-09-06"
process: feature
phases:
  - design
  - json-subscriptions
  - digest-proof-skip-tests
  - entity-manager-event
  - evolution-and-docs
  - dcc-pr-ci-accept
branch_name: feat/lab-paciente0-dlt-telemetry-mvp
persist_ref: docs/features/lab-paciente0-dlt-telemetry-mvp
pbi_ref: docs/todos/pending/PBI-LAB-PACIENTE0-DLT-TELEMETRY-MVP.md
document_id: PBI-LAB-PACIENTE0-DLT-TELEMETRY-MVP
uuid: "17380fcf-0630-45d3-9813-611d80beec0d"
execution_id: "17ed4fb6-e729-4dbe-9813-cf9985aa9bce"
---

# Plan — lab-paciente0-dlt-telemetry-mvp

Corte Diseño: **clarify + objectives + spec + plan + commit**. Ejecución en el mismo ciclo hasta PR verde y `accept-pr`.

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. `execution_id` `17ed4fb6-e729-4dbe-9813-cf9985aa9bce`.

## Fase L0 — Diseño (esta parada)

PBI v1.2.0 + cascada `persist_ref`. Commit de planificación.

## Fase L1 — JSON suscripciones (CA-3)

`SddIA/core/event-domain-subscriptions.json` y `SddIA/core/event-subscriptions.json`: segundo suscriptor Cúmulo/IOTA en `Domain_Entity_Telemetry_Captured`. Ingest intacto. Core ∉ DA-2.

## Fase L2 — Skip + proof + tests (CA-1, CA-4, CA-5)

`route_domain_core.rs` / `route_fractal_core.rs`: `skipped-config-missing`; persistir digest + proof. Tests crate `execute-process`.

```text
cd SddIA && cargo test -p execute-process --lib -- dlt_telemetry
```

## Fase L3 — Genoma evento (CA-2 documental)

Prefijo RAW. Topología `objectives.md` ya en rama.

```text
./sddia-run.sh --process entity-manager --inputs-file .tmp/em-domain-entity-telemetry-captured.json
```

`markdown_body_replacements` sobre § Suscripciones. Prohibido `Write`/`StrReplace` sobre `SddIA/events/`. Coverage del uuid lo sella `emit-domain-mutation`.

## Fase L4 — Evolution + docs de ejecución

`sddia-qa evolution-register` (`modificacion`). `implementation.md` / `execution.md`. Si toca `directories.evolution`: `sddia-qa gate-evolution --json --range` exit 0 antes del push.

## Fase L5 — Cierre, DCC, CI, accept-pr

PBI → `docs/todos/done/` cuando CA-CI verde. `validacion.md` con CA-6 `PENDIENTE-CI` hasta `run_id`; entonces `global: APTO` + `pbi_archived: true`. `delivery-close-cycle`. Tras checks verdes del PR: `accept-pr`.
