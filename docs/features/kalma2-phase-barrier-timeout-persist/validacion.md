---
feature_name: kalma2-phase-barrier-timeout-persist
created: "2026-08-14"
updated: "2026-08-14T10:38:00Z"
process: pull-request-review
phase: Certificación RBAC
agent: cerbero
agents: cerbero
branch: refactor/kalma2-phase-barrier-timeout-persist
branch_name_injected: refactor/kalma2-phase-barrier-timeout-persist
persist_ref: docs/features/kalma2-phase-barrier-timeout-persist
pbi_ref: docs/todos/done/[REFACTOR] Kalma2 — serialización de fases, timeout y rama refactor (KALMA2-AUD-4b9de6).md
document_id: 1de0bdd1-6144-4e45-8efa-92db0f399147
correlation_id: 2b466b03-9125-414e-9893-8ea6d8ef7f93
pr_presented_event_id: 2b466b03-9125-414e-9893-8ea6d8ef7f93
audit_event_reference: 2b466b03-9125-414e-9893-8ea6d8ef7f93
source_correlation_id: 4b9de6b3-c400-49c8-86f2-55f08ec64ce4
execution_id: d630a6cf-1767-4751-a2b9-b1f4210a01fb
pr_url: https://github.com/racso80es/SddIA/pull/174
global: APTO
pbi_archived: true
approval_status: aprobado
verdict: aprobado
delivery_state: pending_downstream_phases
resolution: PASS_F4_RBAC
authorization_status:
  exitCode: 0
  signer_identity_rbac: Vertice_Biologico_Relay
  emitter_agent: delivery-close-cycle
  note: "PASS_F4_RBAC · E1/E2 APTO · VBR×genoma APTO · R1/R2 copia Argos F2 prosthesis_subprocess; Shell git-manager Rejected esta sesión — sin stdout inventado"
git_manager_invoked: false
git_manager_error: "cápsula no invocable en esta sesión (Shell/Auto-review rejected sobre ./sddia-run.sh --tool git-manager); sin stdout físico; R2 = copia Evidence Bridge session prosthesis_subprocess (Argos F2); sin bypass raw"
git_evidence_source: prosthesis_subprocess-evidence-bridge
formal_execute_process: true
handoff_machine_file: present
evidence_bridge_notes: "R1/R2 copia Runtime evidence (machine) Argos F2 source=prosthesis_subprocess; bloque machine presente en persist_ref/_agent_handoff.md; Shell git-manager Rejected esta sesión Cerbero — sin stdout inventado"
shell_git_manager_session: "Rejected (Auto-review); R2 no inventado — copia session prosthesis_subprocess"
checks:
  F2_DOC_GATE: APTO
  F3_TECH_GATE: NO_APTO
  F4_RBAC_GATE: APTO
  DOC_OBJECTIVES: APTO
  DOC_CLARIFY: APTO
  DOC_SPEC: APTO
  DOC_PLAN: APTO
  DOC_IMPLEMENTATION: APTO
  DOC_EXECUTION: APTO
  DOC_FRONTMATTER_YAML: APTO
  DOC_EVOLUTION: APTO
  TECH_FORMAL_EXECUTE_PROCESS: APTO
  GIT_EVIDENCE_VIA_GIT_MANAGER: APTO
  GIT_EVIDENCE_SESSION_SHELL: NO_APTO
  RBAC_SPATIAL_INTEGRITY: APTO
  RBAC_SIGNER_PRESENT: APTO
  RBAC_SIGNER_NOT_REVOKED: APTO
  RBAC_SIGNER_VS_GENOME: APTO
  RBAC_EMITTER_AUTHORIZED: APTO
  RBAC_EMITTER_NOT_REVOKED: APTO
  RBAC_AUTHORING_KM_POLICY: APTO
  RBAC_PROCESS_REGISTRY: APTO
  ECST_SIGNER_PRESENT: APTO
  PERSIST_REF_RESOLVED: APTO
  HANDOFF_MACHINE_FILE: APTO
  HANDOFF_EVIDENCE_BLOCK: APTO
  BRANCH_RUNTIME_INJECT: APTO
  BRANCH_ECST_ALIGN: APTO
  BRANCH_WORKTREE_SYNC: APTO
  PBI_DONE_PRESENT: APTO
  PBI_PENDING_ABSENT: APTO
  AC_DONE_PATH: APTO
  MERGE_ALREADY_OBSERVED: NO_APTO
git_changes:
  - SddIA/engine/execute-process/src/engine/executor.rs
  - SddIA/engine/execute-process/src/engine/workspace_init.rs
  - SddIA/engine/execute-process/src/engine/handlers/task_queue_manager.rs
  - SddIA/engine/execute-process/src/engine/agent_runtime.rs
  - SddIA/engine/execute-process/src/engine/eda_bus_topology.rs
  - SddIA/scripts/tools/kalma2-agent-runtime-cursor.py
  - SddIA/scripts/tools/test_kalma2_runtime_timeout.py
  - docs/features/kalma2-phase-barrier-timeout-persist/
  - docs/todos/done/[REFACTOR] Kalma2 — serialización de fases, timeout y rama refactor (KALMA2-AUD-4b9de6).md
  - SddIA/evolution/d630a6cf-1767-4751-a2b9-b1f4210a01fb.md
  - SddIA/evolution/Evolution_log.md
blocking_findings: []
non_blocking_findings:
  - GIT_EVIDENCE_SESSION_SHELL
  - F3_TECH_GATE
  - MERGE_ALREADY_OBSERVED
---

# Validación — Certificación RBAC (Cerbero · pull-request-review)

## Veredicto de fase

**APTO** — `F4_RBAC_GATE: APTO` · `authorization_status.exitCode: 0` · `resolution: PASS_F4_RBAC`.  
F3 (técnico), Veredicto/bloqueo, Cosecha y Handoff **fuera** de esta fase → `delivery_state: pending_downstream_phases`.

| Gate | Delegado | Estado | Criterio |
|------|----------|--------|----------|
| F2 | Argos (doc) | **APTO** | heredado · `PASS_F2_DOC` |
| F3 | execute-process | **pendiente** | fuera de jurisdicción Certificación RBAC |
| F4 | Cerbero | **APTO** | firmante VBR × área genoma · emisor DCC ∉ revoked |

## Evidence Bridge (R1 / R2 / R3)

Bloque `### Runtime evidence (machine)` **presente** en `persist_ref/_agent_handoff.md` (Argos F2). Copia (no stdout inventado esta sesión):

| Campo | Valor |
|-------|-------|
| `source` | `prosthesis_subprocess` |
| `TECH_FORMAL_EXECUTE_PROCESS` | **APTO** |
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | **APTO** |
| `notes` | copia session prosthesis_subprocess; persist_ref vacío al spawn F2 — bloque materializado por Argos |
| `GIT_EVIDENCE_SESSION_SHELL` | **NO_APTO** — `./sddia-run.sh --tool git-manager` → Shell/Auto-review Rejected; sin `gitStdout` físico esta sesión Cerbero |
| `RBAC_AUTHORING_KM_POLICY` | **APTO** — Cerbero 0 writes bajo `docs/todos/**` |

## Ingesta

| Input | Resolución |
|-------|------------|
| `persist_ref` (inyectado) | vacío → **resuelto** `docs/features/kalma2-phase-barrier-timeout-persist` (`branch_name` + Cúmulo `paths.featurePath`) |
| `pbi_ref` (inyectado) | vacío → **resuelto** `docs/todos/done/[REFACTOR] Kalma2 — serialización de fases, timeout y rama refactor (KALMA2-AUD-4b9de6).md` |
| `correlation_id` / Presented | `2b466b03-9125-414e-9893-8ea6d8ef7f93` |
| `document_id` | `1de0bdd1-6144-4e45-8efa-92db0f399147` |
| ECST `emitter_agent` | `delivery-close-cycle` |
| ECST `signer_identity_rbac` | `Vertice_Biologico_Relay` |
| `branch` (ECST) | `refactor/kalma2-phase-barrier-timeout-persist` |
| `branch_name` (runtime) | `refactor/kalma2-phase-barrier-timeout-persist` |
| `.git/HEAD` (FS) | `refs/heads/refactor/kalma2-phase-barrier-timeout-persist` |
| ref local rama (FS) | `.git/refs/heads/refactor/kalma2-phase-barrier-timeout-persist` → `17779dad…` |
| `pr_url` | `https://github.com/racso80es/SddIA/pull/174` |
| Evento Presented | `.events/processing/2b466b03-….json` · subscriber `argos.pull-request-review` · `state: processing` |
| Evento Merged (este ECST) | **ausente** (path-assert bus; no sello) |

## F4 — Certificación RBAC

| Check | Estado | Evidencia |
|-------|--------|-----------|
| `RBAC_SPATIAL_INTEGRITY` | **APTO** | `directories.norms` → `SddIA/norms/execution-contexts.md` accesible |
| `ECST_SIGNER_PRESENT` | **APTO** | payload `signer_identity_rbac: Vertice_Biologico_Relay` |
| `RBAC_SIGNER_PRESENT` | **APTO** | mismo firmante ECST |
| `RBAC_SIGNER_NOT_REVOKED` | **APTO** | `Vertice_Biologico_Relay` ∉ `.SddIA/cerbero/revoked_entities.json` |
| `RBAC_EMITTER_AUTHORIZED` | **APTO** | `delivery-close-cycle` emisor canónico `PullRequest_Presented` |
| `RBAC_EMITTER_NOT_REVOKED` | **APTO** | `delivery-close-cycle` ∉ revoked (revoked: `emit-pr-audited-event`, `feature`) |
| `RBAC_PROCESS_REGISTRY` | **APTO** | `pull-request-review` ∉ revoked |
| `RBAC_SIGNER_VS_GENOME` | **APTO** | VBR × `engine/execute-process/` + `scripts/tools/` + docs + `evolution/`; genoma DA-2 (`git-operations.md`, `refactorization.md`) **no** mutado (L-GENOME / `implementation.md`) |
| `RBAC_AUTHORING_KM_POLICY` | **APTO** | Cerbero 0 writes `docs/todos/` |
| `F4_RBAC_GATE` | **APTO** | `exitCode: 0` · `PASS_F4_RBAC` |

## F2 — Triaje documental (heredado)

| Check | Estado | Evidencia |
|-------|--------|-----------|
| `F2_DOC_GATE` | **APTO** | Argos fase previa · `resolution: PASS_F2_DOC` |
| Cascada documental | **APTO** | objectives/clarify/spec/plan/implementation/execution + YAML |
| `DOC_EVOLUTION` | **APTO** | `SddIA/evolution/d630a6cf-1767-4751-a2b9-b1f4210a01fb.md` |

## PBI / Done path

| Check | Estado | Evidencia |
|-------|--------|-----------|
| `PBI_DONE_PRESENT` | **APTO** | `docs/todos/done/…KALMA2-AUD-4b9de6.md` · `document_id: 1de0bdd1-…` · `status: done` |
| `PBI_PENDING_ABSENT` | **APTO** | sin `1de0bdd1-…` / KALMA2-AUD-4b9de6 bajo `docs/todos/pending/` |
| `AC_DONE_PATH` | **APTO** | done exclusivo + `pbi_archived: true` |

Cerbero **no** materializa KM.

## Git / rama

| Check | Estado | Evidencia |
|-------|--------|-----------|
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | **APTO** | copia session `prosthesis_subprocess` (R2; Argos F2) |
| `GIT_EVIDENCE_SESSION_SHELL` | **NO_APTO** | cápsula no invocable esta sesión; sin bypass raw |
| `BRANCH_RUNTIME_INJECT` | **APTO** | `branch_name` = `refactor/kalma2-phase-barrier-timeout-persist` |
| `BRANCH_ECST_ALIGN` | **APTO** | ECST `payload.branch` = misma rama |
| `BRANCH_WORKTREE_SYNC` | **APTO** | `.git/HEAD` → `refs/heads/refactor/kalma2-phase-barrier-timeout-persist` (FS; **no** stdout git-manager) |
| `MERGE_ALREADY_OBSERVED` | **NO_APTO** | sin `PullRequest_Merged` para `2b466b03-…` |
| `HANDOFF_MACHINE_FILE` | **APTO** | `{persist_ref}/_agent_handoff.md` presente |
| `HANDOFF_EVIDENCE_BLOCK` | **APTO** | schema v1 materializado (Argos F2) |

`git_changes` por **inventario path-assert** heredado F2 (no `gitStdout` de esta sesión).

## Dictamen final

```json
{
  "phase": "Certificación RBAC",
  "verdict": "aprobado",
  "delivery_state": "pending_downstream_phases",
  "resolution": "PASS_F4_RBAC",
  "audit_event_reference": "2b466b03-9125-414e-9893-8ea6d8ef7f93",
  "authorization_status": {
    "exitCode": 0,
    "signer_identity_rbac": "Vertice_Biologico_Relay",
    "emitter_agent": "delivery-close-cycle"
  },
  "blocking_findings": [],
  "non_blocking_findings": [
    "GIT_EVIDENCE_SESSION_SHELL:NO_APTO",
    "F3_TECH_GATE:NO_APTO",
    "MERGE_ALREADY_OBSERVED:NO_APTO"
  ]
}
```

## Jurisdicción de fase

Cubre **Certificación RBAC** (F4). Downstream: Triaje técnico (si no materializado) → Veredicto → Cosecha → Handoff (`accept-pr`; sin merge directo en aduana). Cerbero **no** escribe bajo `docs/todos/`.

## approval_status

```text
aprobado — PASS_F4_RBAC · exitCode 0 · F4_RBAC_GATE APTO;
E1/E2 APTO (VBR + DCC ∉ revoked); VBR×genoma APTO (DA-2 no mutado);
GIT_EVIDENCE_SESSION_SHELL NO_APTO no bloqueante; R2 APTO vía copia prosthesis_subprocess;
F3 pendiente; MERGE este CID NO_APTO; delivery_state pending_downstream_phases.
```
