---
document_id: PBI-PPR-174-177-REVOKED-PROCESS-THRESHOLDS
title: "[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177)"
format: markdown
version: "1.0.0"
created: "2026-08-16"
updated: "2026-08-17T06:20:00Z"
status: done
priority: media
process: refactorization
type: refactorization
uuid: ba900e95-1a47-4185-b86c-bc7a251b4fe6
suggested_branch: refactor/radamanto-process-threshold-rehab
persist_ref_suggested: docs/features/radamanto-process-threshold-rehab
persist_ref: docs/features/radamanto-process-threshold-rehab
branch_name: refactor/radamanto-process-threshold-rehab
consolidated_from:
  - PBI-PPR-174-REVOKED-REGISTRY
  - PBI-PPR-177-DCC-REVOKED-REGISTRY
olas:
  - id: ola-1
    document_id: PBI-PPR-174-REVOKED-REGISTRY
    uuid: e7a1c4b2-9f36-4d8e-a215-6c0b8d3e5f17
    entity: pull-request-review
    seed_pr: 174
    check: RBAC_PROCESS_REGISTRY
    since: "2026-08-15T08:40:55Z"
  - id: ola-2
    document_id: PBI-PPR-177-DCC-REVOKED-REGISTRY
    uuid: 9d2e4f81-6a3c-4b5e-8f17-c0a9d3e5b728
    entity: delivery-close-cycle
    seed_pr: 177
    check: RBAC_EMITTER_NOT_REVOKED
    since: "2026-08-16T16:40:55Z"
source_correlation_ids:
  - 6vw31k4eoXBCpXXfNWB4EL4FvtxYDbtZao4J47vwVPf8
  - 4b770fd6-99a0-435e-af43-a153aa23e310
  - 5ead1e57-67ec-496c-adb2-2a4bdcf1e3be
  - 94b7f03c-0e4d-4d40-a5c8-2936e29954f3
  - DLKDvjJ7pL86Z3eTdu8Cd3BBPmYZHQKAb89qENbcGzzt
  - DLKDvjJ7pL86Z3eTdu8Cd3BBPmYZHQKAb89qENbcGzzt
source_audits:
  - docs/features/kalma2-phase-barrier-timeout-persist/validacion.md
  - docs/fixes/centinelas-fracture-ola-20260812/validacion.md
incident_ref: "success_rate_below_threshold — process evaluado como tool; PPR #174+#177"
related:
  - .SddIA/cerbero/revoked_entities.json
  - .SddIA/radamanto/stats.json
  - SddIA/agents/radamanto.thresholds.json
  - SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs
  - SddIA/library/codexes/codex-software-engineering/process/pull-request-review.md
  - SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md
  - docs/todos/done/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #124).md
  - docs/todos/done/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #125).md
  - docs/todos/done/[ARQUITECTURA] delivery-close-cycle — revoked_entities y ECST signer (PPR #136).md
---

# [ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177)

PBI canónico. Absorbe `PBI-PPR-174-REVOKED-REGISTRY` y `PBI-PPR-177-DCC-REVOKED-REGISTRY` como **olas** del mismo ciclo `refactorization`. Un PR. Un `persist_ref`. Prohibido despachar ciclos `bug-fix` satélite.

## Fractura sistémica (SSOT)

Radamanto evalúa macro-procesos multi-fase con `success_rate_min` de herramienta atómica. Instancia etiqueta `entity_type: tool` a procesos. Resultado: re-revocación empírica post-rehab histórica (#124/#125 latency; #136 signer).

Exención vigente cubre **solo latency** (`LATENCY_THRESHOLD_EXEMPT`). Vector actual: `success_rate_below_threshold`.

## Mandato unificado

1. Laudo Cerbero: rehabilitar ambas entidades (no revocación permanente).
2. Ontología: `entity_type: process` en registro Cerbero / instancia (`process` ≠ `tool`).
3. Política Radamanto: tabla de umbrales diferenciada por tipo (`process` multi-fase > tolerancia que `tool` atómico).
4. Reset/redención stats (`pending_redemption` → `healthy`) sin reabrir vector `success_rate`.
5. Cascada `docs/features/radamanto-process-threshold-rehab/` + `validacion.md` APTO + este PBI → `done/` (satélites ola archivados en el mismo PR).

## Fuera de alcance (ciclo)

- Residual Kalma2 Shell/`git-manager` (dedup OPERATIVO PPR #136 done).
- Merge / handoff `accept-pr` de PR #174 / #177 históricos.
- Faros Kaizen (abajo): no implementar en este ciclo (Filtro C).

---

## Ola 1 — `pull-request-review` (PPR #174)

Satélite: `docs/todos/pending/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #174).md` (`document_id: PBI-PPR-174-REVOKED-REGISTRY`).

| Campo | Valor |
|-------|--------|
| Entidad | `pull-request-review` |
| Registro | `.SddIA/cerbero/revoked_entities.json` → `revoked.pull-request-review` |
| `entity_type` (instancia) | `tool` (entropía: es proceso) |
| `reason` | `success_rate_below_threshold` |
| `since` | `2026-08-15T08:40:55Z` |
| Radamanto | `status: pending_redemption` · `recovery_attempts: 2` · `rehab_laudo` legado `PBI-PPR-124+125-REVOKED-REGISTRY` |
| ECST origen | `6vw31k4eoXBCpXXfNWB4EL4FvtxYDbtZao4J47vwVPf8` · PR #174 |
| Check aduana | `RBAC_PROCESS_REGISTRY: NO_APTO` (Cerbero F4; no bloqueante) |

### Sightings (dedup Cosecha; sin seed nueva)

- PPR #174 · CID `6vw31k4eo…` (origen seed)
- PPR #175 · CID `7mDbYBoZi…` · `83b18b3a…`
- PPR #176 · CID `34bfbc96…` · `AuweRKSX…`
- PPR #177 · CID `4b770fd6…` · `6pKCg2PFF…`
- PPR #178 · CID `Eq9cotK1…` · `ca6fc6cb…`

Misma revocación `since 2026-08-15T08:40:55Z`.

### Intervención ola 1

1. Rehabilitar clave `pull-request-review` en instancia.
2. Fail-soft intra-proceso: fricción de sub-fase (API externa, lectura puntual) no colapsa la ejecución lineal; éxito parcial registrable.
3. Verificar `RBAC_PROCESS_REGISTRY: APTO` en aduana PPR posterior.

---

## Ola 2 — `delivery-close-cycle` (PPR #177)

Satélite: `docs/todos/pending/[ARQUITECTURA] delivery-close-cycle — rehabilitación revoked_entities (PPR #177).md` (`document_id: PBI-PPR-177-DCC-REVOKED-REGISTRY`).

| Campo | Valor |
|-------|--------|
| Entidad | `delivery-close-cycle` |
| Registro | `.SddIA/cerbero/revoked_entities.json` → `revoked.delivery-close-cycle` |
| `entity_type` (instancia) | `tool` (entropía: es proceso) |
| `reason` | `success_rate_below_threshold` |
| `since` | `2026-08-16T16:40:55Z` (re-revocación post-rehab; previo `16:11:08Z`) |
| ≠ incidente #136 done | #136: `abrupt_success_rate_drop` since `2026-07-13` (cerrado; check APTO residual abierto) |
| ECST origen | `4b770fd6-99a0-435e-af43-a153aa23e310` · PR #177 |
| Sighting adicional | `ca6fc6cb-4ecd-427f-9638-ae1960963cc3` · PR #178 |
| Sighting adicional | `5ead1e57-67ec-496c-adb2-2a4bdcf1e3be` · PR #180 (Cosecha Kaizen tekton-fire-and-forget) |
| Sighting adicional | `94b7f03c-0e4d-4d40-a5c8-2936e29954f3` · sibling `DLKDvjJ7…` · PR #181 (Cosecha Kaizen kaizen-pec-subscribers-circuit-audit) |
| Check aduana | `RBAC_EMITTER_NOT_REVOKED: NO_APTO` (Cerbero F4/F5; no bloqueante) |
| Firmante ECST | `Vertice_Biologico_Relay` (presente — E2 #136 liquidado) |

### Sightings ola 2 (dedup Cosecha; sin seed nueva)

- PPR #177 · CID `4b770fd6…` (origen seed)
- PPR #178 · CID `ca6fc6cb…`
- PPR #180 · CID `5ead1e57…` · sibling `5Zoqf2J6…` — misma clase DCC∈revoked `since 2026-08-16T16:40:55Z`
- PPR #181 · CID `94b7f03c…` (DCC) · sibling `DLKDvjJ7…` (GBW) — misma clase DCC∈revoked `since 2026-08-16T16:40:55Z`

### Intervención ola 2

1. Rehabilitar clave `delivery-close-cycle` en instancia.
2. Fail-soft de handoff: timeout no crítico en `telemetry_receipt` / validación de repo no impide emitir `PullRequest_Presented` firmado con `Vertice_Biologico_Relay` si commit/push cruzó umbral físico.
3. Verificar `RBAC_EMITTER_NOT_REVOKED: APTO` en aduana PPR posterior.

---

## Refinamiento S+ Grade (Protocolo de Acero)

Jurisprudencia única para ambas olas:

1. **Umbrales diferenciados (Radamanto):** `entity_type: process` + `success_rate` con mayor tolerancia para procesos multi-fase (latencia de red Git/GitHub, fases `agent:`, firma ECST).
2. **Kintsugi de fase:** fail-soft por ola (PPR interno; DCC handoff). No colapso térmico lineal.

### Faro Kaizen (descartado Filtro C — este ciclo)

3. Trocear `pull-request-review` en eventos atómicos EDA (`PullRequest_Analysis_Started` → `PullRequest_Diff_Extracted`).
4. Aislar `RBAC_EMITTER_NOT_REVOKED` en centinela EDA independiente.

## Criterio de cierre

- [x] Ola 1: `pull-request-review` rehabilitado; `entity_type: process`; `RBAC_PROCESS_REGISTRY: APTO` (instancia)
- [x] Ola 2: `delivery-close-cycle` rehabilitado; `entity_type: process`; `RBAC_EMITTER_NOT_REVOKED: APTO` (instancia)
- [x] Umbrales Radamanto versionados por tipo (anti-recurrencia `success_rate`)
- [x] `validacion.md` APTO · `pbi_archived: true` · PBI canónico + satélites ola en `docs/todos/done/` en la rama del PR
