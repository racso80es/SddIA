---
feature_name: pbi-stale-pending-purge-ppr-124
created: "2026-07-22"
process: bug-fix
branch_name: fix/pbi-stale-pending-purge-ppr-124
persist_ref: docs/fixes/pbi-stale-pending-purge-ppr-124
pbi_ref: docs/todos/done/[OPERATIVO] PBI stale pending — purga copia duplicada Kalma2-feature (PPR #124).md
document_id: PBI-PPR-124-PBI-STALE-PENDING
uuid: c642aa29-4980-46ed-bf24-c5b7c3cde913
---

# Spec — Purga PBI stale pending (PPR #124)

## Problema

Tras merge de PR #124, coexistían:

| Path | status |
|------|--------|
| `docs/todos/done/[Kaizen] ciclo Kalma2-feature — …` | `done` (canónico) |
| `docs/todos/pending/[Kaizen] ciclo Kalma2-feature — …` | `abierto` (stale) |

Check `PBI_PENDING_STALE_COPY: NO_APTO` en aduana PPR.

## Solución

1. Confirmar canónico en `done/` + `pbi_archived: true` en feature `kaizen-kalma2-feature-cycle-observability`.
2. Eliminar archivo stale de `pending/`.
3. Archivar este PBI OPERATIVO en `docs/todos/done/`.

## Fuera de alcance

Reabrir Kaizen; mutar genoma; merge histórico PR #124.
