---
feature_name: kaizen-pr-aduana-friction-20261012
created: "2026-10-13"
process: bug-fix
branch_name: fix/kaizen-pr-aduana-friction-20261012
persist_ref: docs/fixes/kaizen-pr-aduana-friction-20261012
pbi_ref: docs/todos/pending/[KAIZEN] Aduana PR — corte LLM, dedupe Presented, persist_ref docs y sellos post-merge.md
document_id: PBI-KAIZEN-PR-ADUANA-FRICTION-20261012
uuid: "c4f91a2e-8b7d-4e3a-9c1f-6d0a2b5e8f47"
execution_id: "37dc1867-9940-499b-99a7-1e4d2d933a0d"
correlation_id: "834d80af-cd3f-49b5-958a-c4a7d62f6975"
agent: tekton
phase: Ejecución
---

# Implementation — kaizen-pr-aduana-friction-20261012

Materialización ADU-CA1–ADU-CA14 según `spec.md` / `plan.md` (orden L3→L4→L5→L1→L6→L2).

## Archivos tocados

| Capa | Archivo | Cambio |
|------|---------|--------|
| L3 | `SddIA/engine/execute-process/src/engine/eda_bus_topology.rs` | `infer_persist_ref_from_branch`: prefijo `docs/{slug}` → features luego fixes; tests CA5/CA6 |
| L4 | `SddIA/engine/execute-process/src/engine/accept_pr.rs` | `scan_presented_for_branch` barre pending/processed/dead-letter |
| L5 | `accept_pr.rs` | `is_remote_ref_absent_error` + `record_hygiene_remote`; `closed_branch` solo local∧remoto; tests CA8/CA9 |
| L1 | `residual_runner.rs` | Barrier Veredicto PPR si Triaje documental blocked; flag `documental_triage_blocked`; `phase_reports` acumulados en state; tests CA1/CA2 |
| L6 | `thermodynamic.rs` | Proyecta `branch` / `pr_url` / `notice` al PEC |
| L6 | `route_domain_core.rs` | `build_telegram_message_from_event` contrato CA11–CA14 |
| L2 | `route_domain_core.rs` | Dedupe pre-dispatch PPR por `(rama, commit)` + proof `ppr-presented-dedupe/`; tests CA3/CA4 |

## Decisiones

- Barrier **solo** en fase `Veredicto y bloqueo` de `pull-request-review` (no redirigir PPR al executor feature/bug-fix).
- Dedupe en despacho (no abortar PPR ya arrancado); fail-open si SHA irresoluble; rastro durable bajo proofs (sin clase ECST nueva).
- `notice` no sustituye sellos `hygiene_failure` / `traceability_anomaly`.
- Genoma no forjado.

## Tests añadidos / actualizados

- `adu_ca5_*`, `adu_ca6_*` (eda_bus_topology)
- `t_adu_ca7_*`, `t_adu_ca8_*`, `t_adu_ca9_*`, `t_a2_local_ok_closed_branch` alineado (accept_pr)
- `adu_ca1_*`, `adu_ca2_*` (residual_runner)
- `adu_ca11_*`–`adu_ca13_*`, `adu_ca3_*`, `adu_ca4_*`, `telegram_message_for_pec_includes_correlation` (route_domain_core)
- `adu_ca13_notice_*` (thermodynamic)
