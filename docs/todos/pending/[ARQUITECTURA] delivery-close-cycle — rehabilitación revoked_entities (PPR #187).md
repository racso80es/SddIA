---
document_id: PBI-PPR-187-DCC-REVOKED-REGISTRY
title: "[ARQUITECTURA] delivery-close-cycle — rehabilitación revoked_entities (PPR #187)"
format: markdown
version: "1.0.0"
created: "2026-08-20"
updated: "2026-08-20T14:15:00Z"
status: abierto
priority: media
process: refactorization
dispatch: true
uuid: c4a91e7b-2f68-4d3a-a8e1-5b7c9d0e2f14
source_correlation_id: 4gKBTRCyZzvEFQcbDWFnBmdC3ZjvqTJmauHiYgTWwj32
source_pr_url: https://github.com/racso80es/SddIA/pull/187
feature_ref: docs/features/kaizen-consumer-ignition-filtro-c
incident_ref: "REVOKED_ENTITY_ALERT_DELIVERY_CLOSE_CYCLE — delivery-close-cycle ∈ revoked since 2026-08-20T12:04:10Z (abrupt_success_rate_drop); re-revocación post-rehab #174+#177"
entity: delivery-close-cycle
related:
  - .SddIA/cerbero/revoked_entities.json
  - .SddIA/radamanto/stats.json
  - SddIA/agents/radamanto.thresholds.json
  - SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md
  - docs/todos/done/[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177).md
  - docs/todos/done/[ARQUITECTURA] delivery-close-cycle — rehabilitación revoked_entities (PPR #177).md
  - docs/todos/done/[ARQUITECTURA] delivery-close-cycle — revoked_entities y ECST signer (PPR #136).md
---

# [ARQUITECTURA] delivery-close-cycle — rehabilitación revoked_entities (PPR #187)

## Mandato

Rehabilitar el proceso `delivery-close-cycle` en `.SddIA/cerbero/revoked_entities.json` tras **re-revocación** post-cierre de ola 2 (#174+#177).

| Campo | Valor |
|-------|--------|
| Entidad | `delivery-close-cycle` |
| Registro | `.SddIA/cerbero/revoked_entities.json` → **`revoked.delivery-close-cycle`** |
| `entity_type` | `process` |
| `reason` | `abrupt_success_rate_drop` |
| Since | `2026-08-20T12:04:10Z` |
| ≠ incidente #177 done | #177: `success_rate_below_threshold` since `2026-08-16T16:40:55Z` (cerrado) |
| Check origen | `REVOKED_ENTITY_ALERT_DELIVERY_CLOSE_CYCLE` (F4/F5 · alerta no bloqueante) |

## Sighting Cosecha

PPR #187 · CID `4gKBTRCyZzvEFQcbDWFnBmdC3ZjvqTJmauHiYgTWwj32` · emisor ECST `github-bridge-watcher` ∉ revoked · `PullRequest_Presented` @ `2026-08-20T12:04:09Z` · clave DCC revocada 1s después (`12:04:10Z`).

## Sightings adicionales

| Sighting | CID | Nota |
|----------|-----|------|
| Cosecha Kaizen gemelo | `34736c88-34d3-46f8-a050-75e7775d005b` | Dedup; seed ya materializada @ `4gKBTRCy…` |

## Contexto heredado

Ola 2 (#174+#177) liquidó umbrales Radamanto + rehab instancia DCC. Esta cicatriz es **episodio nuevo** (`since` distinto); no deduplicar contra satélite done #177.

## Criterio de cierre

- [ ] A1 instancia: `delivery-close-cycle` ∉ `revoked` ni `permanent` · stats raíz `healthy` · `recovery_attempts: 0` · `rehab_laudo` + `rehabilitated_at`
- [ ] Laudo anti-recurrencia: umbrales/fail-soft DCC no reintroducen `abrupt_success_rate_drop` en el mismo peaje
- [ ] Argos APTO en `validacion.md` del ciclo
- [ ] Este TODO movido a `docs/todos/done/`
