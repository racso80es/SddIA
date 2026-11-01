---
feature_name: dcc-presented-seal-contract
created: 2026-11-01
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/dcc-presented-seal-contract
tracker_ref: AUT-48
execution_id: 9e85f016-c9be-46de-af20-92455e7817b8
global: APTO
pbi_archived: false
branch: Fix_Events_Multiple
approval_status: aprobado
verdict: aprobado
checks:
  AUT48-T1-GENOMA: APTO
  AUT48-T2-EVOLUTION: APTO
  AUT48-REGRESSION-TEST: APTO
git_changes:
- SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md
- SddIA/library/codexes/codex-software-engineering/process/index.md
- SddIA/evolution/20628bfd-f3f1-4dff-9409-4ef70e82c985.md
- SddIA/evolution/Evolution_log.md
- docs/fixes/dcc-presented-seal-contract/
validated_base_sha: 2d14642644f1acd7ebfa1fa32bac0f941f817736
validated_head_sha: cc2104d57bc8067624eb3d16b0fc393794c9c396
validated_tree_sha: 04216d99ceed96c24230584c66e0d3a38571c969
diff_digest: sha256:c0b4d4f5ea07ed2983bcd69e199a4ec738633dcd36ba72ab4163d223ace5ffb2
---

# Validación — AUT-48

## Veredicto

**APTO** — El genoma `delivery-close-cycle` v1.6.1 documenta el sello Presentación sin «opcional en payload ECST v1.1» y con `blocked` / `F-DCC-PR-URL-MISSING` cuando no hay URL. Handler y firmas (AUT-31) intactos.

## Evidencia

| Check | Medio | Resultado |
|-------|--------|-----------|
| T1 | `entity-manager` ×2; grep sin «opcional en payload ECST v1.1» en sello | OK |
| T2 | `evolution-rehash --id 20628bfd-…`; fila `Evolution_log` | OK |
| Regresión | `cargo test -p execute-process emit_presented_blocks_missing_pr_url` | OK |

## Cierre tracker

Provider `linear`: **AUT-48** → Done con el PR que incluya este diff; sin fichero en `docs/todos/done/`.