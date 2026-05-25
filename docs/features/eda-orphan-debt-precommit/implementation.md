---
feature_name: eda-orphan-debt-precommit
created: "2026-05-25"
process: bug-fix
branch_name: fix/eda-orphan-debt-precommit
items:
  - id: F1-eda-bus-utils-iter
    status: applied
    path: SddIA/scripts/qa/eda_bus_utils.py
  - id: F1-archive-retention
    status: applied
    path: SddIA/scripts/qa/eda_bus_utils.py
    note: "Workaround transitorio — poda en PBI-KAIZEN-EDA-AUDIT-NO-BUS-DEPENDENCY"
  - id: F1-norm-pattern
    status: applied
    path: SddIA/library/norms/features-documentation-pattern.md
  - id: F1-tests
    status: applied
    path: SddIA/scripts/qa/test_eda_bus_v3plus.py
  - id: F2-backfill-merkle
    status: applied
    path: docs/features/eda-orphan-debt-precommit/backfill-manifest.json
  - id: kaizen-follow-up
    status: materialized
    path: docs/todos/pending/[Kaizen] validación genómica EDA sin dependencia del bus — correlación durable.md
---

# Implementación — Deuda EDA orphan_count

## F1 — Correlación multi-estado (aplicado)

`iter_bus_event_files()` agrega cabeceras ECST en `pending/`, `processing/`, `processed/` y legacy.

## F1 — Retención cabecera `Domain_Entity_Created` (workaround transitorio)

`archive_event_after_sweep()`:

- Conserva cabecera en `processed/` cuando `event_type == Domain_Entity_Created`.
- Copia desde `pending/` si la cabecera aún no existe antes del barrido.
- Purga testigos y cabecera `processing/` con normalidad.

**Deuda derivada:** la aduana no debe depender del bus a largo plazo → PBI `PBI-KAIZEN-EDA-AUDIT-NO-BUS-DEPENDENCY`.

## F2 — Backfill Fase C (aplicado)

| Artefacto | Valor |
|-----------|--------|
| `backfill-manifest.json` | `correlation_id: eda-backfill-precommit-20260525`, `emit_ok: 43`, `orphan_count_after: 0` |
| `merkle-acta-eda-backfill-precommit-20260525.json` | `transaction_digest: lab-simulated-29eb0307245943d8` |
| `merkle_root` | `sha256:29eb0307245943d85e943fc1ce5619e678750438d316e35327c6fbc02d30f878` |

## Kaizen follow-up (materializado)

PBI en `docs/todos/pending/[Kaizen] validación genómica EDA sin dependencia del bus — correlación durable.md` — casuística V4 y solución estructural sin acoplamiento al bus.

## F4 — Prevención forward (pendiente)

- Runbook en `execution.md`
- Evaluar nota en `git-operations.md`
