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

## Pendiente (L6)

- `validacion.md` Argos + PBI a `done/` + `delivery-close-cycle`

## L5 — Evolution

`id_cambio`: `eb6fb73a-9ded-49a1-a2a9-314624358b4b` vía `sddia-qa evolution-register`.
