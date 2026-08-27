---
document_id: PBI-PPR-208-ACCEPT-PR-REVOKED-REGISTRY
title: "[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #208)"
format: markdown
version: "1.0.0"
created: "2026-08-27"
updated: "2026-08-27T18:32:00Z"
status: pending
refinement_status: harvested
persist_ref_suggested: docs/features/accept-pr-revoked-registry-rehab-ppr208
branch_name: refactor/accept-pr-revoked-registry-rehab-ppr208
priority: alta
process: refactorization
type: refactorization
dispatch: false
uuid: d4f8e2a1-6c39-4b7e-9a05-1f3c8d7e6b20
suggested_branch: refactor/accept-pr-revoked-registry-rehab-ppr208
source_correlation_id: "4CMsk8z5Gx7mFQHc512m9FoJibvnr463cVyVcWz5imKm"
secondary_correlation_ids:
  - "04ea6960-3089-4523-9461-cccfbe202f1a"
source_pr_url: https://github.com/racso80es/SddIA/pull/208
feature_ref: docs/features/kaizen-aduana-dlt-relay-supervisado
parent_pbi: docs/todos/done/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #203).md
incident_ref: "REVOKED_ENTITY_ALERT_ACCEPT_PR — accept-pr re-revoked post-rehab #203 (abrupt_success_rate_drop since 2026-08-27T18:21:13Z; rehabilitated_at 2026-08-27T16:04:48Z; rehab_laudo residual PBI-PPR-203-ACCEPT-PR-REVOKED-REGISTRY)"
entity: accept-pr
related:
  - .SddIA/cerbero/revoked_entities.json
  - .SddIA/radamanto/stats.json
  - SddIA/agents/radamanto.thresholds.json
  - SddIA/library/codexes/codex-software-engineering/process/accept-pr.md
  - docs/todos/done/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #203).md
  - docs/todos/done/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #200).md
  - docs/todos/done/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #194).md
  - docs/features/kaizen-aduana-dlt-relay-supervisado/validacion.md
  - docs/features/accept-pr-revoked-registry-rehab-ppr203/
  - docs/features/accept-pr-anti-recurrence-ppr203/
source_audits:
  - docs/features/kaizen-aduana-dlt-relay-supervisado/validacion.md
---

# [ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #208)

## Mandato

Rehabilitar el proceso `accept-pr` en `.SddIA/cerbero/revoked_entities.json` tras **re-revocación** post-cierre del ciclo #203 (rehab A1 @ `16:04:48Z` → `revoked` @ `18:21:13Z`), observada en Cosecha Kaizen PPR #208 (`kaizen-aduana-dlt-relay-supervisado`).

| Campo | Valor |
|-------|--------|
| Entidad | `accept-pr` |
| Registro | `.SddIA/cerbero/revoked_entities.json` → **`revoked.accept-pr`** |
| `entity_type` | `process` |
| `reason` | `abrupt_success_rate_drop` |
| Since | `2026-08-27T18:21:13Z` |
| Radamanto | `degraded` · `recovery_attempts: 1` · samples mix exit 0/1 · `structure_valid: false` · `rehab_laudo` residual `PBI-PPR-203-ACCEPT-PR-REVOKED-REGISTRY` · `rehabilitated_at 2026-08-27T16:04:48Z` |
| ≠ incidente #203 done | #203: since `2026-08-27T12:31:30Z` · cerrado (`done/` · rehab @ `16:04:48Z`) |
| Check origen | empírico Cerbero/Radamanto FS en Cosecha Kaizen PPR #208 + F5 `REVOKED_ENTITY_ALERT_ACCEPT_PR` |
| Emisor ECST Presented | `github-bridge-watcher` ∉ revoked |

## Genealogía

| Episodio | PR / ciclo | `since` | Estado |
|----------|------------|---------|--------|
| #194 | accept-pr-revoked-registry-rehab | `2026-08-26T11:42:26Z` | **done** |
| #200 | accept-pr-revoked-registry-rehab | `2026-08-27T11:31:15Z` | **done** |
| #203 | accept-pr-revoked-registry-rehab | `2026-08-27T12:31:30Z` | **done** (rehab @ `16:04:48Z`) |
| **#208 (este PBI)** | Cosecha PPR #208 · kaizen-aduana-dlt-relay | `2026-08-27T18:21:13Z` | **pending** |

## Sighting Cosecha

PPR #208 · CID primario `4CMsk8z5Gx7mFQHc512m9FoJibvnr463cVyVcWz5imKm` · CID lateral `04ea6960-3089-4523-9461-cccfbe202f1a` · `persist_ref` `docs/features/kaizen-aduana-dlt-relay-supervisado` · `PullRequest_Presented` @ `2026-08-27T18:18:48Z` · re-revocación Cerbero/Radamanto @ `18:21:13Z` (post-rehab #203).

Materialización: Cosecha Kaizen (Cúmulo) · `KAIZEN_COSECHA_GATE: APTO` · seed nueva (esta; path id).

## Criterio de cierre

- [ ] Laudo rehabilitación Cerbero / Radamanto (anti-recurrencia post-rehab #203; limpiar `rehab_laudo`/`rehabilitated_at` obsoletos)
- [ ] `accept-pr` ausente de `revoked` (y `permanent` si aplica)
- [ ] Cascada feature/fix + `validacion.md` APTO + PBI en `done/`
- [ ] Smoke handoff sin re-revocación inmediata post-rehab (samples healthy · `structure_valid: true`)

## Fuera de alcance

- Rehab `refactorization` (dedup done PPR #186).
- Residual Kalma2 Shell/`git-manager` (dedup OPERATIVO PPR #136).
- Merge/Handoff soberano del PR #208 (L-HANDOFF-F5 · MERGE ausente; bloqueado mientras `accept-pr` ∈ revoked).
