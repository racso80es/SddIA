---
feature_name: iota-dlt-publish-fetch-failed
created: "2026-09-01"
process: bug-fix
phases:
  - extract-format-relay-error
  - wire-server-catch
  - tests-node
  - diagnose-testnet-cause
  - merkle-e2e
  - doc-closure
branch_name: fix/iota-dlt-publish-fetch-failed
persist_ref: docs/fixes/iota-dlt-publish-fetch-failed
pbi_ref: docs/todos/done/[FIX] route-domain-event — fractura sistémica (a90fad3fa8fa).md
document_id: PBI-FIX-FRACTURE-a90fad3fa8fa
uuid: 832fb2e6-ebde-4ec7-9077-696b16f88b92
execution_id: "479390a2-db58-46c7-857f-445dd26364c2"
---

# Plan — fractura `a90fad3fa8fa` (sella Diseño)

Orden: formateador → catch → tests → diagnóstico Testnet → sello Merkle → cierre. **Este commit sella Diseño** (`spec.md` + `plan.md` + `objectives.md`). Código = fase Ejecución.

Prohibido: mutar `SddIA/tools|skills|actions|process|agents|events|norms` a mano; reabrir taxonomía o ELF R1.

## Fase F0 — Formateador puro (CA1)

Archivo nuevo: `.SddIA/services/iota-publish-relay/relay-error.mjs`

- `serializeCause(err)` → objeto `{name, message, code?, syscall?, errno?}` o `null`.
- `formatPublishFailure(err)` → `{ error, feedback, cause }` con sufijo ` | cause: …` en `error`/`feedback` si hay cause.

Sin IOTA SDK. Export ESM.

## Fase F1 — Catch del server (CA1)

`.SddIA/services/iota-publish-relay/server.mjs`:

1. Importar `formatPublishFailure`.
2. `catch`: `console.error('[iota-publish-relay] publish-error', formatted)`; `sendJson(res, 500, { success: false, ...formatted })`.
3. `/health` intacto.

Reinicio del unit **no** es este commit.

## Fase F2 — Tests

`.SddIA/services/iota-publish-relay/relay-error.test.mjs` + `node --test`.

Casos: Error plano (sin cause); `TypeError: fetch failed` + cause `{code:'ENOTFOUND'}`; cause anidado un nivel.

```bash
PATH=".tools/node-v22.16.0-linux-x64/bin:$PATH"
cd .SddIA/services/iota-publish-relay && node --test relay-error.test.mjs
```

## Fase F3 — Diagnóstico Testnet (CA2)

Desde el **mismo** runtime Node del hijo (no `curl` de entrega): fetch a `getFullnodeUrl("testnet")` o error isomorfo. Literal de cause en `execution.md`. Restaurar red/gas/endpoint si el cause lo nombra. Sin secretos.

## Fase F4 — Sello Merkle (CA3)

Lote `route-domain-event` real con `SIMULATE=0`. Evidencia `transaction_digest` / `merkle_anchored` en `execution.md`. Prohibido `SIMULATE=1`.

## Cierre documental (misma rama)

1. `implementation.md` + `execution.md`.
2. `validacion.md` APTO, `pbi_archived: true`.
3. PBI → `docs/todos/done/`.
4. `delivery-close-cycle` `source_process: bug-fix`.

## Delegación

| Fase proceso | Quién | Artefacto |
|--------------|-------|-----------|
| Diseño | Dedalo (este sello, relevo local) | `spec.md`, `plan.md` |
| Ejecución | Tekton | código + tests + `implementation.md` + `execution.md` |
| Verificación | Argos | `validacion.md` |
| Cierre | `delivery-close-cycle` | PR único |
