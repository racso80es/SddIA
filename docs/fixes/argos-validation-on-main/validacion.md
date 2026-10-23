---
feature_name: argos-validation-on-main
created: "2026-10-23"
updated: "2026-10-23T08:36:00Z"
process: bug-fix
phase: Verificación
agent: argos
agents: argos
branch: fix/argos-validation-on-main
branch_name: fix/argos-validation-on-main
branch_name_injected: ""
persist_ref: docs/fixes/argos-validation-on-main
persist_ref_injected: ""
persist_ref_resolution: "inyección vacía → objectives/spec frontmatter docs/fixes/argos-validation-on-main"
pbi_ref: docs/todos/pending/[FIX] Argos sella validacion en main y bloquea el init.md
document_id: PBI-FIX-ARGOS-VALIDATION-ON-MAIN
uuid: "f126631d-3d98-43c4-b251-911410e06001"
execution_id: "a0509a75-3e2f-4a4d-ae64-18379411a5cb"
execution_id_injected: ""
correlation_id: ""
global: NO_APTO
pbi_archived: false
approval_status: rechazado
verdict: rechazado
git_manager_invoked: false
git_manager_error: "Shell IDE Rejected sobre ./sddia-run.sh --tool git-manager; sin stdout físico"
git_evidence_source: none
formal_execute_process: false
shell_session: "Rejected — echo/cargo/git-manager; Task subagent 9f72ca61 mismo bloqueo"
argos_integrity:
  shell: blocked
  head_ref: refs/heads/fix/argos-validation-on-main
  handoff: "process/phase/? vacíos; persist_ref/branch/execution_id/pbi_ref/correlation_id vacíos"
  reaudit: "2026-10-23T08:36+02:00"
  note: "contexto inferido desde docs/fixes/argos-validation-on-main/*.md + .git/HEAD; L-ARGOS-SYNTHESIS"
checks:
  DOC_OBJECTIVES: APTO
  DOC_SPEC: APTO
  DOC_IMPLEMENTATION: APTO
  DOC_EXECUTION: APTO
  DOC_FRONTMATTER_YAML: APTO
  BRANCH_WORKTREE_SYNC: APTO
  HANDOFF_RUNTIME_INJECT: NO_APTO
  FX_CA1_code_guard: APTO
  FX_CA1_runtime_test: NO_EVIDENCIA
  FX_CA2_code_checkout: APTO
  FX_CA2_runtime_test: NO_EVIDENCIA
  FX_CA3_main_clean_observation: NO_EVIDENCIA
  FX_CA4_dirty_worktree_untouched: APTO
  FX_CA5_t20_out_of_scope: APTO
  CARGO_TEST_VERIFICATION_HANDOFF: NO_EVIDENCIA
  GIT_EVIDENCE_VIA_GIT_MANAGER: NO_APTO
  TECH_FORMAL_EXECUTE_PROCESS: NO_APTO
  RBAC_AUTHORING_KM_POLICY: APTO
  PBI_PENDING_PRESENT: APTO
  PBI_ARCHIVED: NO_APTO
  SCOPE_WIP_TRACKER_VALIDACION: NO_APTO
  IDE_RELAY_BYPASS_RESIDUAL: NO_APTO
git_changes:
  - SddIA/engine/execute-process/src/engine/agent_runtime.rs
  - docs/fixes/argos-validation-on-main/
  - docs/features/tracker-lifecycle-sink/validacion.md
blocking_findings:
  - HANDOFF_RUNTIME_INJECT
  - CARGO_TEST_VERIFICATION_HANDOFF
  - GIT_EVIDENCE_VIA_GIT_MANAGER
  - IDE_RELAY_BYPASS_RESIDUAL
non_blocking_findings:
  - SCOPE_WIP_TRACKER_VALIDACION
  - FX_CA3_main_clean_observation
---

# Validación — argos-validation-on-main (agent:argos · bug-fix · Verificación)

**global: NO_APTO** — sin stdout de cápsulas (`cargo test` / `git-manager`). Shell IDE `Rejected` (padre + Task `9f72ca61`). No se inventa APTO de runtime (L-ARGOS-SYNTHESIS).

## Handoff

| Campo | Inyectado | Resuelto |
|-------|-----------|----------|
| process / phase | `?` / `?` | `bug-fix` / `Verificación` (artefactos persist_ref) |
| persist_ref | vacío | `docs/fixes/argos-validation-on-main` |
| branch_name | vacío | `fix/argos-validation-on-main` (`.git/HEAD`) |
| execution_id | vacío | `a0509a75-3e2f-4a4d-ae64-18379411a5cb` (frontmatter objectives) |
| pbi_ref | vacío | `docs/todos/pending/[FIX] Argos sella validacion en main y bloquea el init.md` |
| correlation_id | vacío | ausente |

`HANDOFF_RUNTIME_INJECT: NO_APTO`. Síntoma F1 repetido: Kalma2/IDE relevó Argos con metadatos vacíos.

## Auditoría estática (código) — reconfirmada 08:36

| CA | Evidencia Read/Grep | Check |
|----|---------------------|-------|
| FX-CA1 | `invoke_agent_phase` L772–783: `Verificación` + `branch`/`execution_id` ausentes → `failed` + `verification-handoff-incomplete`; return antes de spawn | `FX_CA1_code_guard: APTO` |
| FX-CA2 | `ensure_verification_branch` L126–154 → `branch_list`; ausente → `checkout` `create_if_not_exists: false`; error L785–789 → `verification-branch-checkout` | `FX_CA2_code_checkout: APTO` |
| FX-CA4 | `workspace_init.rs` L549 mensaje `dirty-worktree: cambios fuera de persist_ref/pbi_ref:` intacto | `FX_CA4_dirty_worktree_untouched: APTO` |
| FX-CA5 | sin mutación de T20 / `SddIA_AP` / manifiesto Paciente 0 | `FX_CA5_t20_out_of_scope: APTO` |

Tests en fuente: `verification_handoff_without_branch_does_not_spawn` (L1190), `verification_handoff_without_execution_id_does_not_spawn` (L1217), `verification_missing_local_ref_fails_before_spawn_when_checkout_unavailable` (L1244), `branch_name_coalesces_from_pr_branch`, `runtime_evidence_forwards_native_state_flags`. **No ejecutados** → `CARGO_TEST_VERIFICATION_HANDOFF: NO_EVIDENCIA`.

## Cápsulas (bloqueadas)

```bash
./sddia-run.sh --tool git-manager --inputs '{"operation_type":"status",...}'
cd SddIA && cargo test -p execute-process --lib -- verification_handoff verification_missing runtime_evidence_forwards branch_name_coalesces --test-threads=1
```

Resultado re-auditoría: `Rejected:` (sin exitCode, sin stdout). Task best-of-n-runner: mismo bloqueo.

## Hallazgos bloqueantes

1. **Cápsulas sin evidencia** — FX-CA1/CA2 runtime y FX-CA3 observación de `main` limpio no sellables.
2. **IDE relay residual** — el guard Rust corta spawn nativo; el relevo IDE igual inyectó Argos con handoff vacío. F1 del PBI no cubre solo el camino lab-relay.
3. **WIP ajeno** — `docs/features/tracker-lifecycle-sink/validacion.md` modificado en worktree (snapshot). Argos **no** lo reescribe (persist_ref del fix ≠ ese path).

## Corrección (blueprint)

| Fase | intent | delegates_to |
|------|--------|--------------|
| Evidencia | Ejecutar suite `verification_handoff*` y status vía git-manager con Shell/aduana habilitada | `action:execute-process` |
| Relé | Alinear handoff IDE (branch + execution_id) o fallar igual que `verification-handoff-incomplete` antes de prompt Argos | `agent:dedalo` |
| Cierre | Con tests verdes: `global: APTO`, archivar PBI en `done/` en la misma rama | `agent:argos` → `action:execute-process` (`delivery-close-cycle`) |

## PBI

Sigue en `docs/todos/pending/`. `pbi_archived: false`. Argos no escribe bajo `docs/todos/`.
