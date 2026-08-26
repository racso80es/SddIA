---
feature_name: kaizen-aislamiento-multi-instancia
created: "2026-08-26"
updated: "2026-08-26T08:30:00Z"
process: pull-request-review
phase: Cosecha Kaizen
agent: cumulo
agents: cumulo
branch: feat/kaizen-aislamiento-multi-instancia
branch_name: feat/kaizen-aislamiento-multi-instancia
branch_name_injected: feat/kaizen-aislamiento-multi-instancia
persist_ref: docs/features/kaizen-aislamiento-multi-instancia
pbi_ref: docs/todos/done/[KAIZEN] aislamiento multi-instancia centinelas.md
document_id: PBI-KAIZEN-AISLAMIENTO-MULTI-INSTANCIA
uuid: "b5d19318-a0fd-440b-9aac-8c6d93f775ed"
correlation_id: d994ca73-e566-4955-bfe0-dc11678c7e87
pr_presented_event_id: d994ca73-e566-4955-bfe0-dc11678c7e87
audit_event_reference: d994ca73-e566-4955-bfe0-dc11678c7e87
pr_url: https://github.com/racso80es/SddIA/pull/193
execution_id: "3b40b62c-d048-4896-b8c1-1ee267ca7704"
evolution_id: "7e3c1a90-4b2d-4f8a-9c1e-6a0b2c8d4e1f"
merge_event_id: "3555239d-394f-4421-ba93-8a8c0bf426b9"
merge_commit: "fb12e07673cede2c48744120b53058e3b92a57e0"
global: APTO
pbi_archived: true
approval_status: aprobado
verdict: aprobado
delivery_state: success
accept_pr_handoff: false
resolution: KAIZEN_COSECHA_GATE
kaizen_seeds: 0
kaizen_seeds_dedup: 3
scope: "PPR Cosecha Kaizen — kaizen-aislamiento-multi-instancia (PR #193 · ECST d994ca73…)"
authorization_status:
  exitCode: 0
  signer_identity_rbac: Vertice_Biologico_Relay
  emitter_agent: delivery-close-cycle
  note: "KAIZEN_COSECHA_GATE APTO · kaizen_seeds 0 · dedup 3 (#190 PPR permanent+revoked + #186 refactorization + #136 Shell/F3) · F5 heredado APTO · accept_pr_handoff false (Merged 3555239d) · Shell git-manager Rejected — sin stdout inventado · Cúmulo 0 create docs/todos/** (solo sighting #190)"
git_manager_invoked: false
git_manager_error: "cápsula no invocable en esta sesión Cúmulo (Shell Rejected sobre ./sddia-run.sh --tool git-manager); R2 = copia Evidence Bridge native_state; sin bypass raw"
git_evidence_source: native_state-evidence-bridge
formal_execute_process: true
handoff_machine_file: present
evidence_bridge_notes: "R1/R2 copia Runtime evidence (Argos F5 CID d994ca73…) source=native_state notes=idempotent-hit TECH_FORMAL=APTO GIT_EVIDENCE=APTO; machine heredado prosthesis_subprocess @ 2026-08-26T06:21:52Z formal_evidence_detail=verify-process-integrity: OK; Shell git-manager Rejected esta sesión Cúmulo Cosecha — sin stdout inventado"
shell_git_manager_session: "Rejected — sin gitStdout físico esta invocación Cúmulo Cosecha CID d994ca73-e566-4955-bfe0-dc11678c7e87"
revoked_entity_alert: "pull-request-review (permanent+revoked) — dedup pending PPR #190 + sighting PR #193; refactorization (revoked since 2026-08-20T05:48:56Z) — dedup done #186"
checks:
  KAIZEN_COSECHA_GATE: APTO
  KAIZEN_SEEDS_MATERIALIZED: APTO
  KAIZEN_DEDUP: APTO
  DIA_KAIZEN_ALERT_ABSENT: APTO
  KAIZEN_DIA_ALERT: APTO
  KAIZEN_SEED_PPR_REVOKED_REGISTRY: APTO
  KAIZEN_SEED_REFACTORIZATION_REVOKED_REGISTRY: APTO
  KAIZEN_SEED_SHELL_GIT_MANAGER: APTO
  CUMULO_KM_AUTHORITY: APTO
  F5_VERDICT_GATE: APTO
  F2_DOC_GATE: APTO
  F3_TECH_GATE: NO_APTO
  F4_RBAC_GATE: APTO
  TECH_FORMAL_EXECUTE_PROCESS: APTO
  GIT_EVIDENCE_VIA_GIT_MANAGER: APTO
  GIT_EVIDENCE_SESSION_SHELL: NO_APTO
  RBAC_AUTHORING_KM_POLICY: APTO
  RBAC_PROCESS_REGISTRY: NO_APTO
  RBAC_FEATURE_REGISTRY: APTO
  RBAC_CERBERO_CERT: APTO
  BRANCH_WORKTREE_SYNC: APTO
  MERGE_ALREADY_OBSERVED: APTO
  ACCEPT_PR_HANDOFF: APTO
  PERSIST_REF_RESOLVED: APTO
  HANDOFF_MACHINE_FILE: APTO
  HANDOFF_EVIDENCE_BLOCK: APTO
  PBI_DONE_PRESENT: APTO
  PBI_PENDING_ABSENT: APTO
  DOC_EVOLUTION: APTO
  FEATURE_AC_RESIDUAL_AP_TREE: NO_APTO
  branch: APTO
  git_changes: APTO
kaizen_seeds_paths: []
kaizen_seeds_dedup_paths:
  - docs/todos/pending/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #190).md
  - docs/todos/done/[ARQUITECTURA] refactorization — rehabilitación revoked_entities (PPR #186).md
  - docs/todos/done/[OPERATIVO] Kalma2-agent-runtime-cursor — F3 git-manager KM residual (PPR #136).md
git_changes:
  - SddIA/templates/systemd/sddia-daemon@.service.template
  - SddIA/templates/systemd/sddia-email-watcher@.service.template
  - SddIA/engine/execute-process/src/engine/handlers/instance_creator.rs
  - SddIA/scripts/common/sddia_shell_lib.sh
  - SddIA/scripts/daemons/_run_daemon.sh
  - start-sddia.sh
  - SddIA/process/instance-creator.md
  - SddIA/norms/sddia-distribution-protocol.md
  - docs/features/kaizen-aislamiento-multi-instancia/
  - docs/todos/done/[KAIZEN] aislamiento multi-instancia centinelas.md
  - SddIA/evolution/7e3c1a90-4b2d-4f8a-9c1e-6a0b2c8d4e1f.md
  - SddIA/evolution/Evolution_log.md
blocking_findings: []
non_blocking_findings:
  - GIT_EVIDENCE_SESSION_SHELL
  - F3_TECH_GATE
  - RBAC_PROCESS_REGISTRY
  - REVOKED_ENTITY_ALERT_PULL_REQUEST_REVIEW
  - REVOKED_ENTITY_ALERT_REFACTORIZATION
  - FEATURE_AC_RESIDUAL_AP_TREE
situational_notes:
  - "pull-request-review ∈ permanent+revoked — dedup pending #190 + sighting CID d994ca73… / PR #193; 0 create"
  - "refactorization ∈ revoked since 2026-08-20T05:48:56Z — dedup done #186; sighting lateral"
  - "GIT_EVIDENCE_SESSION_SHELL / F3_TECH_GATE → dedup done PPR #136 (sin create)"
  - "MERGE_ALREADY_OBSERVED APTO → accept_pr_handoff false (sin re-handoff)"
  - "FEATURE_AC_RESIDUAL_AP_TREE residual lab — no seed (no deuda genérica nueva)"
  - "DIA: sin Kaizen_Alert_Required para CID d994ca73… en .events/pending (solo PullRequest_Presented)"
  - "Cúmulo 0 create docs/todos/** esta fase (solo update sighting #190)"
  - "Shell ./sddia-run.sh --tool git-manager → Rejected; R2 = copia Evidence Bridge (no inventar gitStdout)"
---

# Validación — Cosecha Kaizen (Cúmulo · pull-request-review)

## Veredicto de fase

**APTO** — `resolution: KAIZEN_COSECHA_GATE` · `kaizen_seeds: 0` · `kaizen_seeds_dedup: 3` · `delivery_state: success` (heredado F5) · `accept_pr_handoff: false`.

| Gate | Estado | Criterio |
|------|--------|----------|
| F5 (heredado) | **APTO** | `PASS_F5_VERDICT` · CID `d994ca73…` |
| Cosecha | **APTO** | 0 seed nueva + 3 dedup; sin DIA alert |
| KM RBAC | **APTO** | Cúmulo 0 create `docs/todos/` (solo sighting #190) |
| Merge | **APTO** | Merged `3555239d…` ↔ CID → `accept_pr_handoff: false` |

## Evidence Bridge (R1 / R2 / R3)

Copia literal machine/session — **no** stdout Shell inventado:

| Campo | Valor |
|-------|-------|
| `source` | `native_state` (Argos F5 CID `d994ca73…`) |
| `notes` | `idempotent-hit` |
| `git_manager_invoked` | `false` (sesión Cúmulo Cosecha) · `true` (bridge native_state / prótesis F2) |
| `formal_execute_process` | `true` |
| `TECH_FORMAL_EXECUTE_PROCESS` | **APTO** |
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | **APTO** |
| `formal_evidence_detail` (heredado) | `verify-process-integrity: OK` · `prosthesis_subprocess` @ `2026-08-26T06:21:52Z` |
| `GIT_EVIDENCE_SESSION_SHELL` | **NO_APTO** — `./sddia-run.sh --tool git-manager` → Shell Rejected; sin `gitStdout` físico esta sesión |
| `RBAC_AUTHORING_KM_POLICY` / `CUMULO_KM_AUTHORITY` | **APTO** — Cúmulo 0 create bajo `docs/todos/**`; update sighting #190 autorizado |

Bloque machine de referencia: `_agent_handoff.md` § Runtime evidence (machine) Argos F5 + session `native_state` / `idempotent-hit`.

## Cosecha — inventario de deuda

| Hallazgo (F5/F4) | Acción Cúmulo | Destino |
|------------------|---------------|---------|
| `REVOKED_ENTITY_ALERT_PULL_REQUEST_REVIEW` / `RBAC_PROCESS_REGISTRY` | **dedup** + sighting | pending `[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #190)` |
| `REVOKED_ENTITY_ALERT_REFACTORIZATION` | **dedup** | done `[ARQUITECTURA] refactorization — rehabilitación revoked_entities (PPR #186)` |
| `GIT_EVIDENCE_SESSION_SHELL` / `F3_TECH_GATE` | **dedup** | done `[OPERATIVO] Kalma2-agent-runtime-cursor — F3 git-manager KM residual (PPR #136)` |
| `MERGE_ALREADY_OBSERVED` | no seed | Merged presente → `accept_pr_handoff: false` |
| `FEATURE_AC_RESIDUAL_AP_TREE` | no seed | residual lab; no deuda genérica nueva |

**DIA:** sin evento `Kaizen_Alert_Required` en `.events/pending` para CID `d994ca73…` (solo `PullRequest_Presented`) → sin `PENDING_AUDIT_DOC_*` nuevo.

**FS Cerbero (lectura empírica):** `pull-request-review` ∈ `permanent` (`max_recovery_attempts_exceeded` since `2026-08-25T16:25:55Z`) + `revoked` (`abrupt_success_rate_drop` since `2026-08-25T17:24:18Z`); `refactorization` ∈ `revoked` since `2026-08-20T05:48:56Z`.

**Semillas nuevas materializadas esta fase:** `0`.

## Ingesta

| Input | Resolución |
|-------|------------|
| `persist_ref` | `docs/features/kaizen-aislamiento-multi-instancia` |
| `pbi_ref` | `docs/todos/done/[KAIZEN] aislamiento multi-instancia centinelas.md` |
| `correlation_id` / ECST Presented | `d994ca73-e566-4955-bfe0-dc11678c7e87` |
| `document_id` | `PBI-KAIZEN-AISLAMIENTO-MULTI-INSTANCIA` |
| ECST `emitter_agent` | `delivery-close-cycle` |
| ECST `signer_identity_rbac` | `Vertice_Biologico_Relay` |
| `pr_url` | `https://github.com/racso80es/SddIA/pull/193` |
| F5 heredado | `verdict: aprobado` · `delivery_state: success` · `PASS_F5_VERDICT` |
| `.git/HEAD` (FS) | `refs/heads/main` (post-merge; coherente) |
| Evento Merged | `.events/dead-letter/3555239d-….json` · `fb12e076…` |
| Evolution | `SddIA/evolution/7e3c1a90-4b2d-4f8a-9c1e-6a0b2c8d4e1f.md` presente |

## Dictamen final

```json
{
  "phase": "Cosecha Kaizen",
  "verdict": "aprobado",
  "delivery_state": "success",
  "accept_pr_handoff": false,
  "resolution": "KAIZEN_COSECHA_GATE",
  "kaizen_seeds": 0,
  "kaizen_seeds_dedup": 3,
  "audit_event_reference": "d994ca73-e566-4955-bfe0-dc11678c7e87",
  "merge_event_id": "3555239d-394f-4421-ba93-8a8c0bf426b9",
  "merge_commit": "fb12e07673cede2c48744120b53058e3b92a57e0",
  "authorization_status": {
    "exitCode": 0,
    "signer_identity_rbac": "Vertice_Biologico_Relay",
    "emitter_agent": "delivery-close-cycle"
  },
  "blocking_findings": [],
  "non_blocking_findings": [
    "GIT_EVIDENCE_SESSION_SHELL:NO_APTO",
    "F3_TECH_GATE:NO_APTO",
    "RBAC_PROCESS_REGISTRY:NO_APTO:dedup_PPR_190",
    "REVOKED_ENTITY_ALERT_REFACTORIZATION:dedup_PPR_186",
    "FEATURE_AC_RESIDUAL_AP_TREE:NO_APTO"
  ]
}
```

## Jurisdicción de fase

Cubre **Cosecha Kaizen**. Downstream: Handoff materialización **omitido** (`accept_pr_handoff: false` — `PullRequest_Merged` ya materializado; sin re-merge). Cúmulo materializa KM solo aquí o vía `Kaizen_Alert_Required`.

## approval_status

```text
aprobado — KAIZEN_COSECHA_GATE · kaizen_seeds 0 · dedup 3 (#190 PPR + #186 refactorization + #136 Shell/F3);
F5 heredado APTO · accept_pr_handoff false (Merged 3555239d); PBI archivado done/;
R1/R2 APTO vía Evidence Bridge native_state; GIT_EVIDENCE_SESSION_SHELL NO_APTO (Shell Rejected; sin stdout inventado);
DIA alert ausente; Cúmulo 0 create docs/todos/** (sighting #190); CID d994ca73….
```
