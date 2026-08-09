---
feature_name: sddia-domain-abstract-03-relocalizacion
created: "2026-08-09"
updated: "2026-08-09"
process: pull-request-review
phase: Certificación RBAC
agent: cerbero
agents: cerbero
branch: feat/sddia-domain-abstract-03-relocalizacion
branch_name_injected: feat/sddia-domain-abstract-03-relocalizacion
persist_ref: docs/features/sddia-domain-abstract-03-relocalizacion
global: APTO
pbi_archived: true
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-03
pbi_ref: docs/todos/done/[REFACTOR] PBI-SDDIA-DOMAIN-ABSTRACT-03 — Relocalización física process software.md
correlation_id: "5uY26bMegPmGzFJJ1aewgUTyHYgYjLhFUSnfaiUVw1zn"
pr_url: https://github.com/racso80es/SddIA/pull/163
pr_presented_event_id: 5uY26bMegPmGzFJJ1aewgUTyHYgYjLhFUSnfaiUVw1zn
approval_status: aprobado
verdict: aprobado
delivery_state: pending_downstream_phases
resolution: PASS_F4_RBAC
laudo: L-PACK-MULTIROOT-SIX-MOVE
audit_event_reference: 5uY26bMegPmGzFJJ1aewgUTyHYgYjLhFUSnfaiUVw1zn
authorization_status:
  exitCode: 0
  signer_identity_rbac: Vertice_Biologico_Relay
  emitter_agent: github-bridge-watcher
  note: "PASS_F4_RBAC · VBR×genoma APTO · emitter github-bridge-watcher ∉ revoked · F3/Shell no bloqueantes"
git_manager_invoked: false
git_manager_error: "cápsula no invocable en esta sesión (Shell/Auto-review rejected sobre ./sddia-run.sh --tool git-manager); sin stdout físico; R2 = copia Evidence Bridge native_state; sin bypass raw"
git_evidence_source: evidence-bridge-native_state
formal_execute_process: true
evidence_bridge_notes: "R1/R2 copia Runtime evidence (machine) source=native_state notes=idempotent-hit-handoff (+ prior digest 0b652601…); Shell git-manager Rejected esta invocación Cerbero — sin stdout inventado"
shell_git_manager_session: "Rejected / no materializado — sin gitStdout en esta invocación Cerbero F4"
scope: "PPR Certificación RBAC — ABSTRACT-03 relocalización (PR #163 · ECST 5uY26b… · emitter github-bridge-watcher)"
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
  PERSIST_REF_RESOLVED: APTO
  BRANCH_RUNTIME_INJECT: APTO
  BRANCH_ECST_ALIGN: APTO
  BRANCH_WORKTREE_SYNC: APTO
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
  PBI_DONE_PRESENT: APTO
  PBI_PENDING_ABSENT: APTO
  AC_DONE_PATH: APTO
  PACKING_PROCESS_DIR: APTO
  CORE_SIX_ABSENT: APTO
  MERGE_ALREADY_OBSERVED: NO_APTO
git_changes:
  - SddIA/core/cumulo.paths.json
  - SddIA/engine/execute-process/src/core/paths.rs
  - SddIA/engine/execute-process/src/core/mod.rs
  - SddIA/engine/execute-process/src/core/resolver.rs
  - SddIA/engine/execute-process/src/engine/capability_di_reactor.rs
  - SddIA/engine/execute-process/src/engine/eda_coverage.rs
  - SddIA/engine/execute-process/src/engine/verify_process_integrity.rs
  - SddIA/engine/execute-process/src/engine/workspace.rs
  - SddIA/library/codexes/codex-software-engineering.md
  - SddIA/library/codexes/codex-software-engineering/process/
  - SddIA/norms/external-ai-constraints.md
  - SddIA/norms/pull-request-orchestration.md
  - SddIA/process/index.md
  - SddIA/evolution/7ade2a5f-be13-41ef-8b11-deb96fd58be3.md
  - docs/features/sddia-domain-abstract-03-relocalizacion/
  - docs/todos/done/[REFACTOR] PBI-SDDIA-DOMAIN-ABSTRACT-03 — Relocalización física process software.md
  - docs/todos/pending/[ARQUITECTURA] process-creator — jurisdicción process_domain_roots (ABSTRACT-03 D7).md
---

# Validación — Certificación RBAC (Cerbero · pull-request-review)

## Veredicto de fase

**APTO** — `F4_RBAC_GATE: APTO` · `resolution: PASS_F4_RBAC` · `authorization_status.exitCode: 0` · `verdict: aprobado`.

F5 (Veredicto/bloqueo), Cosecha y Handoff quedan **fuera** de esta fase → `delivery_state: pending_downstream_phases`.

| Gate | Delegado | Estado | Criterio |
|------|----------|--------|----------|
| F2 | Argos (doc) | **APTO** | heredado · `PASS_F2_DOC` |
| F3 | execute-process | **NO_APTO** | Triaje técnico no materializado este CID (no bloqueante para F4) |
| F4 | Cerbero | **APTO** | firmante/emisor × genoma · exitCode 0 |

## Ingesta

| Input | Resolución |
|-------|------------|
| `persist_ref` | `docs/features/sddia-domain-abstract-03-relocalizacion` |
| `pbi_ref` (inyectado) | vacío → **resuelto** `docs/todos/done/[REFACTOR] PBI-SDDIA-DOMAIN-ABSTRACT-03 — Relocalización física process software.md` |
| `correlation_id` / `event_id` | `5uY26bMegPmGzFJJ1aewgUTyHYgYjLhFUSnfaiUVw1zn` |
| ECST `emitter_agent` | `github-bridge-watcher` |
| ECST `origin_agent` | `jules` |
| ECST `signer_identity_rbac` | `Vertice_Biologico_Relay` |
| `branch` (ECST) | `feat/sddia-domain-abstract-03-relocalizacion` |
| `branch_name` (runtime) | `feat/sddia-domain-abstract-03-relocalizacion` |
| `pr_url` | `https://github.com/racso80es/SddIA/pull/163` |
| Evento Presented | `.events/processing/5uY26b….json` · `PullRequest_Presented` · subscriber `argos.pull-request-review` processing |
| Evento Merged (este ECST) | **ausente** |
| Evidence Bridge | `_agent_handoff.md` § Runtime evidence · `native_state` / `idempotent-hit-handoff` |
| F2 heredado | Triaje documental · `PASS_F2_DOC` · `F2_DOC_GATE: APTO` |
| Matriz RBAC | `directories.norms` → `SddIA/norms/execution-contexts.md` (accesible) |
| Revoked | `.SddIA/cerbero/revoked_entities.json` · keys: `bug-fix`, `emit-pr-audited-event`, `feature` |

## Aduana Evidence Bridge (R1 / R2 — copia machine)

Copia del veredicto machine (no stdout Shell de esta sesión Cerbero):

| Campo | Valor |
|-------|-------|
| `source` | `native_state` |
| `git_manager_invoked` | `true` (bridge / sesión previa) |
| `formal_execute_process` | `true` |
| `TECH_FORMAL_EXECUTE_PROCESS` | **APTO** |
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | **APTO** (canónico vía copia) |
| `notes` | `idempotent-hit-handoff` |
| prior digest | `0b6526015476a73a93d84273ee63c442` |

Sesión Cerbero F4: Shell `./sddia-run.sh --tool git-manager` **Rejected**. **No** se inventa stdout. `GIT_EVIDENCE_SESSION_SHELL: NO_APTO`; R2 canónico permanece **APTO** vía Evidence Bridge.

## F4 — Certificación RBAC

| Check | Estado | Evidencia |
|-------|--------|-----------|
| `RBAC_SPATIAL_INTEGRITY` | **APTO** | `directories.norms` = `SddIA/norms` · `execution-contexts.md` legible (FS) |
| `RBAC_SIGNER_PRESENT` / `ECST_SIGNER_PRESENT` | **APTO** | ECST `payload.signer_identity_rbac: Vertice_Biologico_Relay` |
| `RBAC_SIGNER_NOT_REVOKED` | **APTO** | `Vertice_Biologico_Relay` ∉ revoked |
| `RBAC_SIGNER_VS_GENOME` | **APTO** | VBR × áreas MVP: cumulo 1.6.0 + `process_domain_roots` · execute-process (resolver multi-root) · norms · packing códice 6 process · `process/index.md` · evolution · docs feature/PBI |
| `RBAC_EMITTER_AUTHORIZED` | **APTO** | `github-bridge-watcher` · `context: source-control` · jurisdicción oráculo PR→ECST |
| `RBAC_EMITTER_NOT_REVOKED` | **APTO** | `github-bridge-watcher` ∉ revoked |
| `RBAC_PROCESS_REGISTRY` | **APTO** | `pull-request-review` ∉ revoked · packing path-assert |
| `RBAC_AUTHORING_KM_POLICY` | **APTO** | Cerbero **sin** write bajo `docs/todos/`; seed D7 = autoría legítima `agent:cumulo` (Cosecha previa CID hermano) |
| `F4_RBAC_GATE` | **APTO** | peaje binario `exitCode: 0` · `PASS_F4_RBAC` |

## Path-assert packing / PBI (contexto F4)

| Check | Estado | Evidencia |
|-------|--------|-----------|
| `PACKING_PROCESS_DIR` | **APTO** | 6× `.md` + `index.md` bajo `…/codex-software-engineering/process/` |
| `CORE_SIX_ABSENT` | **APTO** | sin los 6 process en `SddIA/process/` |
| `PBI_DONE_PRESENT` | **APTO** | PBI ABSTRACT-03 en `done/` |
| `PBI_PENDING_ABSENT` | **APTO** | sin PBI ABSTRACT-03 bajo `pending/` (solo seed D7) |
| `AC_DONE_PATH` | **APTO** | done exclusivo + `pbi_archived: true` |

## Git / rama

| Check | Estado | Evidencia |
|-------|--------|-----------|
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | **APTO** | Evidence Bridge `native_state` (copia machine) |
| `GIT_EVIDENCE_SESSION_SHELL` | **NO_APTO** | sin `gitStdout` esta sesión |
| `BRANCH_RUNTIME_INJECT` | **APTO** | `feat/sddia-domain-abstract-03-relocalizacion` |
| `BRANCH_ECST_ALIGN` | **APTO** | ECST `payload.branch` = misma rama |
| `BRANCH_WORKTREE_SYNC` | **APTO** | `.git/HEAD` → `refs/heads/feat/sddia-domain-abstract-03-relocalizacion` (FS; **no** stdout git-manager) |
| `MERGE_ALREADY_OBSERVED` | **NO_APTO** | sin `PullRequest_Merged` para `5uY26b…` |

`git_changes` por inventario path-assert heredado F2 (no por stdout `git-manager`).

## Dictamen final

```json
{
  "phase": "Certificación RBAC",
  "verdict": "aprobado",
  "global": "APTO",
  "delivery_state": "pending_downstream_phases",
  "resolution": "PASS_F4_RBAC",
  "audit_event_reference": "5uY26bMegPmGzFJJ1aewgUTyHYgYjLhFUSnfaiUVw1zn",
  "authorization_status": {
    "exitCode": 0,
    "signer_identity_rbac": "Vertice_Biologico_Relay",
    "emitter_agent": "github-bridge-watcher"
  },
  "blocking_findings": [],
  "non_blocking_findings": [
    "F3_TECH_GATE:NO_APTO",
    "GIT_EVIDENCE_SESSION_SHELL:NO_APTO",
    "MERGE_ALREADY_OBSERVED:NO_APTO",
    "Shell git-manager Rejected (R2 copiado Evidence Bridge)"
  ]
}
```

## Jurisdicción de fase

Cubre **Certificación RBAC** (F4). No sintetiza F5 ni materializa Kaizen. Cerbero **no** escribe bajo `docs/todos/` (Read & Block). Downstream: Veredicto y bloqueo → Cosecha → Handoff (`accept-pr`; sin merge directo en aduana).

## approval_status

```text
aprobado — PASS_F4_RBAC; exitCode 0;
firmante Vertice_Biologico_Relay + emisor github-bridge-watcher autorizados ∉ revoked;
VBR×genoma (cumulo+resolver+norms+packing+evolution+docs) APTO;
Evidence Bridge R1/R2 APTO (copia native_state); Shell git-manager Rejected (sin inventar);
delivery_state pending_downstream_phases; PR #163 / correlation 5uY26b…
```
