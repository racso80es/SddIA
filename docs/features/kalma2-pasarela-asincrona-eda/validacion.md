---
feature_name: kalma2-pasarela-asincrona-eda
created: "2026-07-22"
updated: "2026-07-23"
process: pull-request-review
phase: Cosecha Kaizen
agent: cumulo
branch: feat/kalma2-pasarela-asincrona-eda
branch_name_injected: feat/kalma2-pasarela-asincrona-eda
persist_ref: docs/features/kalma2-pasarela-asincrona-eda
document_id: PBI-044-KALMA2-PASARELA-ASINCRONA-EDA
pbi_uuid: 8c71b50f-7067-472a-a149-40041920b054
pbi_ref: docs/todos/done/[ARQUITECTURA] PBI-044 — Pasarela asíncrona Kalma2 y desacople por bus de eventos.md
correlation_id: 356674d3-aa2f-434c-acaf-3dec075af2c3
pr_url: https://github.com/racso80es/SddIA/pull/146
pr_presented_event_id: 356674d3-aa2f-434c-acaf-3dec075af2c3
global: NO_APTO
pbi_archived: true
approval_status: rechazado
verdict: rechazado
delivery_state: failed
accept_pr_handoff: false
resolution: FAIL_F4_RBAC
audit_event_reference: 356674d3-aa2f-434c-acaf-3dec075af2c3
authorization_status:
  exitCode: null
  signer_identity_rbac: null
  note: "Cerbero fase Certificación RBAC status=blocked (entorno/tooling); sin peaje físico — heredado Argos"
git_manager_invoked: false
git_manager_error: "cápsula no invocable en esta sesión (Shell/Auto-review rejected); sin stdout físico de ./sddia-run.sh --tool git-manager"
scope: "H1+H2 Done mínimo PBI-044 (R1–R5; R6/H3 defer) — aduana PPR Cosecha Kaizen"
checks:
  F2_DOC_GATE: APTO
  F3_TECH_GATE: APTO
  F4_RBAC_GATE: NO_APTO
  VERDICT_SYNTHESIS_GATE: NO_APTO
  DOC_OBJECTIVES: APTO
  DOC_SPEC: APTO
  DOC_PLAN: APTO
  DOC_IMPLEMENTATION: APTO
  DOC_EXECUTION: APTO
  DOC_FRONTMATTER_YAML: APTO
  TECH_AC_SCOPE: APTO
  TECH_BRIDGE_SPAWN_202: APTO
  TECH_SPATIAL_BLINDNESS: APTO
  TECH_ECST_CANONICAL: APTO
  TECH_CARGO_PROXY: APTO
  TECH_FORMAL_EXECUTE_PROCESS: NO_APTO
  RBAC_CERBERO_EVIDENCE: NO_APTO
  RBAC_SIGNER_PRESENT: NO_APTO
  RBAC_EMITTER_NOT_REVOKED: NO_APTO
  RBAC_PROCESS_REGISTRY: APTO
  BRANCH_ECST_ALIGN: APTO
  GIT_EVIDENCE_VIA_GIT_MANAGER: NO_APTO
  GIT_EVIDENCE_HISTORICAL_FEATURE: APTO
  PBI_DONE_PRESENT: APTO
  ACCEPT_PR_HANDOFF: NO_APTO
  KAIZEN_COSECHA_GATE: APTO
  KAIZEN_DIA_ALERT: APTO
  KAIZEN_SEED_DCC_REVOKED_SIGNER: APTO
  KAIZEN_SEED_KALMA2_RUNTIME_RESIDUAL: APTO
kaizen_seeds: []
kaizen_seeds_dedup:
  - docs/todos/pending/[OPERATIVO] Kalma2-agent-runtime-cursor — F3 git-manager KM residual (PPR #136).md
  - docs/todos/pending/[ARQUITECTURA] delivery-close-cycle — revoked_entities y ECST signer (PPR #136).md
git_changes:
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - SddIA/interfaces/kalma2-bridge/Cargo.toml
  - SddIA/Cargo.lock
  - SddIA/engine/execute-process/src/engine/handlers/kalma2.rs
  - interfaces/kalma2/app.js
  - docs/features/kalma2-pasarela-asincrona-eda/
  - docs/todos/done/[ARQUITECTURA] PBI-044 — Pasarela asíncrona Kalma2 y desacople por bus de eventos.md
  - docs/todos/pending/[ARQUITECTURA] PBI-043 — Pasarela asíncrona de Cursor y desacople por bus de eventos.md
---

# Validación — Cosecha Kaizen (Cúmulo · pull-request-review)

## Veredicto de fase

**APTO (cosecha)** — `kaizen_seeds: 0` nuevas · `kaizen_seeds_dedup: 2` · `KAIZEN_COSECHA_GATE: APTO`.

Peaje aduana **heredado** fase 5: `verdict: rechazado` · `delivery_state: failed` · `accept_pr_handoff: false` · `resolution: FAIL_F4_RBAC`. Cosecha **no** altera `delivery_state`.

| Gate | Delegado | Estado | Criterio |
|------|----------|--------|----------|
| F2 | Argos (doc) | **APTO** | heredado · cascada frontmatter |
| F3 | execute-process / proxy | **APTO** | heredado · proxy Tekton `execution.md` |
| F4 | Cerbero | **NO_APTO** | heredado · sin peaje físico |
| Kaizen | Cúmulo | **APTO** | deuda mapeada (dedup); sin DIA |

## Ingesta

| Input | Resolución |
|-------|------------|
| `persist_ref` | `docs/features/kalma2-pasarela-asincrona-eda` |
| `branch` / `branch_name` | `feat/kalma2-pasarela-asincrona-eda` |
| `correlation_id` | `356674d3-aa2f-434c-acaf-3dec075af2c3` |
| `pr_url` | `https://github.com/racso80es/SddIA/pull/146` |
| ECST | `.events/processed/356674d3-…json` · `PullRequest_Presented` · emitter `delivery-close-cycle` · **sin** `signer_identity_rbac` |
| DIA bus | sin `Kaizen_Alert_Required` en `.events/` para este `correlation_id` |

## F2 / F3 / F4 (heredados Argos)

Sin re-litigio. Resumen bloqueante:

- `F4_RBAC_GATE` / `RBAC_CERBERO_EVIDENCE` / `RBAC_SIGNER_PRESENT` → **NO_APTO** (bloqueo fase 5).
- Correction blueprint fase 5 (`ppr-rehab-f4-cerbero-kalma2-pasarela`) permanece vigente; **no** se materializa seed duplicado de rehabilitación F4 (es re-ejecución de peaje, no deuda genérica nueva).

## Git / rama

| Check | Estado | Evidencia |
|-------|--------|-----------|
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | **NO_APTO** | `./sddia-run.sh --tool git-manager` → Shell Rejected; sin stdout físico (sesión Cúmulo) |
| `GIT_EVIDENCE_HISTORICAL_FEATURE` | **APTO** | heredado · `execution.md` sesión Tekton |
| `BRANCH_ECST_ALIGN` | **APTO** | ECST `payload.branch` = `feat/kalma2-pasarela-asincrona-eda` |

## PBI

| Check | Estado | Nota |
|-------|--------|------|
| `PBI_DONE_PRESENT` | **APTO** | `docs/todos/done/[ARQUITECTURA] PBI-044 — …` |
| `pbi_archived` | `true` | no autoriza `accept-pr` con F4 fallido |

## Dictamen final

```json
{
  "phase": "Cosecha Kaizen",
  "verdict": "rechazado",
  "delivery_state": "failed",
  "accept_pr_handoff": false,
  "resolution": "FAIL_F4_RBAC",
  "audit_event_reference": "356674d3-aa2f-434c-acaf-3dec075af2c3",
  "kaizen_seeds": 0,
  "kaizen_seeds_dedup": 2,
  "blocking_findings": [
    "F4_RBAC_GATE:NO_APTO",
    "RBAC_CERBERO_EVIDENCE:NO_APTO",
    "RBAC_SIGNER_PRESENT:NO_APTO"
  ],
  "non_blocking_findings": [
    "TECH_FORMAL_EXECUTE_PROCESS:NO_APTO",
    "GIT_EVIDENCE_VIA_GIT_MANAGER:NO_APTO",
    "RBAC_EMITTER_NOT_REVOKED:NO_APTO"
  ]
}
```

## Cosecha Kaizen — semillas

| Check | Estado | Evidencia |
|-------|--------|-----------|
| `KAIZEN_COSECHA_GATE` | **APTO** | deuda contabilizada; 0 seeds nuevas |
| `KAIZEN_DIA_ALERT` | **APTO** | sin evento `Kaizen_Alert_Required` en bus |
| `KAIZEN_SEED_KALMA2_RUNTIME_RESIDUAL` | **APTO** | dedup → OPERATIVO PPR #136 |
| `KAIZEN_SEED_DCC_REVOKED_SIGNER` | **APTO** | dedup → ARQUITECTURA PPR #136 |
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | **NO_APTO** | invocación cápsula no materializada (Shell rejected) |

### Mapeo findings → seeds

| Finding | Tratamiento Cúmulo |
|---------|-------------------|
| `TECH_FORMAL_EXECUTE_PROCESS` | **dedup** `[OPERATIVO] Kalma2-agent-runtime-cursor — F3 git-manager KM residual (PPR #136)` · R1 |
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | **dedup** mismo OPERATIVO · R2 |
| `RBAC_EMITTER_NOT_REVOKED` | **dedup** `[ARQUITECTURA] delivery-close-cycle — revoked_entities y ECST signer (PPR #136)` · E1; empírico: clave aún en `revoked` |
| `RBAC_SIGNER_PRESENT` | **dedup** mismo ARQUITECTURA · E2; ECST #146 sin firmante |
| `F4_RBAC_GATE` / `RBAC_CERBERO_EVIDENCE` | **sin seed** — correction blueprint fase 5 (re-Cerbero); fallo de peaje/sesión, no deuda genérica nueva |

## Jurisdicción de fase

Cubre **Cosecha Kaizen** (fase 6). Handoff `accept-pr` **no** procede (`accept_pr_handoff: false`). Semillas bajo `docs/todos/` solo Cúmulo / `Kaizen_Alert_Required`.

## approval_status

```text
rechazado — kaizen_seeds: 0 (dedup 2); delivery_state failed heredado;
git-manager sesión NO_APTO (sin stdout); accept-pr bloqueado por F4
```
