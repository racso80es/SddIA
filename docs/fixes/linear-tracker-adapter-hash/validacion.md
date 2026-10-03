---
feature_name: linear-tracker-adapter-hash
created: "2026-10-02"
updated: "2026-10-02T18:43:00Z"
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
correlation_id: "GUxwkGmeCxi3Y36HuANQkXNdQkHsLAVdMZK7Y3UGVYtP"
audit_event_reference: "GUxwkGmeCxi3Y36HuANQkXNdQkHsLAVdMZK7Y3UGVYtP"
local_qa_event_id: "GUxwkGmeCxi3Y36HuANQkXNdQkHsLAVdMZK7Y3UGVYtP"
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
accept_pr_block_reason: "FAIL_F5_VERDICT ← FAIL_F2_DOC_GATE — faltan objectives.md / spec.md / plan.md / implementation.md en persist_ref; pbi_archived true no absuelve F2"
authorization_status:
  exitCode: 1
  emitter_agent: argos
  note: "FAIL_F5_VERDICT · Veredicto y bloqueo · R1/R2 copia Evidence Bridge native_state · sin stdout inventado · Shell git-manager Rejected esta sesión"
git_manager_invoked: false
git_manager_error: "cápsula no invocada esta sesión Argos Veredicto (Shell IDE Rejected / no bypass raw); sin stdout físico; R2 = copia Evidence Bridge native_state"
git_evidence_source: native_state-evidence-bridge
formal_execute_process: true
handoff_machine_file: present
evidence_bridge_notes: "R1/R2 copia Runtime evidence (machine+session) source=native_state notes=idempotent-hit @ 2026-10-02T18:42:08Z; TECH_FORMAL_EXECUTE_PROCESS / GIT_EVIDENCE_VIA_GIT_MANAGER APTO; sin gitStdout inventado"
shell_git_manager_session: "Rejected — sin gitStdout físico esta invocación Argos Veredicto CID GUxwkGme…"
scope: "PPR Veredicto y bloqueo — rama fix/linear-tracker-adapter-hash (CID GUxwkGme… · exec ccd0de37…)"
checks:
  F2_DOC_GATE: NO_APTO
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
  - F5_VERDICT
  - F2_DOC_GATE
  - DOC_OBJECTIVES
  - DOC_SPEC
  - DOC_PLAN
  - DOC_IMPLEMENTATION
non_blocking_findings:
  - DOC_EXECUTION
  - GIT_EVIDENCE_SESSION_SHELL
situational_notes:
  - "Fase Veredicto y bloqueo · CID GUxwkGmeCxi3Y36HuANQkXNdQkHsLAVdMZK7Y3UGVYtP · exec ccd0de37-35d5-4cfb-b1bf-2d43d42ec6d0"
  - "persist_ref legible; solo _agent_handoff.md + validacion.md (+ .tmp/); ausentes objectives/spec/plan/implementation/execution"
  - "Evidence Bridge machine @ 18:42:08Z source=native_state notes=idempotent-hit · TECH/GIT APTO (también prosthesis_subprocess @ 18:38:12Z)"
  - "Shell ./sddia-run.sh --tool git-manager Rejected; sin inventar gitStdout; git_changes = path-assert FS + inventario handoff/producto"
  - "Argos 0 writes docs/todos/** esta fase → RBAC_AUTHORING_KM_POLICY APTO"
  - "PBI en done/ (document_id PBI-DEUDA-TRACKER-ADAPTER-HASH); pbi_archived true no absuelve F2"
  - "Producto físico: hash_signature sha256:8722fcf349c933f3… en ficha — no absuelve F2 documental PPR"
  - "branch APTO vía .git/HEAD = refs/heads/fix/linear-tracker-adapter-hash (FS Read; no stdout git-manager)"
  - "F5: delivery_state failed; accept_pr_handoff false (L-HANDOFF-F5)"
execution_id: "ccd0de37-35d5-4cfb-b1bf-2d43d42ec6d0"
---
# Validación — Veredicto y bloqueo (Argos · pull-request-review)

## Veredicto de fase

**NO_APTO** — `resolution: FAIL_F5_VERDICT` ← cascada `FAIL_F2_DOC_GATE` · artefactos documentales ausentes en `persist_ref`.

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
| `source` | `native_state` @ 2026-10-02T18:42:08Z (+ session inject `idempotent-hit`) |
| `git_manager_invoked` | `true` (bridge) · `false` (sesión Argos Shell) |
| `formal_execute_process` | `true` |
| `formal_evidence_detail` | `verify-process-integrity: OK` (bloque @ 18:38:13Z) |
| `TECH_FORMAL_EXECUTE_PROCESS` | **APTO** |
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | **APTO** |
| `notes` | `idempotent-hit` |
| `GIT_EVIDENCE_SESSION_SHELL` | **NO_APTO** — cápsula Rejected esta sesión |
| `RBAC_AUTHORING_KM_POLICY` | **APTO** — Argos 0 writes bajo `docs/todos/**` |

Bloque machine: `_agent_handoff.md` § Runtime evidence (machine) @ 18:42:08Z (+ session inject).

## Peaje F2 → F5

| Check | Estado | Evidencia |
|-------|--------|-----------|
| `F2_DOC_GATE` | **NO_APTO** | faltan `objectives.md` / `spec.md` / `plan.md` / `implementation.md` |
| `F5_VERDICT` | **NO_APTO** | violación F2 → aborta materialización |
| `DOC_FRONTMATTER_YAML` | **APTO** | PBI + handoff + este `validacion.md` con frontmatter YAML |
| `PERSIST_REF_RESOLVED` | **APTO** | `docs/fixes/linear-tracker-adapter-hash` legible |

## PBI / rama / KM

| Check | Estado | Evidencia |
|-------|--------|-----------|
| `PBI_DONE_PRESENT` | **APTO** | `docs/todos/done/[DEUDA] Tracker — hash de linear-tracker-adapter.md` |
| `PBI_PENDING_ABSENT` | **APTO** | 0 bajo pending/ para este document_id |
| `pbi_archived` | **true** | no absuelve F2 |
| `branch` | **APTO** | inject = HEAD = `fix/linear-tracker-adapter-hash` |
| `git_changes` | **APTO** | path-assert FS (no gitStdout cápsula esta sesión) |
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
  "correlation_id": "GUxwkGmeCxi3Y36HuANQkXNdQkHsLAVdMZK7Y3UGVYtP",
  "execution_id": "ccd0de37-35d5-4cfb-b1bf-2d43d42ec6d0",
  "blocking": ["F5_VERDICT", "F2_DOC_GATE", "DOC_OBJECTIVES", "DOC_SPEC", "DOC_PLAN", "DOC_IMPLEMENTATION"]
}
```

## Remedio

Materializar `objectives.md`, `spec.md`, `plan.md` e `implementation.md` bajo `docs/fixes/linear-tracker-adapter-hash/` (frontmatter YAML) antes de re-disparar PPR. Argos **no** escribe bajo `docs/todos/`.
