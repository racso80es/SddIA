---
document_id: PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL-PPR-REVOKED-REGISTRY
title: "[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (kaizen-aduana-evolution-local)"
format: markdown
version: "1.0.0"
created: "2026-08-28"
updated: "2026-08-28T11:20:00Z"
status: pending
priority: alta
process: refactorization
type: refactorization
dispatch: false
uuid: c4e8f1a2-9b3d-4f7e-a6c1-2d8e5f0b3a71
suggested_branch: refactor/ppr-revoked-registry-rehab-kaizen-aduana-evolution
persist_ref_suggested: docs/features/ppr-revoked-registry-rehab-kaizen-aduana-evolution
branch_name: fix/kaizen-aduana-evolution-local-ca12-ca14
source_correlation_id: "8ZjTzcBwfFAVFQujfjGCJwJeJcj5pbB4SMHAD5bn5ybE"
feature_ref: docs/fixes/kaizen-aduana-evolution-local
parent_pbi: docs/todos/done/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #190).md
incident_ref: "REVOKED_ENTITY_ALERT_PULL_REQUEST_REVIEW — pull-request-review re-revoked post-rehab #190 (success_rate_below_threshold since 2026-08-28T10:10:42Z)"
entity: pull-request-review
related:
  - .SddIA/cerbero/revoked_entities.json
  - .SddIA/radamanto/stats.json
  - SddIA/agents/radamanto.thresholds.json
  - SddIA/library/codexes/codex-software-engineering/process/pull-request-review.md
  - docs/todos/done/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #190).md
  - docs/todos/done/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #174).md
  - docs/fixes/kaizen-aduana-evolution-local/validacion.md
source_audits:
  - docs/fixes/kaizen-aduana-evolution-local/validacion.md
  - .SddIA/cerbero/revoked_entities.json
---

# [ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (kaizen-aduana-evolution-local)

## Mandato

Rehabilitar el proceso `pull-request-review` en `.SddIA/cerbero/revoked_entities.json` tras **re-revocación** post-cierre del ciclo #190 (rehab @ `2026-08-26T18:02:03Z` → `revoked` @ `2026-08-28T10:10:42Z`), observada en Cosecha Kaizen PPR `kaizen-aduana-evolution-local` (F4/F5 bloqueante `FAIL_F4_RBAC`).

| Campo | Valor |
|-------|--------|
| Entidad | `pull-request-review` |
| Registro | `.SddIA/cerbero/revoked_entities.json` → **`revoked.pull-request-review`** |
| `entity_type` | `process` |
| `reason` | `success_rate_below_threshold` |
| Since | `2026-08-28T10:10:42Z` |
| ≠ incidente #190 done | #190: permanent+revoked simultáneos · cerrado (`done/` @ `2026-08-26T18:02:03Z`) |
| Check origen | empírico Cerbero F4/F5 en PPR `kaizen-aduana-evolution-local` + `REVOKED_PROCESS_PULL_REQUEST_REVIEW` |
| Emisor ECST | default contractual `delivery-close-cycle` ∉ revoked |

## Genealogía

| Episodio | PR / ciclo | `since` | Estado |
|----------|------------|---------|--------|
| #124/#125/#174 | olas rehab PPR | varios | **done** |
| #190 | kaizen-paciente0-redeploy | `2026-08-25T16:25:55Z` / `17:24:18Z` | **done** (rehab @ `18:02:03Z`) |
| **este PBI** | Cosecha PPR kaizen-aduana-evolution-local | `2026-08-28T10:10:42Z` | **pending** |

## Sighting Cosecha

PPR kaizen-aduana-evolution-local · CID `8ZjTzcBwfFAVFQujfjGCJwJeJcj5pbB4SMHAD5bn5ybE` · `persist_ref` `docs/fixes/kaizen-aduana-evolution-local` · `branch_name` `fix/kaizen-aduana-evolution-local-ca12-ca14` · F4 `FAIL_F4_RBAC` · F5 `delivery_state: failed` · Handoff **prohibido**.

Materialización: Cosecha Kaizen (Cúmulo) · `KAIZEN_COSECHA_GATE: APTO` · seed nueva (esta; path id).

## Criterio de cierre

- [ ] Laudo rehabilitación Cerbero / Radamanto (anti-recurrencia post-rehab #190)
- [ ] `pull-request-review` ausente de `revoked` (y `permanent` si aplica)
- [ ] Cascada feature/fix + `validacion.md` APTO + PBI en `done/`
