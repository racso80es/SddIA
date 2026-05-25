---
document_id: PBI-KAIZEN-EDA-AUDIT-NO-BUS-DEPENDENCY
title: "[Kaizen] Validación genómica EDA sin dependencia del bus — correlación durable"
format: markdown
version: "1.0.0"
created: "2026-05-25"
status: pendiente
priority: alta
process: feature
branch_name: feat/eda-audit-durable-correlation
feature_ref_target: docs/features/eda-audit-durable-correlation
upstream:
  - docs/todos/pending/[Kaizen] deuda EDA orphan_count — correlación processed y backfill pre-commit.md
  - docs/features/eda-orphan-debt-precommit/
related:
  - SddIA/scripts/qa/audit-entity-eda-coverage.py
  - SddIA/scripts/qa/eda_bus_utils.py
  - SddIA/scripts/qa/git-hooks/pre_commit_gate.py
  - SddIA/scripts/qa/execute_process_capsules.py
  - SddIA/library/norms/features-documentation-pattern.md
  - docs/features/eda-domain-entities-splus/spec.md
blocks: "Cierre arquitectónico de aduana genómica desacoplada del ciclo de vida efímero del bus EDA"
---

# [Kaizen] Validación genómica EDA sin dependencia del bus — correlación durable

**Estatus:** Pendiente  
**Jurisdicción:** Yunque Operativo · Aduana Argos / pre-commit genómico  
**Precedencia:** fix `eda-orphan-debt-precommit` (workaround F1 + retención cabeceras `Domain_Entity_Created`)

---

## 1. Casuística (por qué existe este PBI)

Durante el cierre de **PBI-KAIZEN-EDA-ORPHAN-DEBT** se confirmó empíricamente:

| Fase | Comportamiento | Consecuencia |
|------|----------------|--------------|
| Baseline con eventos en `pending/` | `--scan` → `orphan_count: 0` | Enmascara deuda real |
| Fix C2 (leer `processed/` + `processing/`) | Correlación ampliada | Insuficiente si el bus purga cabeceras |
| `event-watcher --once` post-backfill | `archive_event_after_sweep` elimina instancias ECST | `--scan` → `orphan_count: 43` |
| Workaround opción 2 | Retener cabecera `processed/` solo para `Domain_Entity_Created` | Restaura V4 en 0, pero **acopla la aduana al bus** |

**Laudo:** la validación genómica (`pre-commit`, `delivery-close-cycle` fase Aduana EDA) **no debe depender** de que eventos ECST sigan presentes en carpetas efímeras del bus (`pending` / `processing` / `processed`). El bus es transporte y orquestación; la correlación entidad ↔ sello debe ser **durable e independiente del barrido del watcher**.

---

## 2. Declaración de propósito

Diseñar e implementar una fuente de verdad de correlación genómica que permita a `audit-entity-eda-coverage.py --scan` (y gates derivados) verificar cobertura EDA **sin** exigir instancias vivas en el bus local.

### Principio rector

```text
Aduana genómica  →  consulta correlación durable (índice / manifiesto / DLT)
Bus EDA          →  orquestación reactiva; NO fuente única de verdad para orphan_count
```

---

## 3. Opciones de diseño (evaluar en clarify)

| Opción | Descripción | Pros | Contras |
|--------|-------------|------|---------|
| **A** | Registro versionado `SddIA/core/eda-entity-correlation-index.json` (o tabla en índices existentes) | Determinista; git-auditable | Mutación explícita en emit/sync |
| **B** | Manifiesto Merkle + acta como prueba de cobertura; scan lee actas ancladas | Ya existe Fase C | Múltiples actas; merge de lotes |
| **C** | Consulta DLT/IOTA como SSOT post-anclaje | Soberanía inmutable | Latencia; lab vs prod |
| **D** | Híbrido: índice local + digest Merkle opcional | Balance operativo / auditoría | Más touchpoints |

**Criterio de elección:** `--scan` y pre-commit deben pasar tras `event-watcher --once` **sin** retener cabeceras en `processed/` por workaround.

---

## 4. Backlog atómico (borrador)

| Hito | Objetivo | Criterio |
|------|----------|----------|
| **H1** | Laudo arquitectónico en `clarify.md` | Opción elegida documentada; retención cabeceras marcada como transitoria |
| **H2** | SSOT correlación durable | Artefacto(s) bajo `SddIA/core/` o norma explícita |
| **H3** | Refactor `audit-entity-eda-coverage.py` | `--scan` usa SSOT; bus opcional como hint, no gate |
| **H4** | Integración emit/sync | `emit-domain-mutation` / `sync-entity-index` actualizan correlación |
| **H5** | Poda workaround | Eliminar retención forzada en `archive_event_after_sweep` tras migración |
| **H6** | V4 sostenido sin bus | watcher + scan → `orphan_count: 0` sin cabeceras retenidas |

---

## 5. Relación con fix en curso

| Artefacto | Rol |
|-----------|-----|
| `eda-orphan-debt-precommit` | Mitigación táctica (C2 + retención + backfill Merkle) |
| **Este PBI** | Solución estructural: aduana no dependiente del bus |

Prohibido declarar cerrada la deuda arquitectónica cuando solo exista el workaround de retención de cabeceras.

---

## 6. Criterios de aceptación (Definition of Done)

| ID | Criterio |
|----|----------|
| KA-CA1 | `--scan` correlaciona vía SSOT durable, no solo `iter_bus_event_files()` |
| KA-CA2 | `pre-commit` PASS tras watcher sin retener cabeceras `processed/` |
| KA-CA3 | `delivery-close-cycle` Aduana EDA sin excepción `_backfill_manifest_active` frágil |
| KA-CA4 | Workaround retención podado o detrás de flag deprecado |
| KA-CA5 | `validacion.md` APTO + PBI en `done/` (un PR) |

---

## 7. Referencias

- Ejecución empírica: `docs/features/eda-orphan-debt-precommit/execution.md` § V4 / impasse
- Norma Ruido de Sistema: `features-documentation-pattern` § Cobertura EDA genómica
- Backfill Fase C: `eda-domain-entities-splus/spec.md` §6
