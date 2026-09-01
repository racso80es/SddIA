---
feature_name: iota-dlt-publish-fetch-failed
created: "2026-09-01"
process: bug-fix
branch_name: fix/iota-dlt-publish-fetch-failed
persist_ref: docs/fixes/iota-dlt-publish-fetch-failed
items:
  - relay-error.mjs
  - server.mjs/catch
  - relay-error.test.mjs
---

# Implementation — fetch-failed cause (a90fad3fa8fa)

## Touchpoints

| Artefacto | Cambio |
|-----------|--------|
| `.SddIA/services/iota-publish-relay/relay-error.mjs` | `serializeCause` + `formatPublishFailure`. Puro, sin SDK. |
| `.SddIA/services/iota-publish-relay/server.mjs` | Catch: log `publish-error` + 500 con `error`/`feedback`/`cause`. |
| `.SddIA/services/iota-publish-relay/relay-error.test.mjs` | `node --test`. 5/5. |
| `.SddIA/services/iota-publish-relay/README.md` | Contrato 500 + `/health` ≠ publish. |

Genoma `SddIA/tools/` e `iota-immutable-publisher` **intactos** (L-NO-TAXONOMIA). ELF relay **intacto** (L-NO-ELF).
