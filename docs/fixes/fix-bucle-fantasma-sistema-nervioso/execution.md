---
feature_name: fix-bucle-fantasma-sistema-nervioso
created: "2026-05-29"
process: bug-fix
branch_name: fix/bucle-fantasma-sistema-nervioso
status: ejecutado_parcial
index_prefix: "[ARQUITECTURA]"
---

# Ejecución — [ARQUITECTURA] fix-bucle-fantasma-sistema-nervioso

## Baseline pre-fix (incidente 2026-05-29)

Reproducir acumulación en colas fractal (opcional si el lab aún conserva JSON):

```powershell
# Conteo orientativo de instancias del día en colas activas
python -c "
import json
from pathlib import Path
root = Path('.events')
day = '2026-05-29'
n = 0
for p in root.rglob('*.json'):
    if p.parent.name in ('processed','dead-letter','subscribers'):
        continue
    try:
        t = json.loads(p.read_text(encoding='utf-8')).get('timestamp','')
        if t.startswith(day):
            n += 1
    except Exception:
        pass
print('events', day, ':', n)
"
```

Watcher una pasada (observar re-detección si hay zombies):

```powershell
python SddIA/scripts/daemons/event-watcher.py --once
```

## Smokes post-fix (planificados)

### F1 — Watcher idempotente

```powershell
python -m pytest SddIA/scripts/qa/test_bucle_fantasma_bus.py -k watcher -v
python SddIA/scripts/daemons/event-watcher.py --once
# Esperado: sin segunda línea "Detectado nuevo evento" para mismo UUID en sesión estable
```

### F2 — Unlink resiliente

```powershell
python -m pytest SddIA/scripts/qa/test_bucle_fantasma_bus.py -k safe_remove -v
```

### F3 — Purga zona cero

```powershell
python SddIA/scripts/qa/purge_stale_events.py --dry-run --json
# Revisar candidates; luego solo si operador confirma:
# python SddIA/scripts/qa/purge_stale_events.py --apply --json
```

### Regresión EDA

```powershell
$env:SDDIA_LAB_SIMULATE_IOTA="1"
$env:SDDIA_LAB_SIMULATE_SYNC_INDEX="1"
$env:SDDIA_LAB_ROUTE_SYNC="1"
python SddIA/scripts/qa/run-eda-e2e-lab.py --entity-class tool --json
python SddIA/scripts/daemons/event-sweeper.py --once --json
```

## Resultados (2026-05-29)

```powershell
cd SddIA\scripts\qa
python -m unittest test_bucle_fantasma_bus -v
```

| Smoke | Resultado |
|-------|-----------|
| `test_bucle_fantasma_bus` (4 tests) | OK |
| F3 dry-run (lab local) | `python SddIA/scripts/qa/purge_stale_events.py --dry-run` (operador) |
| Regresión `test_eda_bus_v3plus` / `test_eda_fractal_bus` | Ejecutar pre-PR |

| Smoke | Fecha | Resultado |
|-------|-------|-----------|
| Baseline | — | Documentado arriba |
| F1 | 2026-05-29 | OK (unittest skip in-flight / routed-ok) |
| F2 | 2026-05-29 | OK (unittest safe_remove retries) |
| F3 | 2026-05-29 | OK (unittest dry-run candidate) |
| Regresión EDA | — | Pendiente CI / manual pre-PR |
