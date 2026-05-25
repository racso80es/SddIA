---
feature_name: eda-orphan-debt-precommit
created: "2026-05-25"
process: bug-fix
phases: 5
branch_name: fix/eda-orphan-debt-precommit
---

# Plan — Deuda EDA orphan_count (pre-commit)

## Estado planificación (2026-05-25, refinamiento v1.1)

| Hito | Entregable | Estado |
|------|------------|--------|
| **P0** | Rama `fix/eda-orphan-debt-precommit` | [x] |
| **P0** | `objectives` / `clarify` / `spec` / `plan` (v1.1 impasse) | [x] |
| **F0** | Baseline `.tmp/eda-orphan-baseline.json` | [x] |
| **F0** | `verify-process-integrity.py` | [x] |
| **F1** | Fix `eda_bus_utils.iter_bus_event_files` | [x] |
| **F1** | Nota norma + V4 (`daemons/event-watcher.py`) | [x] |
| **F2** | Emit preliminar (`emit_ok: 43`) | [x] |
| **F2** | `--anchor-merkle` + manifiesto versionado | [x] |
| **F2** | V4 post-retención (`orphan_count: 0`) | [x] |
| **Kaizen** | PBI correlación durable sin bus | [x] |
| **F3** | Gates V1–V5 + `validacion.md` | [x] |
| **F4** | Prevención doc + PBI `done/` | [x] |
| **Cierre** | PR #47 + `delivery-close-cycle` | [x] |

---

## Fase 0 — Baseline e inventario (auditoría en frío)

**Mandato Tekton:** volcar el JSON de estado actual en `.tmp/` **antes de cualquier modificación** de código o bus.

| Paso | Acción | Evidencia |
|------|--------|-----------|
| 0.1 | Artefactos documentales F0 | Este directorio |
| 0.2 | `--scan --json` → `.tmp/eda-orphan-baseline.json` | Snapshot con timestamp; **pre-cambio** |
| 0.3 | `--dry-run` | Lista UUIDs/clases **solo si** baseline `orphan_count > 0` |
| 0.4 | `verify-process-integrity.py` | OK pre-cambio |

```powershell
# F0.2 — obligatorio antes de F1
python SddIA/scripts/qa/audit-entity-eda-coverage.py --scan --json `
  | Out-File -Encoding utf8 .tmp/eda-orphan-baseline.json

# F0.4
python SddIA/scripts/qa/verify-process-integrity.py
```

Archivar ruta y timestamp en `execution.md` (no commitear `.tmp/`).

---

## Fase 1 — Track 1: Fix correlación audit

| Paso | Touchpoint | Entregable |
|------|------------|------------|
| 1.1 | `SddIA/scripts/qa/eda_bus_utils.py` | `iter_bus_event_files()` ampliado: `processed/` + `processing/` |
| 1.2 | `find_existing_domain_event()` | Hereda búsqueda; sin cambio de firma |
| 1.3 | Smoke / regresión | UUID en `processed/` no cuenta como huérfano |
| 1.4 | `features-documentation-pattern.md` | § Ruido de Sistema — correlación multi-estado |

**Mandato Tekton post-F1 (obligatorio, inmediato):**

```powershell
python SddIA/scripts/daemons/event-watcher.py --once
python SddIA/scripts/qa/audit-entity-eda-coverage.py --scan --json `
  | Out-File -Encoding utf8 .tmp/eda-orphan-post-f1-v4.json
```

Este par equivale al gate **V4 decisorio** — no diferir a F3.

---

## Resolución de impasse (post F1 + V4)

| Resultado V4 | Acción Tekton |
|--------------|---------------|
| `orphan_count == 0` | **Saltar Track 2.** Deuda absorbida por lectura de historia. Continuar F3–F4 sin manifiesto Merkle. |
| `orphan_count > 0` | **Detenerse.** Ejecutar `--emit --skip-dlt --json` → manifiesto preliminar; **consultar operador** antes de `--anchor-merkle`. |

---

## Fase 2 — Track 2: Backfill Fase C (solo si impasse > 0)

**Precondición:** F1 aplicado **y** V4 ejecutado **y** `orphan_count > 0`.

**Prohibido** si V4 retorna `orphan_count == 0` (deuda ya absorbida por el fix estructural).

**Pausa obligatoria:** tras `--emit`, emitir manifiesto preliminar y **consultar operador** antes de `--anchor-merkle`.

```powershell
python SddIA/scripts/qa/audit-entity-eda-coverage.py `
  --emit --skip-dlt --json `
  --correlation-id eda-backfill-precommit-20260525

# STOP — consultar operador antes de continuar

$env:SDDIA_LAB_SIMULATE_IOTA = "1"
python SddIA/scripts/qa/audit-entity-eda-coverage.py `
  --anchor-merkle docs/features/eda-orphan-debt-precommit/backfill-manifest.json
```

| Entregable | Path |
|------------|------|
| Manifiesto | `backfill-manifest.json` |
| Acta Merkle | `merkle-acta-eda-backfill-precommit-20260525.json` |
| Digest | En manifiesto + `validacion.md` |

---

## Fase 3 — Validación gates (Aduana)

| ID | Comando | Pass |
|----|---------|------|
| **V1** | `audit-entity-eda-coverage.py --scan --json` | `orphan_count: 0` |
| **V2** | Commit prueba `SddIA/norms/...` | Sin BLOCKED |
| **V3** | `execute-process --process delivery-close-cycle` | `argos_verdict: pass` |
| **V4** | `event-watcher --once` + `--scan` | `orphan_count` sigue 0 |
| **V5** | `verify-process-integrity.py` | OK |

Registrar en `validacion.md` (`checks`, `global: APTO`, `pbi_archived: true`).

---

## Fase 4 — Prevención forward

| Paso | Acción | Prioridad |
|------|--------|-----------|
| 4.1 | Norma: forja productiva vía `entity-manager` | Alta |
| 4.2 | Runbook `execution.md`: orden Track 1 → 2; no watcher entre emit y scan | Alta |
| 4.3 | Pre-commit por diff (futuro) | Media — fuera de merge |
| 4.4 | Remediar `markdown-table-editor` huérfana puntual | Baja — si persiste |

---

## Secuencia Tekton (orden estricto)

```text
F0 auditoría en frío (.tmp/ baseline + integridad) — sin tocar código
  → F1 fix eda_bus_utils
  → V4 inmediato (event-watcher --once + scan → .tmp/ post-f1-v4)
  → impasse:
       orphan_count == 0 → saltar F2; deuda absorbida
       orphan_count > 0  → STOP; emit manifiesto; consultar; luego anchor-merkle
  → F3 gates V1–V5
  → F4 prevención doc
  → validacion.md APTO + PBI → done/
  → delivery-close-cycle → PR único → accept-pr
```

**Paralelismo prohibido:** F2 no arranca si V4 == 0; F2 no continúa a Merkle sin consulta explícita.

---

## Touchpoints código (resumen)

| Archivo | Fase | Cambio |
|---------|------|--------|
| `SddIA/scripts/qa/eda_bus_utils.py` | F1 | Correlación multi-estado |
| `SddIA/library/norms/features-documentation-pattern.md` | F1 | Nota Ruido de Sistema |
| `SddIA/norms/git-operations.md` | F4 | Forja productiva (si aplica) |
| `docs/features/eda-orphan-debt-precommit/backfill-manifest.json` | F2 | Condicional |
| `docs/features/eda-orphan-debt-precommit/validacion.md` | F3 | APTO |

---

## Artefactos pendientes post-planificación

| Path | Fase |
|------|------|
| `implementation.md` | F1–F4 |
| `execution.md` | F0–F3 |
| `validacion.md` | F3 |
| `backfill-manifest.json` | F2 (condicional) |
| `merkle-acta-*.json` | F2 (condicional) |
| PBI en `docs/todos/done/` | Cierre |

---

## Verificación Tekton (orden de ejecución)

```powershell
# F0 — antes de cualquier modificación
python SddIA/scripts/qa/audit-entity-eda-coverage.py --scan --json `
  | Out-File -Encoding utf8 .tmp/eda-orphan-baseline.json
python SddIA/scripts/qa/verify-process-integrity.py

# F1 — tras aplicar fix en eda_bus_utils.py
python SddIA/scripts/daemons/event-watcher.py --once
python SddIA/scripts/qa/audit-entity-eda-coverage.py --scan --json `
  | Out-File -Encoding utf8 .tmp/eda-orphan-post-f1-v4.json
# → decidir impasse (saltar F2 vs STOP + consulta Merkle)
```
