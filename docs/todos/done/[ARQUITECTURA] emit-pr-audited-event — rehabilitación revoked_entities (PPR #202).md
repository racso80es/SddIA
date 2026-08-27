---
document_id: PBI-PPR-202-EMIT-PR-AUDITED-REVOKED-REGISTRY
title: "[ARQUITECTURA] emit-pr-audited-event — rehabilitación revoked_entities (PPR #202)"
format: markdown
version: "1.0.0"
created: "2026-08-27"
updated: "2026-08-27T12:21:30Z"
status: done
refinement_status: implemented
persist_ref: docs/features/emit-pr-audited-revoked-registry-rehab-ppr202
branch_name: refactor/emit-pr-audited-revoked-registry-rehab-ppr202
pbi_archived: true
priority: media
process: refactorization
type: refactorization
dispatch: true
uuid: c2e8f4a1-7b3d-4e9c-a5f6-8d1e2f3a4b5c
suggested_branch: refactor/emit-pr-audited-revoked-registry-rehab-ppr202
persist_ref_suggested: docs/features/emit-pr-audited-revoked-registry-rehab-ppr202
source_correlation_id: "1498e461-3235-483a-b210-907cca744cdd"
source_pr_url: https://github.com/racso80es/SddIA/pull/202
feature_ref: docs/features/accept-pr-revoked-registry-rehab-ppr200
incident_ref: "REVOKED_ENTITY_ALERT_EMIT_PR_AUDITED — emit-pr-audited-event ∈ revoked as tool (abrupt_success_rate_drop since 2026-06-12T10:10:06+00:00); alerta F5 PPR #202 sin PBI canónico previo"
entity: emit-pr-audited-event
related:
  - .SddIA/cerbero/revoked_entities.json
  - .SddIA/radamanto/stats.json
  - SddIA/actions/emit-pr-audited-event.md
  - SddIA/events/domain/pull-request-audited.md
  - docs/features/accept-pr-revoked-registry-rehab-ppr200/validacion.md
source_audits:
  - docs/features/accept-pr-revoked-registry-rehab-ppr200/validacion.md
  - .SddIA/cerbero/revoked_entities.json
---

# [ARQUITECTURA] emit-pr-audited-event — rehabilitación revoked_entities (PPR #202)

## Mandato

Rehabilitar la entidad `emit-pr-audited-event` en `.SddIA/cerbero/revoked_entities.json` tras revocación **`revoked`** (`abrupt_success_rate_drop`) etiquetada como `entity_type: tool` — deuda genérica lateral sin PBI canónico hasta Cosecha PPR #202.

| Campo | Valor |
|-------|--------|
| Entidad | `emit-pr-audited-event` |
| Registro | `.SddIA/cerbero/revoked_entities.json` → **`revoked.emit-pr-audited-event`** |
| `entity_type` (registry) | `tool` (fósil; acción Core — jurisprudencia #174/#194) |
| `reason` | `abrupt_success_rate_drop` |
| Since | `2026-06-12T10:10:06+00:00` |
| Check origen | `REVOKED_ENTITY_ALERT_EMIT_PR_AUDITED` (F5 Argos · alerta no bloqueante) + FS Cosecha #202 |
| Emisor ECST Presented | `delivery-close-cycle` ∉ revoked |
| Nota previa | ABSTRACT-03 D7 documentó «Sin seed — diseño aduana»; Cosecha #202 eleva a seed por alerta F5 + ausencia de PBI |

## Sighting Cosecha

PPR #202 · CID `1498e461-3235-483a-b210-907cca744cdd` · `persist_ref` `docs/features/accept-pr-revoked-registry-rehab-ppr200` · alerta lateral Cerbero elevada a seed (sin dedup previo en `docs/todos/`).

Materialización: Cosecha Kaizen (Cúmulo) · `KAIZEN_COSECHA_GATE: APTO` · seed nueva (esta).

## Sightings adicionales

| Sighting | CID | Nota |
|----------|-----|------|
| Cosecha Kaizen dedup | `3dcf4dfb-cd9c-4733-9925-b80f3f5806f4` | Re-run PPR #202 · seed ya materializada @ `1498e461…` · Cerbero `emit-pr-audited-event`∈revoked since `2026-06-12T10:10:06+00:00` · F5 lateral · 0 create |

## Criterio de cierre

- [x] Laudo rehabilitación Cerbero / Radamanto; alinear `entity_type` si procede (fósil `tool`)
- [x] `emit-pr-audited-event` ausente de `revoked`/`permanent` en `revoked_entities.json` (instancia; jurisprudencia `L-REHAB-INST`)
- [x] Cascada feature/refactor + `validacion.md` APTO + PBI en `done/`
- [x] Este TODO movido a `docs/todos/done/`

## Fuera de alcance

- Rehab `refactorization` (dedup done PPR #186).
- Residual Kalma2 Shell/`git-manager` (dedup OPERATIVO PPR #136).
- Reabrir alcance A1/A2 de accept-pr #200.
