---
document_id: PBI-PPR-210-BUG-FIX-REVOKED-REGISTRY
title: "[ARQUITECTURA] bug-fix — rehabilitación revoked_entities (PPR #210)"
format: markdown
version: "1.0.0"
created: "2026-08-28"
updated: "2026-08-28T06:13:50Z"
status: done
refinement_status: implemented
persist_ref: docs/features/bug-fix-revoked-registry-rehab-ppr210
pbi_archived: true
branch_name: refactor/bug-fix-revoked-registry-rehab-ppr210
priority: alta
process: refactorization
type: refactorization
dispatch: false
uuid: e7a1b2c3-4d5e-6f78-9a0b-1c2d3e4f5a6b
suggested_branch: refactor/bug-fix-revoked-registry-rehab-ppr210
source_correlation_id: "4c2dfd1d-393d-4411-8956-d596ff0eef9c"
source_pr_url: https://github.com/racso80es/SddIA/pull/210
feature_ref: docs/fixes/route-domain-event-fracture-6a49e0ad
parent_pbi: docs/todos/done/[ARQUITECTURA] bug-fix — rehabilitación revoked_entities (PPR #194).md
incident_ref: "REVOKED_ENTITY_ALERT_BUG_FIX — bug-fix re-revoked post-rehab #194 (abrupt_success_rate_drop since 2026-08-28T05:32:55Z; rehabilitated_at 2026-08-27T11:45:00Z; rehab_laudo PBI-PPR-194-BUG-FIX-REVOKED-REGISTRY)"
entity: bug-fix
related:
  - .SddIA/cerbero/revoked_entities.json
  - .SddIA/radamanto/stats.json
  - SddIA/agents/radamanto.thresholds.json
  - SddIA/library/codexes/codex-software-engineering/process/bug-fix.md
  - docs/fixes/route-domain-event-fracture-6a49e0ad/validacion.md
  - docs/todos/done/[ARQUITECTURA] bug-fix — rehabilitación revoked_entities (PPR #194).md
source_audits:
  - docs/fixes/route-domain-event-fracture-6a49e0ad/validacion.md
---

# [ARQUITECTURA] bug-fix — rehabilitación revoked_entities (PPR #210)

## Mandato

Rehabilitar el proceso `bug-fix` en `.SddIA/cerbero/revoked_entities.json` tras **re-revocación** post-cierre del ciclo #194 (rehab A1 @ `11:45:00Z` → `revoked` @ `05:32:55Z`), observada en Cosecha Kaizen PPR #210 (`route-domain-event-fracture-6a49e0ad`).

| Campo | Valor |
|-------|--------|
| Entidad | `bug-fix` |
| Registro | `.SddIA/cerbero/revoked_entities.json` → **`revoked.bug-fix`** |
| `entity_type` | `process` |
| `reason` | `abrupt_success_rate_drop` |
| Since | `2026-08-28T05:32:55Z` |
| Radamanto | `degraded` · `recovery_attempts: 1` · `rehab_laudo` residual `PBI-PPR-194-BUG-FIX-REVOKED-REGISTRY` · `rehabilitated_at 2026-08-27T11:45:00Z` |
| ≠ incidente #194 done | #194: since `2026-08-16T16:09:32Z` · cerrado (`done/` · rehab @ `11:45:00Z`) |
| Check origen | empírico Cerbero/Radamanto FS en Cosecha Kaizen PPR #210 + alerta lateral F4/F5 |
| Emisor ECST Presented | `delivery-close-cycle` ∉ revoked |

## Genealogía

| Episodio | PR / ciclo | `since` | Estado |
|----------|------------|---------|--------|
| #194 | bundle-consumer-telegram-gateway | `2026-08-16T16:09:32Z` | **done** (rehab @ `11:45:00Z`) |
| **#210 (este PBI)** | Cosecha PPR #210 · route-domain-event-fracture | `2026-08-28T05:32:55Z` | **done** (rehab @ `06:13:50Z`) |

## Sighting Cosecha

PPR #210 · CID `4c2dfd1d-393d-4411-8956-d596ff0eef9c` · `persist_ref` `docs/fixes/route-domain-event-fracture-6a49e0ad` · `PullRequest_Presented` @ `2026-08-28T05:59:44Z` · re-revocación Cerbero/Radamanto @ `05:32:55Z` (post-rehab #194).

Materialización: Cosecha Kaizen (Cúmulo) · `KAIZEN_COSECHA_GATE: APTO` · seed nueva (esta; path id).

## Criterio de cierre

- [x] Laudo rehabilitación Cerbero / Radamanto (anti-recurrencia post-rehab #194; limpiar `rehab_laudo`/`rehabilitated_at` obsoletos)
- [x] `bug-fix` ausente de `revoked`/`permanent` en `revoked_entities.json`
- [x] Cascada refactorization + `validacion.md` APTO + PBI en `done/`
- [ ] Smoke proceso `bug-fix` sin re-revocación inmediata post-rehab

## Fuera de alcance

- Rehab `refactorization` (dedup done PPR #186).
- Residual Kalma2 Shell/`git-manager` (dedup OPERATIVO PPR #136).
- Purga copia stale FIX en `docs/todos/pending/` (alcance bug-fix documental PPR #210).
