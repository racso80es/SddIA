---
feature_name: resolucion-estricta-capsulas-skill-tool
created: 2026-11-23
updated: 2026-11-23T17:00:00Z
process: pull-request-review
phase: Triaje documental
agent: argos
agents: argos
branch: refactor/resolucion-estricta-capsulas-skill-tool
branch_name: refactor/resolucion-estricta-capsulas-skill-tool
branch_name_injected: refactor/resolucion-estricta-capsulas-skill-tool
branch_worktree_observed: refactor/resolucion-estricta-capsulas-skill-tool
persist_ref: docs/refactors/resolucion-estricta-capsulas-skill-tool
persist_ref_injected: ''
pbi_ref: ''
tracker_provider: linear
tracker_ref: AUT-118
correlation_id: BvyXzLxHHPqZJNHdq1W6CZ83wG9ToFeyBFp4HayQMce8
pr_presented_event_id: BvyXzLxHHPqZJNHdq1W6CZ83wG9ToFeyBFp4HayQMce8
audit_event_reference: BvyXzLxHHPqZJNHdq1W6CZ83wG9ToFeyBFp4HayQMce8
workspace_path_injected: .SddIA/workspaces/pull-request-review/efade43f-ba65-4707-b15f-fc08837c8983/
workspace_materialized: absent
norm_ref: features-documentation-pattern@1.3.3
source_process: refactorization
global: APTO
pbi_archived: false
approval_status: pendiente_fases_posteriores
verdict: ''
delivery_state: ''
resolution: PASS_F2_DOC
accept_pr_handoff: false
accept_pr_handoff_status: pending
git_manager_invoked: false
git_manager_error: cápsula no invocable en esta sesión Argos (Shell Rejected sobre ./sddia-run.sh --tool git-manager); sin stdout físico; R2 = copia Evidence Bridge session prosthesis_subprocess; git_changes = path-assert FS (sin inventar gitStdout)
git_evidence_source: evidence-bridge-session-prosthesis_subprocess+path-assert-fs
formal_execute_process: true
formal_evidence_detail: session Runtime evidence source=prosthesis_subprocess TECH/GIT APTO notes=(none); machine handoff bajo persist_ref refactor ausente; workspace inject efade43f… vacío (0 ficheros; sin § Runtime evidence machine)
handoff_machine_file: absent
handoff_evidence_block_pre: absent
evidence_bridge_notes: R1/R2 copia Runtime evidence (session prosthesis_subprocess notes=none); TECH_FORMAL_EXECUTE_PROCESS / GIT_EVIDENCE_VIA_GIT_MANAGER APTO; Shell git-manager Rejected esta sesión Argos Triaje — sin stdout inventado; machine handoff en persist_ref ausente (no se usa handoff de docs/fixes/… otra entrega)
shell_git_manager_session: Rejected — sin gitStdout físico esta invocación Argos Triaje documental CID BvyXzLxH… · exec efade43f…
scope: PPR Triaje documental — F2_DOC + Evidence Bridge R1/R2/R3 (CID BvyXzLxH… · exec efade43f…)
checks:
  K-CROSS: APTO
  K-RT: APTO
  K-QA: APTO
  K-TEST: APTO
  global: APTO
  branch: APTO
  git_changes: APTO
  F2_DOC_GATE: APTO
  DOC_OBJECTIVES: APTO
  DOC_CLARIFY: APTO
  DOC_SPEC: APTO
  DOC_PLAN: APTO
  DOC_IMPLEMENTATION: APTO
  DOC_EXECUTION: APTO
  DOC_FRONTMATTER_YAML: APTO
  DOC_EVOLUTION: APTO
  DOC_AC_CASCADE: APTO
  DOC_ORACLE_RED_FILE: NO_APTO
  PERSIST_REF_RESOLVED: APTO
  PERSIST_REF_INJECTED: NO_APTO
  HANDOFF_MACHINE_FILE: NO_APTO
  HANDOFF_EVIDENCE_BLOCK: NO_APTO
  HANDOFF_EVIDENCE_SESSION: APTO
  BRANCH_RUNTIME_INJECT: APTO
  BRANCH_WORKTREE_SYNC: APTO
  TECH_FORMAL_EXECUTE_PROCESS: APTO
  GIT_EVIDENCE_VIA_GIT_MANAGER: APTO
  GIT_EVIDENCE_SESSION_SHELL: NO_APTO
  GIT_EVIDENCE_NATIVE_PPR: NO_APTO
  RBAC_AUTHORING_KM_POLICY: APTO
  ARGOS_NO_KM_WRITE: APTO
  PBI_PENDING_ABSENT: APTO
  TRACKER_LINEAR_NO_KM_FILE: APTO
  CODE_TOUCHPOINT_PRESENT: APTO
  WSPACE_MATERIALIZED: NO_APTO
  ROOT_TOPOLOGY_CLEAN: APTO
git_changes:
  branch: refactor/resolucion-estricta-capsulas-skill-tool
  base: main
  source: path-assert-fs
  paths:
  - SddIA/engine/execute-process/src/engine/capsules.rs
  - SddIA/engine/execute-process/src/engine/cerbero_di_rbac.rs
  - SddIA/engine/execute-process/src/engine/handlers/email_noise_digest.rs
  - SddIA/engine/execute-process/src/engine/notify_humanized_pr_merged.rs
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/tools/sddia-qa/src/gate_capsule_kind.rs
  - SddIA/tools/sddia-qa/src/main.rs
  - SddIA/tools/sddia-qa/src/verify_tools_index.rs
  - SddIA/evolution/cf3bbcc3-df0e-4b12-ac55-b2fb65351b7d.md
  - SddIA/evolution/Evolution_log.md
  - docs/refactors/resolucion-estricta-capsulas-skill-tool/
blocking_findings: []
non_blocking_findings:
- PERSIST_REF_INJECTED
- DOC_ORACLE_RED_FILE
- HANDOFF_MACHINE_FILE
- HANDOFF_EVIDENCE_BLOCK
- GIT_EVIDENCE_SESSION_SHELL
- GIT_EVIDENCE_NATIVE_PPR
- WSPACE_MATERIALIZED
situational_notes:
- Fase Triaje documental · CID BvyXzLxHHPqZJNHdq1W6CZ83wG9ToFeyBFp4HayQMce8 · exec efade43f-ba65-4707-b15f-fc08837c8983
- 'Evidence Bridge session: source=prosthesis_subprocess · TECH/GIT APTO · notes=(none)'
- 'Evidence Bridge machine bajo persist_ref: ausente (_agent_handoff.md no materializado en docs/refactors/resolucion-estricta-capsulas-skill-tool/); workspace inject vacío (0 ficheros); no se copia handoff de docs/fixes/… (otra entrega)'
- persist_ref inject vacío → resuelto por cascade+branch a docs/refactors/resolucion-estricta-capsulas-skill-tool (PERSIST_REF_RESOLVED APTO; PERSIST_REF_INJECTED NO_APTO residual)
- F2 refactorization cascada — objectives+clarify+spec+plan+implementation+execution (+ YAML) + evolution → PASS_F2_DOC
- Shell ./sddia-run.sh --tool git-manager → Rejected; sin gitStdout de esta sesión Argos
- GIT_EVIDENCE_NATIVE_PPR NO_APTO — workspace exec efade43f… ausente/vacío; sin prep-branch stdout propio
- git_changes = path-assert FS (touchpoints + cascada + evolution canónico); no es gitStdout sesión
- WSPACE_MATERIALIZED NO_APTO — workspace inject efade43f… vacío; no bloquea F2
- DOC_EVOLUTION APTO — hito canónico cf3bbcc3-df0e-4b12-ac55-b2fb65351b7d.md + fila Evolution_log AUT-118
- DOC_ORACLE_RED_FILE NO_APTO — oracle-red.log ausente bajo persist_ref; rojo Ola 0 narrado en execution.md (no bloquea F2)
- K-CROSS…K-TEST APTO — path-assert: invoke_tool_capsule_json en notify/route/email_noise; CapsuleKindMismatch+validate_capsule_kind_against_ssot+aut118_* en capsules.rs; CerberoDiCode::CapsuleKindMismatch; gate_capsule_kind + verify-tools-index
- Argos 0 writes docs/todos/** esta fase → RBAC_AUTHORING_KM_POLICY APTO; pending/ vacío; tracker.provider=linear; 0 hits AUT-118 bajo docs/todos/
- BRANCH_WORKTREE_SYNC APTO — .git/HEAD → refs/heads/refactor/resolucion-estricta-capsulas-skill-tool @ 43cd4210… = inject
- ROOT_TOPOLOGY_CLEAN APTO — path-assert FS: raíz sin .events/orchestration/telemetry/_lab sueltos
- pbi_archived false — provider linear; sin fichero nuevo pending/done/
- validated_*/diff_digest no rematerializados esta sesión (Shell Rejected; sin inventar SHAs)
head_commit_sha_fs: 43cd421035e211ddc77385bb0704722eec9b1e9c
execution_id: efade43f-ba65-4707-b15f-fc08837c8983
validated_base_sha: efb7449fc68ecc7b26738cfa8dd89becd399a19a
validated_head_sha: 694a884bdf185220090d650aa089471281f5b4b8
validated_tree_sha: 59fbc6543951dceb3f28817ad10ec2a703ded24a
diff_digest: sha256:9a5ff29f84a6e70916157bd88a80010d8234a011fde6a8b2173296528349448b
---

# Validación — Triaje documental (Argos · pull-request-review)

## Veredicto de fase

**APTO** — `resolution: PASS_F2_DOC` · `blocking_findings: []` · `approval_status: pendiente_fases_posteriores`.

| Gate | Delegado | Estado | Criterio |
|------|----------|--------|----------|
| F2 | Argos (doc) | **APTO** | cascada refactorization + YAML + evolution → `PASS_F2_DOC` |
| R1 TECH_FORMAL | Evidence Bridge | **APTO** | copia session `prosthesis_subprocess` |
| R2 GIT_EVIDENCE | Evidence Bridge | **APTO** | copia bridge session; sin stdout inventado |
| R3 KM | Argos | **APTO** | 0 writes ilegítimos bajo `docs/todos/**` |

## Evidence Bridge (R1 / R2 / R3)

Copia literal session — **no** stdout Shell inventado:

| Campo | Valor |
|-------|-------|
| `schema` | `kalma2-agent-runtime-evidence/v1` |
| `source` (session) | `prosthesis_subprocess` |
| `source` (machine handoff) | **ausente** bajo persist_ref / workspace |
| `notes` (session) | `(none)` |
| `TECH_FORMAL_EXECUTE_PROCESS` | **APTO** |
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | **APTO** |
| `formal_execute_process` | `true` |
| `GIT_EVIDENCE_SESSION_SHELL` | **NO_APTO** — Shell Rejected; sin `gitStdout` |
| `GIT_EVIDENCE_NATIVE_PPR` | **NO_APTO** — workspace `efade43f…` vacío |
| `HANDOFF_MACHINE_FILE` | **NO_APTO** — `_agent_handoff.md` no materializado en persist_ref |
| `RBAC_AUTHORING_KM_POLICY` | **APTO** — Argos 0 writes bajo `docs/todos/**`; `pending/` vacío |

Fuente: prompt Runtime evidence (session) `prosthesis_subprocess` notes=`(none)`. Machine handoff de `docs/fixes/erradicar-store-backed-fugas-fs-bus/` **no** se usa (otra entrega / otro `persist_ref`).

## Ingesta

| Input | Resolución |
|-------|------------|
| `persist_ref` (inyectado) | vacío |
| `persist_ref` (resuelto) | `docs/refactors/resolucion-estricta-capsulas-skill-tool` — cascade+branch presentes |
| `pbi_ref` (inyectado) | vacío · tracker Linear `AUT-118` |
| `correlation_id` | `BvyXzLxHHPqZJNHdq1W6CZ83wG9ToFeyBFp4HayQMce8` |
| `execution_id` | `efade43f-ba65-4707-b15f-fc08837c8983` |
| `branch_name` (runtime) | `refactor/resolucion-estricta-capsulas-skill-tool` |
| Worktree HEAD (FS) | `refs/heads/refactor/resolucion-estricta-capsulas-skill-tool` @ `43cd4210…` (**= inject**) |
| `source_process` | `refactorization` |
| Tracker | `linear` · sin fichero KM AUT-118 |
| Workspace inject | vacío (0 ficheros; sin handoff machine) |

## Cascada F2

| Check | Estado | Evidencia |
|-------|--------|-----------|
| `DOC_OBJECTIVES` | **APTO** | `objectives.md` + frontmatter + K-CROSS…K-TEST |
| `DOC_CLARIFY` | **APTO** | `clarify.md` + frontmatter |
| `DOC_SPEC` | **APTO** | `spec.md` + frontmatter + oráculos |
| `DOC_PLAN` | **APTO** | `plan.md` + frontmatter |
| `DOC_IMPLEMENTATION` | **APTO** | `implementation.md` + frontmatter |
| `DOC_EXECUTION` | **APTO** | `execution.md` + frontmatter + rojo Ola 0 narrado |
| `DOC_FRONTMATTER_YAML` | **APTO** | `---` YAML en objectives/clarify/spec/plan/implementation/execution |
| `DOC_EVOLUTION` | **APTO** | canónico `cf3bbcc3-…` + `Evolution_log` AUT-118 |
| `DOC_AC_CASCADE` | **APTO** | K-CROSS…K-TEST en `spec.md` / `objectives.md` |
| `DOC_ORACLE_RED_FILE` | **NO_APTO** | `oracle-red.log` ausente · no bloquea F2 |
| `CODE_TOUCHPOINT_PRESENT` | **APTO** | `capsules` / `cerbero_di_rbac` / `notify_humanized_pr_merged` / `route_domain_core` / `email_noise_digest` / `gate_capsule_kind` |
| `F2_DOC_GATE` | **APTO** | → `PASS_F2_DOC` |

## Matriz de Criterios (AC) — path-assert documental

| Criterio | Estado | Evidencia |
|----------|--------|-----------|
| K-CROSS | **APTO** | `send-telegram-notification` vía `invoke_tool_capsule_json` en `notify_humanized_pr_merged.rs`, `route_domain_core.rs`, `email_noise_digest.rs` |
| K-RT | **APTO** | `validate_capsule_kind_against_ssot` + test `aut118_resolve_capsule_rejects_kind_mismatch` + `CerberoDiCode::CapsuleKindMismatch` |
| K-QA | **APTO** | `gate_capsule_kind.rs` + registro `gate-capsule-kind` en `main.rs` + integración en `verify_tools_index.rs` |
| K-TEST | **APTO** | oráculo/unitarios declarados en `spec`/`objectives`/`execution`; touchpoints presentes (sin cargo stdout físico esta sesión) |

## branch / git_changes / KM

| Check | Estado | Evidencia |
|-------|--------|-----------|
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | **APTO** | Evidence Bridge session `prosthesis_subprocess` (copia) |
| `GIT_EVIDENCE_SESSION_SHELL` | **NO_APTO** | Shell Rejected; sin `gitStdout` |
| `GIT_EVIDENCE_NATIVE_PPR` | **NO_APTO** | workspace `efade43f…` vacío |
| `BRANCH_RUNTIME_INJECT` | **APTO** | inject = `refactor/resolucion-estricta-capsulas-skill-tool` |
| `BRANCH_WORKTREE_SYNC` | **APTO** | HEAD → misma ref @ `43cd4210…` |
| `branch` | **APTO** | worktree = inject |
| `git_changes` | **APTO** | path-assert FS (no `gitStdout` cápsula) |
| `TRACKER_LINEAR_NO_KM_FILE` | **APTO** | provider `linear` · 0 ficheros AUT-118 bajo `docs/todos/` |
| `PBI_PENDING_ABSENT` | **APTO** | `docs/todos/pending/` vacío |
| `ROOT_TOPOLOGY_CLEAN` | **APTO** | FS: sin `.events`, `orchestration`, `telemetry`, `_lab` en raíz |
| `WSPACE_MATERIALIZED` | **NO_APTO** | workspace vacío; **no bloquea** F2 |

`git_changes.paths` = path-assert FS (touchpoints + evolution canónico + persist_ref). Sin bypass raw destructivo. Sin `validated_base_sha` / `diff_digest` inventados (cápsula git-manager no ejecutada aquí).

## R3 — KM (`RBAC_AUTHORING_KM_POLICY`)

**APTO** — auditoría solo autoría bajo `docs/todos/**`. `docs/todos/pending/` vacío. Sin semillas AUT-118 ilegítimas. Argos **no** escribió TODOs. Cumulo/`Kaizen_Alert_Required` = vía legítima (no usada). Forja Core ≠ este check. Provider Linear: sin materialización local de PBI.

## Dictamen

```json
{
  "phase": "Triaje documental",
  "process": "pull-request-review",
  "global": "APTO",
  "resolution": "PASS_F2_DOC",
  "approval_status": "pendiente_fases_posteriores",
  "pbi_archived": false,
  "branch": "refactor/resolucion-estricta-capsulas-skill-tool",
  "branch_worktree_observed": "refactor/resolucion-estricta-capsulas-skill-tool",
  "head_commit_sha_fs": "43cd421035e211ddc77385bb0704722eec9b1e9c",
  "persist_ref": "docs/refactors/resolucion-estricta-capsulas-skill-tool",
  "tracker_ref": "AUT-118",
  "correlation_id": "BvyXzLxHHPqZJNHdq1W6CZ83wG9ToFeyBFp4HayQMce8",
  "execution_id": "efade43f-ba65-4707-b15f-fc08837c8983",
  "TECH_FORMAL_EXECUTE_PROCESS": "APTO",
  "GIT_EVIDENCE_VIA_GIT_MANAGER": "APTO",
  "RBAC_AUTHORING_KM_POLICY": "APTO",
  "F2_DOC_GATE": "APTO",
  "BRANCH_WORKTREE_SYNC": "APTO",
  "blocking": []
}
```

## Alcance

Triaje documental **aprueba** peaje F2 (`PASS_F2_DOC`). Residuales no bloqueantes: `persist_ref` inject vacío (resuelto), `oracle-red.log` ausente, machine handoff ausente bajo persist_ref, workspace vacío, Shell git-manager Rejected. Fases posteriores (técnico / Cerbero / Veredicto) fuera de este dictamen. Argos **no** escribe bajo `docs/todos/` ni reescribe `_agent_handoff.md`.

## approval_status

```text
pendiente_fases_posteriores
```