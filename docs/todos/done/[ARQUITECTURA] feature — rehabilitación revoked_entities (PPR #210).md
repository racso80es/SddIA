---
document_id: PBI-PPR-210-FEATURE-REVOKED-REGISTRY
title: "[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #210)"
format: markdown
version: "1.0.0"
created: "2026-08-28"
updated: "2026-08-28T06:13:50Z"
status: done
refinement_status: implemented
persist_ref: docs/features/feature-revoked-registry-rehab-ppr210
pbi_archived: true
branch_name: refactor/feature-revoked-registry-rehab-ppr210
priority: media
process: refactorization
type: refactorization
dispatch: false
uuid: f8b2c3d4-5e6f-7a89-0b1c-2d3e4f5a6b7c
suggested_branch: refactor/feature-revoked-registry-rehab-ppr210
source_correlation_id: "4c2dfd1d-393d-4411-8956-d596ff0eef9c"
source_pr_url: https://github.com/racso80es/SddIA/pull/210
feature_ref: docs/fixes/route-domain-event-fracture-6a49e0ad
parent_pbi: docs/todos/done/[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #185).md
incident_ref: "REVOKED_ENTITY_ALERT_FEATURE — feature re-revoked post-rehab #185 (abrupt_success_rate_drop since 2026-08-28T05:25:41Z; rehab A1–A3 #185 cerrado 2026-08-20)"
entity: feature
related:
  - .SddIA/cerbero/revoked_entities.json
  - .SddIA/radamanto/stats.json
  - SddIA/agents/radamanto.thresholds.json
  - SddIA/library/codexes/codex-software-engineering/process/feature.md
  - docs/fixes/route-domain-event-fracture-6a49e0ad/validacion.md
  - docs/todos/done/[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #185).md
source_audits:
  - docs/fixes/route-domain-event-fracture-6a49e0ad/validacion.md
---

# [ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #210)

## Mandato

Rehabilitar el proceso `feature` en `.SddIA/cerbero/revoked_entities.json` tras **re-revocación** post-cierre del ciclo #185 (A1–A3 @ `2026-08-20` → `revoked` @ `05:25:41Z`), observada en Cosecha Kaizen PPR #210 (`route-domain-event-fracture-6a49e0ad`).

| Campo | Valor |
|-------|--------|
| Entidad | `feature` |
| Registro | `.SddIA/cerbero/revoked_entities.json` → **`revoked.feature`** |
| `entity_type` | `process` |
| `reason` | `abrupt_success_rate_drop` |
| Since | `2026-08-28T05:25:41Z` |
| Radamanto | `degraded` · `recovery_attempts: 1` · samples mix exit 1 |
| ≠ incidente #185 done | #185: `permanent.feature` eliminado · A1–A3 cerrados · `done/` |
| Check origen | empírico Cerbero/Radamanto FS en Cosecha Kaizen PPR #210 + alerta lateral F4 |
| Emisor ECST Presented | `delivery-close-cycle` ∉ revoked |

## Genealogía

| Episodio | PR / ciclo | `since` | Estado |
|----------|------------|---------|--------|
| #185 | feature-revoked-registry-rehab | `2026-08-19T07:59:05Z` (permanent) | **done** (A1–A3 @ `2026-08-20`) |
| **#210 (este PBI)** | Cosecha PPR #210 · route-domain-event-fracture | `2026-08-28T05:25:41Z` | **done** (rehab @ `06:13:50Z`) |

## Sighting Cosecha

PPR #210 · CID `4c2dfd1d-393d-4411-8956-d596ff0eef9c` · `persist_ref` `docs/fixes/route-domain-event-fracture-6a49e0ad` · `PullRequest_Presented` @ `2026-08-28T05:59:44Z` · re-revocación Cerbero/Radamanto @ `05:25:41Z` (post-rehab #185).

Materialización: Cosecha Kaizen (Cúmulo) · `KAIZEN_COSECHA_GATE: APTO` · seed nueva (esta; path id).

## Criterio de cierre

- [x] Laudo rehabilitación Cerbero / Radamanto (anti-recurrencia post-rehab #185; A1+A2+A3 obligatorios si aplica)
- [x] `feature` ausente de `revoked`/`permanent` en `revoked_entities.json`
- [x] Cascada refactorization + `validacion.md` APTO + PBI en `done/`
- [x] Smoke proceso `feature` sin re-revocación inmediata post-rehab (`execution_id` `458d194c-a5b4-4e53-918d-91901c2f1d5d` · `2026-08-28`)

## Fuera de alcance

- Rehab `refactorization` (dedup done PPR #186).
- Rehab `accept-pr` (dedup pending PPR #208 + sighting esta Cosecha).
- Residual Kalma2 Shell/`git-manager` (dedup OPERATIVO PPR #136).
