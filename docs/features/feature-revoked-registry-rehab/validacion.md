---
feature_name: feature-revoked-registry-rehab
created: "2026-08-20"
updated: "2026-08-20T05:46:00Z"
process: refactorization
phase: Verificación
agent: argos
agents: argos
branch: refactor/feature-revoked-registry-rehab
branch_name: refactor/feature-revoked-registry-rehab
persist_ref: docs/features/feature-revoked-registry-rehab
pbi_ref: docs/todos/done/[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #185).md
document_id: PBI-FEATURE-185-REVOKED-REGISTRY
uuid: c8f4e2a1-7b3d-4e59-9f6a-2d1e0c9b8a7f
correlation_id: ""
global: APTO
pbi_archived: true
git_manager_invoked: false
git_manager_error: "cápsula no invocable esta sesión Argos (Shell Rejected sobre ./sddia-run.sh --tool git-manager); sin gitStdout físico; R2 = copia Evidence Bridge prosthesis_subprocess; sin bypass raw"
git_evidence_source: prosthesis_subprocess-evidence-bridge
formal_execute_process: true
handoff_machine_file: present
evidence_bridge_notes: "R1/R2 copia Runtime evidence (machine) @ 2026-08-20T05:46:32Z source=prosthesis_subprocess git_manager_invoked=true formal_execute_process=true formal_evidence_detail=verify-process-integrity: OK git_evidence_digest=7a87e2704163974675fb535571abe1fb; TECH_FORMAL_* / GIT_EVIDENCE_VIA_GIT_MANAGER APTO; Shell git-manager Rejected esta sesión Argos — sin stdout inventado"
shell_git_manager_session: "Rejected — sin gitStdout físico esta invocación Argos Verificación"
checks:
  AC-A1: APTO
  AC-ONTO: APTO
  AC-A2: APTO
  AC-A3: APTO
  AC-THRESH: APTO
  AC-DOC: APTO
  TECH_FORMAL_EXECUTE_PROCESS: APTO
  GIT_EVIDENCE_VIA_GIT_MANAGER: APTO
  GIT_EVIDENCE_SESSION_SHELL: NO_APTO
  RBAC_AUTHORING_KM_POLICY: APTO
  branch: APTO
  git_changes: APTO
git_changes:
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - SddIA/engine/execute-process/src/engine/thermodynamic.rs
  - SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs
  - SddIA/engine/execute-process/src/engine/delivery_close.rs
  - SddIA/evolution/c041bfd2-3be0-4956-83ec-be28fadee390.md
  - SddIA/evolution/Evolution_log.md
  - docs/features/feature-revoked-registry-rehab/
  - docs/todos/done/[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #185).md
---

# Validación — feature-revoked-registry-rehab

## Veredicto

**APTO** — A1 instancia (fuera de diff). A2/A3 motor alineado a `spec.md` (lectura de helpers + tests unitarios en fuente). Umbrales 1.1.0 y `phase_terminal` sin mutación funcional. PBI en `docs/todos/done/`. `cargo test` **sin acuse** esta sesión (Shell Rejected); no se declara verde por cifra.

| AC | Estado | Evidencia |
|----|--------|-----------|
| AC-A1 | APTO | `.SddIA/cerbero/revoked_entities.json`: `permanent: {}`; `feature` ∉ `revoked`. Laterales `bug-fix` / `emit-pr-audited-event` intactos. Stats raíz `feature`: `healthy` · `recovery_attempts: 0` · `consecutive_success_count: 0` · `degraded_at: null` · `samples: []` · `rehab_laudo: PBI-FEATURE-185-REVOKED-REGISTRY` · `rehabilitated_at: 2026-08-20T05:40:37Z`. Paths instancia **no** en `git_changes`. |
| AC-ONTO | APTO | Rehab = borrado de clave; no reescritura `entity_type: tool`. Fósiles `entities.feature` / `process:feature` no usados como cierre. |
| AC-A2 | APTO | `capsule_feature_invoke_delivery_close` → `invoke_process_full`; merge `delivery_push`/`pr_url`/`snapshot_commit_hash`. `feature_dcc_parent_fail_soft`: físico ∧ ¬causal_hard ∧ (telemetry_io / phase.fail_soft / error cola). `Ok`+`fail_soft` vs `Err` causal. Fallback fase `Publicación remota` executed. `delivery_close.rs` copia `delivery_push` a `data`. `residual_runner.rs` / `phase_terminal.rs` sin patch. Test fuente `aggregator_treats_parent_fail_soft_as_success`. |
| AC-A3 | APTO | REF: `cycle_phase` vía `survival_cycle_phase`; `lab_hollow` solo lab-skip de cierre. Batch: `is_survival_hollow` pre-`samples.push` → `skipped: survival_hollow`. Hueco `initialized`/`awaiting_agents`/`lab_hollow`; `failed` no hueco. PEC no castrado (`event_type: Process_Execution_Completed` permanece). |
| AC-THRESH | APTO | `SddIA/agents/radamanto.thresholds.json` ausente de `git_changes`; disco `version 1.1.0` / `process 0.70` / `max_recovery_attempts 3`. |
| AC-DOC | APTO | Cascada `clarify`/`objectives`/`spec`/`plan`/`implementation`/`execution`/`validacion`. PBI `status: done` en `docs/todos/done/`. `pbi_archived: true`. Nota: `execution.md` aún dice pending (stale T3); archivo físico ya en `done/`. |

## Evidence Bridge (R1 / R2)

Copia literal del bloque `### Runtime evidence (machine)` en `_agent_handoff.md`. **Sin stdout inventado.**

| Campo | Valor |
|-------|-------|
| `source` | `prosthesis_subprocess` |
| `git_manager_invoked` | `true` (bridge) / `false` (esta sesión Argos Shell) |
| `formal_execute_process` | `true` |
| `TECH_FORMAL_EXECUTE_PROCESS` | **APTO** |
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | **APTO** |
| `git_evidence_digest` | `7a87e2704163974675fb535571abe1fb` |
| `formal_evidence_detail` | `verify-process-integrity: OK` |
| `GIT_EVIDENCE_SESSION_SHELL` | **NO_APTO** — `./sddia-run.sh --tool git-manager` → Shell Rejected |

`git_changes` / `branch`: path-assert FS (`.git/HEAD` = `refs/heads/refactor/feature-revoked-registry-rehab`) + árbol de entrega en disco. No es `gitStdout`.

## KM (R3) — `RBAC_AUTHORING_KM_POLICY`

**APTO.** Argos esta sesión: 0 writes bajo `docs/todos/**`. Semillas `[FIX] *-watcher — fractura sistémica` = Cúmulo (`Incidente (auto-generado por Cúmulo)`). Archive PBI-185 ya materializado en `done/` (cierre documental; no semilla Kaizen de Argos). Forja Core (`SddIA/actions|skills|process|…`) fuera de este check.

## Fuera / no bloqueante

- T5 `delivery-close-cycle` (PR) no es esta fase.
- `cargo test -p execute-process --lib`: sin acuse (Shell Rejected). Tests existen en fuente; no se finge ejecución.
- Rehab Cerbero `bug-fix` / umbrales / agregador: fuera de alcance; no tocados.
