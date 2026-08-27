---
document_id: PBI-PPR-200-ACCEPT-PR-REVOKED-REGISTRY
title: "[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #200)"
format: markdown
version: "1.0.0"
created: "2026-08-27"
updated: "2026-08-27T11:42:00Z"
status: pending
priority: alta
process: refactorization
type: refactorization
dispatch: false
uuid: a8f3c1e2-9b4d-4e7a-8c5f-1d2e3f4a5b6c
suggested_branch: refactor/accept-pr-revoked-registry-rehab-ppr200
persist_ref_suggested: docs/features/accept-pr-revoked-registry-rehab-ppr200
source_correlation_id: "7c215675-2ad2-436a-9749-ff635c52c8b3"
source_pr_url: https://github.com/racso80es/SddIA/pull/200
feature_ref: docs/features/accept-pr-revoked-registry-rehab-ppr194
parent_pbi: docs/todos/done/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #194).md
incident_ref: "REVOKED_ENTITY_ALERT_ACCEPT_PR — accept-pr re-revoked post-rehab #194 (abrupt_success_rate_drop since 2026-08-27T11:31:15Z; rehabilitated_at 11:20:00Z; merge 6528d115… @ 11:31:11Z)"
entity: accept-pr
related:
  - .SddIA/cerbero/revoked_entities.json
  - .SddIA/radamanto/stats.json
  - SddIA/agents/radamanto.thresholds.json
  - SddIA/library/codexes/codex-software-engineering/process/accept-pr.md
  - docs/todos/done/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #194).md
  - docs/features/accept-pr-revoked-registry-rehab-ppr194/validacion.md
source_audits:
  - docs/features/accept-pr-revoked-registry-rehab-ppr194/validacion.md
  - .SddIA/cerbero/revoked_entities.json
---

# [ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #200)

## Mandato

Rehabilitar el proceso `accept-pr` en `.SddIA/cerbero/revoked_entities.json` tras **re-revocación** post-cierre del ciclo #194 (rehab A1 @ `11:20:00Z` → `revoked` @ `11:31:15Z`), observada en Cosecha Kaizen PPR #200.

| Campo | Valor |
|-------|--------|
| Entidad | `accept-pr` |
| Registro | `.SddIA/cerbero/revoked_entities.json` → **`revoked.accept-pr`** |
| `entity_type` | `process` |
| `reason` | `abrupt_success_rate_drop` |
| Since | `2026-08-27T11:31:15Z` |
| Radamanto | `degraded` · `recovery_attempts: 1` · samples 2×exit 1 / 1×exit 0 · `structure_valid: false` · `rehab_laudo` residual `PBI-PPR-194-ACCEPT-PR-REVOKED-REGISTRY` |
| ≠ incidente #194 done | #194: since `2026-08-26T11:42:26Z` · cerrado (`done/` · merge `6528d115…`) |
| Check origen | empírico Cerbero/Radamanto FS en Cosecha Kaizen PPR #200 + F5 `AC-A1`/`REVOKED_ENTITY_ALERT_ACCEPT_PR` |
| Emisor ECST Presented | `delivery-close-cycle` ∉ revoked |

## Genealogía

| Episodio | PR / ciclo | `since` | Estado |
|----------|------------|---------|--------|
| #194 | accept-pr-revoked-registry-rehab | `2026-08-26T11:42:26Z` | **done** (rehab + merge `6528d115…`) |
| **#200 (este PBI)** | Cosecha post-merge #200 | `2026-08-27T11:31:15Z` | **pending** |

## Sighting Cosecha

PPR #200 · CID `7c215675-2ad2-436a-9749-ff635c52c8b3` · `persist_ref` `docs/features/accept-pr-revoked-registry-rehab-ppr194` · `PullRequest_Merged` dead-letter `c24d84a7…` @ `11:31:11Z` (hash alineado `main`) · re-revocación 4s después.

Materialización: Cosecha Kaizen (Cúmulo) · `KAIZEN_COSECHA_GATE: APTO` · seed nueva (esta).

## Criterio de cierre (borrador)

- [ ] Laudo rehabilitación Cerbero / Radamanto (anti-recurrencia post-rehab; limpiar `rehab_laudo`/`rehabilitated_at` obsoletos)
- [ ] `accept-pr` ausente de `revoked` (y `permanent` si aplica)
- [ ] Cascada feature/fix + `validacion.md` APTO + PBI en `done/`
- [ ] Smoke handoff sin re-revocación inmediata post-merge
