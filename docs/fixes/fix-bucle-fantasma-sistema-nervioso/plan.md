---
feature_name: fix-bucle-fantasma-sistema-nervioso
created: "2026-05-29"
process: bug-fix
branch_name: fix/bucle-fantasma-sistema-nervioso
persist_ref: docs/fixes/fix-bucle-fantasma-sistema-nervioso
version_plan: "1.0.0"
index_prefix: "[ARQUITECTURA]"
status: implementado
implementation: completado
---

# Plan — [ARQUITECTURA] fix-bucle-fantasma-sistema-nervioso

## Estado

| Fase documental | Estado |
|-----------------|--------|
| clarify.md | ✅ |
| objectives.md | ✅ |
| spec.md | ✅ |
| plan.md | ✅ |
| implementation.md | ⏳ stub (Tekton) |
| execution.md | ⏳ stub (Tekton) |
| validacion.md | ⏳ PENDIENTE (Argos) |
| Código F1–F3 | ❌ no iniciado |

## Secuencia de implementación (Tekton)

### Paso 0 — Rama y entorno

```powershell
git checkout main
git pull
git checkout -b fix/bucle-fantasma-sistema-nervioso
```

Variables lab recomendadas (sin cambio de contrato):

```powershell
$env:SDDIA_LAB_WATCH_FRACTAL="1"
$env:SDDIA_LAB_ROUTE_SYNC="1"   # solo regresión pytest
```

### Paso 1 — Fase 1: Idempotencia watcher

| # | Tarea | Archivo |
|---|-------|---------|
| 1.1 | Introducir `processing_uuids` y `routed_ok_pending_absent` (D3) | `SddIA/scripts/daemons/event-watcher.py` |
| 1.2 | Sustituir lógica `in_flight` por UUID según spec §1.2 | idem |
| 1.3 | Añadir logs §1.3 | idem |
| 1.4 | Tests T1, T2 | `SddIA/scripts/qa/test_bucle_fantasma_bus.py` |

**Gate:** `python -m pytest SddIA/scripts/qa/test_bucle_fantasma_bus.py -k watcher -q`

### Paso 2 — Fase 2: safe_remove_path

| # | Tarea | Archivo |
|---|-------|---------|
| 2.1 | Implementar `safe_remove_path` | `SddIA/scripts/qa/eda_bus_utils.py` |
| 2.2 | Refactor call sites fractal | `route_fractal_event_core.py` |
| 2.3 | Refactor `maybe_purge_fractal_telemetry_when_terminal` y purge padre | `eda_bus_utils.py` |
| 2.4 | Ajustar `purged` / logging en respuesta route | `route_fractal_event_core.py` |
| 2.5 | Tests T3 | `test_bucle_fantasma_bus.py` |

**Gate:** `python -m pytest SddIA/scripts/qa/test_bucle_fantasma_bus.py -k safe_remove -q`

### Paso 3 — Fase 3: purge_stale_events

| # | Tarea | Archivo |
|---|-------|---------|
| 3.1 | Crear CLI §3.2 | `SddIA/scripts/qa/purge_stale_events.py` |
| 3.2 | Implementar escaneo colas fractal + criterios §3.3 | idem |
| 3.3 | Tests T4 | `test_bucle_fantasma_bus.py` |
| 3.4 | Documentar comandos en `execution.md` | `docs/fixes/.../execution.md` |

**Gate lab (dry-run sobre estado actual):**

```powershell
python SddIA/scripts/qa/purge_stale_events.py --dry-run --json
```

### Paso 4 — Regresión EDA

```powershell
python SddIA/scripts/daemons/event-sweeper.py --once --json
python SddIA/scripts/qa/run-eda-e2e-lab.py --entity-class tool --json
python -m pytest SddIA/scripts/qa/test_eda_bus_v3plus.py SddIA/scripts/qa/test_eda_fractal_bus.py -q
```

### Paso 5 — Documentación de cierre (mismo PR)

| # | Tarea |
|---|-------|
| 5.1 | Completar `implementation.md` y `execution.md` |
| 5.2 | Argos → `validacion.md` con `global: APTO`, `pbi_archived: true` |
| 5.3 | Mover `docs/todos/pending/PBI-FIX-BUCLE-FANTASMA-SISTEMA-NERVIOSO.md` → `docs/todos/done/` |
| 5.4 | Actualizar `README.md` y `events-contract.md` (spec Hito 5) |

### Paso 6 — Entrega

- PR único con prefijo título: `[ARQUITECTURA] fix: bucle fantasma bus EDA (Windows E/S)`
- `delivery-close-cycle` con `source_process: bug-fix`, `persist_ref: docs/fixes/fix-bucle-fantasma-sistema-nervioso`

## Orden de dependencias

```text
F1 (watcher) ──┬──> F4 regresión manual
F2 (unlink)  ──┤
F3 (purge)   ──┴──> ejecutar F3 tras F1+F2 en lab contaminado
```

**Recomendación:** implementar F1 y F2 en el mismo commit; F3 en commit separado reviewable.

## Riesgos y mitigaciones

| Riesgo | Mitigación |
|--------|------------|
| UUID atascado en `processing_uuids` si subprocess cuelga | Documentar timeout; KeyboardInterrupt ya detiene watcher |
| `routed_ok_pending_absent` crece sin límite | Limpiar entrada cuando `not path.is_file()` en cada poll |
| Purga agresiva F3 | Default `--dry-run`; criterios conservadores §3.3 |

## Estimación

| Fase | Esfuerzo relativo |
|------|-------------------|
| F1 | S |
| F2 | M |
| F3 | M |
| Tests + validación | M |
