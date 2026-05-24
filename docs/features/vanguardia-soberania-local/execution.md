---
feature_name: vanguardia-soberania-local
created: "2026-05-24"
process: feature
branch: feat/vanguardia-soberania-local
---

# Ejecución — Vanguardia Soberanía Local

## Entorno

- Workspace: `c:\Proyectos\SddIA`
- Perfil lab: push real habilitado (`SDDIA_LAB_SKIP_GIT_PUSH` unset)

## L1 — Higiene `_delete_branch_hygiene`

```powershell
python -c "import sys; sys.path.insert(0, 'SddIA/scripts/qa'); from pathlib import Path; from execute_process_capsules import _delete_branch_hygiene; closed, hf = _delete_branch_hygiene(Path('.'), 'feat/nonexistent-vanguardia-hygiene-smoke'); print(closed, hf is not None)"
```

Resultado: `None True`

## L1 — `accept-pr` Fase 4 integrada

```powershell
$env:SDDIA_LAB_SKIP_GIT_PUSH=""
python SddIA/scripts/qa/execute-process.py --process accept-pr --inputs-file docs/features/vanguardia-soberania-local/_smoke-accept-pr-hygiene-fail.json
```

| Campo | Valor |
|-------|-------|
| `success` | `true` |
| `data.closed_branch` | `null` |
| `data.hygiene_failure.survived_branch` | `feat/nonexistent-vanguardia-hygiene-smoke` |
| Fase 4 `execution_report` | Incluye `hygiene_failure` + `operations[]` |

Evento Merged emitido (smoke): `ebcf14ba-a267-42c1-be6d-84709210a6fb`

## E2 — Emisión válida

```powershell
Get-Content -Raw docs/features/vanguardia-soberania-local/_smoke-emit-domain-mutation-valid.json | python SddIA/scripts/qa/execute-action.py --action emit-domain-mutation
```

| Campo | Valor |
|-------|-------|
| `success` | `true` |
| `event_id` | `8916baa3-1595-4cd2-b2fc-4715e9550145` |
| `target_path` | `.events/pending/8916baa3-1595-4cd2-b2fc-4715e9550145.json` |

## E2 — Emisión inválida (FORBIDDEN en create)

```powershell
Get-Content -Raw docs/features/vanguardia-soberania-local/_smoke-emit-domain-mutation-invalid.json | python SddIA/scripts/qa/execute-action.py --action emit-domain-mutation
```

| Campo | Valor |
|-------|-------|
| `success` | `false` |
| `exitCode` | `1` |
| `error` | `forbidden payload.hash_signature_old (must be null if present)` |
| Archivo en `pending/` | **Ninguno** nuevo |

## Integridad genoma

```powershell
python SddIA/scripts/qa/verify-process-integrity.py
```

Resultado: `verify-process-integrity: OK`

## Backfill EDA — Fase C (pre-commit)

```powershell
python SddIA/scripts/qa/audit-entity-eda-coverage.py --emit --skip-dlt --json --correlation-id vanguardia-soberania-local
# orphan_count_after: 0 | emit_ok: 43

$env:SDDIA_LAB_SIMULATE_IOTA="1"
python SddIA/scripts/qa/audit-entity-eda-coverage.py --anchor-merkle docs/features/eda-domain-entities-splus/backfill-manifest.json
# merkle_root: sha256:021665f48f9bd584911686f1412756c6d4bbc2a2c89531f21ad86f9fe0e264ea
# transaction_digest: lab-simulated-021665f48f9bd584

python SddIA/scripts/qa/audit-entity-eda-coverage.py --scan --json
# orphan_count: 0 (con eventos en pending; requerido para pre-commit)
```

**Nota:** ejecutar `event-watcher --once` **después** del merge desacopla correlación pending↔scan; mantener instancias en `pending/` hasta commit si la aduana pre-commit está activa.
