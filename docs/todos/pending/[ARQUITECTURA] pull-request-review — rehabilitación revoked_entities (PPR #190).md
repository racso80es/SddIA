---
document_id: PBI-PPR-190-REVOKED-REGISTRY
title: "[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #190)"
format: markdown
version: "1.1.0"
created: "2026-08-25"
updated: "2026-08-26T08:30:00Z"
status: pending
priority: alta
process: refactorization
type: refactorization
dispatch: false
uuid: e2b9a4f1-7c83-4d5e-9a16-0f8b3c5d7e21
suggested_branch: refactor/ppr-revoked-registry-rehab-ppr190
persist_ref_suggested: docs/features/ppr-revoked-registry-rehab-ppr190
source_correlation_id: "5a4683c0-db46-4e8e-b5f4-b865ba417e0d"
source_pr_url: https://github.com/racso80es/SddIA/pull/190
feature_ref: docs/features/kaizen-paciente0-redeploy-20260825
incident_ref: "REVOKED_ENTITY_ALERT_PULL_REQUEST_REVIEW — pull-request-review ∈ permanent (max_recovery_attempts_exceeded since 2026-08-25T16:25:55Z) + revoked (abrupt_success_rate_drop since 2026-08-25T17:24:18Z); re-revocación post-rehab #174+#125+#124"
entity: pull-request-review
parent_pbi: docs/todos/done/[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177).md
related:
  - .SddIA/cerbero/revoked_entities.json
  - .SddIA/radamanto/stats.json
  - SddIA/agents/radamanto.thresholds.json
  - SddIA/library/codexes/codex-software-engineering/process/pull-request-review.md
  - docs/todos/done/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #174).md
  - docs/todos/done/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #125).md
  - docs/todos/done/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #124).md
  - docs/todos/done/[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177).md
  - docs/features/kaizen-paciente0-redeploy-20260825/validacion.md
source_audits:
  - docs/features/kaizen-paciente0-redeploy-20260825/validacion.md
  - .SddIA/cerbero/revoked_entities.json
---

# [ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #190)

## Mandato

Rehabilitar el proceso `pull-request-review` en `.SddIA/cerbero/revoked_entities.json` tras **re-revocación** post-cierre de olas #124/#125/#174 — buckets **permanent** y **revoked** simultáneos.

| Campo | Valor |
|-------|--------|
| Entidad | `pull-request-review` |
| Permanent | `reason: max_recovery_attempts_exceeded` · `since: 2026-08-25T16:25:55Z` |
| Revoked | `reason: abrupt_success_rate_drop` · `since: 2026-08-25T17:24:18Z` |
| `entity_type` | `process` |
| ≠ incidente #174 done | #174: `success_rate_below_threshold` since `2026-08-15T08:40:55Z` (cerrado) |
| ≠ incidente #125/#124 done | rehab conjunta latency/threshold (cerrado) |
| Check origen | empírico Cerbero en Cosecha Kaizen PPR #190 (alerta no bloqueante) |
| Emisor ECST | `github-bridge-watcher` ∉ revoked |

## Genealogía de revocaciones PPR

| Episodio | PR / ciclo | Bucket / reason | `since` | Estado |
|----------|------------|-----------------|---------|--------|
| #124/#125 | `ppr-rehab-revoked-entities` | registry / latency | 2026-07 | **done** |
| #174 (ola 1) | `radamanto-process-threshold-rehab` | revoked · `success_rate_below_threshold` | `2026-08-15T08:40:55Z` | **done** |
| **#190 (este PBI)** | Kaizen Paciente 0 redeploy | permanent · `max_recovery_attempts_exceeded` + revoked · `abrupt_success_rate_drop` | `16:25:55Z` / `17:24:18Z` | **abierto** |

## Sighting Cosecha

PPR #190 · CID `5a4683c0-db46-4e8e-b5f4-b865ba417e0d` · emisor ECST `github-bridge-watcher` ∉ revoked · lectura empírica `.SddIA/cerbero/revoked_entities.json` en Cosecha.

Materialización: Cosecha Kaizen (Cúmulo) · `KAIZEN_COSECHA_GATE: APTO` · `kaizen_seeds: 1`.

## Sightings adicionales

| Sighting | CID | Nota |
|----------|-----|------|
| Cosecha PR #193 | `d994ca73-e566-4955-bfe0-dc11678c7e87` | `@ 2026-08-26T08:30:00Z` · kaizen-aislamiento-multi-instancia · 0 create · affirm #190 · Cerbero permanent+revoked OK · Merged 3555239d |
| Cosecha reinject PR #192 | `d4f010fb-7118-4d9a-831f-1d1255b79465` | `@ 2026-08-26T07:35:00Z` · kaizen-ignicion-soberana-centinelas · 0 create · affirm #190 · Cerbero permanent+revoked OK |
| Cosecha PR #192 | `d4f010fb-7118-4d9a-831f-1d1255b79465` | `@ 2026-08-26T07:35:00Z` · kaizen-ignicion-soberana-centinelas · 0 create · FS permanent+revoked confirmado · peaje validacion KAIZEN_COSECHA_GATE |
| Cosecha PR #192 | `d4f010fb-7118-4d9a-831f-1d1255b79465` | `@ 2026-08-26T05:55:00Z` · kaizen-ignicion-soberana-centinelas · 0 create · FS permanent+revoked confirmado · affirm/dedup seed #190 |
| Cosecha PR #192 affirm | `d4f010fb-7118-4d9a-831f-1d1255b79465` | `@ 2026-08-26T07:36:00Z` · idempotent affirm · 0 create · Cerbero permanent+revoked reconfirmado · peaje KAIZEN_COSECHA_GATE |
| Cosecha reinject | `5a4683c0-db46-4e8e-b5f4-b865ba417e0d` | `@ 2026-08-25T19:28:30Z` · seed viva v1.0.1 · 0 create · FS permanent+revoked confirmado |
| Cosecha reinject | `5a4683c0-db46-4e8e-b5f4-b865ba417e0d` | `@ 2026-08-25T17:32:40Z` · stamp `_cumulo_cosecha_session_5a4683c0_173240.md` · 0 create · affirm seed #190 · Cerbero permanent+revoked OK |
| Cosecha reinject | `5a4683c0-db46-4e8e-b5f4-b865ba417e0d` | `@ 2026-08-25T19:28:50Z` · stamp `_cumulo_cosecha_session_5a4683c0_192850.md` · 0 create · peaje SSOT alineado seeds=1 |
| Cosecha reinject | `5a4683c0-db46-4e8e-b5f4-b865ba417e0d` | `@ 2026-08-25T17:32:45Z` · stamp `_cumulo_cosecha_session_5a4683c0_173245.md` · 0 create · affirm #190 · Cerbero OK |

## Criterio de cierre (borrador)

- [ ] Laudo rehabilitación Cerbero / Radamanto (anti-recurrencia `abrupt_success_rate_drop` + `max_recovery_attempts_exceeded`)
- [ ] `pull-request-review` ausente de `permanent` y `revoked` en `revoked_entities.json`
- [ ] Cascada feature/fix + `validacion.md` APTO + PBI en `done/`
