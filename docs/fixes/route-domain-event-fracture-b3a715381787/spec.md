---
feature_name: route-domain-event-fracture-b3a715381787
created: "2026-08-29"
process: bug-fix
base: main
scope: merkle-batch-preseal-friction-taxonomy
branch_name: fix/route-domain-event-fracture-b3a715381787
persist_ref: docs/fixes/route-domain-event-fracture-b3a715381787
pbi_ref: docs/todos/pending/[FIX] route-domain-event — fractura sistémica (b3a715381787).md
document_id: PBI-FIX-FRACTURE-b3a715381787
---

# Especificación — fractura `b3a715381787` (publish 500 ≠ relay caído)

## Problema

`merkle-batch-preseal` falló con:

```
iota-relay-unreachable: http://127.0.0.1:8787/v1/publish: status code 500
```

El relay **respondió** (HTTP 500). `ureq` envuelve todo no-2xx en `send_string(...).map_err` como `iota-relay-unreachable`. `classify_batch_anchor_friction` mapea ese prefijo a `F-DLT-RELAY-SIN-SUPERVISOR`. El cuerpo `{error,feedback}` del `server.mjs` no llega a la traza.

Distinto de `6a49e0ad310e` (payload inválido por relay caído; remediado por Kaizen DLT #208: centinela + `/health`). `/health` OK no implica publish OK.

## Cambio requerido

| Área | Artefacto | Vía |
|------|-----------|-----|
| Prefijos de error HTTP | `SddIA/tools/iota-immutable-publisher` (`publish_via_relay`) | `entity-manager` (genoma `directories.tools`) |
| Taxonomía de fricción | `classify_batch_anchor_friction` en `route_domain_core.rs` | parche motor (fuera de DA-2) |
| Tests | unitarios publisher + `emit_dlt_batch_fracture` | mismo PR |
| Diagnóstico 500 | causa física documentada en `execution.md` (bóveda/logs hijo Node; sin `curl` de entrega) | documental |

### Contrato de prefijos

| Condición | Prefijo | `friction_id` |
|-----------|---------|---------------|
| Transporte / connection refused / timeout / DNS | `iota-relay-unreachable:` | `F-DLT-RELAY-SIN-SUPERVISOR` |
| HTTP no-2xx **con relay respondiendo** | `iota-relay-publish-error:` + status + cuerpo `error`/`feedback` | `F-DLT-PUBLISH-ERROR` |
| JSON 2xx con `success != true` | cuerpo `error`/`feedback` (ya existe) | `F-DLT-PUBLISH-ERROR` si el mensaje no es unreachable |

Orden de clasificación: `F-DLT-PUBLISH-ERROR` **antes** de `contains("iota-relay-unreachable")`.

### Infra (instancia, no genoma)

Identificar el 500 real (wallet / package / red Testnet). Restaurar precondiciones en bóveda `.SddIA/.dev/.env`. Drenar `.SddIA/dlt/reanchor-queue/` solo cuando publish deje de fallar. No forma parte del bisturí de código; se registra en `execution.md`.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA-1 | HTTP 500 en `/v1/publish` produce traza `iota-relay-publish-error` (no `unreachable`) e incluye cuerpo de error del relay |
| CA-2 | `classify_batch_anchor_friction` emite `F-DLT-PUBLISH-ERROR` para CA-1 |
| CA-3 | Connection refused / timeout sigue siendo `F-DLT-RELAY-SIN-SUPERVISOR` |
| CA-4 | Test unitario `emit_dlt_batch_fracture` existente (refused) permanece verde |
| CA-5 | Causa del 500 de esta instancia documentada (candidato 1–4 del PBI) |
| CA-6 | Cascada `spec`/`plan`/`implementation`/`execution`/`validacion` APTO; PBI en `docs/todos/done/` en el mismo PR |

## Fuera de alcance

- Reabrir `delivery-close-cycle` sobre fases `simulated` (PBI `c51acf014c0f`).
- Mutación manual de `SddIA/tools/` (DA-2).
- Bypass raw de entrega (`gh`/`curl` publish).
- Rediseño del centinela `/health` (ya cubierto por #208).
