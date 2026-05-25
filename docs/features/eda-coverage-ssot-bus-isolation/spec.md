---
feature_name: eda-coverage-ssot-bus-isolation
created: "2026-05-25"
process: feature
base: main
scope: eda-coverage-ssot-bus-isolation
version_spec: "1.0.0"
---

# Especificación — SSOT eda-coverage y desacople bus EDA

## 1. Invariante arquitectónico (post-feature)

```text
Correlación genómica válida  ⇔  coverage_matrix[entity_uuid].is_covered == true
                               ∧  last_hash coherente con artefacto (modo estricto)

Bus EDA                      →  transporte + orquestación; barrido absoluto
Aduana (--scan)              →  solo lectura eda-coverage.json + índices Core
```

Prohibido usar instancias ECST en `pending/` / `processing/` / `processed/` como **gate** de `orphan_count`.

## 2. SSOT — `SddIA/core/eda-coverage.json`

### 2.1 Esquema

```json
{
  "version": "1.0.0",
  "coverage_matrix": {
    "<entity_uuid>": {
      "is_covered": true,
      "last_emitted_event": "Domain_Entity_Created",
      "last_hash": "sha256:<hex>",
      "correlation_timestamp": "2026-05-25T12:00:00+00:00"
    }
  }
}
```

| Campo | Tipo | Reglas |
|-------|------|--------|
| `version` | string semver | `"1.0.0"` inicial |
| `coverage_matrix` | object | Clave = `entity_uuid` v4 |
| `is_covered` | boolean | `true` si sello vigente |
| `last_emitted_event` | string | Tipo ECST (`Domain_Entity_*`) |
| `last_hash` | string | Prefijo `sha256:` obligatorio |
| `correlation_timestamp` | string | ISO 8601 UTC |

### 2.2 Registro en cúmulo

En `SddIA/core/cumulo.paths.json`:

```json
"eda_coverage": "SddIA/core/eda-coverage.json"
```

### 2.3 Módulo `eda_coverage_utils.py` (nuevo)

| Función | Contrato |
|---------|----------|
| `coverage_path(repo) -> Path` | Resuelve vía cumulo `eda_coverage` |
| `load_coverage(repo) -> dict` | Parse JSON; `{}` matrix si ausente |
| `upsert_entity_coverage(repo, entity_uuid, *, event_type, last_hash) -> None` | Upsert atómico (write tmp + replace) |
| `is_entity_covered(repo, entity_uuid) -> bool` | Lookup matrix |
| `backfill_from_indexed(repo) -> dict` | Inventario + upsert masivo |

Escritura atómica: `{path}.tmp` → `os.replace`.

## 3. Parametrización `EVENT_BUS_PATH`

### 3.1 Precedencia en `load_eda_bus(repo)`

```text
1. os.environ["EVENT_BUS_PATH"]  (normalizado, sin trailing slash)
2. cumulo.paths.json → event_bus / eda_bus.*
3. default "./.events"
```

Si `EVENT_BUS_PATH` está definido, reconstruir claves `pending`, `processing`, `processed`, `dead_letter` y `*/subscribers` bajo esa raíz.

### 3.2 Perfil test

**`.dev/.env.test.example`** (commiteado):

```dotenv
SDDIA_ENV=test
EVENT_BUS_PATH=.tmp/events_test
SDDIA_LAB_SIMULATE_IOTA=1
SDDIA_LAB_SIMULATE_SYNC_INDEX=1
```

**`run-eda-e2e-lab.py`:** al inicio de `main()`:

1. `load_hierarchical_env(repo)`
2. Si existe `.dev/.env.test`, merge overlay (test prevalece sobre global)
3. `ensure_event_bus_topology(repo)` bajo ruta resuelta

Extensión opcional en `env_loader.py`: `load_test_env_overlay(repo) -> dict`.

## 4. Emisión doble fase — `emit-domain-mutation`

### 4.1 Secuencia en `_run_emit_domain_mutation`

```python
# Pseudocódigo objetivo
if lifecycle != "delete" and entity_uuid:
    upsert_entity_coverage(repo, entity_uuid, event_type=..., last_hash=hash_signature_new)
seal = _write_pending_event(repo, event)
```

| Regla | Detalle |
|-------|---------|
| Orden | SSOT (A) → pending (B) |
| Idempotencia bus | Conservar check `find_existing_domain_event` para no duplicar pending |
| Idempotencia SSOT | Upsert sobrescribe entrada existente |
| Delete | Marcar `is_covered: false` o eliminar entrada (decisión: **eliminar** entrada en delete) |

### 4.2 Backfill SSOT (one-shot)

Nuevo flag CLI en `audit-entity-eda-coverage.py`:

```powershell
python SddIA/scripts/qa/audit-entity-eda-coverage.py --backfill-coverage --json
```

Comportamiento:

1. Recorrer `ENTITY_DIRS` + `index.md` (misma lógica que `--scan`).
2. Por cada entidad con artefacto `.md`: calcular hash, upsert matrix con `Domain_Entity_Created`.
3. **No** escribir en bus pending (solo SSOT).
4. Emitir reporte `{backfilled: N, skipped: M}`.

Ejecutar **antes** de activar scan SSOT-only en producción del hook.

## 5. Aduana — refactor `scan_orphans`

### 5.1 Comportamiento objetivo

```python
for each indexed entity_uuid:
    cov = coverage_matrix.get(uuid)
    if not cov or not cov.get("is_covered"):
        orphans.append(entry)
    elif strict_hash_mode and cov["last_hash"] != current_hash:
        orphans.append(entry)  # hash drift
```

| Flag | Default | Efecto |
|------|---------|--------|
| `--strict-hash` | off en v1.0 implementación; on en V2 gate | Compara `last_hash` vs artefacto |

Eliminar llamada gate a `find_existing_domain_event` en ruta `--scan` principal.

Mantener `find_existing_domain_event` / `iter_bus_event_files` para `--emit` idempotente y diagnóstico.

### 5.2 Integraciones existentes

| Consumidor | Cambio |
|------------|--------|
| `pre_commit_gate.py` | Ninguno (hereda JSON scan) |
| `execute_process_capsules.py` delivery-close | Retirar dependencia `_backfill_manifest_active` cuando V1 PASS |
| `features-documentation-pattern` | Actualizar § Cobertura EDA: SSOT eda-coverage |

## 6. Sweep vacío y router topológico

### 6.1 `archive_event_after_sweep`

Eliminar:

```python
retain_processed = event_type == "Domain_Entity_Created"
```

Purgar siempre pending, cabeceras processing/processed, testigos processed.

### 6.2 `route_domain_event_core.py`

Rama `if not subscribers:` — **no retornar** sin sweep. Invocar:

```python
result["data"]["sweep"] = try_sweep_event(repo, bus, event_uuid, registry=registry)
```

### 6.3 `try_sweep_event` — suscriptores aplicables

Nueva función interna (sin fallback global):

```python
def applicable_subscriber_ids(registry, event_type, payload) -> list[str]:
    origin = resolve_origin_topology(payload)
    return [
        subscriber_id(sub)
        for sub in registry.get(event_type) or []
        if isinstance(sub, dict) and subscriber_applies_to_topology(sub, origin)
    ]
```

En `try_sweep_event`:

| Condición | Acción |
|-----------|--------|
| `applicable == []` | `archive_event_after_sweep` → `status: purged`, `purged: true` |
| `applicable != []` y todos processed | Purga (comportamiento actual) |
| In-flight | `status: awaiting` |

**No** usar fallback `required_subscriber_ids(registry, event_type)` para decisión de purga.

## 7. Smoke E2E — contrato `run-eda-e2e-lab.py`

Criterio éxito **sin cambio**:

```python
success = route_ok and not pending.is_file() and sweep.get("status") == "purged"
```

Con A+A′ + bus aislado + SSOT, el lab debe:

1. Forjar entidad local en bus test.
2. Enrutar → sweep purga padre.
3. Teardown limpia `.tmp/` (higiene Kaizen existente).

## 8. Fase 0 — Baseline (pre-modificación)

| Paso | Comando |
|------|---------|
| 0.1 | `audit-entity-eda-coverage.py --scan --json` → `.tmp/eda-coverage-baseline.json` |
| 0.2 | `run-eda-e2e-lab.py --entity-class tool --json` → `.tmp/eda-e2e-baseline.json` |
| 0.3 | `verify-process-integrity.py` |

Prohibido modificar código antes de 0.1–0.3.

## 9. Criterios de aceptación

| ID | Criterio |
|----|----------|
| **CA1** | `eda-coverage.json` + ref cumulo existen |
| **CA2** | `load_eda_bus` respeta `EVENT_BUS_PATH` |
| **CA3** | `emit-domain-mutation` upsertea SSOT antes de pending |
| **CA4** | `--scan` → `orphan_count: 0` vía SSOT post-backfill |
| **CA5** | V2: watcher + scan sin cabeceras retenidas |
| **CA6** | `run-eda-e2e-lab.py --json` → `success: true` |
| **CA7** | CI `eda-bus-e2e-smoke` SUCCESS |
| **CA8** | Workaround `retain_processed` eliminado |
| **CA9** | `validacion.md` APTO + PBI en `done/` (un PR) |
