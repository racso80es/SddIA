---
feature_name: radamanto-process-threshold-rehab
created: "2026-08-16"
updated: "2026-08-16T16:46:00Z"
process: pull-request-review
phase: Certificación RBAC
agent: cerbero
agents: cerbero
branch: refactor/radamanto-process-threshold-rehab
branch_name_injected: refactor/radamanto-process-threshold-rehab
persist_ref: docs/features/radamanto-process-threshold-rehab
pbi_ref: docs/todos/done/[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177).md
document_id: PBI-PPR-174-177-REVOKED-PROCESS-THRESHOLDS
document_ids:
  - PBI-PPR-174-177-REVOKED-PROCESS-THRESHOLDS
  - PBI-PPR-174-REVOKED-REGISTRY
  - PBI-PPR-177-DCC-REVOKED-REGISTRY
uuid: ba900e95-1a47-4185-b86c-bc7a251b4fe6
correlation_id: DnqKMKqD6RKdskM3kY2uJCidx7QJWdWP1jpSpgStPz8V
pr_presented_event_id: DnqKMKqD6RKdskM3kY2uJCidx7QJWdWP1jpSpgStPz8V
audit_event_reference: DnqKMKqD6RKdskM3kY2uJCidx7QJWdWP1jpSpgStPz8V
sibling_pr_presented_event_id: ca0f4b61-b48f-4b18-a669-79856cf133db
sibling_merge_event_id: 314d155b-5a4e-48b6-8180-6a528ce673c8
sibling_merge_commit_hash: 571c0850cf6ce2497d5fa825a85f63a57cb94e94
pr_url: https://github.com/racso80es/SddIA/pull/179
global: APTO
pbi_archived: true
approval_status: pendiente_veredicto
verdict: pendiente
delivery_state: pending_downstream_phases
accept_pr_handoff: false
resolution: PASS_F4_RBAC
authorization_status:
  exitCode: 0
  signer_identity_rbac: Vertice_Biologico_Relay
  emitter_agent: github-bridge-watcher
  note: "PASS_F4_RBAC · VBR×engine/agents-json+docs+evolution APTO · GBW∉revoked · PPR∉revoked · sibling merge ca0f4b61 → handoff false · Shell git-manager Rejected — sin stdout inventado"
git_manager_invoked: false
git_manager_error: "cápsula no invocable en esta sesión Cerbero (Shell Rejected sobre ./sddia-run.sh --tool git-manager); sin Evidence Bridge machine PPR previo este CID; path-assert FS/EDA; sin bypass raw"
git_evidence_source: path-assert-fs-and-eda
formal_execute_process: false
handoff_machine_file: absent_for_cid
shell_git_manager_session: "Rejected — sin gitStdout físico esta invocación Cerbero"
scope: "PPR Certificación RBAC — radamanto-process-threshold-rehab (PR #179 · ECST DnqKMKqD…)"
checks:
  F4_RBAC_GATE: APTO
  RBAC_PROCESS_REGISTRY: APTO
  RBAC_EMITTER_NOT_REVOKED: APTO
  RBAC_AUTHORING_KM_POLICY: APTO
  TECH_FORMAL_EXECUTE_PROCESS: NO_APTO
  GIT_EVIDENCE_VIA_GIT_MANAGER: NO_APTO
  GIT_EVIDENCE_SESSION_SHELL: NO_APTO
  BRANCH_WORKTREE_SYNC: NO_APTO
  MERGE_ALREADY_OBSERVED: NO_APTO
  SIBLING_MERGE_SAME_BRANCH: APTO
  ACCEPT_PR_HANDOFF: NO_APTO
  PERSIST_REF_RESOLVED: APTO
  PBI_DONE_PRESENT: APTO
  F2_DOC_GATE: pendiente
  F3_TECH_GATE: NO_APTO
git_changes:
  - SddIA/agents/radamanto.thresholds.json
  - SddIA/agents/radamanto.instructions.json
  - SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs
  - SddIA/engine/execute-process/src/engine/fractal_bus.rs
  - SddIA/engine/execute-process/src/engine/delivery_close.rs
  - SddIA/engine/execute-process/src/engine/residual_runner.rs
  - SddIA/engine/execute-process/src/engine/pull_request_review.rs
  - SddIA/engine/execute-process/src/engine/agent_runtime.rs
  - SddIA/evolution/ef2b0ef2-b792-4cb7-ac1b-bfea203f4bde.md
  - SddIA/evolution/Evolution_log.md
  - docs/features/radamanto-process-threshold-rehab/
  - docs/todos/done/[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177).md
  - docs/todos/done/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #174).md
  - docs/todos/done/[ARQUITECTURA] delivery-close-cycle — rehabilitación revoked_entities (PPR #177).md
blocking_findings: []
non_blocking_findings:
  - GIT_EVIDENCE_SESSION_SHELL
  - GIT_EVIDENCE_VIA_GIT_MANAGER
  - TECH_FORMAL_EXECUTE_PROCESS
  - BRANCH_WORKTREE_SYNC
  - MERGE_ALREADY_OBSERVED
  - ACCEPT_PR_HANDOFF
  - F2_DOC_GATE
  - F3_TECH_GATE
situational_notes:
  - "delivery-close-cycle ∈ revoked since 2026-08-16T16:40:55Z (success_rate_below_threshold) — no aplica a E1 este CID (emisor=github-bridge-watcher); Cerbero 0 writes KM"
  - "pull-request-review ∉ revoked · status healthy · rehab_laudo PBI-PPR-174-177-REVOKED-PROCESS-THRESHOLDS"
---

# Validación — Certificación RBAC (Cerbero · pull-request-review)

## Veredicto de fase

**APTO** — `resolution: PASS_F4_RBAC` · `exitCode: 0` · `F4_RBAC_GATE: APTO` · `delivery_state: pending_downstream_phases`.

| Gate | Estado | Criterio |
|------|--------|----------|
| F4 RBAC | **APTO** | Firmante `Vertice_Biologico_Relay` × área genoma del diff |
| PROCESS_REGISTRY | **APTO** | `pull-request-review` ∉ `.SddIA/cerbero/revoked_entities.json` |
| EMITTER | **APTO** | `github-bridge-watcher` ∉ revoked |
| KM autoría | **APTO** | Cerbero 0 writes `docs/todos/**` |

## ECST / registro

| Campo | Valor |
|-------|-------|
| CID / Presented | `DnqKMKqD6RKdskM3kY2uJCidx7QJWdWP1jpSpgStPz8V` |
| Firmante | `Vertice_Biologico_Relay` |
| Emisor | `github-bridge-watcher` |
| PR | https://github.com/racso80es/SddIA/pull/179 |
| Rama | `refactor/radamanto-process-threshold-rehab` |
| Sibling Presented | `ca0f4b61-b48f-4b18-a669-79856cf133db` (emisor `delivery-close-cycle`) |
| Sibling Merged | `314d155b-…` · hash `571c0850…` · correlation `ca0f4b61…` |
| `.git/HEAD` (FS) | `refs/heads/main` |
| Ref local rama PR | **ausente** |

## VBR × genoma

Área afectada (path-assert / cascada): `SddIA/engine/execute-process/src/engine/*`, `SddIA/agents/radamanto.*.json`, `SddIA/evolution/`, `docs/features/…`, `docs/todos/done/`. **Sin** mutación DA-2 forja (`tools/` / `skills/` / `actions/` / `process/` / `agents/*.md` / `norms/` / `library/`).

## Evidence Bridge (R1 / R2)

| Campo | Valor |
|-------|-------|
| `source` | `path-assert-fs-and-eda` |
| `git_manager_invoked` | `false` (sesión) |
| `formal_execute_process` | `false` |
| `TECH_FORMAL_EXECUTE_PROCESS` | **NO_APTO** |
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | **NO_APTO** — sin bloque machine PPR previo este CID |
| `GIT_EVIDENCE_SESSION_SHELL` | **NO_APTO** — `./sddia-run.sh --tool git-manager` → Shell Rejected; sin `gitStdout` inventado |

## Situacional (no bloqueante F4)

- `BRANCH_WORKTREE_SYNC` NO_APTO — worktree `main`; ref local rama PR ausente.
- `MERGE_ALREADY_OBSERVED` NO_APTO este CID; `SIBLING_MERGE_SAME_BRANCH` APTO → `accept_pr_handoff: false`.
- `F2_DOC_GATE` pendiente (sin handoff Argos Triaje este CID; cascada FS presente).
- `F3_TECH_GATE` NO_APTO (sin evidencia formal execute-process este CID).
- Instancia: `delivery-close-cycle` re-revocado `since 2026-08-16T16:40:55Z` — fuera de E1 este ECST; Cúmulo/Kaizen downstream.

## Dictamen

```json
{
  "phase": "Certificación RBAC",
  "resolution": "PASS_F4_RBAC",
  "exitCode": 0,
  "F4_RBAC_GATE": "APTO",
  "delivery_state": "pending_downstream_phases",
  "accept_pr_handoff": false,
  "audit_event_reference": "DnqKMKqD6RKdskM3kY2uJCidx7QJWdWP1jpSpgStPz8V",
  "blocking_findings": [],
  "non_blocking_findings": [
    "GIT_EVIDENCE_SESSION_SHELL:NO_APTO",
    "GIT_EVIDENCE_VIA_GIT_MANAGER:NO_APTO",
    "TECH_FORMAL_EXECUTE_PROCESS:NO_APTO",
    "BRANCH_WORKTREE_SYNC:NO_APTO",
    "MERGE_ALREADY_OBSERVED:NO_APTO",
    "ACCEPT_PR_HANDOFF:NO_APTO",
    "F2_DOC_GATE:pendiente",
    "F3_TECH_GATE:NO_APTO"
  ]
}
```

## Jurisdicción de fase

Cubre **Certificación RBAC**. Downstream: Veredicto y bloqueo (Argos) → Cosecha Kaizen (Cúmulo) → Handoff (no re-merge; sibling ya materializado).
