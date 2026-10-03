---
feature_name: merge-thermo-01-observability
created: "2026-10-03"
process: feature
pbi_document_id: PBI-MERGE-THERMO-01-OBSERVABILITY
---

# Especificación — observabilidad de aduana (PBI 01)

## Alcance

- `elapsed_ms` por fase en `executor` y `residual_runner`.
- `execution_report.json` en workspace (best-effort; exentos termodinámicos omitidos).
- `sddia-qa workspace-prune --empty --json`.
- JSONL `{proofs}/hook-timings/hook-timings.jsonl` desde `pre-push` y `post-merge`.
- Baseline ≥10 muestras (rama ya presentada → rama `gate-evolution`).

## Fuera de alcance

Triaje por delta, atestación, poda de red (PBI 02–04).
