---
feature_name: merge-thermo-06-e2e-measure
branch_name: feat/merge-thermo-06-e2e-measure
global: APTO
pbi_archived: true
pbi_document_id: PBI-MERGE-THERMO-06-E2E-MEASURE
persist_ref: docs/features/merge-thermo-06-e2e-measure
---

## Baseline (PBI-01, pre-thermo)

| # | branch | total_ms | invoked_processes | notas |
|---|--------|----------|-------------------|-------|
| 1 | feat/merge-thermo-01-observability | 183125 | route-domain-event, delivery-close-cycle | cadena completa pre-optimización |

## Post (PBIs 02–05 en main)

| # | branch | total_ms | delta_class | attestation_hit | notas |
|---|--------|----------|-------------|---------------|-------|
| 1 | _evolution_gate | 99–727 | — | — | solo gate-evolution |
| 2 | feat/merge-thermo-03-delta-profile | 11 | passive | — | sin Local_QA (delta pasivo) |
| 3 | post-merge accept-pr | 1476–1720 | — | — | sin LLM genómico redundante |

Mediana baseline cadena completa: **183125 ms**. Mediana post rama presentada / gate: **~110 ms** (reducción >90 % vs baseline en camino sin PPR completo).

## Regresión R-4

| Check | Estado |
|-------|--------|
| test-merge-thermo-06-regression.sh | APTO |
| merge-thermo-timings-report | APTO |
| verify-domain-subscription-parity | APTO |
