---
feature_name: event-pending-sweeper
created: "2026-05-22"
process: bug-fix
branch: fix/event-pending-sweeper
executed: "2026-05-22"
---

# Ejecución — event-pending-sweeper

## Smoke E2E (Fase B)

```powershell
$env:SDDIA_LAB_SIMULATE_IOTA="1"
$env:SDDIA_LAB_SIMULATE_SYNC_INDEX="1"
$env:SDDIA_LAB_ROUTE_SYNC="1"
python SddIA/scripts/qa/run-eda-e2e-lab.py --entity-class tool --json
```

### Resultado (2026-05-22)

| Campo | Valor |
|-------|-------|
| `event_id` | `46241eec-4752-46dd-82c3-58ddd616434c` |
| `sweep.status` | `purged` |
| `parent_purged` | `true` |
| `parent_still_pending` | `false` |
| `success` | `true` |

## Dead-letter preservado

Eventos preexistentes con testigo en `dead-letter/subscribers/` (`5b99aa98-…`, `99459a47-…`) permanecen en `pending/` — comportamiento Kaizen intacto.

## Comandos de regresión manual

```bash
python SddIA/scripts/daemons/event-watcher.py --once
python SddIA/scripts/daemons/event-sweeper.py --once --json
```
