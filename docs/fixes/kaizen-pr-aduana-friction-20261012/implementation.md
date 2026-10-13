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
correlation_id: "d96571be-c2e1-4a07-96be-efb0d18b81d8"
agent: tekton
phase: Ejecución
execution_id: "c71cb9cb-56cb-48e3-a6b8-69b45dddc156"
---
# Implementation — kaizen-pr-aduana-friction-20261012

Materialización ADU-CA1–ADU-CA14 según `spec.md` / `plan.md` (orden L3→L4→L5→L1→L6→L2). Código ya en rama; esta fase revalida coherencia estática y documenta el estado.

## Commits en rama (evidencia FS `.git/logs` + `.git/refs`)

| Hash | Subject |
|------|---------|
| `344dbf701ceaf8675650f2a2658123fb47df4fc3` | `fix(aduana-pr): Kaizen PBI-KAIZEN-PR-ADUANA-FRICTION-20261012 (ADU-CA1–CA14)` |
| `2ebe43dfbe1e5557de7da88c681a570e08dc825c` | `fix(execute-process): compilar first_nonempty en pec_branch_from_context` |
| `b9d258e92f70fa853e0a73629a5427f63eac4014` | `docs(kaizen-pr-aduana): actualizar handoff tras reintento TQM` (HEAD tip FS) |

Rama: `fix/kaizen-pr-aduana-friction-20261012` (`.git/HEAD` → `refs/heads/fix/kaizen-pr-aduana-friction-20261012`).

## Archivos tocados (código)

| Capa | Archivo | Cambio |
|------|---------|--------|
| L3 | `SddIA/engine/execute-process/src/engine/eda_bus_topology.rs` | `infer_persist_ref_from_branch`: prefijo `docs/{slug}` → features luego fixes; tests CA5/CA6 |
| L4 | `SddIA/engine/execute-process/src/engine/accept_pr.rs` | `scan_presented_for_branch` barre pending/processed/dead-letter |
| L5 | `accept_pr.rs` | `is_remote_ref_absent_error` + `record_hygiene_remote`; `closed_branch` solo local∧remoto; tests CA8/CA9 |
| L1 | `residual_runner.rs` | Barrier Veredicto PPR si Triaje documental blocked; `ppr_verdict_skip_llm`; tests CA1/CA2 |
| L6 | `thermodynamic.rs` | `pec_branch_from_context` / `pec_notice_from_context` / `project_pec_operator_fields` |
| L6 | `route_domain_core.rs` | `build_telegram_message_from_event` contrato CA11–CA14 |
| L2 | `route_domain_core.rs` | `try_acquire_ppr_dedupe` + proof `ppr-presented-dedupe/`; tests CA3/CA4 |

## Revisión estática (ciclo `d96571be…`)

- L1: `ppr_verdict_skip_llm` + handler `ppr-verdict-barrier` antes de `invoke_agent_phase` — presente.
- L2: dedupe pre-dispatch `(branch, head_commit)` fail-open — presente.
- L3: `docs/` → features|fixes — presente.
- L4: scan 3 dirs — presente.
- L5: idempotent absent + `closed_branch` estricto — presente.
- L6: proyección PEC + humanizador — presente.
- Genoma no forjado (ADU-CA3 sin clase ECST nueva).

## Decisiones

- Barrier **solo** en fase `Veredicto y bloqueo` de `pull-request-review` (no redirigir PPR al executor feature/bug-fix).
- Dedupe en despacho; fail-open si SHA irresoluble; rastro durable bajo proofs.
- `notice` no sustituye sellos `hygiene_failure` / `traceability_anomaly`.

## Tests (código presente; ejecución Shell pendiente)

- `adu_ca1_*`, `adu_ca2_*` (residual_runner)
- `adu_ca3_*`, `adu_ca4_*`, `adu_ca11_*`–`adu_ca13_*`, `telegram_message_for_pec_*` (route_domain_core)
- `adu_ca5_*`, `adu_ca6_*` (eda_bus_topology)
- `t_adu_ca7_*`, `t_adu_ca8_*`, `t_adu_ca9_*` (accept_pr)
- `adu_ca13_notice_*` (thermodynamic)
