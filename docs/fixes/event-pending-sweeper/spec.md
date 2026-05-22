---
feature_name: event-pending-sweeper
created: "2026-05-22"
process: bug-fix
base: main
scope: event-pending-sweeper
version_spec: "1.0.0"
---

# Especificación — Cierre automático pending/ post-enrutamiento

## Hito 1 — Helper compartido (`eda_bus_utils.py`)

### 1.1 `try_sweep_event(repo, bus, event_uuid) -> dict`

Extraer la lógica de consenso de `event-sweeper.sweep_once` para **un solo UUID**:

| Paso | Regla |
|------|-------|
| 1 | Leer padre en `pending/<uuid>.json`; si ausente → `{"status":"absent"}` |
| 2 | Si testigo en `dead-letter/subscribers/` → `{"status":"kaizen","purged":false}` |
| 3 | Resolver suscriptores requeridos (`event-subscriptions.json` + `applies_to_origin_topology`) |
| 4 | Si suscriptores in-flight en `processing/subscribers/` → `{"status":"in-flight","purged":false}` |
| 5 | Si `required ⊆ processed_subscribers` → `archive_event_after_sweep()` → `{"status":"purged","purged":true,...counts}` |
| 6 | Else → `{"status":"awaiting","pending":[...]}` |

Reutilizar funciones existentes: `list_witnesses`, `in_flight_subscriber_names`, `archive_event_after_sweep`, `required_subscriber_ids`.

### 1.2 Refactor sweeper

- `event-sweeper.py` → iterar `pending/*.json` y delegar en `try_sweep_event` por UUID.
- Sin cambio de comportamiento externo del daemon.

---

## Hito 2 — Cierre en `route_domain_event_core.py`

Al final de `route_domain_event()`, tras fan-out sync/async:

```python
sweep = try_sweep_event(repo, bus, event_uuid)
result["data"]["sweep"] = sweep
```

| Condición | Acción |
|-----------|--------|
| `sweep["status"] == "purged"` | Incluir en `data` paths purgados |
| `dead-letter` presente | No purgar; propagar status kaizen |
| Suscriptores pendientes | Normal en async parcial (no aplica si pool espera) |

---

## Hito 3 — Watcher (`event-watcher.py`)

| Cambio | Detalle |
|--------|---------|
| Log post-route | Sustituir `"padre permanece en pending"` por mensaje según `sweep.status` del stdout JSON de route (o re-lectura bus) |
| Opcional | Tras route exitoso sin dead-letter, invocar `try_sweep_event` si route no lo hizo (defensa en profundidad) |

---

## Hito 4 — Smoke regresión

```bash
# Baseline / post-fix
python SddIA/scripts/qa/execute-action.py --action emit-domain-mutation --inputs @tmp/smoke-sweep.json
python SddIA/scripts/daemons/event-watcher.py --once
# Assert: pending/<event_id>.json ausente si consenso
# Assert: dead-letter existente NO purgado
```

Documentar en `validacion.md`.

---

## Hito 5 — Documentación

| Artefacto | Cambio |
|-----------|--------|
| `README.md` | Nota: cierre automático post-route; sweeper como recolector stale |
| `events-contract.md` | §4.6: purga puede ocurrir al cierre de `route-domain-event` **o** vía sweeper |

---

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | Emit + watcher `--once` → padre purgado cuando todos los suscriptores OK |
| CA2 | Evento con dead-letter → padre permanece; alerta Kaizen |
| CA3 | `event-sweeper.py --once` sigue funcionando (idempotente) |
| CA4 | Sin regresión en `SDDIA_LAB_ROUTE_SYNC=1` |
