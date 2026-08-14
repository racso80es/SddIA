---
feature_name: evolution-history-normalization
created: "2026-08-14"
updated: "2026-08-14T10:20:00Z"
process: pull-request-review
phase: Certificación RBAC
agent: cerbero
agents: cerbero
branch: refactor/evolution-history-normalization
branch_name_injected: refactor/evolution-history-normalization
persist_ref: docs/features/evolution-history-normalization
pbi_ref: docs/todos/done/[REFACTOR] Evolution — migrar históricos y extraer borradores (EV-AUD-002-007).md
document_id: 7bb37ff1-decd-4ec5-968b-344a5334f9eb
correlation_id: e8fb3a94-e9f2-443c-8547-c50aa091af20
source_correlation_id: 4b9de6b3-c400-49c8-86f2-55f08ec64ce4
execution_id: 63062872-e707-496e-b1b3-1ea736e256f0
pr_url: https://github.com/racso80es/SddIA/pull/173
pr_presented_event_id: e8fb3a94-e9f2-443c-8547-c50aa091af20
source_audit: docs/audits/evolution/2026-08-11.md
findings:
  - EV-AUD-002
  - EV-AUD-007
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
handoff_machine_file: created
evidence_bridge_notes: "R1/R2 copia Runtime evidence (session) Argos F2 source=prosthesis_subprocess; _agent_handoff.md ausente al inicio F4; Shell git-manager Rejected esta sesión Cerbero — sin stdout inventado"
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
  BRANCH_RUNTIME_INJECT: APTO
  BRANCH_ECST_ALIGN: APTO
  BRANCH_WORKTREE_SYNC: NO_APTO
  PBI_DONE_PRESENT: APTO
  PBI_PENDING_ABSENT: APTO
  AC_DONE_PATH: APTO
  MERGE_ALREADY_OBSERVED: NO_APTO
git_changes:
  - SddIA/tools/sddia-qa/src/migrate_evolution_history.rs
  - SddIA/tools/sddia-qa/src/validate_evolution_contract.rs
  - SddIA/tools/sddia-qa/src/main.rs
  - SddIA/tools/sddia-qa/Cargo.toml
  - SddIA/evolution/
  - SddIA/evolution/63062872-e707-496e-b1b3-1ea736e256f0.md
  - docs/audits/evolution/drafts/
  - docs/features/evolution-history-normalization/
  - docs/todos/done/[REFACTOR] Evolution — migrar históricos y extraer borradores (EV-AUD-002-007).md
blocking_findings: []
non_blocking_findings:
  - GIT_EVIDENCE_SESSION_SHELL
  - BRANCH_WORKTREE_SYNC
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

`persist_ref/_agent_handoff.md` **ausente al inicio** → copia del bloque **Runtime evidence (session)** Argos F2 (no stdout inventado esta sesión):

| Campo | Valor |
|-------|-------|
| `source` | `prosthesis_subprocess` |
| `TECH_FORMAL_EXECUTE_PROCESS` | **APTO** |
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | **APTO** |
| `notes` | (none) |
| `GIT_EVIDENCE_SESSION_SHELL` | **NO_APTO** — `./sddia-run.sh --tool git-manager` → Shell/Auto-review Rejected; sin `gitStdout` físico esta sesión Cerbero |
| `RBAC_AUTHORING_KM_POLICY` | **APTO** — Cerbero 0 writes bajo `docs/todos/**` |

## Ingesta

| Input | Resolución |
|-------|------------|
| `persist_ref` (inyectado) | vacío → **resuelto** `docs/features/evolution-history-normalization` |
| `pbi_ref` (inyectado) | vacío → **resuelto** `docs/todos/done/[REFACTOR] Evolution — migrar históricos y extraer borradores (EV-AUD-002-007).md` |
| `correlation_id` / Presented | `e8fb3a94-e9f2-443c-8547-c50aa091af20` |
| `document_id` | `7bb37ff1-decd-4ec5-968b-344a5334f9eb` |
| ECST `emitter_agent` | `delivery-close-cycle` |
| ECST `signer_identity_rbac` | `Vertice_Biologico_Relay` |
| `branch` (ECST) | `refactor/evolution-history-normalization` |
| `branch_name` (runtime) | `refactor/evolution-history-normalization` |
| `.git/HEAD` (FS) | `refs/heads/main` |
| ref local rama | `.git/refs/heads/refactor/evolution-history-normalization` → `aece128a…` (FS) |
| `pr_url` | `https://github.com/racso80es/SddIA/pull/173` |
| Evento Presented | `.events/processing/e8fb3a94-….json` · subscriber `argos.pull-request-review` · `state: processing` |
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
| `RBAC_SIGNER_VS_GENOME` | **APTO** | VBR × `tools/sddia-qa/` + `evolution/` + docs; cadena refactorization + `evolution-register` (`execution.md`) |
| `RBAC_AUTHORING_KM_POLICY` | **APTO** | Cerbero 0 writes `docs/todos/` |
| `F4_RBAC_GATE` | **APTO** | `exitCode: 0` · `PASS_F4_RBAC` |

## F2 — Triaje documental (heredado)

| Check | Estado | Evidencia |
|-------|--------|-----------|
| `F2_DOC_GATE` | **APTO** | Argos fase previa · `resolution: PASS_F2_DOC` |
| Cascada documental | **APTO** | objectives/clarify/spec/plan/implementation/execution + YAML |
| `DOC_EVOLUTION` | **APTO** | `SddIA/evolution/63062872-e707-496e-b1b3-1ea736e256f0.md` |

## PBI / Done path

| Check | Estado | Evidencia |
|-------|--------|-----------|
| `PBI_DONE_PRESENT` | **APTO** | `docs/todos/done/…EV-AUD-002-007.md` · `document_id: 7bb37ff1-…` · `status: done` |
| `PBI_PENDING_ABSENT` | **APTO** | sin `7bb37ff1-…` bajo `docs/todos/pending/` |
| `AC_DONE_PATH` | **APTO** | done exclusivo + `pbi_archived: true` |

Fuera de jurisdicción F4: PBI Kalma2 `1de0bdd1-…` en `docs/todos/pending/` (ciclo distinto). Cerbero **no** materializa KM.

## Git / rama

| Check | Estado | Evidencia |
|-------|--------|-----------|
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | **APTO** | copia session `prosthesis_subprocess` (R2; Argos F2) |
| `GIT_EVIDENCE_SESSION_SHELL` | **NO_APTO** | cápsula no invocable esta sesión; sin bypass raw |
| `BRANCH_RUNTIME_INJECT` | **APTO** | `branch_name` = `refactor/evolution-history-normalization` |
| `BRANCH_ECST_ALIGN` | **APTO** | ECST `payload.branch` = misma rama |
| `BRANCH_WORKTREE_SYNC` | **NO_APTO** | `.git/HEAD` → `refs/heads/main` (FS; **no** stdout git-manager) |
| `MERGE_ALREADY_OBSERVED` | **NO_APTO** | sin `PullRequest_Merged` para `e8fb3a94-…` |

`git_changes` por **inventario path-assert** heredado F2 (no `gitStdout` de esta sesión).

## Dictamen final

```json
{
  "phase": "Certificación RBAC",
  "verdict": "aprobado",
  "delivery_state": "pending_downstream_phases",
  "resolution": "PASS_F4_RBAC",
  "audit_event_reference": "e8fb3a94-e9f2-443c-8547-c50aa091af20",
  "authorization_status": {
    "exitCode": 0,
    "signer_identity_rbac": "Vertice_Biologico_Relay",
    "emitter_agent": "delivery-close-cycle"
  },
  "blocking_findings": [],
  "non_blocking_findings": [
    "GIT_EVIDENCE_SESSION_SHELL:NO_APTO",
    "BRANCH_WORKTREE_SYNC:NO_APTO",
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
E1/E2 APTO (VBR + DCC ∉ revoked); VBR×genoma APTO;
GIT_EVIDENCE_SESSION_SHELL NO_APTO no bloqueante; R2 APTO vía copia prosthesis_subprocess;
F3 pendiente; MERGE este CID NO_APTO; delivery_state pending_downstream_phases.
```
