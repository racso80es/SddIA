# Ejecución — Ola C V3 Coreografía

## Smoke (laboratorio)

```powershell
$env:SDDIA_LAB_SIMULATE_IOTA='1'
$env:SDDIA_LAB_SIMULATE_SYNC_INDEX='1'
python tmp/smoke-ola-c-v3.py
```

## Comandos operativos

```powershell
# Enrutar un evento pending
python SddIA/scripts/daemons/event-watcher.py --event-file-path .events/pending/{UUID}.json

# Watcher un ciclo
python SddIA/scripts/daemons/event-watcher.py --once

# Sweeper un ciclo
python SddIA/scripts/daemons/event-sweeper.py --once --json
```

## Evidencia smoke 2026-05-22

- Route: `delivery_status` → `cumulo.sync-entity-index: success`, `cumulo.iota-immutable-publisher: success`
- Padre permanece en `pending/` post-route
- Sweeper purga padre + 2 testigos processed
