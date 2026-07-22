---
feature_name: ppr-rehab-revoked-entities
created: "2026-07-22"
process: bug-fix
branch_name: fix/ppr-rehab-revoked-entities
persist_ref: docs/fixes/ppr-rehab-revoked-entities
pbi_ref: docs/todos/done/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #124).md
document_ids:
  - PBI-PPR-124-REVOKED-REGISTRY
  - PBI-PPR-125-REVOKED-REGISTRY
uuid: 23a81b0e-3930-4589-b5db-25ddd8eb5717
---

# Spec — Rehabilitación `pull-request-review` en revoked_entities

## Problema

`RBAC_PROCESS_REGISTRY: NO_APTO` en aduanas PR #124 y #125: `pull-request-review` figuraba en `.SddIA/cerbero/revoked_entities.json` (`latency_threshold`, since 2026-07-21) pese a peaje F2–F4 APTO.

Causa: Radamanto degrada por `avg_duration > latency_ms_p95_threshold` (30s). PPR incluye fases `agent:` (Kalma2/IDE) con wall-clock de minutos → falso positivo.

Nota: `revoked_entities.json` y `.SddIA/radamanto/` están en `.gitignore` (estado de instancia).

## Laudo

**Rehabilitación** (no permanente).

1. Retirar clave `pull-request-review` de `revoked` en instancia.
2. Stats Radamanto → `healthy`; podar outlier ≥300s.
3. Exención versionada: `LATENCY_THRESHOLD_EXEMPT = ["pull-request-review"]` en `radamanto_batch_core.rs`.

## Fuera de alcance

- Reescritura genoma `pull-request-review.md`.
- Rehabilitar `feature` / `delivery-close-cycle` / `emit-pr-audited-event` (revocaciones distintas; residual).
