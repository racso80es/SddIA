---
document_id: PBI-PPR-194-ACCEPT-PR-REVOKED-REGISTRY
title: "[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #194)"
format: markdown
version: "1.0.0"
created: "2026-08-26"
updated: "2026-08-27T11:22:00Z"
status: done
refinement_status: implemented
persist_ref: docs/features/accept-pr-revoked-registry-rehab-ppr194
branch_name: refactor/accept-pr-revoked-registry-rehab-ppr194
pbi_archived: true
priority: alta
process: refactorization
type: refactorization
dispatch: false
uuid: 7f3a9c2e-4b1d-4e8a-9c5f-6d7e8a9b0c1d
suggested_branch: refactor/accept-pr-revoked-registry-rehab-ppr194
persist_ref_suggested: docs/features/accept-pr-revoked-registry-rehab-ppr194
source_correlation_id: "59606407-eed3-4da8-ac13-3cf6205b2147"
source_pr_url: https://github.com/racso80es/SddIA/pull/194
feature_ref: docs/fixes/bundle-consumer-telegram-gateway
incident_ref: "REVOKED_ENTITY_ALERT_ACCEPT_PR — accept-pr ∈ revoked (abrupt_success_rate_drop since 2026-08-26T11:42:26Z); bloquea handoff soberano post-aduana (accept_pr_handoff true · merge ausente)"
entity: accept-pr
related:
  - .SddIA/cerbero/revoked_entities.json
  - .SddIA/radamanto/stats.json
  - SddIA/agents/radamanto.thresholds.json
  - SddIA/library/codexes/codex-software-engineering/process/accept-pr.md
  - docs/todos/done/[FIX] accept-pr delete_branch payload vs git-manager.md
  - docs/fixes/bundle-consumer-telegram-gateway/validacion.md
source_audits:
  - docs/fixes/bundle-consumer-telegram-gateway/validacion.md
  - .SddIA/cerbero/revoked_entities.json
---

# [ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #194)

## Mandato

Rehabilitar el proceso `accept-pr` en `.SddIA/cerbero/revoked_entities.json` tras revocación **`revoked`** (`abrupt_success_rate_drop`) observada en Cosecha Kaizen PPR #194 — crítica porque el peaje F5 fijó `accept_pr_handoff: true` con merge ausente.

| Campo | Valor |
|-------|--------|
| Entidad | `accept-pr` |
| Registro | `.SddIA/cerbero/revoked_entities.json` → **`revoked.accept-pr`** |
| `entity_type` | `process` |
| `reason` | `abrupt_success_rate_drop` |
| Since | `2026-08-26T11:42:26Z` |
| Check origen | empírico Cerbero FS en Cosecha Kaizen PPR #194 (alerta no bloqueante aduana; riesgo handoff) |
| Emisor ECST Presented | `delivery-close-cycle` ∉ revoked |

## Sighting Cosecha

PPR #194 · CID `59606407-eed3-4da8-ac13-3cf6205b2147` · `persist_ref` `docs/fixes/bundle-consumer-telegram-gateway` · lectura empírica `.SddIA/cerbero/revoked_entities.json` en Cosecha.

Materialización: Cosecha Kaizen (Cúmulo) · `KAIZEN_COSECHA_GATE: APTO` · seed nueva (esta).

## Criterio de cierre (borrador)

- [ ] Laudo rehabilitación Cerbero / Radamanto (anti-recurrencia `abrupt_success_rate_drop`)
- [ ] `accept-pr` ausente de `revoked` (y `permanent` si aplica) en `revoked_entities.json`
- [ ] Cascada feature/fix + `validacion.md` APTO + PBI en `done/`
- [ ] Handoff `accept-pr` operable post-rehab (smoke merge lab sin re-revocación)
