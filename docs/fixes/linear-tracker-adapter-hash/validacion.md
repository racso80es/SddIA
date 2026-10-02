---
feature_name: linear-tracker-adapter-hash
created: "2026-10-02"
updated: "2026-10-02T18:40:30Z"
process: pull-request-review
phase: Veredicto y bloqueo
agent: argos
agents: argos
branch: fix/linear-tracker-adapter-hash
branch_name: fix/linear-tracker-adapter-hash
branch_name_injected: fix/linear-tracker-adapter-hash
persist_ref: docs/fixes/linear-tracker-adapter-hash
pbi_ref: docs/todos/done/[DEUDA] Tracker — hash de linear-tracker-adapter.md
document_id: PBI-DEUDA-TRACKER-ADAPTER-HASH
uuid: "4504d70c-7cb7-42a4-95bf-0032d14f2300"
correlation_id: "db8bb10e-18dc-45aa-94cd-dc7d175404bf"
audit_event_reference: "db8bb10e-18dc-45aa-94cd-dc7d175404bf"
local_qa_event_id: "db8bb10e-18dc-45aa-94cd-dc7d175404bf"
execution_id: "66697afb-ffa3-4bb2-8439-a3b7fe1e79dd"
pr_presented_event_id: ""
pr_url: ""
global: NO_APTO
pbi_archived: true
approval_status: rechazado
verdict: requiere_cambios
delivery_state: failed
resolution: FAIL_F5_VERDICT
accept_pr_handoff: false
accept_pr_handoff_status: blocked
accept_pr_block_reason: "FAIL_F5_VERDICT; F2_DOC_GATE NO_APTO (faltan objectives.md / spec.md / plan.md / implementation.md); pbi_archived true no absuelve (L-HANDOFF-F5)"
authorization_status:
  exitCode: 1
  emitter_agent: argos
  note: "FAIL_F5_VERDICT · aborta por violación F2 · Shell git-manager Rejected esta sesión — R1/R2 copia Evidence Bridge native_state · sin stdout inventado"
git_manager_invoked: false
git_manager_error: "cápsula no invocable en esta sesión Argos Veredicto (Shell Rejected / no bypass raw); sin stdout físico; R2 = copia Evidence Bridge native_state"
git_evidence_source: native_state-evidence-bridge
formal_execute_process: true
handoff_machine_file: present
evidence_bridge_notes: "R1/R2 copia Runtime evidence (machine+session) source=native_state notes=idempotent-hit @ 2026-10-02T18:39:42Z; TECH_FORMAL_EXECUTE_PROCESS / GIT_EVIDENCE_VIA_GIT_MANAGER APTO; sin gitStdout inventado"
shell_git_manager_session: "Rejected — sin gitStdout físico esta invocación Argos Veredicto CID db8bb10e…"
scope: "PPR Veredicto y bloqueo — rama fix/linear-tracker-adapter-hash (CID db8bb10e… · exec 66697afb…)"
checks:
  F2_DOC_GATE: NO_APTO
  F3_TECH_GATE: NO_APTO
  F4_RBAC_GATE: NO_APTO
  F5_VERDICT: NO_APTO
  DOC_OBJECTIVES: NO_APTO
  DOC_SPEC: NO_APTO
  DOC_PLAN: NO_APTO
  DOC_IMPLEMENTATION: NO_APTO
  DOC_EXECUTION: NO_APTO
  DOC_FRONTMATTER_YAML: APTO
  PERSIST_REF_RESOLVED: APTO
  HANDOFF_MACHINE_FILE: APTO
  HANDOFF_EVIDENCE_BLOCK: APTO
  BRANCH_RUNTIME_INJECT: APTO
  BRANCH_WORKTREE_SYNC: APTO
  TECH_FORMAL_EXECUTE_PROCESS: APTO
  GIT_EVIDENCE_VIA_GIT_MANAGER: APTO
  GIT_EVIDENCE_SESSION_SHELL: NO_APTO
  RBAC_AUTHORING_KM_POLICY: APTO
  PBI_DONE_PRESENT: APTO
  PBI_PENDING_ABSENT: APTO
  AC_DONE_PATH: APTO
  branch: APTO
  git_changes: APTO
git_changes:
  - docs/fixes/linear-tracker-adapter-hash/_agent_handoff.md
  - docs/fixes/linear-tracker-adapter-hash/validacion.md
  - docs/todos/done/[DEUDA] Tracker — hash de linear-tracker-adapter.md
  - SddIA/tools/linear-tracker-adapter.md
  - SddIA/core/eda-coverage.json
blocking_findings:
  - F2_DOC_GATE
  - DOC_OBJECTIVES
  - DOC_SPEC
  - DOC_PLAN
  - DOC_IMPLEMENTATION
  - F5_VERDICT
non_blocking_findings:
  - DOC_EXECUTION
  - GIT_EVIDENCE_SESSION_SHELL
  - F3_TECH_GATE
  - F4_RBAC_GATE
situational_notes:
  - "Fase Veredicto y bloqueo · CID db8bb10e-18dc-45aa-94cd-dc7d175404bf · exec 66697afb-ffa3-4bb2-8439-a3b7fe1e79dd"
  - "persist_ref legible; solo _agent_handoff.md + validacion.md; ausentes objectives/spec/plan/implementation/execution"
  - "Evidence Bridge machine @ 18:39:42Z source=native_state notes=idempotent-hit · TECH/GIT APTO (también prosthesis_subprocess @ 18:38:12Z)"
  - "Shell ./sddia-run.sh --tool git-manager Rejected; sin inventar gitStdout; git_changes = path-assert FS + inventario handoff"
  - "Argos 0 writes docs/todos/** esta fase → RBAC_AUTHORING_KM_POLICY APTO"
  - "F3/F4 NO_APTO — no materializados este CID; proxy R1 no levanta peaje"
  - "PBI en done/ (document_id PBI-DEUDA-TRACKER-ADAPTER-HASH); pbi_archived true no absuelve F2/F5"
  - "Producto físico: hash_signature sha256:8722fcf349c933f3… en ficha + eda-coverage — no absuelve F2 documental PPR"
  - "branch APTO vía .git/HEAD = refs/heads/fix/linear-tracker-adapter-hash (FS Read; no stdout git-manager)"
  - "accept_pr_handoff blocked (L-HANDOFF-F5)"
---
# Validación — Veredicto y bloqueo (Argos · pull-request-review)

## Veredicto de fase

**NO_APTO** — `resolution: FAIL_F5_VERDICT` · `F5_VERDICT: NO_APTO` · aborta por violación F2 (cascada documental ausente en `persist_ref`).

| Artefacto | Estado |
|-----------|--------|
| `objectives.md` | **ausente** |
| `spec.md` | **ausente** |
| `plan.md` | **ausente** |
| `implementation.md` | **ausente** |
| `execution.md` | **ausente** (no bloqueante estricto PPR F2) |
| `_agent_handoff.md` | **presente** |
| `validacion.md` | **presente** (este informe) |

## Evidence Bridge (R1 / R2 / R3)

Copia literal machine/session — **no** stdout Shell inventado:

| Campo | Valor |
|-------|-------|
| `source` | `native_state` @ 2026-10-02T18:39:42Z (+ session inject `idempotent-hit`) |
| `git_manager_invoked` | `true` (bridge) · `false` (sesión Argos Shell) |
| `formal_execute_process` | `true` |
| `formal_evidence_detail` | `verify-process-integrity: OK` (bloque @ 18:38:13Z) |
| `TECH_FORMAL_EXECUTE_PROCESS` | **APTO** |
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | **APTO** |
| `notes` | `idempotent-hit` |
| `GIT_EVIDENCE_SESSION_SHELL` | **NO_APTO** — cápsula Rejected esta sesión |
| `RBAC_AUTHORING_KM_POLICY` | **APTO** — Argos 0 writes bajo `docs/todos/**` |

Bloque machine: `_agent_handoff.md` § Runtime evidence (machine) @ 18:39:42Z (+ session inject).

## Peajes F2 → F5

| Check | Estado | Evidencia |
|-------|--------|-----------|
| `F2_DOC_GATE` | **NO_APTO** | faltan `objectives.md` / `spec.md` / `plan.md` / `implementation.md` |
| `F3_TECH_GATE` | **NO_APTO** | no materializado este CID |
| `F4_RBAC_GATE` | **NO_APTO** | no materializado este CID |
| `F5_VERDICT` | **NO_APTO** | violación F2 ⇒ aborta; `accept_pr_handoff` blocked (L-HANDOFF-F5) |

## PBI / rama / KM

| Check | Estado | Evidencia |
|-------|--------|-----------|
| `PBI_DONE_PRESENT` | **APTO** | `docs/todos/done/[DEUDA] Tracker — hash de linear-tracker-adapter.md` |
| `PBI_PENDING_ABSENT` | **APTO** | 0 bajo pending/ para este document_id |
| `pbi_archived` | **true** | no absuelve F2/F5 |
| `branch` | **APTO** | inject = HEAD = `fix/linear-tracker-adapter-hash` |
| `git_changes` | **APTO** | path-assert (no gitStdout cápsula esta sesión) |
| `RBAC_AUTHORING_KM_POLICY` | **APTO** | 0 writes Argos en `docs/todos/**` |

## Dictamen

```json
{
  "phase": "Veredicto y bloqueo",
  "global": "NO_APTO",
  "resolution": "FAIL_F5_VERDICT",
  "verdict": "requiere_cambios",
  "delivery_state": "failed",
  "accept_pr_handoff": false,
  "accept_pr_handoff_status": "blocked",
  "pbi_archived": true,
  "branch": "fix/linear-tracker-adapter-hash",
  "correlation_id": "db8bb10e-18dc-45aa-94cd-dc7d175404bf",
  "execution_id": "66697afb-ffa3-4bb2-8439-a3b7fe1e79dd",
  "blocking": ["DOC_OBJECTIVES", "DOC_SPEC", "DOC_PLAN", "DOC_IMPLEMENTATION", "F2_DOC_GATE", "F5_VERDICT"]
}
```

## Remedio

Materializar `objectives.md`, `spec.md`, `plan.md` e `implementation.md` bajo `docs/fixes/linear-tracker-adapter-hash/` (frontmatter YAML) antes de re-disparar PPR. Argos **no** escribe bajo `docs/todos/`.
