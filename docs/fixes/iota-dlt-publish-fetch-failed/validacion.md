---
feature_name: iota-dlt-publish-fetch-failed
created: "2026-09-01"
updated: "2026-09-01"
process: bug-fix
branch_name: fix/iota-dlt-publish-fetch-failed
persist_ref: docs/fixes/iota-dlt-publish-fetch-failed
pbi_ref: docs/todos/done/[FIX] route-domain-event — fractura sistémica (a90fad3fa8fa).md
document_id: PBI-FIX-FRACTURE-a90fad3fa8fa
uuid: 832fb2e6-ebde-4ec7-9077-696b16f88b92
incident_ref: "System_Fracture_Detected — a90fad3fa8fa"
global: APTO
pbi_archived: true
branch: fix/iota-dlt-publish-fetch-failed
pr_url: https://github.com/racso80es/SddIA/pull/245
approval_status: aprobado
verdict: aprobado
checks:
  DLT-FETCH-CA1: APTO
  DLT-FETCH-CA2: APTO
  DLT-FETCH-CA3: APTO
  DLT-FETCH-CA4: APTO
  CASCADE_SPEC: APTO
  CASCADE_PLAN: APTO
  CASCADE_IMPLEMENTATION: APTO
  CASCADE_EXECUTION: APTO
  CASCADE_VALIDACION: APTO
  NODE_TESTS: APTO
git_changes:
  - .SddIA/services/iota-publish-relay/relay-error.mjs
  - .SddIA/services/iota-publish-relay/relay-error.test.mjs
  - .SddIA/services/iota-publish-relay/server.mjs
  - .SddIA/services/iota-publish-relay/README.md
  - docs/fixes/iota-dlt-publish-fetch-failed/
  - docs/todos/done/[FIX] route-domain-event — fractura sistémica (a90fad3fa8fa).md
---

# Validación — fractura `a90fad3fa8fa` (Argos)

## Veredicto

**APTO** — cause de publish serializado; tests 5/5; fullnode Testnet 200 en ejecución; digest on-chain vía cápsula `iota-immutable-publisher`. Taxonomía y ELF R1 no reabiertos. `SIMULATE=0`.

## Checks

| Check | Estado | Evidencia |
|-------|--------|-----------|
| DLT-FETCH-CA1 | APTO | `formatPublishFailure` + catch `console.error`; test ENOTFOUND con sufijo `cause:` |
| DLT-FETCH-CA2 | APTO | Sello original lossy. Literal F3: `https://api.testnet.iota.cafe` HTTP 200 `result=258893231` |
| DLT-FETCH-CA3 | APTO | digest `8TaP3rFM27J76NDzKoGRPd72ysAr7TWSkJdKnk5HpAdG` (cápsula, no curl) |
| DLT-FETCH-CA4 | APTO | `/health` 200 **y** publish `success:true` como filas distintas |
| NODE_TESTS | APTO | `node --test relay-error.test.mjs` 5/5 |

## Fuera

Cubo Mayeuta `iota-relay-publish-error`. Reabrir `b3a715381787` / `701c77ebeab8-R1`.
