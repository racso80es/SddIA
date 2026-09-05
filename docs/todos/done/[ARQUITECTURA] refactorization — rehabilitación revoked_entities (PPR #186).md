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
| Cosecha Kaizen dedup | `1e9972cf-2ffd-47f0-8cf8-c9427e7023d8` | PPR #206 · accept-pr-revoked-registry-rehab-ppr203 · 0 create · affirm #186 · Cerbero `refactorization`∈revoked since `2026-08-20T05:48:56Z` · F5 lateral |
| Cosecha Kaizen dedup | `4CMsk8z5Gx7mFQHc512m9FoJibvnr463cVyVcWz5imKm` | PPR #208 · kaizen-aduana-dlt-relay-supervisado · 0 create · affirm #186 · Cerbero `refactorization`∈revoked since `2026-08-20T05:48:56Z` · F5 lateral · seed accept-pr #208 aparte |
| Cosecha Kaizen dedup | `04ea6960-3089-4523-9461-cccfbe202f1a` | PPR #208 · kaizen-aduana-dlt-relay-supervisado · 0 create · affirm #186 · Cerbero `refactorization`∈revoked since `2026-08-20T05:48:56Z` · F5 lateral |
| Cosecha Kaizen dedup | `7d9260e1-b308-4312-80ee-bf2b5fa0a0ad` | PPR #211 · bug-fix-revoked-registry-rehab-ppr210 · 0 create · affirm #186 · Cerbero `refactorization`∈revoked since `2026-08-20T05:48:56Z` · F5 lateral · `bug-fix`∉revoked (healthy) |
| Cosecha Kaizen dedup | `HL5aytKdLW8NosiKrUjvaxJtQxzrb8XsAsQMz45sm6ps` | PPR #212 · feature-revoked-registry-rehab-ppr210 · 0 create · affirm #186 · Cerbero `refactorization`∈revoked since `2026-08-20T05:48:56Z` · F5 lateral · `feature`∉revoked (healthy · rehab A1 @ `06:13:50Z`) · execution `5beaf62e…` |
| Cosecha Kaizen dedup | `HAS1wo6bQXZUdH7MQHMzLsyTTcDraMU2UAoyVjffAhth` | PPR #208 · accept-pr-revoked-registry-rehab-ppr208 · 0 create · affirm #186 · Cerbero `refactorization`∈revoked since `2026-08-20T05:48:56Z` · F5 lateral · `accept-pr`∉revoked (healthy · rehab A1 @ `06:13:50Z`) · execution `9cdc6d8a…` |
| Cosecha Kaizen dedup | `8ZjTzcBwfFAVFQujfjGCJwJeJcj5pbB4SMHAD5bn5ybE` | PPR kaizen-aduana-evolution-local · 0 create · affirm #186 · Cerbero `refactorization`∈revoked since `2026-08-20T05:48:56Z` · F5 lateral · seed PPR aparte |
| Cosecha Kaizen dedup | `600cd25c-7d3d-4be4-a53b-54a9ff64be51` | PPR #247 restore-pbi-kaizen-ci-step-archive · 0 create · affirm #186 · Cerbero `refactorization`∈revoked since `2026-08-20T05:48:56Z` · F5 lateral · seed PPR pending aparte · exec `ab27081e…` |
| Cosecha Kaizen dedup | `AU1AzkrREQVTRhGHexuqiumPXPw8iP2SgCSLB7AcFKfc` | PPR restore-pbi-kaizen-ci-step-archive · 0 create · affirm #186 · Cerbero `refactorization`∈revoked since `2026-08-20T05:48:56Z` · F5 lateral · seed PPR aparte (`PBI-RESTORE-…-PPR-REVOKED-REGISTRY`) · exec `a315ae3e…` |
| Cosecha Kaizen dedup | `064918a2-af08-441f-a5b5-d34ad312c489` | PPR #251 ignition-pre-push-guard · 0 create · affirm #186 · Cerbero `refactorization`∈revoked since `2026-08-20T05:48:56Z` · F5 lateral · seed PPR pending aparte · exec `d712f728…` · Presented `2Wkh9xq…` |
| Cosecha Kaizen dedup | `2Wkh9xqgpu1C8LPAhWzfrvL8LQXdTa5Rz55r81GWReda` | PPR #251 ignition-pre-push-guard · 0 create · affirm #186 · Cerbero `refactorization`∈revoked since `2026-08-20T05:48:56Z` · F5 lateral · seed PPR dedup pending · exec `7dd9caa4…` |
| Cosecha Kaizen dedup | `cf977edc-706b-4b01-ba70-4beec1fcca82` | PPR #251 dcc-lab-residual-capsules · 0 create · affirm #186 · Cerbero `refactorization`∈revoked since `2026-08-20T05:48:56Z` · F5 lateral · seed PPR dedup pending · exec `95a54dc9…` · emitter `delivery-close-cycle` |
| Cosecha Kaizen dedup | `c368985f-2c03-4852-a9aa-0bc363f6c94e` | PPR docs/ppr-cosecha-kaizen-20260904 · 0 create · affirm #186 · Cerbero `refactorization`∈revoked since `2026-08-20T05:48:56Z` · F5 lateral · seed PPR dedup pending · exec `db1b9e3f…` · emitter `git-hook-pre-push` · ECST `Local_QA_Requested` |
| Cosecha Kaizen dedup | `2dYUXN7nq9DsKAs7xZ5vWgq5G3fkfErbwT2Ke2ffnv3Z` | PPR docs/ppr-cosecha-kaizen-20260904 · 0 create · affirm #186 · Cerbero `refactorization`∈revoked since `2026-08-20T05:48:56Z` · F5 lateral · seed PPR dedup pending · exec `66954b4b…` · PR #253 · emitter `github-bridge-watcher` · `PullRequest_Presented` |
| Cosecha Kaizen dedup | `2fad80c0-6ee1-42a2-8d6f-c1399113fbdc` | PPR docs/ppr-cosecha-kaizen-20260904 · 0 create · affirm #186 · Cerbero `refactorization`∈revoked since `2026-08-20T05:48:56Z` · F5 lateral · seed PPR dedup pending · exec `72f5e494…` · PR #253 · emitter `delivery-close-cycle` (∈revoked L-OUT) · `PullRequest_Presented` |
| Cosecha Kaizen dedup | `7293fada-4fbc-4aac-8881-8061e9c0583d` | PPR docs/ppr-cosecha-kaizen-20260904 · 0 create · affirm #186 · Cerbero `refactorization`∈revoked since `2026-08-20T05:48:56Z` · F5 lateral · seed PPR dedup pending · exec `e21fc03d…` · PR #253 · emitter `delivery-close-cycle` (∈revoked L-OUT) · `PullRequest_Presented` |
| Cosecha Kaizen dedup | `9c9cd653-dabe-4fe2-a54d-17f868cd427e` | PPR docs/ppr-cosecha-kaizen-20260904 · 0 create · affirm #186 · Cerbero `refactorization`∈revoked since `2026-08-20T05:48:56Z` · F5 lateral · seed PPR dedup pending · exec `6362eb00…` · PR #253 · emitter `delivery-close-cycle` (∈revoked L-OUT) · `PullRequest_Presented` |
| Cosecha Kaizen dedup | `74a57c11-6764-4a6a-92e6-7943faa48d35` | PPR docs/ppr-cosecha-kaizen-20260904 · 0 create · affirm #186 · Cerbero `refactorization`∈revoked since `2026-08-20T05:48:56Z` · F5 lateral · seed PPR dedup pending · exec `e431afdf…` · emitter `git-hook-pre-push` · ECST `Local_QA_Requested` · sibling `8d2567b6…` |
| Cosecha Kaizen dedup | `74a57c11-6764-4a6a-92e6-7943faa48d35` | PPR docs/ppr-cosecha-kaizen-20260904 · 0 create · affirm #186 · Cerbero `refactorization`∈revoked since `2026-08-20T05:48:56Z` · F5 lateral · seed PPR dedup pending · exec `8d2567b6…` · emitter `git-hook-pre-push` · ECST `Local_QA_Requested` · sibling `e431afdf…` |
| Cosecha Kaizen dedup | `DK5QuSSudtQmSiSMZikUXN83xiF7fwEHxGHGRCUBz1tm` | PPR docs/ppr-cosecha-kaizen-20260904 · 0 create · affirm #186 · Cerbero `refactorization`∈revoked since `2026-08-20T05:48:56Z` · F5 lateral · seed PPR dedup pending · exec `0b826e3b…` · PR #255 · emitter `github-bridge-watcher` · `PullRequest_Presented` · sibling `d50a40ba…` |
| Cosecha Kaizen dedup | `e4c9970f-9e15-40fe-857f-07c44c1bada5` | PPR docs/ppr-cosecha-kaizen-20260904 · 0 create · affirm #186 · Cerbero `refactorization`∈revoked since `2026-08-20T05:48:56Z` · F5 lateral · seed PPR dedup pending · exec `d50a40ba…` · PR #255 · emitter `delivery-close-cycle` (∈revoked L-OUT) · `PullRequest_Presented` · sibling `0b826e3b…` |

## Contexto heredado (PBI-185)

El ciclo hermano `#185`/`#186` aplicó A2 fail-soft padre + A3 poda `survival_hollow` en motor. Reutilizar jurisprudencia `L-REHAB-INST` y laudos A1–A3 de `docs/todos/done/[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #185).md` adaptados a bucket stats raíz `refactorization`.

## Criterio de cierre

- [ ] A1 instancia: `refactorization` ∉ `revoked` ni `permanent` · stats raíz `healthy` · `recovery_attempts: 0` · `rehab_laudo` + `rehabilitated_at`
- [ ] A2/A3 motor: simetría peaje lifecycle (`residual_runner`, DCC anidado si aplica) sin regresionar rehab `feature`
- [ ] Argos APTO en `validacion.md` del ciclo
- [ ] Este TODO movido a `docs/todos/done/`
