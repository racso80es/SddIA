---

feature_name: kaizen-pr-aduana-friction-20261012
created: "2026-10-13"
updated: "2026-10-13"
process: bug-fix
branch_name: fix/kaizen-pr-aduana-friction-20261012
persist_ref: docs/fixes/kaizen-pr-aduana-friction-20261012
pbi_ref: docs/todos/pending/[KAIZEN] Aduana PR — corte LLM, dedupe Presented, persist_ref docs y sellos post-merge.md
document_id: PBI-KAIZEN-PR-ADUANA-FRICTION-20261012
uuid: "c4f91a2e-8b7d-4e3a-9c1f-6d0a2b5e8f47"
correlation_id: "77f88be2-3509-416f-a426-8e03df9591cb"
agent: tekton
phase: Ejecución
execution_id: "13753665-38ae-4ac7-88c8-48264a7cefde"
---
# Implementation — kaizen-pr-aduana-friction-20261012

Materialización ADU-CA1–ADU-CA14 según `spec.md` / `plan.md` (orden L3→L4→L5→L1→L6→L2).

## Commits en rama (evidencia FS `.git/logs`)

| Hash | Subject |
|------|---------|
| `344dbf701ceaf8675650f2a2658123fb47df4fc3` | `fix(aduana-pr): Kaizen PBI-KAIZEN-PR-ADUANA-FRICTION-20261012 (ADU-CA1–CA14)` |
| `2ebe43dfbe1e5557de7da88c681a570e08dc825c` | `fix(execute-process): compilar first_nonempty en pec_branch_from_context` (HEAD) |

Rama: `fix/kaizen-pr-aduana-friction-20261012` (`.git/HEAD`).

## Archivos tocados

| Capa | Archivo | Cambio |
|------|---------|--------|
| L3 | `SddIA/engine/execute-process/src/engine/eda_bus_topology.rs` | `infer_persist_ref_from_branch`: prefijo `docs/{slug}` → features luego fixes; tests CA5/CA6 |
| L4 | `SddIA/engine/execute-process/src/engine/accept_pr.rs` | `scan_presented_for_branch` barre pending/processed/dead-letter |
| L5 | `accept_pr.rs` | `is_remote_ref_absent_error` + `record_hygiene_remote`; `closed_branch` solo local∧remoto; tests CA8/CA9 |
| L1 | `residual_runner.rs` | Barrier Veredicto PPR si Triaje documental blocked; `ppr_verdict_skip_llm`; tests CA1/CA2 |
| L6 | `thermodynamic.rs` | Proyecta `branch` / `pr_url` / `notice` al PEC |
| L6 | `route_domain_core.rs` | `build_telegram_message_from_event` contrato CA11–CA14 |
| L2 | `route_domain_core.rs` | Dedupe pre-dispatch PPR por `(rama, commit)` + proof `ppr-presented-dedupe/`; tests CA3/CA4 |

## Decisiones

- Barrier **solo** en fase `Veredicto y bloqueo` de `pull-request-review` (no redirigir PPR al executor feature/bug-fix).
- Dedupe en despacho (no abortar PPR ya arrancado); fail-open si SHA irresoluble; rastro durable bajo proofs (sin clase ECST nueva).
- `notice` no sustituye sellos `hygiene_failure` / `traceability_anomaly`.
- Genoma no forjado.

## Tests añadidos / actualizados (código presente; ejecución pendiente)

- `adu_ca5_*`, `adu_ca6_*` (eda_bus_topology)
- `t_adu_ca7_*`, `t_adu_ca8_*`, `t_adu_ca9_*`, `t_a2_local_ok_closed_branch` alineado (accept_pr)
- `adu_ca1_*`, `adu_ca2_*` (residual_runner)
- `adu_ca11_*`–`adu_ca13_*`, `adu_ca3_*`, `adu_ca4_*`, `telegram_message_for_pec_includes_correlation` (route_domain_core)
- `adu_ca13_notice_*` (thermodynamic)
