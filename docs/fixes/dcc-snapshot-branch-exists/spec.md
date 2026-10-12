---
feature_name: dcc-snapshot-branch-exists
created: "2026-10-12"
process: bug-fix
branch_name: fix/dcc-snapshot-branch-exists
persist_ref: docs/fixes/dcc-snapshot-branch-exists
pbi_ref: docs/todos/done/[FIX] delivery-close-cycle — fractura sistémica (187079f2d82d).md
document_id: PBI-FIX-FRACTURE-187079f2d82d
fracture_hash: 187079f2d82d
execution_id: "2b207e9f-8af2-49ae-94cf-4f2a06f7ac08"
---

# Especificación — fractura `187079f2d82d`

## Problema

Snapshot final de `delivery-close-cycle` emitió `SNAPSHOT_BRANCH_CHECKOUT` con `fatal: una rama llamada '…' ya existe` cuando `git-manager` hacía `checkout -b` sin comprobar `refs/heads/`. El parche conductual (`ad968d80`) ya está en `main`; faltaba firma Mayeuta y candado de tests.

## Traza canónica

```text
[SNAPSHOT_BRANCH_CHECKOUT] fatal: una rama llamada 'fix/dcc-snapshot-missing-branch' ya existe
```

## Entregas

| ID | Artefacto |
|----|-----------|
| CA1 | Test `git-manager`: `checkout` + `create_if_not_exists: true` con rama local previa → exit 0 |
| CA2 | `F-DCC-SNAPSHOT-BRANCH-EXISTS` en `fracture-signatures.json` + tests enrich + fila `187079f2d82d` en `fracture_corpus_regression` |
| CA3 | PBI archivado; `validacion.md` APTO |

## Fuera de alcance

Re-parchear `phase_capsules.rs`; worktree «already checked out»; reabrir `969f05933a46`.
