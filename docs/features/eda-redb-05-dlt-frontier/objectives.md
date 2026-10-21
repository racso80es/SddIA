---
feature_name: eda-redb-05-dlt-frontier
created: "2026-10-21"
process: feature
branch_name: feat/eda-redb-05-dlt-frontier
persist_ref: docs/features/eda-redb-05-dlt-frontier
pbi_ref: docs/todos/pending/[ARQUITECTURA] EDA Redb 05 — Frontera de auditoría DLT.md
document_id: PBI-EDA-REDB-05-DLT-FRONTIER
execution_id: "9584e62d-e512-47bd-8341-7d1df48ae998"
---

# Objetivos — eda-redb-05-dlt-frontier

## Misión

Frontera DLT del laudo L-REDB-07. Solo `PullRequest_Merged` y `System_Immunity_Certified` invocan `iota-immutable-publisher`. El resto del bus es `local_only`.

## Alcance

- AC-5: las clases locales y `Raw_Execution_Finished` no llaman al publisher. Las dos clases externas sí, y dejan `tx_hash` en `dlt_audit_seals` cuando el store está activo.
- AC-6: `participating_agents` y `authorizer` son opcionales en `PullRequest_Merged`. Ausentes, el ancla sigue. Presentes, entran en el digest.
- AC-10: `verify-domain-subscription-parity` en verde. `iota-immutable-publisher` queda solo en las dos claves de interés público.

## Fuera de alcance

Corte del bus a filesystem, descriptores de agente, rediseño de `iota-publish-relay` y el alta de `IOTA_ANCHOR_PACKAGE_ID`.
