---
document_id: PBI-PPR-194-BUG-FIX-REVOKED-REGISTRY
title: "[ARQUITECTURA] bug-fix — rehabilitación revoked_entities (PPR #194)"
format: markdown
version: "1.0.0"
created: "2026-08-26"
updated: "2026-08-27T11:49:44Z"
status: done
refinement_status: implemented
persist_ref: docs/features/bug-fix-revoked-registry-rehab-ppr194
branch_name: refactor/bug-fix-revoked-registry-rehab-ppr194
pbi_archived: true
priority: media
process: refactorization
type: refactorization
dispatch: false
uuid: 8a4b0d3f-5c2e-4f9b-8d6a-7e8f9a0b1c2d
suggested_branch: refactor/bug-fix-revoked-registry-rehab-ppr194
persist_ref_suggested: docs/features/bug-fix-revoked-registry-rehab-ppr194
source_correlation_id: "59606407-eed3-4da8-ac13-3cf6205b2147"
source_pr_url: https://github.com/racso80es/SddIA/pull/194
feature_ref: docs/fixes/bundle-consumer-telegram-gateway
incident_ref: "REVOKED_ENTITY_ALERT_BUG_FIX — bug-fix ∈ revoked as tool (abrupt_success_rate_drop since 2026-08-16T16:09:32Z); alerta F4/F5 PPR #194 sin PBI canónico previo"
entity: bug-fix
related:
  - .SddIA/cerbero/revoked_entities.json
  - .SddIA/radamanto/stats.json
  - SddIA/agents/radamanto.thresholds.json
  - SddIA/library/codexes/codex-software-engineering/process/bug-fix.md
  - docs/fixes/bundle-consumer-telegram-gateway/validacion.md
  - docs/features/accept-pr-revoked-registry-rehab-ppr194/validacion.md
source_audits:
  - docs/fixes/bundle-consumer-telegram-gateway/validacion.md
  - docs/features/accept-pr-revoked-registry-rehab-ppr194/validacion.md
  - .SddIA/cerbero/revoked_entities.json
---

# [ARQUITECTURA] bug-fix — rehabilitación revoked_entities (PPR #194)

## Mandato

Rehabilitar la entidad `bug-fix` en `.SddIA/cerbero/revoked_entities.json` tras revocación **`revoked`** (`abrupt_success_rate_drop`) etiquetada como `entity_type: tool` — deuda genérica sin PBI canónico hasta Cosecha PPR #194.

| Campo | Valor |
|-------|--------|
| Entidad | `bug-fix` |
| Registro | `.SddIA/cerbero/revoked_entities.json` → **`revoked.bug-fix`** |
| `entity_type` (registry) | `tool` (fósil; post-A1 stats `process` — jurisprudencia #174) |
| `reason` | `abrupt_success_rate_drop` |
| Since | `2026-08-16T16:09:32Z` |
| Check origen | `REVOKED_ENTITY_ALERT_BUG_FIX` (F4/F5 Cerbero/Argos · alerta no bloqueante) + FS Cosecha #194 |
| Emisor ECST Presented | `delivery-close-cycle` ∉ revoked |

## Sighting Cosecha

PPR #194 · CID `59606407-eed3-4da8-ac13-3cf6205b2147` · `persist_ref` `docs/fixes/bundle-consumer-telegram-gateway` · alerta lateral Cerbero elevada a seed (sin dedup previo en `docs/todos/`).

Materialización: Cosecha Kaizen (Cúmulo) · `KAIZEN_COSECHA_GATE: APTO` · seed nueva (esta).

## Sightings adicionales

| Sighting | CID | Nota |
|----------|-----|------|
| Cosecha PR #200 | `7c215675-2ad2-436a-9749-ff635c52c8b3` | `@ 2026-08-27T11:42:00Z` · accept-pr-revoked-registry-rehab-ppr194 · 0 create · affirm #194 · Cerbero `bug-fix`∈revoked since `2026-08-16T16:09:32Z` · F5 lateral |

## Criterio de cierre

- [x] Laudo rehabilitación Cerbero / Radamanto; `entity_type: process` (fósil `tool` borrado)
- [x] `bug-fix` ausente de `revoked`/`permanent` en `revoked_entities.json` (instancia; fuera del PR)
- [x] Cascada feature + `validacion.md` APTO + PBI en `done/`
