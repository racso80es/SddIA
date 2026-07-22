---
feature_name: inyeccion-dependencias-h8-familia-route
created: "2026-07-22"
updated: "2026-07-22"
process: feature
branch_name: feat/inyeccion-dependencias-h8-familia-route
persist_ref: docs/features/inyeccion-dependencias-h8-familia-route
document_id: PBI-043-H8-FAMILIA-ROUTE
execution_id: a7c3e91f-2b84-4d6e-9f01-5c8a2e7d4b63
runtime: tekton-kalma2-cursor
verdict: ready_for_argos
q1_laudo: alta-bus-route
ac_h8_branch: A
phase: r4-r5-done
---

# Implementation — H8 Familia route (Rama A)

## R4 — Alta `bus:route`

| Artefacto | Estado |
|-----------|--------|
| `capability-taxonomy` v1.0.3 | `bus:route` / `bus.route` / 1.0.0 |
| `bus.route.schema.json` | nuevo (`success`, `exitCode`) |
| `capability-bindings` v1.2.0 | → `skill:bus-operator` |
| `bus-operator` v1.1.0 | `provides` += `bus:route` |

## R5 — Ola N_ola=3

| ED | Capacidad | Path | event_id |
|----|-----------|------|----------|
| `route-domain` | `bus:route` | mixto + `agent:cumulo` | `c392ecd3-71f4-4a3c-8ae4-2596bb43e84d` |
| `route-orchestration` | `bus:route` | mixto + `agent:cumulo` | `4205ff38-95b9-42c3-b87c-b072bfa807aa` |
| `route-telemetry` | `bus:route` | mixto + `agent:cumulo` | `72c1759c-8d6e-43fe-b46b-e3773abf423a` |

## Q8

`route-domain-event` — `fs:persist` ×3; **noop**.

## Evidencia

| Check | Resultado |
|-------|-----------|
| Inventario | with=29 / without=13 |
| `audit-eda-coverage --scan` | `orphan_count: 0` |
| `cargo test` capability_di | 17/17 |
| `cargo test` cerbero_di | 7/7 |
| verify-process-integrity ×3 | OK |

## Pendiente

Argos → `validacion.md` · delivery-close (lab skip hasta orden Racso). PBI-043 en `pending/`.
