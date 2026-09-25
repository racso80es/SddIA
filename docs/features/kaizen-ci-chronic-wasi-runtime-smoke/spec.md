---
feature_name: kaizen-ci-chronic-wasi-runtime-smoke
created: "2026-09-25"
process: feature
base: main
scope: evolution-rehash-83d6eb73
branch_name: feat/kaizen-ci-chronic-wasi-runtime-smoke
persist_ref: docs/features/kaizen-ci-chronic-wasi-runtime-smoke
pbi_ref: docs/todos/pending/[KAIZEN] CI crónica — wasi-runtime-smoke.md
document_id: PBI-KAIZEN-CI-CHRONIC-WASI-RUNTIME-SMOKE
pbi_uuid: "30a026cf-8227-4315-a521-53ec34a8cc0a"
pbi_version: "1.1.0"
execution_id: "915ff612-6aad-4d82-b92a-6d455d2cfe3d"
---

# Spec — kaizen-ci-chronic-wasi-runtime-smoke

## Slice único — rehash

Registro ya indexado en `SddIA/evolution/Evolution_log.md` (fila `83d6eb73-0936-4acb-9f1e-5d519987f1ab`, 2026-09-20, CANONICO).

Estado en `main`:

```yaml
hash_integrity: "sha256:pending-pr-292"
```

`is_valid_hash_integrity` exige prefijo `sha256:` y 64 hex. `pending-pr-292` no cumple. El universo emite `EVOL_HASH_MISMATCH` con el detalle que ya cita `evolution-rehash --id`.

Operación:

```text
SddIA/target/debug/sddia-qa evolution-rehash --id 83d6eb73-0936-4acb-9f1e-5d519987f1ab --json
```

Contrato del skill (`rehash`): `canonical_hash` = SHA-256 del markdown con la línea `hash_integrity:` eliminada; `patch_hash_integrity_line` escribe ese valor; `idempotent` si el fichero no cambia. Persistir solo si `success` y no `idempotent`.

No se añade otro `id_cambio`. No se toca `Evolution_log.md` salvo que el rehash lo reescriba idéntico (el handler devuelve el índice actual).

## Gate

```text
SddIA/target/debug/sddia-qa gate-evolution --json --all
SddIA/target/debug/sddia-qa gate-evolution --json --range --require-synced-base
```

Delta: paths bajo `SddIA/` fuera de `SddIA/evolution/` son material. Este diff (registro evolution + `docs/`) es `L-SELF / sin material` si no aparece otro path material. Universo: el único finding conocido de `35526123813` debe desaparecer.

## Superficie que no se toca

- `.github/workflows/sddia-index-qa.yml`
- `SddIA/scripts/qa/build-wasi-capsules.sh`
- `.SddIA/radamanto/ci_failures.json`
- Genoma DA-2 (`tools/`, `skills/`, `actions/`, `process/`, `agents/`, `events/`, `norms/`, `library/`)
