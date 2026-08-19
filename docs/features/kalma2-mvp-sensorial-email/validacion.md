---
feature_name: kalma2-mvp-sensorial-email
created: "2026-08-18"
updated: "2026-08-18T17:52:00Z"
process: pull-request-review
phase: Triaje documental
agent: argos
agents: argos
branch: feat/kalma2-mvp-sensorial-email
branch_name: feat/kalma2-mvp-sensorial-email
branch_name_injected: feat/kalma2-mvp-sensorial-email
persist_ref: docs/features/kalma2-mvp-sensorial-email
pbi_ref: docs/todos/done/[OPERATIVO] Kalma2 MVP 01A — Circuito sensorial de correo (Paciente 0).md
document_id: PBI-KALMA2-MVP-01A
uuid: "c209c150-8ab4-4f0d-bcf7-8fa7a6101de0"
correlation_id: 2XyNciPL7yiQuKGFY77qJASEEBjTP572gFt1VjK2HQVY
pr_presented_event_id: 2XyNciPL7yiQuKGFY77qJASEEBjTP572gFt1VjK2HQVY
audit_event_reference: 2XyNciPL7yiQuKGFY77qJASEEBjTP572gFt1VjK2HQVY
pr_url: https://github.com/racso80es/SddIA/pull/182
execution_id: "fa4dde03-a0ec-426f-ade7-850246ba7575"
global: APTO
pbi_archived: true
approval_status: aprobado
verdict: aprobado
delivery_state: pending_downstream_phases
resolution: PASS_F2_DOC
git_manager_invoked: false
git_manager_error: "cápsula no invocable en esta sesión Argos (Shell Rejected sobre ./sddia-run.sh --tool git-manager); sin stdout físico; R2 = copia Evidence Bridge prosthesis_subprocess; sin bypass raw"
git_evidence_source: prosthesis_subprocess-evidence-bridge
formal_execute_process: true
handoff_machine_file: present
evidence_bridge_notes: "R1/R2 copia Runtime evidence (machine) @ 2026-08-18T17:50:33Z source=prosthesis_subprocess formal_evidence_detail=verify-process-integrity: OK; TECH_FORMAL_* / GIT_EVIDENCE_VIA_GIT_MANAGER APTO; Shell git-manager Rejected esta sesión Argos F2 — sin stdout inventado"
shell_git_manager_session: "Rejected — sin gitStdout físico esta invocación Argos Triaje documental CID 2XyNciPL…"
checks:
  F2_DOC_GATE: APTO
  DOC_OBJECTIVES: APTO
  DOC_CLARIFY: APTO
  DOC_SPEC: APTO
  DOC_PLAN: APTO
  DOC_IMPLEMENTATION: APTO
  DOC_EXECUTION: APTO
  DOC_FRONTMATTER_YAML: APTO
  DOC_EVOLUTION: NO_APTO
  PERSIST_REF_RESOLVED: APTO
  HANDOFF_MACHINE_FILE: APTO
  HANDOFF_EVIDENCE_BLOCK: APTO
  BRANCH_RUNTIME_INJECT: APTO
  BRANCH_ECST_ALIGN: APTO
  BRANCH_WORKTREE_SYNC: APTO
  TECH_FORMAL_EXECUTE_PROCESS: APTO
  GIT_EVIDENCE_VIA_GIT_MANAGER: APTO
  GIT_EVIDENCE_SESSION_SHELL: NO_APTO
  RBAC_AUTHORING_KM_POLICY: APTO
  PBI_DONE_PRESENT: APTO
  PBI_PENDING_ABSENT: APTO
  AC_DONE_PATH: APTO
  MERGE_ALREADY_OBSERVED: NO_APTO
git_changes:
  - docs/features/kalma2-mvp-sensorial-email/
  - docs/todos/done/[OPERATIVO] Kalma2 MVP 01A — Circuito sensorial de correo (Paciente 0).md
  - SddIA/library/codexes/codex-contract.md
  - SddIA/core/cumulo.paths.json
  - SddIA/core/event-domain-subscriptions.json
  - SddIA/core/capability-bindings.md
  - SddIA/library/norms/email-triage-matrix.md
  - SddIA/library/norms/index.md
  - SddIA/library/codexes/codex-kalma2-assistant.md
  - SddIA/library/codexes/codex-kalma2-assistant/process/email-triage-gateway.md
  - SddIA/library/codexes/codex-kalma2-assistant/process/index.md
  - SddIA/library/codexes/index.md
  - SddIA/events/domain/email-received.md
  - SddIA/events/domain/email-triaged.md
  - SddIA/daemons/email-watcher.md
  - SddIA/daemons/email-watcher/
  - SddIA/daemons/index.md
  - SddIA/daemons/email-watcher.sh
  - SddIA/scripts/daemons/email-watcher.sh
  - SddIA/templates/systemd/sddia-email-watcher@.service.template
  - SddIA/templates/index.md
  - SddIA/skills/agenda-manager.md
  - SddIA/engine/execute-process/src/engine/handlers/email_triage.rs
  - start-sddia.sh
  - start-sddia.md
blocking_findings: []
non_blocking_findings:
  - GIT_EVIDENCE_SESSION_SHELL
  - DOC_EVOLUTION
  - MERGE_ALREADY_OBSERVED
---

# Validación — Triaje documental (Argos · pull-request-review)

## Veredicto de fase

**APTO** — `resolution: PASS_F2_DOC` · `F2_DOC_GATE: APTO` · `verdict: aprobado`.  
F3 (triaje técnico), F4 (Cerbero), Veredicto/bloqueo, Cosecha y Handoff quedan **fuera** de esta fase → `delivery_state: pending_downstream_phases`.

| Gate | Delegado | Estado | Criterio |
|------|----------|--------|----------|
| F2 | Argos (doc) | **APTO** | Frontmatter YAML + `objectives`/`spec`/`plan`/`implementation` (+ clarify/execution) |
| F3 | execute-process | **pendiente** | fuera de jurisdicción Triaje documental |
| F4 | Cerbero | **pendiente** | fuera de jurisdicción Triaje documental |

## Evidence Bridge (R1 / R2)

Copia literal machine/session — **no** stdout Shell inventado:

| Campo | Valor |
|-------|-------|
| `source` | `prosthesis_subprocess` |
| `git_manager_invoked` | `true` (bridge / prótesis) |
| `formal_execute_process` | `true` |
| `TECH_FORMAL_EXECUTE_PROCESS` | **APTO** |
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | **APTO** |
| `formal_evidence_detail` | `verify-process-integrity: OK` |
| `materialized_at` | `2026-08-18T17:50:33Z` |
| `GIT_EVIDENCE_SESSION_SHELL` | **NO_APTO** — `./sddia-run.sh --tool git-manager` → Shell Rejected; sin `gitStdout` físico esta sesión Argos |

Bloque machine: `_agent_handoff.md` § Runtime evidence (machine) @ `2026-08-18T17:50:33Z`.

## Ingesta

| Input | Resolución |
|-------|------------|
| `persist_ref` | `docs/features/kalma2-mvp-sensorial-email` |
| `pbi_ref` (inyectado) | vacío → **resuelto** `docs/todos/done/[OPERATIVO] Kalma2 MVP 01A — Circuito sensorial de correo (Paciente 0).md` |
| `correlation_id` / Presented | `2XyNciPL7yiQuKGFY77qJASEEBjTP572gFt1VjK2HQVY` |
| ECST `emitter_agent` | `github-bridge-watcher` |
| ECST `origin_agent` | `jules` |
| ECST `signer_identity_rbac` | `Vertice_Biologico_Relay` |
| `branch` (ECST) | `feat/kalma2-mvp-sensorial-email` |
| `branch_name` (runtime) | `feat/kalma2-mvp-sensorial-email` |
| `pr_url` | `https://github.com/racso80es/SddIA/pull/182` |
| Evento Presented | `.events/processing/2XyNciPL….json` · subscriber `argos.pull-request-review` · `state: processing` |
| Evento Merged (este ECST) | **ausente** |
| DIA bus | sin `Kaizen_Alert_Required` para este `correlation_id` |

## F2 — Triaje documental

| Check | Estado | Evidencia |
|-------|--------|-----------|
| `DOC_OBJECTIVES` | **APTO** | YAML + misión circuito correo → `Email_Triaged` → WUI |
| `DOC_CLARIFY` | **APTO** | YAML + `mayeuta_verdict: ok` · 0 decisiones abiertas |
| `DOC_SPEC` | **APTO** | YAML + perímetro 01A · entidades + invariantes G4/G5 |
| `DOC_PLAN` | **APTO** | YAML + fases T0–T5/T9a/T10 marcadas |
| `DOC_IMPLEMENTATION` | **APTO** | YAML + touchpoints T0–T10 / F-01…F-07 |
| `DOC_EXECUTION` | **APTO** | YAML `status: executed` · G0–G5/G9a/T10 APTO |
| `DOC_FRONTMATTER_YAML` | **APTO** | artefactos base con `---` YAML |
| `DOC_EVOLUTION` | **NO_APTO** | 0 registro bajo `SddIA/evolution/` ligado a `PBI-KALMA2-MVP-01A` / `c209c150-…`; `execution.md` difiere evolution a G9b |
| `F2_DOC_GATE` | **APTO** | criterios proceso § Triaje documental cumplidos |

`pbi_ref` en `objectives.md`/`clarify.md`/`_init-feature.json` aún apunta a path `pending/` histórico; assert físico: PBI 01A solo en `done/`.

## PBI / Done path

| Check | Estado | Evidencia |
|-------|--------|-----------|
| `PBI_DONE_PRESENT` | **APTO** | `docs/todos/done/[OPERATIVO] Kalma2 MVP 01A…` · `document_id: PBI-KALMA2-MVP-01A` · `status: done` |
| `PBI_PENDING_ABSENT` | **APTO** | sin fichero `PBI-KALMA2-MVP-01A` bajo `docs/todos/pending/` |
| `AC_DONE_PATH` | **APTO** | done exclusivo + `pbi_archived: true` |
| `pbi_archived` | **true** | coherente con archivo en `done/` |

Paraguas `PBI-KALMA2-MVP-01` y ola `01B` en `pending/` son **otros** `document_id`; no anulan 01A.

## Git / rama

| Check | Estado | Evidencia |
|-------|--------|-----------|
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | **APTO** | Evidence Bridge `prosthesis_subprocess` (copia machine) |
| `GIT_EVIDENCE_SESSION_SHELL` | **NO_APTO** | Shell Rejected; sin `gitStdout` |
| `BRANCH_RUNTIME_INJECT` | **APTO** | `branch_name` = `feat/kalma2-mvp-sensorial-email` |
| `BRANCH_ECST_ALIGN` | **APTO** | ECST `payload.branch` = misma rama |
| `BRANCH_WORKTREE_SYNC` | **APTO** | `.git/HEAD` → `refs/heads/feat/kalma2-mvp-sensorial-email` (FS; **no** stdout git-manager) |
| `MERGE_ALREADY_OBSERVED` | **NO_APTO** | sin `PullRequest_Merged` para `2XyNciPL…` |

`git_changes` por **inventario path-assert** (cascada + genoma documentado en `implementation.md`/`execution.md` + PBI done). **No** es `gitStdout` de esta sesión.

## R3 — KM (`RBAC_AUTHORING_KM_POLICY`)

**APTO** — 0 writes Argos/Tekton bajo `docs/todos/**` esta fase.

Sighting FS (no semilla nueva Argos): `docs/todos/pending/[FIX] email-watcher — fractura sistémica (521b4f60d746).md` — autoría **Cúmulo** (`materialize-fracture-pbi` / «auto-generado por Cúmulo»; incidente `System_Fracture_Detected`). Vía legítima. Forja Core ≠ este check.

## Alcance de fase

Triaje documental **no** certifica F3/F4 ni reabre genoma. Downstream: Triaje técnico → Certificación RBAC → Veredicto → Cosecha → Handoff (`accept-pr`; sin merge directo en aduana).

## Dictamen

```json
{
  "phase": "Triaje documental",
  "global": "APTO",
  "verdict": "aprobado",
  "delivery_state": "pending_downstream_phases",
  "resolution": "PASS_F2_DOC",
  "pbi_archived": true,
  "branch": "feat/kalma2-mvp-sensorial-email",
  "document_id": "PBI-KALMA2-MVP-01A",
  "audit_event_reference": "2XyNciPL7yiQuKGFY77qJASEEBjTP572gFt1VjK2HQVY",
  "pr_url": "https://github.com/racso80es/SddIA/pull/182"
}
```
