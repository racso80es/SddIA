---
document_id: PBI-REFACTORIZATION-186-REVOKED-REGISTRY
title: "[ARQUITECTURA] refactorization — rehabilitación revoked_entities (PPR #186)"
format: markdown
version: "1.0.0"
created: "2026-08-20"
status: done
priority: media
process: refactorization
dispatch: true
uuid: a3f7b2c8-4e1d-4f59-9c6a-8d2e1f0b9a5c
suggested_branch: docs/features/feature-revoked-registry-rehab
persist_ref: docs/features/feature-revoked-registry-rehab
branch_name: refactor/refactorization-revoked-registry-rehab
source_correlation_id: CNwwfDm7Hqb1zd23zRtkjP2o7QqgR5PaH26YBpbN8Wz3
source_pr_url: https://github.com/racso80es/SddIA/pull/186
feature_ref: docs/features/feature-revoked-registry-rehab
incident_ref: "RBAC_PROCESS_SIGNER_REVOKED — process:refactorization ∈ revoked_entities.revoked since 2026-08-20T05:48:56Z (abrupt_success_rate_drop)"
entity: refactorization
parent_pbi: docs/todos/done/[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #185).md
related:
  - .SddIA/cerbero/revoked_entities.json
  - .SddIA/radamanto/stats.json
  - SddIA/agents/radamanto.thresholds.json
  - SddIA/library/codexes/codex-software-engineering/process/refactorization.md
  - SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs
  - SddIA/engine/execute-process/src/engine/thermodynamic.rs
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - SddIA/engine/execute-process/src/engine/residual_runner.rs
  - docs/features/feature-revoked-registry-rehab/execution.md
---

# [ARQUITECTURA] refactorization — rehabilitación revoked_entities (PPR #186)

## Mandato

Rehabilitar el proceso `refactorization` en `.SddIA/cerbero/revoked_entities.json` tras revocación **`revoked`** (`abrupt_success_rate_drop`).

| Campo | Valor |
|-------|--------|
| Entidad | `refactorization` |
| Registro | `.SddIA/cerbero/revoked_entities.json` → **`revoked.refactorization`** |
| `entity_type` | `process` |
| `reason` | `abrupt_success_rate_drop` |
| Since | `2026-08-20T05:48:56Z` |
| Check origen | `REVOKED_ENTITY_ALERT_REFACTORIZATION` (F4/F5 Cerbero/Argos · alerta no bloqueante) |

## Sighting Cosecha

PPR #186 · CID `CNwwfDm7Hqb1zd23zRtkjP2o7QqgR5PaH26YBpbN8Wz3` · ciclo authoring `refactorization` sobre `feature-revoked-registry-rehab` · A1 `feature` rehab materializada en instancia; motor A2/A3 comparte peaje con `refactorization` pero **no** borra la clave `revoked.refactorization`.

## Sightings adicionales

| Sighting | CID | Nota |
|----------|-----|------|
| Cosecha Kaizen dedup | `45c01cfe-4b80-4d5a-acbb-3b3ae64c7ed5` | Re-run PPR #185 · seed ya materializada @ `CNwwfDm7…` |
| Cosecha PR #202 | `1498e461-3235-483a-b210-907cca744cdd` | `@ 2026-08-27T12:20:00Z` · accept-pr-revoked-registry-rehab-ppr200 · 0 create · affirm #186 · Cerbero `refactorization`∈revoked since `2026-08-20T05:48:56Z` · F5 lateral |
| Cosecha Kaizen dedup | `3dcf4dfb-cd9c-4733-9925-b80f3f5806f4` | Re-run PPR #202 · 0 create · affirm #186 · Cerbero `refactorization`∈revoked since `2026-08-20T05:48:56Z` · F5 lateral |
| Cosecha Kaizen dedup | `6237015f-0f8d-42ea-97ea-a44afac5318d` | PPR #203 · emit-pr-audited-revoked-registry-rehab-ppr202 · 0 create · affirm #186 · Cerbero `refactorization`∈revoked since `2026-08-20T05:48:56Z` · F5 lateral |

## Contexto heredado (PBI-185)

El ciclo hermano `#185`/`#186` aplicó A2 fail-soft padre + A3 poda `survival_hollow` en motor. Reutilizar jurisprudencia `L-REHAB-INST` y laudos A1–A3 de `docs/todos/done/[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #185).md` adaptados a bucket stats raíz `refactorization`.

## Criterio de cierre

- [ ] A1 instancia: `refactorization` ∉ `revoked` ni `permanent` · stats raíz `healthy` · `recovery_attempts: 0` · `rehab_laudo` + `rehabilitated_at`
- [ ] A2/A3 motor: simetría peaje lifecycle (`residual_runner`, DCC anidado si aplica) sin regresionar rehab `feature`
- [ ] Argos APTO en `validacion.md` del ciclo
- [ ] Este TODO movido a `docs/todos/done/`
