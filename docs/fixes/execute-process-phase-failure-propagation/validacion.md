---
feature_name: execute-process-phase-failure-propagation
created: "2026-08-11"
updated: "2026-08-13"
process: bug-fix
phase: Verificación
agent: argos
agents: argos
branch: fix/execute-process-phase-failure-propagation
branch_name_injected: fix/execute-process-phase-failure-propagation
persist_ref: docs/fixes/execute-process-phase-failure-propagation
pbi_ref: docs/todos/done/[FIX] execute-process — fallo de fase debe fallar ejecución global (EV-AUD-005).md
document_id: 04f8f435-450b-477a-970a-4a05dd0224cb
correlation_id: dcb9efed-2268-4298-8108-7a55cf4db323
finding: EV-AUD-005
global: APTO
pbi_archived: true
approval_status: aprobado
verdict: aprobado
uuid: 1018806c-0553-44b9-8a63-373be11cb6fc
git_manager_invoked: true
git_evidence_source: sddia-run-tool-git-manager
formal_execute_process: true
evidence_bridge_notes: "Re-auditoría 2026-08-13 Vía A: cargo test stdout físico + git-manager status via ./sddia-run.sh"
checks:
  TECH_FORMAL_EXECUTE_PROCESS: APTO
  GIT_EVIDENCE_VIA_GIT_MANAGER: APTO
  GIT_EVIDENCE_SESSION_SHELL: APTO
  RBAC_AUTHORING_KM_POLICY: APTO
  DOC_OBJECTIVES: APTO
  DOC_SPEC: APTO
  DOC_PLAN: APTO
  DOC_IMPLEMENTATION: APTO
  DOC_EXECUTION: APTO
  DOC_FRONTMATTER_YAML: APTO
  CA1_phase_failed_global_fail: APTO
  CA2_neutrals_not_fail: APTO
  CA3_failsoft_peaje: APTO
  CA3b_failsoft_phase_field: NO_APTO
  CA4_failed_phase_fields: APTO
  CA5_thermo_parity: APTO
  CA6_tests_coverage_vectors: APTO
  CA7_regression_62b201cf_runtime: APTO
  CARGO_TEST_PHASE_TERMINAL: APTO
  SCOPE_WIP_CONTAMINATION: APTO
  PERSIST_REF_DUAL_TREE: APTO
  PBI_ARCHIVED: APTO
  CA8_touchpoints_only: APTO
  CA9_single_persist_ref: APTO
  CA10_test_evidence_cited: APTO
  CA11_pbi_in_done: APTO
  BRANCH_RUNTIME_INJECT: APTO
git_changes:
  - SddIA/engine/execute-process/src/engine/phase_terminal.rs
  - SddIA/engine/execute-process/src/engine/mod.rs
  - SddIA/engine/execute-process/src/engine/residual_runner.rs
  - SddIA/engine/execute-process/src/engine/executor.rs
  - SddIA/engine/execute-process/src/engine/delivery_close.rs
  - SddIA/engine/execute-process/src/engine/capsule_invoke_smoke.rs
  - SddIA/engine/execute-process/src/engine/thermodynamic.rs
  - docs/fixes/execute-process-phase-failure-propagation/
  - docs/todos/done/[FIX] execute-process — fallo de fase debe fallar ejecución global (EV-AUD-005).md
blocking_findings: []
non_blocking_findings:
  - CA3b_failsoft_phase_field
---

# Validación — EV-AUD-005 (Argos · bug-fix · re-auditoría Vía A)

**global: APTO** — evidencia física de `cargo test -p execute-process --lib phase_terminal` (13 passed, incl. `t9_regression_62b201cf_persistencia_oficial`); diff aislado a touchpoints EV-AUD-005; un solo `persist_ref`; PBI en `docs/todos/done/`.

## Aduana Evidence Bridge (R1 / R2 / R3)

| Campo | Valor |
|-------|-------|
| `source` | `./sddia-run.sh --tool git-manager` (esta sesión) |
| `TECH_FORMAL_EXECUTE_PROCESS` | **APTO** |
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | **APTO** — `exitCode:0`, `success:true` |
| `GIT_EVIDENCE_SESSION_SHELL` | **APTO** — stdout físico citado abajo |

Invocación:

```bash
./sddia-run.sh --tool git-manager --inputs '{"operation_type":"status","repository_path":"/home/racso/Proyectos/SddIA","operation_payload_json":{}}'
```

`gitStdout` (post-aislamiento, pre-archivo PBI): solo consumidores `phase_terminal` + `?? phase_terminal.rs` + `?? docs/fixes/execute-process-phase-failure-propagation/`. WIP Kalma2 (`kalma2.rs`, TQM, event-watcher, bridge, `app.js`, `evolution-contract-index-v11`) **ausente**.

## CARGO_TEST_PHASE_TERMINAL / CA7

```bash
cd SddIA && env -u CARGO_TARGET_DIR cargo test -p execute-process --lib phase_terminal
```

```text
running 13 tests
test engine::phase_terminal::tests::fail_soft_declared_does_not_fail_global ... ok
test engine::phase_terminal::tests::t2_skipped_simulated_awaiting_are_not_failure ... ok
test engine::phase_terminal::tests::code_priority_cerbero_before_di_gate ... ok
test engine::phase_terminal::tests::blocked_status_is_global_failure ... ok
test engine::phase_terminal::tests::t1_mix_executed_and_failed_is_global_failure ... ok
test engine::phase_terminal::tests::t3_cerbero_config_error_code_propagated ... ok
test engine::phase_terminal::tests::t2b_argos_block_without_phase_failure ... ok
test engine::phase_terminal::tests::t5_cerbero_rbac_failed_fixture ... ok
test engine::phase_terminal::tests::t6_capsule_invoke_failed_fixture ... ok
test engine::phase_terminal::tests::t4_di_gate_failed_fixture ... ok
test engine::phase_terminal::tests::t7_agent_runtime_failed_fixture ... ok
test engine::phase_terminal::tests::t9_regression_62b201cf_persistencia_oficial ... ok
test engine::phase_terminal::tests::t8_persistencia_capability_failed ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 144 filtered out; finished in 0.00s
```

Cita canónica: `execution.md` § Verificación local (2026-08-13 · retoma Vía A).

## Cascada documental

| Artefacto | Estado |
|-----------|--------|
| `objectives.md` | presente · frontmatter |
| `spec.md` | presente · CA1–CA11; P1–P4 cerrados |
| `plan.md` | presente |
| `implementation.md` | H1–H7 |
| `execution.md` | items_applied + stdout cargo |
| `validacion.md` | este informe (SSOT canónico = esta ruta) |
| `clarify.md` | presente · motivos NO_APTO previo |
| `procedimiento-retoma.md` | presente · Vía A/B |

Slug legado `docs/fixes/execute-processfallodefasedebefallarejecucinglobalev-aud-005/` **eliminado** → `PERSIST_REF_DUAL_TREE: APTO` / CA9.

## Criterios de aceptación

| ID | Check | Estado | Evidencia |
|----|-------|--------|-----------|
| CA1 | Fase `failed` ⇒ global fail | **APTO** | `phase_terminal::aggregate_execution_terminal` + runners |
| CA2 | Neutros no fallan | **APTO** | `t2_*` |
| CA3 | Fail-soft peaje no blanquea | **APTO** | peaje recibe `verdict.success` |
| CA3b | Campo `fail_soft` de fase | **NO_APTO** (no bloqueante) | helper honra `fail_soft:true` sin contrato genómico; P5 / Dedalo |
| CA4 | `failed_phase*` + diagnóstico | **APTO** | `apply_failed_phase_fields` |
| CA5 | Telemetría/orquestación | **APTO** | `thermodynamic.rs` espeja `failed_phase*` si `!success`; early-PEC Kalma2 **no** incluido |
| CA6 | Vectores DI/Cerbero/cápsula/agente/persistencia | **APTO** | `t4`–`t8` ejecutados |
| CA7 | Regresión `62b201cf-…` runtime | **APTO** | `t9_*` ok en stdout citado |
| CA8 | PR sin diffs ajenos | **APTO** | stash Kalma2; sin `kalma2.rs` / TQM / watcher / UI |
| CA9 | Un `persist_ref` | **APTO** | árbol legado podado |
| CA10 | Evidencia física tests | **APTO** | comando + 13 passed |
| CA11 | PBI en `done/` | **APTO** | `pbi_archived: true` |

## Findings bloqueantes

Ninguno.

## Findings no bloqueantes

- `CA3b_failsoft_phase_field: NO_APTO` — forward-compat documentado; no impide APTO de entrega (spec P5).

## correction_blueprint_md

```yaml
name: none
intent: Sin corrección bloqueante. Deuda P5 (fail-soft de fase) y P6 (ola Kalma2) fuera de este PR.
phases: []
```

## Dictamen

```json
{
  "phase": "Verificación",
  "verdict": "aprobado",
  "global": "APTO",
  "pbi_archived": true,
  "correlation_id": "dcb9efed-2268-4298-8108-7a55cf4db323",
  "persist_ref": "docs/fixes/execute-process-phase-failure-propagation",
  "blocking_findings": [],
  "non_blocking_findings": [
    "CA3b_failsoft_phase_field:NO_APTO"
  ],
  "evidence": {
    "CARGO_TEST_PHASE_TERMINAL": "13 passed",
    "GIT_EVIDENCE_SESSION_SHELL": "APTO"
  }
}
```

## Jurisdicción

Cubre fase **Verificación** (`agent:argos`) — re-auditoría Vía A 2026-08-13. PBI archivado en la misma rama (cierre documental pre-merge).
