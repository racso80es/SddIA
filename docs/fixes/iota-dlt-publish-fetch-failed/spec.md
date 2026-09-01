---
feature_name: iota-dlt-publish-fetch-failed
created: "2026-09-01"
process: bug-fix
base: main
scope: relay-publish-error-cause-propagation
branch_name: fix/iota-dlt-publish-fetch-failed
persist_ref: docs/fixes/iota-dlt-publish-fetch-failed
pbi_ref: docs/todos/pending/[FIX] route-domain-event — fractura sistémica (a90fad3fa8fa).md
document_id: PBI-FIX-FRACTURE-a90fad3fa8fa
uuid: 832fb2e6-ebde-4ec7-9077-696b16f88b92
execution_id: "479390a2-db58-46c7-857f-445dd26364c2"
fracture_hash: a90fad3fa8fa
---

# Especificación — fractura `a90fad3fa8fa` (fetch failed con relay vivo)

## Problema

`merkle-batch-preseal` falló con:

```
iota-relay-publish-error: status=500 fetch failed
```

El relay **respondió** (HTTP 500). Prefijo `iota-relay-publish-error` = taxonomía `b3a715381787` operando. `/health` 200. ELF R1 desplegado. El cuerpo `fetch failed` es `err.message` de undici; `err.cause` no viaja.

Distinto de `701c77ebeab8` (connection refused) y de `6a49e0ad310e` (payload inválido / relay caído).

## Cambio requerido

| Área | Artefacto | Vía |
|------|-----------|-----|
| Serializar `err.cause` + log catch | `.SddIA/services/iota-publish-relay/server.mjs` (+ módulo puro testable) | instancia; **no** genoma DA-2 |
| Tests | `node --test` sobre el formateador | mismo PR |
| Causa física | `execution.md` con literal de cause (DNS/timeout/TLS/fullnode/gas) | documental; sin secretos |
| Sello Merkle | un lote post-fix con `transaction_digest` | operación Testnet; `SIMULATE=0` |

### Contrato del 500

`POST /v1/publish` en fallo:

```json
{
  "success": false,
  "error": "<message>[ | cause: <serialized>]",
  "feedback": "<igual que error>",
  "cause": { "name": "...", "message": "...", "code": "...", "syscall": "...", "errno": "..." }
}
```

`cause` omitido si no hay `err.cause`. `error`/`feedback` **siempre** incluyen el sufijo `cause:` cuando existe, para que `iota-immutable-publisher` (lee `error`/`feedback`, no un campo nuevo) propague el detalle a la traza `iota-relay-publish-error:`.

Log: `console.error('[iota-publish-relay] publish-error', …)` → stderr / journal del unit.

### Infra (instancia)

Alcanzabilidad del fullnode Testnet **desde el hijo Node**. Restaurar red/gas/endpoint si el cause lo nombra. No `curl` de entrega. No volcar bóveda.

## Laudos

| ID | Decisión |
|----|----------|
| L-NO-TAXONOMIA | No mutar `iota-immutable-publisher` ni `classify_batch_anchor_friction`. |
| L-NO-ELF | No reabrir Ola 0/1 ni resolutor de `701c77ebeab8-R1`. |
| L-NO-PROMPT | `prompt_adjustment` Mayeuta inválido. Cubo Mayeuta = Kaizen aparte. |
| L-NO-SIMULATE | `SDDIA_LAB_SIMULATE_IOTA=1` no es Done. |
| L-HEALTH | `/health` 200 y publish 2xx son checks distintos (CA4). |
| L-INSTANCIA | Bisturí = `.SddIA/services/iota-publish-relay/`. Fuera de DA-2. |

## Criterios de aceptación

SSOT: PBI v1.1.0 §8.

| ID | Criterio |
|----|----------|
| DLT-FETCH-CA1 | 500 incluye message **y** cause (o equivalente en `error`/`feedback`). Catch en stderr/journal. |
| DLT-FETCH-CA2 | Cause documentado en `execution.md` (literal, no paráfrasis). Sin secretos. |
| DLT-FETCH-CA3 | Un evento DLT-suscrito post-fix con `transaction_digest` / `merkle_anchored`. `SIMULATE=0`. |
| DLT-FETCH-CA4 | Health y publish no se colapsan. Nueva fractura `fetch failed` **sin** cause = regresión CA1. |

## Fuera de alcance

- Reabrir `PBI-FIX-FRACTURE-b3a715381787` / `701c77ebeab8-R1`.
- Touchpoint Kintsugi.
- Cubo Mayeuta `iota-relay-publish-error`.
- POST raw a `/v1/publish` o `gh`/`git` de entrega.
