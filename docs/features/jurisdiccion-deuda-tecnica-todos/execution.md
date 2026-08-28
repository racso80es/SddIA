---
feature_name: jurisdiccion-deuda-tecnica-todos
created: "2026-08-28"
process: feature
items_applied:
  - l1-norm-creator
  - l2-migrate-deuda
  - l3-tmp-discard
  - l4-tests-ca5
---

# Ejecución — jurisdiccion-deuda-tecnica-todos

## L1 — entity-manager

```text
./sddia-run.sh --process entity-manager --inputs-file .tmp/entity-manager-norm-todos-jurisdiction.json
```

`success: true`, `handoff_entity_uuid: f0b8ce4a-2f79-4516-bee0-acfe0d25bd58`.

## L2/L3 — Migración física

`git mv` ×3 DeudaTecnica → pending; `git rm` ×5 tmp; `rmdir` buckets vacíos.

## L4 — Tests

```text
cd SddIA && cargo test -p execute-process extract_pbi -- --nocapture
```

```
running 4 tests
test ... extract_pbi_prefers_pending_over_inert_when_both_present ... ok
test ... extract_pbi_with_spaces_and_emdash ... ok
test ... extract_pbi_ignores_inert_bucket_paths ... ok
test ... extract_pbi_migrated_deuda_tecnica_paths ... ok
test result: ok. 4 passed
```

```text
cd SddIA && cargo test -p execute-process pending_pbi_path -- --nocapture
```

```
test ... pending_pbi_path_accepted_for_archive_gate ... ok
```

## L6 — Cierre

`validacion.md` APTO + PBI en `docs/todos/done/`. PR: https://github.com/racso80es/SddIA/pull/219

### Intento 1 — bloqueado

`delivery-close-cycle` acusó fallo en Aduana EDA (`orphan_count: 2` preexistentes: `github-raw-fetcher`, `download-remote-asset`). El proceso **no emitió** `System_Fracture_Detected`, de modo que el Protocolo Kintsugi no pudo dispararse. El operador abrió push y PR por vía raw: desviación normativa autorreportada como `F-TEKTON-BYPASS-RAW-POST-COLAPSO`.

### Desbloqueo

Backfill de cobertura por emisión canónica (`--action emit-domain-mutation` ×2, correlación `a6f93bdc-a04d-4d7e-a3ae-d112386d10b1`) → `orphan_count: 0`.

### Intento 2 — cierre canónico

```text
./sddia-run.sh --process delivery-close-cycle --inputs '{"source_process":"feature","persist_ref":"docs/features/jurisdiccion-deuda-tecnica-todos","branch_name":"feat/jurisdiccion-deuda-tecnica-todos", ...}'
```

Ocho fases en `executed`. Aduana EDA `argos_verdict: pass`. Sello ECST `PullRequest_Presented` emitido (`78f4187f-13f2-4bfd-b089-16c93050b1a8`), que era lo que faltaba tras la apertura irregular. `execution_id: 94bacd29-36e3-41db-905a-45ec7f2765c4`.

Fricción del ciclo escalada a `PBI-KAIZEN-CICLO-JURISDICCION-TODOS`.


`id_cambio`: `eb6fb73a-9ded-49a1-a2a9-314624358b4b` vía `sddia-qa evolution-register`.
