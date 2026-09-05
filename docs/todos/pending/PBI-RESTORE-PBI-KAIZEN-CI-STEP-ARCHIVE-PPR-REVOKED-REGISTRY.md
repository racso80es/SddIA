---
document_id: PBI-RESTORE-PBI-KAIZEN-CI-STEP-ARCHIVE-PPR-REVOKED-REGISTRY
title: "[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (restore-pbi-kaizen-ci-step-archive)"
format: markdown
version: "1.0.0"
created: "2026-09-01"
updated: "2026-09-04T13:45:30Z"
status: pending
refinement_status: seed
priority: alta
process: refactorization
executor_vehicle: feature
type: refactorization
dispatch: false
uuid: e2f8a1c4-7b3d-4e9f-a612-8c5d0b9e4f17
entity: pull-request-review
entity_type: process
olas:
  - A1
suggested_branch: refactor/ppr-revoked-registry-rehab-restore-kaizen-ci-step
persist_ref_suggested: docs/features/ppr-revoked-registry-rehab-restore-kaizen-ci-step
source_branch: fix/restore-pbi-kaizen-ci-step-archive
source_correlation_id: "AU1AzkrREQVTRhGHexuqiumPXPw8iP2SgCSLB7AcFKfc"
source_pr_url: https://github.com/racso80es/SddIA/pull/247
feature_ref: docs/fixes/restore-pbi-kaizen-ci-step-archive
parent_pbi: docs/todos/done/PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL-PPR-REVOKED-REGISTRY.md
incident_ref: "REVOKED_ENTITY_ALERT_PULL_REQUEST_REVIEW — pull-request-review re-revoked post-rehab kaizen-aduana (abrupt_success_rate_drop since 2026-08-29T05:01:52Z; rehabilitated_at 2026-08-29T04:47:57Z; rehab_laudo residual PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL-PPR-REVOKED-REGISTRY; 20×exit1 short-ms → abrupt drop)"
blocked_by:
  - "refactorization ∈ revoked since 2026-08-20T05:48:56Z (dedup done PPR #186) → vehículo DCC = feature"
related:
  - .SddIA/cerbero/revoked_entities.json
  - .SddIA/radamanto/stats.json
  - SddIA/agents/radamanto.thresholds.json
  - SddIA/library/codexes/codex-software-engineering/process/pull-request-review.md
  - docs/todos/done/PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL-PPR-REVOKED-REGISTRY.md
  - docs/todos/done/PBI-PPR-ANTI-RECURRENCE-HOLLOW-A2-KAIZEN-ADUANA-EVOLUTION.md
  - docs/fixes/restore-pbi-kaizen-ci-step-archive/validacion.md
  - docs/fixes/ignition-pre-push-guard/validacion.md
source_audits:
  - docs/fixes/restore-pbi-kaizen-ci-step-archive/validacion.md
  - docs/fixes/ignition-pre-push-guard/validacion.md
  - docs/ppr-cosecha-kaizen-20260904/validacion.md
  - .SddIA/cerbero/revoked_entities.json
  - .SddIA/radamanto/stats.json
---

# [ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (restore-pbi-kaizen-ci-step-archive)

## Mandato

Rehabilitar el proceso `pull-request-review` en `.SddIA/cerbero/revoked_entities.json` **podando el estado Radamanto que causa la recidiva**, tras re-revocación post-cierre del ciclo kaizen-aduana-evolution-local (rehab @ `2026-08-29T04:47:57Z` → `revoked` @ `2026-08-29T05:01:52Z`), observada en Cosecha Kaizen PPR `restore-pbi-kaizen-ci-step-archive` (F4/F5 bloqueante `FAIL_F4_RBAC`).

| Campo | Valor |
|-------|--------|
| Entidad | `pull-request-review` |
| Registro | `.SddIA/cerbero/revoked_entities.json` → **`revoked.pull-request-review`** |
| `entity_type` | `process` |
| `reason` | `abrupt_success_rate_drop` |
| Since | `2026-08-29T05:01:52Z` |
| ≠ incidente kaizen-aduana done | kaizen-aduana: `success_rate_below_threshold` @ `2026-08-28T10:10:42Z` · cerrado (`done/` @ rehab `2026-08-29T04:47:57Z`) |
| Check origen | empírico Cerbero F4/F5 en PPR `restore-pbi-kaizen-ci-step-archive` + `RBAC_PROCESS_REGISTRY` |
| Emisor ECST | `github-bridge-watcher` ∉ revoked (firmante `Vertice_Biologico_Relay`) |

## Estado empírico verificado (`2026-09-01T14:35:00Z`)

`.SddIA/cerbero/revoked_entities.json`:

| Bucket | Contenido relevante |
|--------|---------------------|
| `permanent` | `{}` (vacío) |
| `revoked.pull-request-review` | `abrupt_success_rate_drop` @ `2026-08-29T05:01:52Z` |
| Laterales (no este PBI) | `bug-fix` · `delivery-close-cycle` · `entity-manager` · `feature` · `refactorization` |

`.SddIA/radamanto/stats.json` → `pull-request-review`:

| Campo | Valor | Lectura |
|-------|-------|---------|
| `status` | `degraded` | no redimible sin `structure_valid: true` |
| `structure_valid` | `false` | bloquea transición `degraded → pending_redemption` |
| `consecutive_success_count` | `0` | |
| `recovery_attempts` | `1` | margen `max_recovery_attempts: 3` |
| `samples` | **20** (0 exit 0 / 20 exit≠0; `duration_ms` ≈ `645–1353`) | perfil aborto temprano; `success_rate` **0.0** |
| `rehab_laudo` | `PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL-PPR-REVOKED-REGISTRY` | residual del ciclo cerrado |
| `rehabilitated_at` | `2026-08-29T04:47:57Z` | residual; `degraded_at` posterior (`2026-08-29T05:01:52Z`) |

## Diagnóstico — recidiva post-A1

El rehab A1 de kaizen-aduana limpió Cerbero y fijó `samples: []` + `healthy`, pero **~14 min después** (`04:47:57Z` → `05:01:52Z`) Radamanto re-degradó con 20 muestras KO de latencia corta. Coincide con el patrón de abortos de gobernanza ya descrito en el PBI padre (hipótesis hollow / denegación temprana). La ola A2 anti-recurrencia (`PBI-PPR-ANTI-RECURRENCE-HOLLOW-A2…`) está en `done/`; esta recidiva exige **re-aplicar A1** (poda instancia) y auditar si A2 cubre el `failed_phase_code` real de estas muestras.

## Genealogía

| Episodio | PR / ciclo | `since` | Estado |
|----------|------------|---------|--------|
| #190 | kaizen-paciente0-redeploy | `2026-08-25…` | **done** |
| kaizen-aduana | Cosecha PPR kaizen-aduana-evolution-local | `2026-08-28T10:10:42Z` | **done** (rehab @ `2026-08-29T04:47:57Z`) |
| A2 hollow | `PBI-PPR-ANTI-RECURRENCE-HOLLOW-A2…` | — | **done** |
| **este PBI** | Cosecha PPR restore-pbi-kaizen-ci-step-archive | `2026-08-29T05:01:52Z` | **pending** (seed) |

## Sighting Cosecha

PPR restore-pbi-kaizen-ci-step-archive · CID `AU1AzkrREQVTRhGHexuqiumPXPw8iP2SgCSLB7AcFKfc` · PR #247 · `persist_ref` `docs/fixes/restore-pbi-kaizen-ci-step-archive` · `source_branch` `fix/restore-pbi-kaizen-ci-step-archive` · F4 `FAIL_F4_RBAC` · F5 `delivery_state: failed` · Handoff **prohibido**.

Materialización: Cosecha Kaizen (Cúmulo) · `KAIZEN_COSECHA_GATE: APTO` · seed nueva (esta; path id) · execution `a315ae3e-200f-4565-b4ae-fb9f6db3e68a`.

| Sighting | CID | Nota |
|----------|-----|------|
| Cosecha Kaizen create | `AU1AzkrREQVTRhGHexuqiumPXPw8iP2SgCSLB7AcFKfc` | exec `a315ae3e…` · seed path id · since `2026-08-29T05:01:52Z` |
| Cosecha Kaizen dedup | `600cd25c-7d3d-4be4-a53b-54a9ff64be51` | exec `ab27081e…` · 0 create · affirm pending · Cerbero PPR∈revoked same since · F5 `FAIL_F4_RBAC` · ECST emitter `delivery-close-cycle` |
| Cosecha Kaizen dedup | `064918a2-af08-441f-a5b5-d34ad312c489` | exec `d712f728…` · 0 create · affirm pending · Cerbero PPR∈revoked same since · F5 `FAIL_F5_VERDICT` · PR #251 · Presented `2Wkh9xq…` |
| Cosecha Kaizen dedup | `2Wkh9xqgpu1C8LPAhWzfrvL8LQXdTa5Rz55r81GWReda` | exec `7dd9caa4…` · 0 create · affirm pending · Cerbero PPR∈revoked same since · F5 `FAIL_F5_VERDICT` · PR #251 ignition-pre-push-guard · emitter `github-bridge-watcher` |
| Cosecha Kaizen dedup | `cf977edc-706b-4b01-ba70-4beec1fcca82` | exec `95a54dc9…` · 0 create · affirm pending · Cerbero PPR∈revoked same since · F5 `FAIL_F5_VERDICT` · PR #251 dcc-lab-residual-capsules · emitter `delivery-close-cycle` |
| Cosecha Kaizen dedup | `c368985f-2c03-4852-a9aa-0bc363f6c94e` | exec `db1b9e3f…` · 0 create · affirm pending · Cerbero PPR∈revoked same since · F5 `FAIL_F5_VERDICT` · sink `docs/ppr-cosecha-kaizen-20260904` · ECST `Local_QA_Requested` · emitter `git-hook-pre-push` |
| Cosecha Kaizen dedup | `2dYUXN7nq9DsKAs7xZ5vWgq5G3fkfErbwT2Ke2ffnv3Z` | exec `66954b4b…` · 0 create · affirm pending · Cerbero PPR∈revoked same since · F5 `FAIL_F5_VERDICT` · sink `docs/ppr-cosecha-kaizen-20260904` · PR #253 · `PullRequest_Presented` · emitter `github-bridge-watcher` |
| Cosecha Kaizen dedup | `2fad80c0-6ee1-42a2-8d6f-c1399113fbdc` | exec `72f5e494…` · 0 create · affirm pending · Cerbero PPR∈revoked same since · F5 `FAIL_F5_VERDICT` · sink `docs/ppr-cosecha-kaizen-20260904` · PR #253 · `PullRequest_Presented` · emitter `delivery-close-cycle` (∈revoked L-OUT) |
| Cosecha Kaizen dedup | `7293fada-4fbc-4aac-8881-8061e9c0583d` | exec `e21fc03d…` · 0 create · affirm pending · Cerbero PPR∈revoked same since · F5 `FAIL_F5_VERDICT` · sink `docs/ppr-cosecha-kaizen-20260904` · PR #253 · `PullRequest_Presented` · emitter `delivery-close-cycle` (∈revoked L-OUT) |
| Cosecha Kaizen dedup | `9c9cd653-dabe-4fe2-a54d-17f868cd427e` | exec `6362eb00…` · 0 create · affirm pending · Cerbero PPR∈revoked same since · F5 `FAIL_F5_VERDICT` · sink `docs/ppr-cosecha-kaizen-20260904` · PR #253 · `PullRequest_Presented` · emitter `delivery-close-cycle` (∈revoked L-OUT) |
| Cosecha Kaizen dedup | `74a57c11-6764-4a6a-92e6-7943faa48d35` | exec `e431afdf…` · 0 create · affirm pending · Cerbero PPR∈revoked same since · F5 `FAIL_F5_VERDICT` · sink `docs/ppr-cosecha-kaizen-20260904` · ECST `Local_QA_Requested` · emitter `git-hook-pre-push` · sibling race exec `8d2567b6…` |
| Cosecha Kaizen dedup | `74a57c11-6764-4a6a-92e6-7943faa48d35` | exec `8d2567b6…` · 0 create · affirm pending · Cerbero PPR∈revoked same since · F5 `FAIL_F5_VERDICT` · sink `docs/ppr-cosecha-kaizen-20260904` · ECST `Local_QA_Requested` · emitter `git-hook-pre-push` · sibling race exec `e431afdf…` |
| Cosecha Kaizen dedup | `DK5QuSSudtQmSiSMZikUXN83xiF7fwEHxGHGRCUBz1tm` | exec `0b826e3b…` · 0 create · affirm pending · Cerbero PPR∈revoked same since · F5 `FAIL_F5_VERDICT` · sink `docs/ppr-cosecha-kaizen-20260904` · PR #255 · `PullRequest_Presented` · emitter `github-bridge-watcher` · sibling race exec `d50a40ba…` / CID `e4c9970f…` |
| Cosecha Kaizen dedup | `e4c9970f-9e15-40fe-857f-07c44c1bada5` | exec `d50a40ba…` · 0 create · affirm pending · Cerbero PPR∈revoked same since · F5 `FAIL_F5_VERDICT` · sink `docs/ppr-cosecha-kaizen-20260904` · PR #255 · `PullRequest_Presented` · emitter `delivery-close-cycle` (∈revoked L-OUT) · sibling race exec `0b826e3b…` / CID `DK5Qu…` |

## Alcance

### Ola A1 — rehab de instancia (fuera del diff del PR)

`.SddIA/cerbero/` y `.SddIA/radamanto/` están en `.gitignore`: mutación de instancia; **no** viaja en el PR.

1. `revoked_entities.json`: eliminar `revoked.pull-request-review`. **No tocar** laterales ajenos (`bug-fix`, `delivery-close-cycle`, `entity-manager`, `feature`, `refactorization`).
2. `stats.json` → `pull-request-review`: `status: healthy` · `samples: []` · `consecutive_success_count: 0` · `recovery_attempts: 0` · `degraded_at: null` · `structure_valid: true` · `entity_type: process` · `rehab_laudo: PBI-RESTORE-PBI-KAIZEN-CI-STEP-ARCHIVE-PPR-REVOKED-REGISTRY` · `rehabilitated_at` = timestamp del laudo.
3. Smoke post-rehab: una ejecución PPR real sin re-revocación inmediata; registrar `execution_id` en `execution.md`.

### Fuera de alcance

- Seeds laterales (`bug-fix` / `feature` / `delivery-close-cycle` / `entity-manager`) — episodios con `since` propios; no se absorben aquí.
- Mutación de genoma motor (A2 ya cerrada en PBI anti-recurrencia); solo reabrir si T0 demuestra hueco normativo nuevo.

## Criterio de cierre

- [ ] A1 instancia: `pull-request-review` ∉ `revoked` ni `permanent` · stats raíz `healthy` · `samples: []` · `structure_valid: true` · `rehab_laudo` + `rehabilitated_at` de este `document_id`
- [ ] Smoke PPR sin re-revocación inmediata documentado
- [ ] Argos APTO en `validacion.md` del ciclo de rehab
- [ ] Este TODO movido a `docs/todos/done/`
