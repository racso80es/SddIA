---
feature_name: eda-orphan-debt-precommit
created: "2026-05-25"
process: bug-fix
version_clarify: "1.1.0"
---

# Clarificación — Correlación bus EDA y pre-commit

## 1. Problema confirmado

| Campo | Valor |
|-------|--------|
| **Síntoma** | `SddIA pre-commit: BLOCKED — Argos orphan_count=43` |
| **Alcance incidente** | 46 indexadas; 43 huérfanas en 8 clases |
| **Bloqueo** | Commits locales con hook activo; `delivery-close-cycle` con mutaciones `SddIA/` |
| **Contexto** | Deuda preexistente; expuesta tras Kaizen higiene `.tmp/` (PR #44) |

### Causas raíz (laudo PBI)

| # | Causa | Tipo | Acción en este fix |
|---|--------|------|-------------------|
| **C1** | Forja histórica Core sin `entity-manager` / sin evento | Deuda histórica | Backfill Fase C (Track 2) si scan > 0 post-F1 |
| **C2** | `iter_bus_event_files()` no incluye `processed/` ni cabeceras `processing/` | **Bug estructural** | Track 1 — obligatorio |
| **C3** | Backfill lab sin anclaje durable o watcher antes de commit estable | Operativa | Protocolo F3 V4 + manifiesto en PR |

## 2. Baseline F0 (2026-05-25, rama `fix/eda-orphan-debt-precommit`)

```powershell
python SddIA/scripts/qa/audit-entity-eda-coverage.py --scan --json
```

| Campo | Valor |
|-------|--------|
| `orphan_count` | **0** |
| `indexed_entities` | 46 |
| Eventos en `.events/pending/` | 44 |
| Eventos en `.events/processed/` (cabeceras) | 10 |

**Interpretación:** el scan actual está **enmascarado** por eventos aún en `pending/` (backfill vanguardia). La regresión C2 se manifiesta cuando el watcher promueve instancias a `processed/` y el audit deja de verlas. **Track 1 sigue siendo obligatorio** aunque el baseline sea 0.

Evidencia de regresión documentada: `docs/features/vanguardia-soberania-local/execution.md` § Backfill EDA Fase C — nota «desacople pending↔scan».

Snapshot efímero (no commitear): `.tmp/eda-orphan-baseline.json` — **mandato Tekton F0.2:** volcar el JSON de estado actual en `.tmp/` **antes de cualquier modificación** de código o bus.

## 2.1 Mandato Tekton — F0.2–F0.4 (auditoría en frío)

| Paso | Acción | Regla |
|------|--------|-------|
| **0.2** | `audit-entity-eda-coverage.py --scan --json` | Redirigir stdout → `.tmp/eda-orphan-baseline.json` **antes** de tocar `eda_bus_utils.py` o el bus |
| **0.3** | `--dry-run` | Solo si baseline `orphan_count > 0`; inventario UUID/clases |
| **0.4** | `verify-process-integrity.py` | Debe retornar OK pre-cambio |

**Prohibido** iniciar F1 sin baseline persistido en `.tmp/`. Referenciar ruta y timestamp en `execution.md` (no commitear el JSON).

## 3. Laudo de diseño

```text
Correlación válida = evento Domain_Entity_Created cuyo payload.entity_uuid
                     existe en pending ∪ processed ∪ processing ∪ legacy

Backfill Fase C     = --emit --skip-dlt --correlation-id <lote>
                     + --anchor-merkle (obligatorio cierre)
                     + orphan_count_after: 0 en manifiesto versionado

Orden Tekton        = Fix correlación (Track 1) ANTES de backfill (Track 2)
                     salvo excepción documentada aquí

Post-cierre         = event-watcher --once + --scan → orphan_count permanece 0
```

### 3.1 Decisión D1 — Resolución de impasse (Track 2)

| Condición post F1 + V4 | Decisión |
|------------------------|----------|
| `orphan_count == 0` tras fix **y** tras `event-watcher --once` + re-scan | **No ejecutar Track 2.** La deuda queda absorbida por la capacidad del auditor para leer la historia (`pending` ∪ `processed` ∪ `processing` ∪ legacy). O4 no aplica. |
| `orphan_count > 0` tras fix + V4 | **Detener Tekton.** Emitir manifiesto preliminar (`--emit --skip-dlt --json`); **consultar operador** antes de `--anchor-merkle`. Prohibido anclaje Merkle autónomo. |
| Scan=0 pero V4 eleva `orphan_count` | **F1 incompleto** — revisar cobertura de cabeceras `processing/` antes de cualquier emit |

**Laudo:** V4 es gate **obligatorio e inmediato** tras F1 (no diferir a F3). Track 2 solo si la deuda es **real** (huérfanas no cubiertas por historia del bus).

### 3.2 Decisión D2 — Ubicación `persist_ref`

| Opción | Decisión |
|--------|----------|
| `docs/fixes/eda-orphan-debt-precommit` | Rechazada — PBI y handoff fijan `docs/features/` |
| `docs/features/eda-orphan-debt-precommit` | **Elegida** — coherente con backfill-manifest propio |

### 3.3 Decisión D3 — Paralelismo F1 / F2

**Prohibido** iniciar F2 antes de mergear F1 en la rama de trabajo. Excepción: documentar en este archivo con justificación explícita (ninguna al cierre de planificación).

## 4. Gates de validación (referencia F3)

| ID | Gate | Pass |
|----|------|------|
| **V1** | `--scan --json` | `orphan_count: 0` |
| **V2** | Commit prueba `SddIA/` | Sin `BLOCKED` |
| **V3** | `delivery-close-cycle` (lab) | `argos_verdict: pass` |
| **V4** | `event-watcher --once` + `--scan` | `orphan_count` sigue 0 |
| **V5** | `verify-process-integrity.py` | OK |

## 5. Riesgos (matriz PBI)

| Riesgo | Mitigación |
|--------|------------|
| Re-emit duplica eventos | Idempotencia `find_existing_domain_event` en `--emit` |
| Watcher antes de F1 | Protocolo V4 obligatorio |
| Pre-commit sigue bloqueando | No declarar Done sin V2 PASS |
| Manifiesto sin Merkle | `--anchor-merkle` gate en F2 |
| Falsos positivos en processed | Smoke UUID conocido en F1 |

## 6. Handoff operador IA (post-planificación)

1. Rama `fix/eda-orphan-debt-precommit` creada desde `main`.
2. F0.2–F0.4: auditoría en frío + baseline en `.tmp/` + integridad — **sin modificar código**.
3. Tekton F1 → **V4 inmediato** (watcher + scan) → resolución impasse D1 → F2 solo si `orphan_count > 0` (con pausa pre-Merkle).
4. F3–F4 → cierre: `validacion.md` APTO + PBI en `done/` + PR único vía `delivery-close-cycle`.
