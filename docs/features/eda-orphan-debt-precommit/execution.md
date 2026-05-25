---
feature_name: eda-orphan-debt-precommit
created: "2026-05-25"
process: bug-fix
branch_name: fix/eda-orphan-debt-precommit
---

# Ejecución — Deuda EDA orphan_count (pre-commit)

## F0 — Baseline en frío (pre-modificación)

| Paso | Resultado |
|------|-----------|
| **0.2** | `.tmp/eda-orphan-baseline.json` — `orphan_count: 0`, `indexed_entities: 46` |
| **0.3** | Omitido (`orphan_count == 0` en baseline) |
| **0.4** | `verify-process-integrity.py` → **OK** |

Inventario bus pre-cambio: 44 `pending/`, 10 cabeceras `processed/`.

---

## F1 — Fix correlación + retención (opción 2)

| Touchpoint | Cambio |
|------------|--------|
| `eda_bus_utils.py` | `iter_bus_event_files()` multi-estado |
| `eda_bus_utils.py` | `archive_event_after_sweep()` retiene cabecera `processed/` para `Domain_Entity_Created` |
| `features-documentation-pattern.md` | Nota correlación multi-estado |
| `test_eda_bus_v3plus.py` | Tests retención / purga |

### V4 — primera pasada (solo C2, sin retención)

| Campo | Valor |
|-------|--------|
| Post watcher | `orphan_count: **43**` |
| Causa | `archive_event_after_sweep` eliminaba cabeceras correlacionables |

### V4 — segunda pasada (C2 + retención)

```powershell
python SddIA/scripts/daemons/event-watcher.py --once
python SddIA/scripts/qa/audit-entity-eda-coverage.py --scan --json
```

| Campo | Valor |
|-------|--------|
| `.tmp/eda-orphan-post-f1-v4-retention.json` | `orphan_count: **0**`, `indexed_entities: 46` |
| Cabeceras `processed/` post-V4 | 53 (43 `Domain_Entity_Created` retenidas + legado) |

---

## F2 — Backfill + Merkle

```powershell
$env:SDDIA_LAB_SIMULATE_IOTA = "1"
python SddIA/scripts/qa/audit-entity-eda-coverage.py `
  --anchor-merkle docs/features/eda-orphan-debt-precommit/backfill-manifest.json
```

| Campo | Valor |
|-------|--------|
| `emit_ok` | 43 (emit preliminar previo) |
| `transaction_digest` | `lab-simulated-29eb0307245943d8` |
| Acta | `merkle-acta-eda-backfill-precommit-20260525.json` |

---

## Kaizen derivado — correlación sin bus

Materializado PBI:

`docs/todos/pending/[Kaizen] validación genómica EDA sin dependencia del bus — correlación durable.md`

**Casuística documentada:** la validación genómica no debe depender de instancias ECST en carpetas efímeras del bus; la retención de cabeceras es mitigación táctica hasta SSOT durable.

---

## F3 — Gates (completado)

| Gate | Resultado |
|------|-----------|
| **V1** | `--scan` → `orphan_count: 0` |
| **V2** | `pre_commit_gate.py` exit 0 con genoma staged |
| **V3** | `delivery-close-cycle` → ver `validacion.md` |
| **V4** | watcher + scan → `orphan_count: 0` (retención) |
| **V5** | `verify-process-integrity.py` → OK |

## F4 — Prevención forward

- `SddIA/norms/git-operations.md` v1.2.0 §3.1 Cobertura EDA genómica
- Runbook orden Tekton en §3.1
- Kaizen derivado: PBI correlación durable sin bus

## Cierre documental

- PBI en `docs/todos/done/`
- `validacion.md` → `global: APTO`, `pbi_archived: true`
