---
feature_name: eda-coverage-ssot-bus-isolation
created: "2026-05-25"
process: feature
branch_name: feat/eda-coverage-ssot-bus-isolation
---

# Ejecución — SSOT eda-coverage y desacople bus EDA

## F0 — Baseline (2026-05-25T17:38Z)

| Artefacto | Resultado |
|-----------|-----------|
| `.tmp/eda-coverage-baseline.json` | `orphan_count: 0`, `indexed_entities: 48` (modo bus legacy) |
| `.tmp/eda-e2e-baseline.json` | `success: false`, `parent_still_pending: true` |
| `verify-process-integrity.py` | OK |

## F5 — Backfill SSOT

```powershell
python SddIA/scripts/qa/audit-entity-eda-coverage.py --backfill-coverage --json
```

| Campo | Valor |
|-------|--------|
| `backfilled` | 45 |
| `skipped` | 3 |
| `indexed_entities` | 48 |
| `orphan_count_after` | 0 |

## F8 — Gates

### V1 — Scan SSOT

```powershell
python SddIA/scripts/qa/audit-entity-eda-coverage.py --scan --json
```

`orphan_count: 0`, `scan_source: eda-coverage.json`, `indexed_entities: 48`.

### V2 — Watcher + scan

```powershell
python SddIA/scripts/daemons/event-watcher.py --once
python SddIA/scripts/qa/audit-entity-eda-coverage.py --scan --json
```

`orphan_count: 0` post-watcher (sin dependencia de cabeceras retenidas).

### V4 — Smoke E2E

```powershell
python SddIA/scripts/qa/run-eda-e2e-lab.py --entity-class tool --json
```

| Campo | Valor |
|-------|--------|
| `success` | **true** |
| `parent_purged` | true |
| `sweep.status` | purged |
| Bus usado | `.tmp/events_test/pending/` |

### V6 — Integridad

`verify-process-integrity.py` → OK.

### Tests unitarios

```powershell
cd SddIA/scripts/qa
python -m unittest test_eda_bus_v3plus -v
```

8 tests OK (sweep vacío, EVENT_BUS_PATH, topología local).

### V3 — Pre-commit (commit prueba)

Toca genoma `SddIA/library/norms/features-documentation-pattern.md` (evidencia en commit `v3-precommit`).

```powershell
python SddIA/scripts/qa/git-hooks/pre_commit_gate.py
# exit 0 — orphan_count: 0, sin BLOCKED
```

### UNI-CA5 — Smoke E2E (reconfirmación pre-PR)

```powershell
python SddIA/scripts/qa/run-eda-e2e-lab.py --entity-class tool --json
# success: true
```
