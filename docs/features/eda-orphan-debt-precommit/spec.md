---
feature_name: eda-orphan-debt-precommit
created: "2026-05-25"
process: bug-fix
base: main
scope: eda-orphan-debt-precommit
version_spec: "1.1.0"
---

# Especificación — Correlación EDA y cierre pre-commit

## 1. Invariante de correlación (post-fix)

Toda entidad indexada en catálogos Core (`SddIA/*/index.md`) debe correlacionarse con al menos un evento `Domain_Entity_Created` cuyo `payload.entity_uuid` coincida, buscando instancias ECST en:

| Ubicación bus | Patrón | Incluido hoy | Incluido post-fix |
|---------------|--------|--------------|-------------------|
| `eda_bus.pending/` | `*.json` | ✅ | ✅ |
| `eda_bus.processed/` | `{event_uuid}.json` (cabecera) | ❌ | ✅ |
| `eda_bus.processing/` | `{event_uuid}.json` (cabecera) | ❌ | ✅ |
| Legacy `docs/events/*/` | `*.json` | ✅ | ✅ |

**No** incluir en correlación: testigos en `*/subscribers/`, cabeceras `dead-letter/` (eventos terminales Kaizen no crean entidad).

Contrato público de `find_existing_domain_event()` **sin cambio** — hereda ampliación vía `iter_bus_event_files()`.

## 1.1 Fase 0 — Baseline en frío (pre-modificación)

| Paso | Comando | Mandato |
|------|---------|---------|
| 0.2 | `audit-entity-eda-coverage.py --scan --json` | Volcar stdout en `.tmp/eda-orphan-baseline.json` **antes** de editar código o bus |
| 0.4 | `verify-process-integrity.py` | Debe retornar OK |

Prohibido iniciar Track 1 sin baseline persistido en `.tmp/`.

## 2. Track 1 — Cambio estructural

### 2.1 `eda_bus_utils.py`

Función `iter_bus_event_files(repo: Path) -> list[Path]`:

```python
# Comportamiento objetivo (pseudocódigo)
files = pending/*.json
files += processed/{uuid}.json   # cabeceras ECST, no subscribers/
files += processing/{uuid}.json
files += legacy docs/events/**/*.json (sin duplicar pending)
return sorted_unique(files)
```

Reglas:

- Resolver rutas vía `load_eda_bus(repo)` (SSOT `cumulo.paths.json`).
- Excluir `*/subscribers/*.json` en todos los estados.
- Orden estable: `sorted()` por path relativo.
- Sin duplicados si legacy apunta al mismo árbol que V3+.

### 2.2 Validación obligatoria post-F1 (V4 inmediato)

Tras aplicar el fix en `eda_bus_utils.py`, Tekton **debe** ejecutar en secuencia:

```powershell
python SddIA/scripts/daemons/event-watcher.py --once
python SddIA/scripts/qa/audit-entity-eda-coverage.py --scan --json
```

Volcar resultado en `.tmp/eda-orphan-post-f1-v4.json`. Este gate **no** se difiere a F3.

### 2.3 Smoke / regresión F1

| Escenario | Expectativa |
|-----------|-------------|
| UUID con evento solo en `processed/` | `find_existing_domain_event` retorna match; `--scan` no lo marca huérfano |
| UUID sin evento en ningún estado | Sigue en lista `orphans` |
| `--emit` idempotente | No duplica si match en `processed/` |

### 2.4 Documentación normativa (F1.4)

Añadir nota en `features-documentation-pattern` § Ruido de Sistema:

- Correlación audit incluye cabeceras `processed/` y `processing/`, no solo `pending/`.
- Prohibido inferir cobertura EDA con eventos únicamente en pending tras watcher.

## 3. Track 2 — Backfill Fase C (solo deuda real)

### 3.0 Resolución de impasse (post F1 + V4)

| Resultado | Acción |
|-----------|--------|
| `orphan_count == 0` | **No ejecutar Track 2.** La deuda histórica queda absorbida por la lectura multi-estado del auditor. CA5/O4 no aplican. |
| `orphan_count > 0` | **Detener Tekton** tras `--emit --skip-dlt --json`. Emitir manifiesto preliminar; **consultar operador** antes de `--anchor-merkle`. |

Ejecutar Track 2 **solo si** persiste deuda real tras F1 y V4.

### 3.1 Comandos canónicos (solo tras consulta si orphan_count > 0)

```powershell
python SddIA/scripts/qa/audit-entity-eda-coverage.py `
  --emit --skip-dlt --json `
  --correlation-id eda-backfill-precommit-20260525

$env:SDDIA_LAB_SIMULATE_IOTA = "1"
python SddIA/scripts/qa/audit-entity-eda-coverage.py `
  --anchor-merkle docs/features/eda-orphan-debt-precommit/backfill-manifest.json
```

### 3.2 Manifiesto `backfill-manifest.json`

Campos mínimos post-ejecución:

| Campo | Valor esperado |
|-------|----------------|
| `correlation_id` | `eda-backfill-precommit-20260525` |
| `emit_ok` | N (entidades emitidas) |
| `orphan_count_after` | `0` |
| `transaction_digest` | Registrado tras `--anchor-merkle` |

Acta Merkle: `merkle-acta-eda-backfill-precommit-20260525.json` (o derivado de `correlation_id`).

## 4. Integración gates existentes

### 4.1 `pre_commit_gate.py`

Invoca `audit-entity-eda-coverage.py --scan --json`. Bloquea si `orphan_count > 0` **y** staged toca genoma `SddIA/`.

**Criterio O3:** tras fix, commit de prueba sobre `SddIA/norms/...` sin `SDDIA_SKIP_HOOKS` → exit 0.

### 4.2 `delivery-close-cycle`

Fase Aduana EDA genómica (`execute_process_capsules.py`): mismo script `--scan`. Excepción temporal si `_backfill_manifest_active` — este fix cierra la deuda para no depender de excepción.

## 5. Track 4 — Prevención forward (documentación)

| Touchpoint | Contenido |
|------------|-----------|
| `SddIA/norms/git-operations.md` o norma EDA | Forja productiva solo vía `entity-manager` |
| `execution.md` (este feature) | Orden Track 1 → Track 2; prohibido watcher entre emit y scan pre-commit |
| `implementation.md` | Lista touchpoints preventivos |

Opcional futuro (no gate merge): pre-commit por diff incremental (F4.3 PBI).

## 6. Criterios de aceptación

| ID | Criterio |
|----|----------|
| **CA1** | `iter_bus_event_files` incluye processed + processing cabeceras |
| **CA2** | `--scan` → `orphan_count: 0` sostenido post `event-watcher --once` (V4) |
| **CA3** | Pre-commit PASS en commit prueba genoma (V2) |
| **CA4** | `delivery-close-cycle` lab → `argos_verdict: pass` (V3) |
| **CA5** | Manifiesto Merkle en PR **solo si** Track 2 ejecutado (`orphan_count > 0` post V4) |
| **CA6** | `validacion.md` APTO + PBI en `done/` en un único PR |
