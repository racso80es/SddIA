# Ejecución — Ola C V3+ Coreografía

## Unit tests

```powershell
cd SddIA/scripts/qa
python -m unittest test_eda_bus_v3plus -v
```

## E2E laboratorio (simulate)

```powershell
$env:SDDIA_LAB_SIMULATE_IOTA = '1'
$env:SDDIA_LAB_SIMULATE_SYNC_INDEX = '1'
$env:SDDIA_LAB_ROUTE_SYNC = '1'   # secuencial — regresión CI
python SddIA/scripts/qa/run-eda-e2e-lab.py --entity-class tool --json
```

Éxito esperado: `success: true`, `sweep.status: purged`, `parent_purged: true`.

## Comandos operativos

```powershell
# Enrutar un evento pending concreto
python SddIA/scripts/daemons/event-watcher.py --event-file-path .events/pending/{UUID}.json

# Watcher un ciclo
python SddIA/scripts/daemons/event-watcher.py --once

# Sweeper un ciclo (recolector stale / idempotente post-E2E)
python SddIA/scripts/daemons/event-sweeper.py --once --json
```

## Fan-out async (default producción)

Sin `SDDIA_LAB_ROUTE_SYNC`: dispatch paralelo vía `ThreadPoolExecutor` en `route_domain_event_core.py`.

## CI (GitHub Actions)

Job `eda-bus-e2e-smoke` en `.github/workflows/sddia-index-qa.yml`:

1. `run-eda-e2e-lab.py --entity-class tool --json` (simulate)
2. `event-sweeper.py --once --json`

## Evidencia

| Fecha | Escenario | Resultado |
|-------|-----------|-----------|
| 2026-05-22 | Smoke inicial PR #24 | Route + sweeper manual |
| 2026-05-25 | E2E lab + unit tests | 4/4 tests OK; E2E `2553772d-…` purged |

## Referencias

- Spec consolidada: `spec.md`
- Triaje: `clarify.md`
- Topología V3+ delta: `docs/features/refactor-topologia-eventos-ola-c-v3/`
