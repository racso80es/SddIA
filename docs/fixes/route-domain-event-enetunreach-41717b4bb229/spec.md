---
feature_name: route-domain-event-enetunreach-41717b4bb229
created: "2026-09-08"
process: bug-fix
base: main
scope: dlt-f4c-enetunreach
branch_name: fix/route-domain-event-enetunreach-41717b4bb229
persist_ref: docs/fixes/route-domain-event-enetunreach-41717b4bb229
pbi_ref: docs/todos/pending/[FIX] route-domain-event — fractura sistémica (41717b4bb229).md
document_id: PBI-FIX-FRACTURE-41717b4bb229
execution_id: "7065b7fe-c55c-43cd-876c-78dfca1f99b1"
---

# Especificación — fractura `41717b4bb229` (ENETUNREACH ≠ colapso)

## Problema

`route-domain-event` `merkle-batch-preseal` abortó con:

```
merkle-batch-preseal failed: iota-relay-publish-error: status=500 fetch failed | cause: ENETUNREACH
```

El abort de publish es **correcto**. Defectos:

| ID | Defecto |
|----|---------|
| F1 | Detonante infra: host sin ruta hacia fullnode Testnet (`ENETUNREACH`). No es código. |
| F2 | `emit_dlt_batch_fracture` escala todo error de cápsula a `System_Fracture_Detected` aunque `stamp_batch_anchor_error` ya encola `dlt_reanchor`. |
| F3 | `analyze_fracture_kaizen` catch-all token `failed` → `prompt_adjustment`. Deuda explícita de `a90fad3fa8fa`. |

`relay-error.mjs` (PR **#245**) ya serializa `err.cause`. Taxonomía `b3a715381787` ya opera. Prohibido reabrirlos.

Patrón homólogo: F4c (`dcc_transient_network_trace` + `dcc_net_block_suppresses_fracture`). **No** reabrir DCC. **No** retry/backoff (DA-5).

## Cambio requerido

Motor: `route_domain_core.rs` + `enrich_fracture_pbi_kaizen.rs`. **No** genoma. **No** `server.mjs`.

### Predicado `dlt_transient_network_trace` (case-insensitive)

| Token | Origen |
|-------|--------|
| `enetunreach` | Node/undici `err.cause.code` (sello PBI) |
| `etimedout` | timeout POSIX |
| `enotfound` | DNS Node |
| `network is unreachable` | paridad F4c |
| `connection timed out` | paridad F4c |

Si match: **no** llamar el cuerpo de `emit_dlt_batch_fracture` (pending fractura vacío). `stamp_batch_anchor_error` + `enqueue_dlt_reanchor` **intactos**.

500 / `iota-relay-publish-error` **sin** token de red (p. ej. `config-missing`) **sigue** emitiendo. CA de `emit_dlt_batch_fracture_publish_error_friction` intacto.

### Cubo Mayeuta

Antes del catch-all `failed`: traza con `iota-relay-publish-error` o `F-DLT-PUBLISH-ERROR` → `process_fix` (publish con relay vivo / transporte). El catch-all **niega** este cubo (si no, `verdict_priority` gana `prompt_adjustment`).

Veredicto inventado `infrastructure_resilience`: **prohibido**.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| DLT-NET-CA1 | Traza PBI no materializa `System_Fracture_Detected`; cola `dlt_reanchor` sí se escribe |
| DLT-NET-CA2 | `iota-relay-publish-error` + `config-missing` **sí** emite (regresión) |
| DLT-NET-CA3 | `analyze_fracture_kaizen` sobre traza PBI → `process_fix`; sin `prompt_adjustment`; catch-all no-DLT intacto |
| DLT-NET-CA4 | Publish `SIMULATE=0` vía cápsula con `transaction_digest`; `/health` ≠ publish |
| CA-CI | `global: APTO` solo con `run_id` verde |

## Fuera de alcance

- Reabrir taxonomía, cause-propagation, DCC, ELF R1.
- Resolver conectividad del host.
- Retry/polling de IOTA.
- Bypass raw. Simular IOTA como Done.
- Mutar genoma `iota-immutable-publisher.md`.
