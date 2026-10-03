---
feature_name: merge-thermo-01-observability
created: "2026-10-03"
process: feature
branch: feat/merge-thermo-01-observability
pbi_document_id: PBI-MERGE-THERMO-01-OBSERVABILITY
global: APTO
pbi_archived: true
---

# Validación — PBI-MERGE-THERMO-01-OBSERVABILITY

## AC-11 — Motor y purga

| Check | Estado | Evidencia |
|-------|--------|-----------|
| Tests `execution_workspace_report` | OK | `cargo test -p execute-process execution_workspace_report` (4 tests) |
| Tests `workspace_prune` + hook JSONL | OK | `cargo test -p sddia-qa workspace_prune` (2 tests) |
| `workspace-prune --empty` idempotente | OK | 1ª pasada `pruned_count: 2105`; 2ª `pruned_count: 0` (2026-10-03, `cargo run -p sddia-qa`) |

## AC-OBS-1 — Hook timings

Cada invocación de `pre-push` / `post-merge` appendea en `.SddIA/proofs/hook-timings/hook-timings.jsonl` (`hook`, `branch`, `total_ms`, `invoked_processes[]`, `delta_class: null`, `attestation_hit: null`).

## AC-7a — Baseline pre-push (rama ya presentada)

Escenario: `pre_push_hook_runs_evolution_gate` (sin ramas nuevas en stdin). Proxy medido: `gate-evolution --range --if-touched --sync-base` (dominante en ese camino). Host: laboratorio local, 2026-10-03.

| # | Escenario | total_ms |
|---|-----------|--------:|
| 1 | pre-push / evolution_gate | 973 |
| 2 | pre-push / evolution_gate | 1288 |
| 3 | pre-push / evolution_gate | 814 |
| 4 | pre-push / evolution_gate | 912 |
| 5 | pre-push / evolution_gate | 787 |
| 6 | pre-push / evolution_gate | 1026 |
| 7 | pre-push / evolution_gate | 875 |
| 8 | pre-push / evolution_gate | 722 |
| 9 | pre-push / evolution_gate | 802 |
| 10 | pre-push / evolution_gate | 918 |

Mediana: **875 ms**. Referencia para AC-7 del PBI 06.

## Veredicto

**APTO** — observabilidad instalada; baseline capturado antes de PBI 02–04.
