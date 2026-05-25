---
document_id: PBI-KAIZEN-EDA-ORPHAN-DEBT
title: "[Kaizen] Deuda EDA orphan_count — correlación processed y backfill pre-commit"
format: markdown
version: "1.0.0"
created: "2026-05-25"
status: pendiente
priority: alta
process: bug-fix
branch_name: fix/eda-orphan-debt-precommit
feature_ref_target: docs/features/eda-orphan-debt-precommit
related:
  - SddIA/scripts/qa/audit-entity-eda-coverage.py
  - SddIA/scripts/qa/eda_bus_utils.py
  - SddIA/scripts/qa/git-hooks/pre_commit_gate.py
  - SddIA/process/delivery-close-cycle.md
  - SddIA/library/norms/features-documentation-pattern.md
  - docs/features/eda-domain-entities-splus/backfill-manifest.json
  - docs/features/vanguardia-soberania-local/execution.md
  - docs/todos/done/[Kaizen] higiene ficheros temporales — .tmp fuera de control y limpieza post-uso.md
  - docs/todos/pending/[Kaizen] validación genómica EDA sin dependencia del bus — correlación durable.md "pre-commit BLOCKED Argos orphan_count=43; delivery-close aduana genómica inconsistente tras event-watcher"
blocks: "Commits locales con hook pre-commit activo; delivery-close-cycle con mutaciones SddIA/"
---

# [Kaizen] Deuda EDA orphan_count — correlación processed y backfill pre-commit

## 0. Mandato

Iniciar como **`bug-fix`** bajo `docs/features/eda-orphan-debt-precommit/`.

**Rama:** `fix/eda-orphan-debt-precommit`

**Done (documental v1.2.0):** un único PR mergeado en `main` + `validacion.md` APTO (`pbi_archived: true`) + este PBI en `docs/todos/done/` en la misma rama.

| ID | Objetivo | Criterio de cierre |
|----|----------|-------------------|
| **O1** | **Correlación EDA estable** | `find_existing_domain_event` considera `pending` ∪ `processed` ∪ `processing` ∪ legacy |
| **O2** | **orphan_count = 0 sostenido** | `--scan` en 0 tras backfill **y** tras `event-watcher --once` |
| **O3** | **Pre-commit desbloqueado** | Hook pasa en commit de prueba que toque `SddIA/` sin `SDDIA_SKIP_HOOKS` |
| **O4** | **Fase C cerrada** | Manifiesto + acta Merkle con `transaction_digest` en el PR |
| **O5** | **Prevención forward** | Norma/documentación: no repetir protocolo frágil «solo pending» |

---

## 1. Incidente

| Campo | Valor |
|-------|--------|
| **Síntoma** | `SddIA pre-commit: BLOCKED — Argos orphan_count=43` en cualquier commit con hook activo |
| **Alcance** | 46 entidades indexadas; 43 huérfanas en 8 clases (`skill`, `event`, `process`, `agent`, `tool`, `action`, `norm`, `codex`) |
| **Contexto** | Tras Kaizen higiene ficheros temporales (PR #44); deuda **preexistente** al backfill Fase C de `eda-domain-entities-splus` |
| **Regresión C2** | `audit-entity-eda-coverage --scan` solo correlaciona eventos en `pending/`; tras watcher, eventos en `processed/` no cuentan → huérfanas reaparecen |
| **Evidencia** | `vanguardia-soberania-local/execution.md` § Backfill EDA — nota sobre desacople pending↔scan |

### Causas raíz

| # | Causa | Tipo |
|---|--------|------|
| **C1** | Forja histórica Core sin `entity-manager` / sin `Domain_Entity_Created` | Deuda histórica |
| **C2** | `iter_bus_event_files()` no incluye `processed/` ni cabeceras `processing/` | Bug de correlación |
| **C3** | Backfill ejecutado en lab sin anclaje durable o watcher ejecutado antes de commit estable | Operativa |

---

## 2. Laudo de diseño

```text
Correlación válida = evento Domain_Entity_Created cuyo payload.entity_uuid
                     existe en pending ∪ processed ∪ processing ∪ legacy

Backfill Fase C     = --emit --skip-dlt --correlation-id <lote>
                     + --anchor-merkle (obligatorio cierre)
                     + orphan_count_after: 0 en manifiesto versionado

Orden Tekton        = Fix correlación (Track 1) ANTES de backfill (Track 2)
                     salvo aceptar protocolo frágil «mantener pending hasta commit»

Post-cierre         = event-watcher --once + --scan → orphan_count permanece 0
```

---

## 3. Fases de aplicación (todas obligatorias)

### Fase 0 — Baseline e inventario

| Paso | Acción | Evidencia |
|------|--------|-----------|
| 0.1 | Crear `objectives.md`, `clarify.md`, `spec.md`, `plan.md` en `persist_ref` | Frontmatter válido |
| 0.2 | `--scan --json` → `.tmp/eda-orphan-baseline.json` | `orphan_count: 43` snapshot |
| 0.3 | `--dry-run` | Lista UUIDs / clases |
| 0.4 | `verify-process-integrity.py` | OK pre-cambio |

**Criterio F0:** baseline JSON archivado en `execution.md` (ruta `.tmp/`, no commitear).

---

### Fase 1 — Track 1: Fix correlación audit (estructural)

| Paso | Touchpoint | Entregable |
|------|------------|------------|
| 1.1 | `SddIA/scripts/qa/eda_bus_utils.py` | `iter_bus_event_files()` ampliado: `processed/*.json`, `processing/*.json` |
| 1.2 | `find_existing_domain_event()` | Sin cambio de contrato; hereda búsqueda ampliada |
| 1.3 | Smoke / regresión | Evento mock o existente en `processed/` reduce `orphan_count` |
| 1.4 | Documentación | `spec.md` § invariante correlación; nota en `features-documentation-pattern` § Ruido de Sistema |

**Criterio F1:** re-scan tras 1.1; documentar delta (`orphan_count` antes/después). Si delta > 0, continuar Fase 2.

---

### Fase 2 — Track 2: Backfill Fase C consolidado

Ejecutar **solo si** Fase 1 no deja `orphan_count: 0`.

```powershell
python SddIA/scripts/qa/audit-entity-eda-coverage.py `
  --emit --skip-dlt --json `
  --correlation-id eda-backfill-precommit-20260525

$env:SDDIA_LAB_SIMULATE_IOTA = "1"
python SddIA/scripts/qa/audit-entity-eda-coverage.py `
  --anchor-merkle docs/features/eda-orphan-debt-precommit/backfill-manifest.json
```

| Paso | Entregable |
|------|------------|
| 2.1 | `backfill-manifest.json` en `persist_ref` (`emit_ok`, `orphan_count_after: 0`) |
| 2.2 | `merkle-acta-eda-backfill-precommit-20260525.json` (o nombre derivado de `correlation_id`) |
| 2.3 | `transaction_digest` registrado en manifiesto y `validacion.md` |

**Criterio F2:** `--scan --json` → `orphan_count: 0`; manifiesto versionado en el PR.

---

### Fase 3 — Validación de gates (Aduana)

| ID | Gate | Comando | Pass |
|----|------|---------|------|
| **V1** | Scan limpio | `audit-entity-eda-coverage.py --scan --json` | `orphan_count: 0` |
| **V2** | Pre-commit | Commit de prueba tocando `SddIA/norms/...` | Sin BLOCKED |
| **V3** | Delivery-close | `execute-process --process delivery-close-cycle` (lab) | `argos_verdict: pass`, `orphan_count: 0` |
| **V4** | Regresión watcher | `event-watcher --once` + `--scan` | `orphan_count` sigue 0 |
| **V5** | Integridad genoma | `verify-process-integrity.py` | OK |

**Criterio F3:** los cinco gates PASS documentados en `validacion.md` (`checks`).

---

### Fase 4 — Prevención forward (Kaizen, no bloqueante del merge)

| Paso | Acción | Prioridad |
|------|--------|-----------|
| 4.1 | Reforzar en `git-operations.md` o norma EDA: forja productiva solo vía `entity-manager` | Alta |
| 4.2 | Runbook en `execution.md`: orden Track 1 → Track 2; prohibido watcher entre emit y scan pre-commit | Alta |
| 4.3 | Opcional: pre-commit falla solo si **diff** introduce fila `index.md` sin evento correlato | Media (futuro) |
| 4.4 | Remediar puntual `markdown-table-editor` si sigue huérfana tras lote | Baja |

**Criterio F4:** `implementation.md` lista touchpoints preventivos; no exige código adicional si F1–F3 cierran la deuda.

---

## 4. Secuencia Tekton (orden estricto)

```text
F0 baseline → F1 fix eda_bus_utils → re-scan (delta)
           → F2 backfill + merkle (si orphan_count > 0)
           → F3 gates V1–V5
           → F4 prevención doc
           → validacion.md APTO + PBI → done/
           → delivery-close-cycle → PR único → accept-pr
```

**Paralelismo prohibido:** F2 no arranca antes de F1 mergeado en la rama (salvo excepción documentada en `clarify.md`).

---

## 5. Matriz de riesgos

| Riesgo | Impacto | Mitigación |
|--------|---------|------------|
| Re-emit duplica eventos | Ruido en bus | Idempotencia `find_existing_domain_event` en `--emit` |
| Watcher antes de F1 | orphan_count vuelve a 43 | Protocolo F3 V4 obligatorio |
| Pre-commit sigue bloqueando | No merge | No declarar Done sin V2 PASS |
| Manifiesto sin Merkle | Fase C incompleta | `--anchor-merkle` gate en F2 |
| Falsos positivos processed | Correlación incorrecta | Smoke con UUID conocido en F1 |

---

## 6. Artefactos esperados en el PR

| Path | Fase |
|------|------|
| `docs/features/eda-orphan-debt-precommit/objectives.md` | F0 |
| `docs/features/eda-orphan-debt-precommit/clarify.md` | F0 |
| `docs/features/eda-orphan-debt-precommit/spec.md` | F0–F1 |
| `docs/features/eda-orphan-debt-precommit/plan.md` | F0 |
| `docs/features/eda-orphan-debt-precommit/implementation.md` | F1–F4 |
| `docs/features/eda-orphan-debt-precommit/execution.md` | F2–F3 |
| `docs/features/eda-orphan-debt-precommit/validacion.md` | F3 |
| `docs/features/eda-orphan-debt-precommit/backfill-manifest.json` | F2 (si aplica) |
| `docs/features/eda-orphan-debt-precommit/merkle-acta-*.json` | F2 (si aplica) |
| `SddIA/scripts/qa/eda_bus_utils.py` | F1 |
| `docs/todos/done/[Kaizen] deuda EDA orphan_count — …` | Cierre |

---

## 7. Handoff operador IA

1. Crear rama `fix/eda-orphan-debt-precommit` desde `main`.
2. Ejecutar fases F0 → F4 en orden; inputs efímeros solo en `.tmp/` (Kaizen PR #44).
3. Completar `validacion.md` con `global: APTO`, `pbi_archived: true`, `branch` coherente.
4. Mover este PBI a `docs/todos/done/` **en la misma rama** antes del merge.
5. Cierre entrega vía `delivery-close-cycle`; fusión vía `accept-pr` — **prohibido** push documental suelto a `main`.

---

## 8. Referencias

- Plan de análisis: conversación Kaizen 2026-05-25 (orphan_count / pre-commit).
- Backfill canónico: `docs/features/eda-domain-entities-splus/spec.md` §6.
- Protocolo lab: `docs/features/vanguardia-soberania-local/execution.md` § Backfill EDA Fase C.
- Cierre documental: `.cursor/rules/task-closure-documental.mdc`.
