---
document_id: PBI-FEATURE-185-REVOKED-REGISTRY
title: "[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #185)"
format: markdown
version: "1.0.0"
created: "2026-08-19"
updated: "2026-08-19T15:52:00Z"
status: pending
priority: media
process: refactorization
dispatch: false
uuid: c8f4e2a1-7b3d-4e59-9f6a-2d1e0c9b8a7f
source_correlation_id: 17043d6d-c978-4245-b554-2c5edcf94422
source_pr_url: https://github.com/racso80es/SddIA/pull/185
feature_ref: docs/features/kaizen-capsula-imap-triaje
incident_ref: "RBAC_PROCESS_SIGNER_REVOKED — process:feature ∈ revoked_entities.permanent since 2026-08-19T07:59:05Z (max_recovery_attempts_exceeded)"
entity: feature
parent_pbi: "docs/todos/done/[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177).md"
related:
  - .SddIA/cerbero/revoked_entities.json
  - .SddIA/radamanto/stats.json
  - SddIA/library/codexes/codex-software-engineering/process/feature.md
---

# [ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #185)

## Mandato

Rehabilitar el proceso `feature` en `.SddIA/cerbero/revoked_entities.json` tras revocación **permanente** (`max_recovery_attempts_exceeded`).

| Campo | Valor |
|-------|--------|
| Entidad | `feature` |
| Registro | `.SddIA/cerbero/revoked_entities.json` → `revoked.feature` |
| Since | `2026-08-19T07:59:05Z` |
| Check origen | `RBAC_PROCESS_SIGNER_REVOKED` (F4 Cerbero · alerta auditoría no bloqueante) |

## Sighting Cosecha

PPR #185 · CID `17043d6d-c978-4245-b554-2c5edcf94422` · firmante tekton vía cadena `feature → tekton (+ entity-manager T4/T5)` · artefactos materializados pre/post revocación.

## Sightings adicionales

| Sighting | CID | Nota |
|----------|-----|------|
| Cosecha Kaizen dedup | `AicZf7SdgwpED4pyQq1KcmFUQHitACabDB8csNsmMTiC` | Re-run PPR #185 · seed ya materializada @ `17043d6d…` |

## Criterio de cierre

- [ ] `feature` ∉ `revoked_entities.permanent`
- [ ] Ontología Radamanto: `entity_type: process` (no `tool`) para macro-proceso `feature`
- [ ] Umbrales / stats auditados; redención sin reabrir vector `success_rate`

## Fuera de alcance

- Residual Kalma2 Shell/`git-manager` (dedup OPERATIVO PPR #136 done).
- Lab IMAP/Telegram vivo (`LAB_*_LIVE: DIFERIDO` en feature #185).
