---
document_id: PBI-PPR-203-ACCEPT-PR-REVOKED-REGISTRY
title: "[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #203)"
format: markdown
version: "1.0.0"
created: "2026-08-27"
updated: "2026-08-27T12:35:45Z"
status: pending
priority: alta
process: refactorization
type: refactorization
dispatch: false
uuid: b7e4a91c-2f5d-4c8b-9e1a-6d3f0a8b2c7e
suggested_branch: refactor/accept-pr-revoked-registry-rehab-ppr203
persist_ref_suggested: docs/features/accept-pr-revoked-registry-rehab-ppr203
source_correlation_id: "6237015f-0f8d-42ea-97ea-a44afac5318d"
source_pr_url: https://github.com/racso80es/SddIA/pull/203
feature_ref: docs/features/emit-pr-audited-revoked-registry-rehab-ppr202
parent_pbi: docs/todos/done/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #200).md
incident_ref: "REVOKED_ENTITY_ALERT_ACCEPT_PR — accept-pr re-revoked post-rehab #200 / post-merge PR #203 (abrupt_success_rate_drop since 2026-08-27T12:31:30Z; rehabilitated_at 12:00:00Z; merge 120d741… @ ~12:31:26Z)"
entity: accept-pr
related:
  - .SddIA/cerbero/revoked_entities.json
  - .SddIA/radamanto/stats.json
  - SddIA/agents/radamanto.thresholds.json
  - SddIA/library/codexes/codex-software-engineering/process/accept-pr.md
  - docs/todos/done/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #200).md
  - docs/todos/done/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #194).md
  - docs/features/emit-pr-audited-revoked-registry-rehab-ppr202/validacion.md
source_audits:
  - docs/features/emit-pr-audited-revoked-registry-rehab-ppr202/validacion.md
  - .SddIA/cerbero/revoked_entities.json
---

# [ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #203)

## Mandato

Rehabilitar el proceso `accept-pr` en `.SddIA/cerbero/revoked_entities.json` tras **re-revocación** post-cierre del ciclo #200 (rehab A1 @ `12:00:00Z` → `revoked` @ `12:31:30Z`), observada en Cosecha Kaizen PPR #203 tras merge del rehab `emit-pr-audited-event`.

| Campo | Valor |
|-------|--------|
| Entidad | `accept-pr` |
| Registro | `.SddIA/cerbero/revoked_entities.json` → **`revoked.accept-pr`** |
| `entity_type` | `process` |
| `reason` | `abrupt_success_rate_drop` |
| Since | `2026-08-27T12:31:30Z` |
| Radamanto | `degraded` · `recovery_attempts: 1` · samples mix exit 0/1 · `structure_valid: false` · `rehab_laudo` residual `PBI-PPR-200-ACCEPT-PR-REVOKED-REGISTRY` |
| ≠ incidente #200 done | #200: since `2026-08-27T11:31:15Z` · cerrado (`done/` · merge `42fff076…`) |
| Check origen | empírico Cerbero/Radamanto FS en Cosecha Kaizen PPR #203 + F5 `REVOKED_ENTITY_ALERT_ACCEPT_PR` |
| Emisor ECST Presented | `delivery-close-cycle` ∉ revoked |

## Genealogía

| Episodio | PR / ciclo | `since` | Estado |
|----------|------------|---------|--------|
| #194 | accept-pr-revoked-registry-rehab | `2026-08-26T11:42:26Z` | **done** |
| #200 | accept-pr-revoked-registry-rehab | `2026-08-27T11:31:15Z` | **done** (rehab A1+A2 · laudo #200) |
| **#203 (este PBI)** | Cosecha post-merge #203 | `2026-08-27T12:31:30Z` | **pending** |

## Sighting Cosecha

PPR #203 · CID `6237015f-0f8d-42ea-97ea-a44afac5318d` · `persist_ref` `docs/features/emit-pr-audited-revoked-registry-rehab-ppr202` · `PullRequest_Merged` `4afbf976…` · merge `120d741c33fe8c3e6e8b9fc423651c0f8768f446` @ `2026-08-27T12:31:26Z` · re-revocación ~4s después.

Materialización: Cosecha Kaizen (Cúmulo) · `KAIZEN_COSECHA_GATE: APTO` · seed nueva (esta).

## Criterio de cierre (borrador)

- [ ] Laudo rehabilitación Cerbero / Radamanto (anti-recurrencia post-rehab; limpiar `rehab_laudo`/`rehabilitated_at` obsoletos)
- [ ] `accept-pr` ausente de `revoked` (y `permanent` si aplica)
- [ ] Cascada feature/fix + `validacion.md` APTO + PBI en `done/`
- [ ] Smoke handoff sin re-revocación inmediata post-merge

## Fuera de alcance

- Rehab `refactorization` (dedup done PPR #186).
- Residual Kalma2 Shell/`git-manager` (dedup OPERATIVO PPR #136).
- Reabrir alcance A1 del rehab `emit-pr-audited-event` #202 (ya done).
