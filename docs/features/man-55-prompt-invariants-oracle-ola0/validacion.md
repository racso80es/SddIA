---
feature_name: man-55-prompt-invariants-oracle-ola0
created: 2026-11-22
updated: 2026-11-22T19:21:30Z
process: pull-request-review
phase: Veredicto y bloqueo
agent: argos
agents: argos
branch: feat/man-55-prompt-invariants-oracle-ola0
branch_name: feat/man-55-prompt-invariants-oracle-ola0
branch_name_injected: feat/man-55-prompt-invariants-oracle-ola0
branch_worktree_observed: feat/man-55-prompt-invariants-oracle-ola0
persist_ref: docs/features/man-55-prompt-invariants-oracle-ola0
pbi_ref: MAN-55
document_id: PBI-344ba8be
tracker_provider: linear
tracker_ref: MAN-55
correlation_id: c3b86d7a-1441-4a45-a269-9e281fd225a4
pr_presented_event_id: c3b86d7a-1441-4a45-a269-9e281fd225a4
audit_event_reference: c3b86d7a-1441-4a45-a269-9e281fd225a4
workspace_path_injected: /home/racso/Proyectos/SddIA/.SddIA/workspaces/pull-request-review/3c7ca0f9-c295-4b6b-bd37-b1a477fd6f94/
workspace_materialized: absent
norm_ref: features-documentation-pattern@1.3.3
global: APTO
pbi_archived: false
approval_status: aprobado
verdict: aprobado
delivery_state: success
resolution: PASS_F5_VERDICT
accept_pr_handoff: false
accept_pr_handoff_status: pending
accept_pr_block_reason: pr_url/MERGE ausentes este CID; L-HANDOFF-F5 — prohibido accept_pr_handoff true sin vía de merge soberano
git_manager_invoked: false
git_manager_error: cápsula no invocable en esta sesión Argos (Shell Rejected sobre ./sddia-run.sh --tool git-manager); sin stdout físico; R2 = copia Evidence Bridge native_state; git_changes = path-assert FS (sin inventar gitStdout)
git_evidence_source: evidence-bridge-session-native_state+handoff-machine+path-assert-fs
formal_execute_process: true
formal_evidence_detail: 'session Runtime evidence source=native_state TECH/GIT APTO notes=idempotent-hit; machine handoff native_state @ 2026-11-22T19:20:17Z TECH/GIT APTO notes=idempotent-hit; formal_evidence_detail heredado verify-process-integrity: OK (prosthesis_subprocess @ 2026-11-22T19:18:04Z)'
handoff_machine_file: present
handoff_evidence_block_pre: present
evidence_bridge_notes: R1/R2 copia Runtime evidence (session native_state notes=idempotent-hit + machine native_state @ 2026-11-22T19:20:17Z); TECH_FORMAL_EXECUTE_PROCESS / GIT_EVIDENCE_VIA_GIT_MANAGER APTO; Shell git-manager Rejected esta sesión Argos Veredicto — sin stdout inventado
shell_git_manager_session: Rejected — sin gitStdout físico esta invocación Argos Veredicto y bloqueo CID c3b86d7a… · exec 3c7ca0f9…
scope: PPR Veredicto y bloqueo — F5 + Evidence Bridge R1/R2/R3 + gates F2/F3/F4 (CID c3b86d7a… · exec 3c7ca0f9…)
checks:
  AC-1: APTO
  AC-2: APTO
  AC-3: APTO
  AC-4: APTO
  global: APTO
  branch: APTO
  git_changes: APTO
  F2_DOC_GATE: APTO
  F3_TECH_GATE: APTO
  F4_RBAC_GATE: APTO
  F4_CERBERO_RBAC: APTO
  F5_VERDICT: APTO
  F5_VERDICT_GATE: APTO
  ROOT_TOPOLOGY_CLEAN: APTO
  DOC_OBJECTIVES: APTO
  DOC_CLARIFY: APTO
  DOC_SPEC: APTO
  DOC_PLAN: APTO
  DOC_IMPLEMENTATION: APTO
  DOC_EXECUTION: APTO
  DOC_FRONTMATTER_YAML: APTO
  DOC_EVOLUTION: APTO
  DOC_AC_CASCADE: APTO
  DOC_ORACLE_RED: APTO
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
  CERBERO_PPR_NOT_REVOKED: APTO
  PPR_NOT_REVOKED: APTO
  CODE_TOUCHPOINT_PRESENT: APTO
  WSPACE_MATERIALIZED: NO_APTO
  MERGE_ALREADY_OBSERVED: NO_APTO
  ACCEPT_PR_HANDOFF: NO_APTO
git_changes:
  branch: feat/man-55-prompt-invariants-oracle-ola0
  base: main
  source: path-assert-fs
  paths:
  - .cursor/rules/task-closure-documental.mdc
  - SddIA/agents/argos.md
  - SddIA/agents/dedalo.md
  - SddIA/agents/tekton.md
  - SddIA/core/eda-coverage.json
  - SddIA/evolution/9823b787-cf0b-48f3-bc9e-91fec7f1eb66.md
  - SddIA/evolution/Evolution_log.md
  - SddIA/library/norms/features-documentation-pattern.md
  - SddIA/library/norms/index.md
  - SddIA/norms/external-ai-constraints.md
  - SddIA/templates/spec-template/spec.md
  - SddIA/tools/sddia-qa/src/gate_allowlist_ratchet.rs
  - SddIA/tools/sddia-qa/src/gate_prompt_invariants.rs
  - SddIA/tools/sddia-qa/src/main.rs
  - docs/features/man-55-prompt-invariants-oracle-ola0/
blocking_findings: []
non_blocking_findings:
- GIT_EVIDENCE_SESSION_SHELL
- GIT_EVIDENCE_NATIVE_PPR
- WSPACE_MATERIALIZED
- MERGE_ALREADY_OBSERVED
- ACCEPT_PR_HANDOFF
situational_notes:
- Fase Veredicto y bloqueo · CID c3b86d7a-1441-4a45-a269-9e281fd225a4 · exec 3c7ca0f9-c295-4b6b-bd37-b1a477fd6f94
- 'Evidence Bridge session: source=native_state · TECH/GIT APTO · notes=idempotent-hit'
- 'Evidence Bridge machine: source=native_state @ 2026-11-22T19:20:17Z · TECH/GIT APTO · notes=idempotent-hit (copia; sin stdout inventado)'
- F2 heredado Triaje documental PASS_F2_DOC — cascada objectives→execution + YAML + AC-1…AC-4 + evolution 9823b787… + oracle-red.log → APTO
- ? F3_TECH_GATE APTO — proxy TECH_FORMAL_EXECUTE_PROCESS APTO + execution.md T4 (cargo test/clippy sddia-qa, gates verify-* / gate-allowlist-ratchet, sync-live-organs, eda_event_token_hygiene) + formal_evidence_detail verify-process-integrity
  : OK; sin degradar a residual
- F4_CERBERO_RBAC APTO — .SddIA/cerbero/revoked_entities.json · pull-request-review ∉ revoked/permanent
- ROOT_TOPOLOGY_CLEAN APTO — path-assert FS: raíz sin .events/orchestration/telemetry/pending/domain/dead-letter/progress/_lab
- Shell ./sddia-run.sh --tool git-manager → Rejected; sin gitStdout de esta sesión Argos
- GIT_EVIDENCE_NATIVE_PPR NO_APTO — workspace exec 3c7ca0f9… ausente/vacío; sin prep-branch propio
- git_changes = path-assert FS (touchpoints + cascada + evolution); no es gitStdout sesión
- WSPACE_MATERIALIZED NO_APTO — workspace inject 3c7ca0f9… ausente/vacío; no bloquea F5
- DOC_EVOLUTION APTO — hito SddIA/evolution/9823b787-cf0b-48f3-bc9e-91fec7f1eb66.md + fila Evolution_log MAN-55
- AC-1…AC-4 APTO — path-assert touchpoints (gates sddia-qa, normas/agentes/plantillas, cursor rule) + execution.md T1–T4 aplicadas + oracle-red.log
- Argos 0 writes docs/todos/** esta fase → RBAC_AUTHORING_KM_POLICY APTO; pending/ vacío; tracker.provider=linear
- accept_pr_handoff pending — L-HANDOFF-F5; MERGE/pr_url ausentes
- pbi_archived false — provider linear; sin fichero nuevo pending/done/
- validated_*/diff_digest no rematerializados esta sesión (Shell Rejected; sin inventar SHAs)
execution_id: 3c7ca0f9-c295-4b6b-bd37-b1a477fd6f94
validated_base_sha: b6516a0026b10aa649ea18a2643443dc116d92d5
validated_head_sha: b3709da112904008d0124b06b3aeb3c545b58ae6
validated_tree_sha: aceaad6bfca4f14a99967cff9e8ccd37d565fced
diff_digest: sha256:fb17580a1261b86f53531fa0988043579ea5ddd48ac0d4bda165d180bad013c4
---

# Validación — Veredicto y bloqueo (Argos · pull-request-review)

## Veredicto de fase

**APTO** — `resolution: PASS_F5_VERDICT` · `verdict: aprobado` · `delivery_state: success` · `accept_pr_handoff: pending` · `blocking_findings: []`.

| Gate | Delegado | Estado | Criterio |
|------|----------|--------|----------|
| F2 | Argos (doc, heredado) | **APTO** | cascada + YAML + AC-1…AC-4 + evolution + oracle-red (`PASS_F2_DOC`) |
| F3 | Triaje técnico (proxy) | **APTO** | TECH_FORMAL + `execution.md` T4 + `verify-process-integrity: OK` |
| F4 | Cerbero / RBAC | **APTO** | PPR ∉ revoked/permanent; R3 KM APTO |
| F5 | Argos (veredicto) | **APTO** | R1/R2/R3 APTO · F2–F4 APTO · `ROOT_TOPOLOGY_CLEAN` · sin blocking |
| R1 TECH_FORMAL | Evidence Bridge | **APTO** | copia session/machine `native_state` |
| R2 GIT_EVIDENCE | Evidence Bridge | **APTO** | copia bridge; sin stdout inventado |
| R3 KM | Argos | **APTO** | 0 writes ilegítimos bajo `docs/todos/**` |

## Evidence Bridge (R1 / R2 / R3)

Copia literal machine/session — **no** stdout inventado:

| Campo | Valor |
|-------|-------|
| `schema` | `kalma2-agent-runtime-evidence/v1` |
| `source` (session) | `native_state` |
| `source` (machine handoff) | `native_state` @ `2026-11-22T19:20:17Z` |
| `notes` (session/machine) | `idempotent-hit` |
| `TECH_FORMAL_EXECUTE_PROCESS` | **APTO** |
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | **APTO** |
| `formal_execute_process` | `true` |
| `git_manager_invoked` (machine) | `true` |
| `formal_evidence_detail` (heredado) | `verify-process-integrity: OK` (`prosthesis_subprocess` @ `2026-11-22T19:18:04Z`) |
| `GIT_EVIDENCE_SESSION_SHELL` | **NO_APTO** — Shell Rejected; sin `gitStdout` |
| `GIT_EVIDENCE_NATIVE_PPR` | **NO_APTO** — workspace `3c7ca0f9…` ausente/vacío |
| `RBAC_AUTHORING_KM_POLICY` | **APTO** — Argos 0 writes bajo `docs/todos/**`; `pending/` vacío |

Fuente: prompt Runtime evidence (session) + `docs/features/man-55-prompt-invariants-oracle-ola0/_agent_handoff.md` § Runtime evidence (machine) @ `2026-11-22T19:20:17Z`.

## Ingesta

| Input | Resolución |
|-------|------------|
| `persist_ref` | `docs/features/man-55-prompt-invariants-oracle-ola0` — presente |
| `pbi_ref` (inyectado) | vacío → resuelto `MAN-55` / `document_id: PBI-344ba8be` (clarify.md) |
| `correlation_id` | `c3b86d7a-1441-4a45-a269-9e281fd225a4` |
| `branch_name` (runtime) | `feat/man-55-prompt-invariants-oracle-ola0` |
| Worktree (FS HEAD) | `refs/heads/feat/man-55-prompt-invariants-oracle-ola0` |
| `workspace_path` | `.SddIA/workspaces/pull-request-review/3c7ca0f9…/` — **ausente/vacío** |
| `tracker.provider` | `linear` → sin fichero nuevo en `docs/todos/pending\|done/` |
| Norma | `features-documentation-pattern@1.3.3` |

## Cascada documental (F2 heredado)

| Check | Estado | Evidencia |
|-------|--------|-----------|
| `DOC_OBJECTIVES` | **APTO** | `objectives.md` + YAML + AC-1…AC-4 |
| `DOC_CLARIFY` | **APTO** | `clarify.md` + YAML + D-P1…D-P6 |
| `DOC_SPEC` | **APTO** | `spec.md` + YAML + oráculos AC-1…AC-4 |
| `DOC_PLAN` | **APTO** | `plan.md` T0–T4 + YAML |
| `DOC_IMPLEMENTATION` | **APTO** | `implementation.md` + YAML + touchpoints |
| `DOC_EXECUTION` | **APTO** | `execution.md` baseline Ola 0 + T1–T4 aplicadas + YAML |
| `DOC_ORACLE_RED` | **APTO** | `oracle-red.log` presente bajo persist_ref |
| `DOC_FRONTMATTER_YAML` | **APTO** | feature_name / branch_name / persist_ref / tracker_ref coherentes |
| `DOC_AC_CASCADE` | **APTO** | AC ↔ plan ↔ execution ↔ touchpoints alineados |
| `DOC_EVOLUTION` | **APTO** | hito `9823b787-…` + `Evolution_log` MAN-55 |
| `F2_DOC_GATE` | **APTO** | → `PASS_F2_DOC` heredado |

## Matriz de Criterios (AC)

| Criterio | Estado | Evidencia |
|----------|--------|-----------|
| AC-1 (`K-ORA` / `KZ-PRM-01`) | **APTO** | plantillas/normas/dedalo + `oracle-red.log` + `gate_prompt_invariants` |
| AC-2 (`K-EVA` + `K-RATCH`) | **APTO** | `external-ai-constraints` DA-8 + `gate_allowlist_ratchet` + tekton |
| AC-3 (`K-HERM` + `K-GEN`) | **APTO** | baseline hermético en `execution.md` + protocolo entity-manager |
| AC-4 (`K-HOST` + `K-ROLE`) | **APTO** | paridad host T4 + separación Tekton/Argos en normas/agentes/cursor rule |

## F3 / F4 / topología

| Check | Estado | Evidencia |
|-------|--------|-----------|
| `F3_TECH_GATE` | **APTO** | proxy `TECH_FORMAL_EXECUTE_PROCESS` APTO + `execution.md` T4 (tests/gates/sync) + `verify-process-integrity: OK`; **no** degradado a residual |
| `F4_RBAC_GATE` / `F4_CERBERO_RBAC` | **APTO** | `pull-request-review` ∉ `revoked`/`permanent` |
| `ROOT_TOPOLOGY_CLEAN` | **APTO** | FS: sin `.events`, `orchestration`, `telemetry`, `pending`, `domain`, `dead-letter`, `progress`, `_lab` en raíz |
| `F5_VERDICT` / `F5_VERDICT_GATE` | **APTO** | `blocking_findings=[]` → `PASS_F5_VERDICT` · `verdict: aprobado` |

## branch / git_changes

| Check | Estado | Evidencia |
|-------|--------|-----------|
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | **APTO** | Evidence Bridge `native_state` (copia) |
| `GIT_EVIDENCE_SESSION_SHELL` | **NO_APTO** | Shell Rejected; sin `gitStdout` sesión Argos |
| `GIT_EVIDENCE_NATIVE_PPR` | **NO_APTO** | workspace `3c7ca0f9…` sin prep-branch stdout |
| `BRANCH_RUNTIME_INJECT` | **APTO** | `feat/man-55-prompt-invariants-oracle-ola0` |
| `BRANCH_WORKTREE_SYNC` | **APTO** | `.git/HEAD` ≡ inject (FS; **no** stdout git-manager) |
| `branch` | **APTO** | inject ≡ HEAD |
| `git_changes` | **APTO** | inventario path-assert FS; **no** es `gitStdout` de esta sesión |
| `WSPACE_MATERIALIZED` | **NO_APTO** | workspace ausente/vacío; **no bloquea** F5 |
| `ACCEPT_PR_HANDOFF` | **NO_APTO** | `pending` (L-HANDOFF-F5); merge soberano fuera de esta fase |

`git_changes.paths` = path-assert FS (touchpoints + cascada + evolution). Sin bypass raw destructivo. `validated_*` / `diff_digest` **no** rematerializados (Shell Rejected; sin inventar SHAs).

## R3 — KM (`RBAC_AUTHORING_KM_POLICY`)

**APTO** — auditoría solo autoría bajo `docs/todos/**`. `docs/todos/pending/` vacío (0 files). Sin semillas MAN-55 ilegítimas. Argos **no** escribió TODOs. Cumulo/`Kaizen_Alert_Required` = vía legítima (no usada esta fase). Forja Core ≠ este check. Provider Linear: sin materialización local de PBI.

## Dictamen

```json
{
  "phase": "Veredicto y bloqueo",
  "process": "pull-request-review",
  "global": "APTO",
  "resolution": "PASS_F5_VERDICT",
  "verdict": "aprobado",
  "delivery_state": "success",
  "accept_pr_handoff": false,
  "accept_pr_handoff_status": "pending",
  "branch": "feat/man-55-prompt-invariants-oracle-ola0",
  "branch_worktree_observed": "feat/man-55-prompt-invariants-oracle-ola0",
  "persist_ref": "docs/features/man-55-prompt-invariants-oracle-ola0",
  "document_id": "PBI-344ba8be",
  "correlation_id": "c3b86d7a-1441-4a45-a269-9e281fd225a4",
  "execution_id": "3c7ca0f9-c295-4b6b-bd37-b1a477fd6f94",
  "TECH_FORMAL_EXECUTE_PROCESS": "APTO",
  "GIT_EVIDENCE_VIA_GIT_MANAGER": "APTO",
  "RBAC_AUTHORING_KM_POLICY": "APTO",
  "F2_DOC_GATE": "APTO",
  "F3_TECH_GATE": "APTO",
  "F4_RBAC_GATE": "APTO",
  "ROOT_TOPOLOGY_CLEAN": "APTO",
  "F5_VERDICT": "APTO",
  "blocking": []
}
```

## Alcance

Veredicto y bloqueo **aprueba** peaje F5 (`PASS_F5_VERDICT`). Residuales no bloqueantes: `GIT_EVIDENCE_SESSION_SHELL`, `GIT_EVIDENCE_NATIVE_PPR`, `WSPACE_MATERIALIZED`, `MERGE_ALREADY_OBSERVED`, `ACCEPT_PR_HANDOFF`. Downstream: Cosecha Kaizen → Handoff materialización (`accept-pr`; sin merge directo en aduana). Argos **no** escribe bajo `docs/todos/` ni reescribe `_agent_handoff.md`.

## approval_status

```text
aprobado
```