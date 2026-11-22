---
feature_name: man-48-familia-capability-contract
created: 2026-11-22
updated: 2026-11-22T13:36:00Z
process: pull-request-review
phase: Triaje documental
agent: argos
agents: argos
branch: feat/man-48-familia-capability-contract
branch_name: feat/man-48-familia-capability-contract
branch_name_injected: feat/man-48-familia-capability-contract
branch_worktree_observed: feat/man-48-familia-capability-contract
persist_ref: docs/features/man-48-familia-capability-contract
pbi_ref: MAN-48
document_id: PBI-FAMILIA-CAPABILITY-CONTRACT-POLIMORFISMO
tracker_provider: linear
tracker_ref: MAN-48
correlation_id: c15bcaa8-d393-4eee-a92c-42438016491c
pr_presented_event_id: c15bcaa8-d393-4eee-a92c-42438016491c
audit_event_reference: c15bcaa8-d393-4eee-a92c-42438016491c
workspace_path_injected: .SddIA/workspaces/pull-request-review/a5fba947-f8c0-406c-b61f-9118bd175466/
workspace_materialized: absent
norm_ref: features-documentation-pattern@1.3.1
global: APTO
pbi_archived: false
approval_status: pendiente_fases_downstream
verdict: pendiente
delivery_state: pending
resolution: PASS_F2_DOC
accept_pr_handoff: false
accept_pr_handoff_status: pending
accept_pr_block_reason: L-HANDOFF-F5 — fuera de jurisdicción Triaje documental
git_manager_invoked: false
git_manager_error: cápsula no invocable en esta sesión Argos (Shell Rejected sobre ./sddia-run.sh --tool git-manager); sin stdout físico; R2 = copia Evidence Bridge native_state; git_changes = path-assert FS (sin inventar gitStdout)
git_evidence_source: evidence-bridge-session-native_state+handoff-machine+path-assert-fs
formal_execute_process: true
formal_evidence_detail: 'session Runtime evidence source=native_state TECH/GIT APTO notes=handoff-formal-scan; idempotent-hit; machine handoff native_state @ 2026-11-22T12:35:58Z TECH/GIT APTO notes=handoff-formal-scan; idempotent-hit; prior prosthesis_subprocess @ 2026-11-22T11:55:06Z formal_evidence_detail=verify-process-integrity: OK'
handoff_machine_file: present
handoff_evidence_block_pre: present
evidence_bridge_notes: R1/R2 copia Runtime evidence (session native_state notes=handoff-formal-scan; idempotent-hit + machine native_state @ 2026-11-22T12:35:58Z); TECH_FORMAL_EXECUTE_PROCESS / GIT_EVIDENCE_VIA_GIT_MANAGER APTO; Shell git-manager Rejected esta sesión Argos Triaje — sin stdout inventado
shell_git_manager_session: Rejected — sin gitStdout físico esta invocación Argos Triaje documental CID c15bcaa8… · exec a5fba947…
scope: PPR Triaje documental — F2 + Evidence Bridge R1/R2/R3 (CID c15bcaa8… · exec a5fba947…)
checks:
  CA-1: APTO
  CA-2: APTO
  CA-3: APTO
  CA-4: APTO
  CA-5: APTO
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
  PERSIST_REF_RESOLVED: APTO
  HANDOFF_MACHINE_FILE: APTO
  HANDOFF_EVIDENCE_BLOCK: APTO
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
git_changes:
  branch: feat/man-48-familia-capability-contract
  base: main
  source: path-assert-fs
  paths:
  - SddIA/core/cumulo.paths.json
  - SddIA/engine/execute-process/src/engine/eda_bus_topology.rs
  - SddIA/engine/execute-process/src/engine/eda_coverage.rs
  - SddIA/engine/execute-process/src/engine/entity_manager.rs
  - SddIA/engine/execute-process/src/engine/sync_entity_index.rs
  - SddIA/engine/execute-process/src/forges/capability_contract.rs
  - SddIA/evolution/7966d58f-b59b-4f56-8957-8ab35abdc919.md
  - SddIA/evolution/Evolution_log.md
  - SddIA/library/norms/capability-contracts/capability-contracts-contract.md
  - SddIA/library/norms/capability-contracts/index.md
  - SddIA/norms/entidades-dominio-ecosistema-sddia.md
  - SddIA/tools/sddia-qa/Cargo.toml
  - SddIA/tools/sddia-qa/src/main.rs
  - SddIA/tools/sddia-qa/src/verify_capability_contracts.rs
  - docs/features/man-48-familia-capability-contract/
blocking_findings: []
non_blocking_findings:
- GIT_EVIDENCE_SESSION_SHELL
- GIT_EVIDENCE_NATIVE_PPR
- WSPACE_MATERIALIZED
situational_notes:
- Fase Triaje documental · CID c15bcaa8-d393-4eee-a92c-42438016491c · exec a5fba947-f8c0-406c-b61f-9118bd175466
- 'Evidence Bridge session: source=native_state · TECH/GIT APTO · notes=handoff-formal-scan; idempotent-hit'
- 'Evidence Bridge machine: source=native_state @ 2026-11-22T12:35:58Z · TECH/GIT APTO · notes=handoff-formal-scan; idempotent-hit (copia; sin stdout inventado)'
- 'Prior prosthesis_subprocess @ 2026-11-22T11:55:06Z · formal_evidence_detail=verify-process-integrity: OK'
- F2_DOC_GATE APTO — cascada objectives→execution + YAML frontmatter + CA-1…CA-5 + DOC_EVOLUTION
- Shell ./sddia-run.sh --tool git-manager → Rejected; sin gitStdout de esta sesión Argos
- GIT_EVIDENCE_NATIVE_PPR NO_APTO — workspace exec a5fba947… ausente/vacío; sin gitStdout prep-branch propio este exec (sibling 4e1e9ab1… tiene status docs-only; no usado como stdout inventado de esta sesión)
- git_changes = path-assert FS (touchpoints + cascada + evolution); no es gitStdout sesión
- WSPACE_MATERIALIZED NO_APTO — workspace inject a5fba947… ausente/vacío en FS (no bloquea F2)
- DOC_EVOLUTION APTO — hito SddIA/evolution/7966d58f-b59b-4f56-8957-8ab35abdc919.md + fila Evolution_log MAN-48
- CA-1…CA-5 APTO — path-assert touchpoints (contrato/índice/cumulo, norma, forja/motor, QA) + execution.md T1–T4 Forjada
- Argos 0 writes docs/todos/** esta fase → RBAC_AUTHORING_KM_POLICY APTO; pending/ vacío; tracker.provider=linear
- Downstream: F3 técnico / F4 Cerbero / Veredicto — fuera de esta fase
execution_id: a5fba947-f8c0-406c-b61f-9118bd175466
validated_base_sha: 66197e98a050c95dbbec5ad1910f8ce95e2a5702
validated_head_sha: 414c7e613f02ef74315b714679f823b9bfca28a0
validated_tree_sha: fb89afe941068eb8025fb012735d6808a2c7b3fe
diff_digest: sha256:d0f5fe00b44b18db219d1ca7305c64239e86529d4a3483c56da1bdf16b798166
---

# Validación — Triaje documental (Argos · pull-request-review)

## Veredicto de fase

**APTO** — `resolution: PASS_F2_DOC` · `global: APTO` · `blocking_findings: []`.

| Gate | Delegado | Estado | Criterio |
|------|----------|--------|----------|
| F2 | Argos (doc) | **APTO** | cascada + YAML + CA-1…CA-5 + evolution |
| R1 TECH_FORMAL | Evidence Bridge | **APTO** | copia session/machine `native_state` |
| R2 GIT_EVIDENCE | Evidence Bridge | **APTO** | copia bridge; sin stdout inventado |
| R3 KM | Argos | **APTO** | 0 writes ilegítimos bajo `docs/todos/**` |

## Evidence Bridge (R1 / R2 / R3)

Copia literal machine/session — **no** stdout inventado:

| Campo | Valor |
|-------|-------|
| `schema` | `kalma2-agent-runtime-evidence/v1` |
| `source` (session) | `native_state` |
| `source` (machine handoff) | `native_state` @ `2026-11-22T12:35:58Z` |
| `notes` (session/machine) | `handoff-formal-scan; idempotent-hit` |
| `TECH_FORMAL_EXECUTE_PROCESS` | **APTO** |
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | **APTO** |
| `formal_execute_process` | `true` |
| `formal_evidence_detail` (prior prosthesis) | `verify-process-integrity: OK` @ `2026-11-22T11:55:06Z` |
| `GIT_EVIDENCE_SESSION_SHELL` | **NO_APTO** — `./sddia-run.sh --tool git-manager` → Shell Rejected |
| `GIT_EVIDENCE_NATIVE_PPR` | **NO_APTO** — workspace `a5fba947…` vacío; sin prep-branch propio este exec |
| `RBAC_AUTHORING_KM_POLICY` | **APTO** — Argos 0 writes bajo `docs/todos/**`; `pending/` vacío |

Fuente: prompt Runtime evidence (session) + `docs/features/man-48-familia-capability-contract/_agent_handoff.md` § Runtime evidence (machine) @ `2026-11-22T12:35:58Z`.

## Ingesta

| Input | Resolución |
|-------|------------|
| `persist_ref` | `docs/features/man-48-familia-capability-contract` — presente |
| `pbi_ref` (inyectado) | vacío → resuelto `MAN-48` / `document_id: PBI-FAMILIA-CAPABILITY-CONTRACT-POLIMORFISMO` |
| `correlation_id` | `c15bcaa8-d393-4eee-a92c-42438016491c` |
| `branch_name` (runtime) | `feat/man-48-familia-capability-contract` |
| Worktree (FS HEAD) | `refs/heads/feat/man-48-familia-capability-contract` |
| `workspace_path` | `.SddIA/workspaces/pull-request-review/a5fba947…/` — **ausente/vacío** |
| `tracker.provider` | `linear` → sin fichero nuevo en `docs/todos/pending\|done/` |
| Norma | `features-documentation-pattern@1.3.1` |

## Cascada documental (F2)

| Check | Estado | Evidencia |
|-------|--------|-----------|
| `DOC_OBJECTIVES` | **APTO** | `objectives.md` + YAML + CA-1…CA-5 |
| `DOC_CLARIFY` | **APTO** | `clarify.md` + YAML |
| `DOC_SPEC` | **APTO** | `spec.md` + YAML |
| `DOC_PLAN` | **APTO** | `plan.md` T1–T4 + YAML |
| `DOC_IMPLEMENTATION` | **APTO** | `implementation.md` + YAML |
| `DOC_EXECUTION` | **APTO** | `execution.md` T1–T4 **Forjada** + YAML |
| `DOC_FRONTMATTER_YAML` | **APTO** | feature_name / branch_name / persist_ref / pbi_ref / document_id coherentes |
| `DOC_AC_CASCADE` | **APTO** | CA ↔ plan ↔ execution alineados |
| `DOC_EVOLUTION` | **APTO** | hito `7966d58f-…` + `Evolution_log` MAN-48 |
| `CA-1`…`CA-5` | **APTO** | path-assert touchpoints + execution T1–T4 Forjada |
| `F2_DOC_GATE` | **APTO** | → `PASS_F2_DOC` |

## branch / git_changes

| Check | Estado | Evidencia |
|-------|--------|-----------|
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | **APTO** | Evidence Bridge `native_state` (copia) |
| `GIT_EVIDENCE_SESSION_SHELL` | **NO_APTO** | Shell Rejected; sin `gitStdout` sesión Argos |
| `GIT_EVIDENCE_NATIVE_PPR` | **NO_APTO** | workspace `a5fba947…` vacío este exec |
| `BRANCH_RUNTIME_INJECT` | **APTO** | `feat/man-48-familia-capability-contract` |
| `BRANCH_WORKTREE_SYNC` | **APTO** | `.git/HEAD` ≡ inject (FS; **no** stdout git-manager) |
| `branch` | **APTO** | inject ≡ HEAD |
| `git_changes` | **APTO** | inventario path-assert FS; **no** es `gitStdout` de esta sesión |
| `WSPACE_MATERIALIZED` | **NO_APTO** | workspace `a5fba947…` ausente; **no bloquea** |

`git_changes.paths` = path-assert FS (touchpoints + cascada + evolution). Sin bypass raw destructivo.

## R3 — KM (`RBAC_AUTHORING_KM_POLICY`)

**APTO** — auditoría solo autoría bajo `docs/todos/**`. `docs/todos/pending/` vacío (0 files). Sin semillas MAN-48 ilegítimas. Argos **no** escribió TODOs. Cumulo/`Kaizen_Alert_Required` = vía legítima (no usada esta fase). Forja Core ≠ este check. Provider Linear: sin materialización local de PBI.

## Dictamen

```json
{
  "phase": "Triaje documental",
  "process": "pull-request-review",
  "global": "APTO",
  "resolution": "PASS_F2_DOC",
  "branch": "feat/man-48-familia-capability-contract",
  "branch_worktree_observed": "feat/man-48-familia-capability-contract",
  "persist_ref": "docs/features/man-48-familia-capability-contract",
  "document_id": "PBI-FAMILIA-CAPABILITY-CONTRACT-POLIMORFISMO",
  "correlation_id": "c15bcaa8-d393-4eee-a92c-42438016491c",
  "execution_id": "a5fba947-f8c0-406c-b61f-9118bd175466",
  "TECH_FORMAL_EXECUTE_PROCESS": "APTO",
  "GIT_EVIDENCE_VIA_GIT_MANAGER": "APTO",
  "RBAC_AUTHORING_KM_POLICY": "APTO",
  "F2_DOC_GATE": "APTO",
  "DOC_EVOLUTION": "APTO",
  "blocking": []
}
```

## Alcance

Triaje documental **aprueba** peaje F2 (`PASS_F2_DOC`). Residuales no bloqueantes: `GIT_EVIDENCE_SESSION_SHELL`, `GIT_EVIDENCE_NATIVE_PPR`, `WSPACE_MATERIALIZED`. Downstream: F3 técnico / F4 Cerbero / Veredicto — fuera de esta fase. Argos **no** escribe bajo `docs/todos/` ni reescribe `_agent_handoff.md`.

## approval_status

```text
pendiente_fases_downstream
```