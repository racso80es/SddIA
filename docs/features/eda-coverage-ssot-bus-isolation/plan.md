---
feature_name: eda-coverage-ssot-bus-isolation
created: "2026-05-25"
process: feature
phases: 9
branch_name: feat/eda-coverage-ssot-bus-isolation
---

# Plan — SSOT eda-coverage y desacople bus EDA

## Estado planificación (2026-05-25)

| Hito | Entregable | Estado |
|------|------------|--------|
| **P0** | Rama `feat/eda-coverage-ssot-bus-isolation` | [x] |
| **P0** | `objectives` / `clarify` / `spec` / `plan` | [x] |
| **F0** | Baseline `.tmp/` | [x] |
| **F1** | SSOT esqueleto + cumulo | [x] |
| **F2** | `EVENT_BUS_PATH` + `.env.test` | [x] |
| **F3** | `eda_coverage_utils.py` | [x] |
| **F4** | Emisión doble fase | [x] |
| **F5** | Backfill SSOT | [x] |
| **F6** | Refactor audit `--scan` | [x] |
| **F7** | Sweep vacío + router/sweep topológico | [x] |
| **F8** | Gates V1–V6 (local) | [x] |
| **F9** | Cierre documental + PBI `done/` | [x] |
| **Cierre** | PR + CI `eda-bus-e2e-smoke` | [ ] |

**Detención explícita:** planificación completada. Tekton inicia en **F0** (siguiente fase del proceso).

---

## Fase 0 — Baseline e integridad (auditoría en frío)

**Mandato:** volcar estado actual en `.tmp/` **antes** de cualquier modificación de código o bus.

| Paso | Acción | Evidencia |
|------|--------|-----------|
| 0.1 | `--scan --json` → `.tmp/eda-coverage-baseline.json` | orphan_count, indexed_entities |
| 0.2 | `run-eda-e2e-lab.py --entity-class tool --json` → `.tmp/eda-e2e-baseline.json` | Confirmar `success: false` pre-fix |
| 0.3 | `verify-process-integrity.py` | OK pre-cambio |

```powershell
python SddIA/scripts/qa/audit-entity-eda-coverage.py --scan --json `
  | Out-File -Encoding utf8 .tmp/eda-coverage-baseline.json

python SddIA/scripts/qa/run-eda-e2e-lab.py --entity-class tool --json `
  | Out-File -Encoding utf8 .tmp/eda-e2e-baseline.json

python SddIA/scripts/qa/verify-process-integrity.py
```

Archivar timestamp en `execution.md` (no commitear `.tmp/`).

---

## Fase 1 — SSOT esqueleto y cúmulo

| Paso | Touchpoint | Entregable |
|------|------------|------------|
| 1.1 | `SddIA/core/eda-coverage.json` | `{ "version": "1.0.0", "coverage_matrix": {} }` |
| 1.2 | `SddIA/core/cumulo.paths.json` | Clave `eda_coverage` |
| 1.3 | Smoke JSON | `python -m json.tool SddIA/core/eda-coverage.json` |

---

## Fase 2 — Parametrización bus y perfil test

| Paso | Touchpoint | Entregable |
|------|------------|------------|
| 2.1 | `eda_bus_utils.load_eda_bus()` | Lectura `EVENT_BUS_PATH` con precedencia documentada |
| 2.2 | `.dev/.env.test.example` | Plantilla test commiteada |
| 2.3 | `.dev/.env.example` | Comentario `EVENT_BUS_PATH` |
| 2.4 | `env_loader.py` (opc.) | `load_test_env_overlay()` |
| 2.5 | `run-eda-e2e-lab.py` | Carga overlay test al inicio |
| 2.6 | `test_eda_bus_v3plus.py` | Test unitario ruta env |

**Gate parcial F2:** con `EVENT_BUS_PATH=.tmp/events_test`, lab crea pending bajo test path, no bajo `.events/`.

---

## Fase 3 — Módulo eda-coverage

| Paso | Touchpoint | Entregable |
|------|------------|------------|
| 3.1 | `SddIA/scripts/qa/eda_coverage_utils.py` | load / upsert / path |
| 3.2 | Tests | Upsert idempotente; atomic write |

---

## Fase 4 — Emisión doble fase

| Paso | Touchpoint | Entregable |
|------|------------|------------|
| 4.1 | `execute-action.py` → `_run_emit_domain_mutation` | Fase A upsert antes de `_write_pending_event` |
| 4.2 | Delete lifecycle | Eliminar entrada matrix en delete |
| 4.3 | Smoke | Forja lab → entrada en SSOT + pending test bus |

**Orden:** F4 depende de F1 + F3. No activar gate scan SSOT-only hasta F5.

---

## Fase 5 — Backfill SSOT one-shot

| Paso | Acción | Criterio |
|------|--------|----------|
| 5.1 | `--backfill-coverage --json` | N ≈ indexed_entities |
| 5.2 | Verificar matrix | 46 entradas (o count actual index) |
| 5.3 | `--scan` (modo legacy aún) | Baseline post-backfill documentado |

```powershell
python SddIA/scripts/qa/audit-entity-eda-coverage.py --backfill-coverage --json
```

**Pausa:** si `backfilled` << `indexed_entities`, detener y documentar gaps en `execution.md`.

---

## Fase 6 — Refactor audit `--scan`

| Paso | Touchpoint | Entregable |
|------|------------|------------|
| 6.1 | `audit-entity-eda-coverage.py` → `scan_orphans` | Lookup SSOT; sin gate bus |
| 6.2 | Flag `--backfill-coverage` | CLI wiring |
| 6.3 | `features-documentation-pattern.md` | § SSOT eda-coverage |
| 6.4 | `delivery-close-cycle.md` / capsules | Retirar excepción manifiesto si V1 PASS |

**Gate V1:**

```powershell
python SddIA/scripts/qa/audit-entity-eda-coverage.py --scan --json
# orphan_count: 0
```

---

## Fase 7 — Sweep vacío y router topológico

| Paso | Touchpoint | Entregable |
|------|------------|------------|
| 7.1 | `eda_bus_utils.archive_event_after_sweep` | Eliminar `retain_processed` |
| 7.2 | `eda_bus_utils.try_sweep_event` | `applicable_subscriber_ids` sin fallback; purga si `[]` |
| 7.3 | `route_domain_event_core.py` | Sweep en rama `subscribers == []` |
| 7.4 | `test_eda_bus_v3plus.py` | Tests retención → sweep vacío; local topology purge |

**Gate V2 (obligatorio inmediato post-F7):**

```powershell
python SddIA/scripts/daemons/event-watcher.py --once
python SddIA/scripts/qa/audit-entity-eda-coverage.py --scan --json
# orphan_count: 0 — sin cabeceras Domain_Entity_Created retenidas en processed/
```

---

## Fase 8 — Validación gates (Aduana + CI)

| ID | Comando | Pass |
|----|---------|------|
| **V1** | `--scan --json` | `orphan_count: 0` |
| **V2** | watcher + `--scan` | Sin retención cabeceras |
| **V3** | Commit prueba `SddIA/` | Pre-commit sin BLOCKED |
| **V4** | `run-eda-e2e-lab.py --json` | `success: true` |
| **V5** | CI `eda-bus-e2e-smoke` | SUCCESS en PR |
| **V6** | `verify-process-integrity.py` | OK |

Registrar en `validacion.md` (`global: APTO`, `pbi_archived: true`).

---

## Fase 9 — Cierre documental

| Paso | Acción |
|------|--------|
| 9.1 | `implementation.md` + `execution.md` |
| 9.2 | `validacion.md` APTO |
| 9.3 | PBI → `docs/todos/done/` |
| 9.4 | `README.md` — sección EVENT_BUS_PATH |
| 9.5 | `delivery-close-cycle` → PR único |

---

## Secuencia Tekton (orden estricto)

```text
F0 baseline (.tmp/) + integridad
  → F1 SSOT esqueleto + cumulo
  → F2 EVENT_BUS_PATH + .env.test + lab overlay
  → F3 eda_coverage_utils
  → F4 emit doble fase
  → F5 backfill SSOT
  → F6 refactor audit scan
  → F7 sweep vacío + router topológico
  → V2 inmediato (watcher + scan)
  → F8 gates V1–V6
  → F9 validacion + PBI done + PR
```

**Paralelismo prohibido:**

- F6 no activa scan SSOT-only antes de F5.
- F7 no poda retención antes de F6 confirme V1.
- F8 V4 antes de F7 completo → expectativa fallo (baseline documentado en F0.2).

---

## Touchpoints código (resumen)

| Archivo | Fase | Cambio |
|---------|------|--------|
| `SddIA/core/eda-coverage.json` | F1 | Nuevo SSOT |
| `SddIA/core/cumulo.paths.json` | F1 | Ref `eda_coverage` |
| `SddIA/scripts/qa/eda_bus_utils.py` | F2, F7 | EVENT_BUS_PATH; sweep topológico; sweep vacío |
| `SddIA/scripts/qa/eda_coverage_utils.py` | F3 | Nuevo módulo |
| `SddIA/scripts/qa/execute-action.py` | F4 | Doble fase emit |
| `SddIA/scripts/qa/audit-entity-eda-coverage.py` | F5, F6 | backfill + scan SSOT |
| `SddIA/scripts/qa/route_domain_event_core.py` | F7 | Sweep sin suscriptores |
| `SddIA/scripts/qa/run-eda-e2e-lab.py` | F2 | Carga `.env.test` |
| `SddIA/scripts/qa/env_loader.py` | F2 | Overlay test (opc.) |
| `.dev/.env.test.example` | F2 | Plantilla |
| `SddIA/scripts/qa/test_eda_bus_v3plus.py` | F2, F7 | Regresión |
| `SddIA/library/norms/features-documentation-pattern.md` | F6 | Norma SSOT |
| `README.md` | F9 | Documentación operador |

---

## Artefactos pendientes post-planificación

| Path | Fase |
|------|------|
| `implementation.md` | F1–F7 |
| `execution.md` | F0–F8 |
| `validacion.md` | F8 |
| PBI en `docs/todos/done/` | F9 |

---

## Verificación Tekton (orden de ejecución)

```powershell
# F0 — antes de cualquier modificación
python SddIA/scripts/qa/audit-entity-eda-coverage.py --scan --json `
  | Out-File -Encoding utf8 .tmp/eda-coverage-baseline.json
python SddIA/scripts/qa/run-eda-e2e-lab.py --entity-class tool --json `
  | Out-File -Encoding utf8 .tmp/eda-e2e-baseline.json
python SddIA/scripts/qa/verify-process-integrity.py

# F8 — tras F1–F7
python SddIA/scripts/qa/audit-entity-eda-coverage.py --scan --json
python SddIA/scripts/daemons/event-watcher.py --once
python SddIA/scripts/qa/audit-entity-eda-coverage.py --scan --json
python SddIA/scripts/qa/run-eda-e2e-lab.py --entity-class tool --json
python SddIA/scripts/qa/verify-process-integrity.py
```
