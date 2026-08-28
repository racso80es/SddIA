---
feature_name: feature-revoked-registry-rehab-ppr210
created: "2026-08-28"
purpose: Estabilización Mayeuta — ola A1 PBI-PPR-210-FEATURE-REVOKED-REGISTRY (Yunque instancia feature)
process: refactorization
phase: mayeuta-stabilization
agents: mayeuta
branch_name: refactor/feature-revoked-registry-rehab-ppr210
persist_ref: docs/features/feature-revoked-registry-rehab-ppr210
pbi_ref: docs/todos/done/[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #210).md
document_id: PBI-PPR-210-FEATURE-REVOKED-REGISTRY
uuid: f8b2c3d4-5e6f-7a89-0b1c-2d3e4f5a6b7c
source_correlation_id: "4c2dfd1d-393d-4411-8956-d596ff0eef9c"
source_pr_url: https://github.com/racso80es/SddIA/pull/210
feature_ref: docs/fixes/route-domain-event-fracture-6a49e0ad
parent_pbi: docs/todos/done/[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #185).md
incident_ref: "REVOKED_ENTITY_ALERT_FEATURE — feature re-revoked post-rehab #185 (abrupt_success_rate_drop since 2026-08-28T05:25:41Z)"
ola: A1
olas:
  - A1
runtime_execution_id: "532a36c1-d46e-4c49-82ec-dbfc2ea50315"
---

# Clarificación — ola A1 feature-revoked-registry-rehab-ppr210

Transcript Mayeuta. **Qué / por qué.** Sin diseño de cápsulas.

## D0 — Semilla

| Vector | Hecho |
|--------|--------|
| PBI | `PBI-PPR-210-FEATURE-REVOKED-REGISTRY` · uuid `f8b2c3d4-…` |
| Ola | **A1** este `persist_ref`. A2/A3 #185 ya en `main` — no reabrir sin vector nuevo. |
| Vehículo CLI | `--process feature` (`refactorization` ∈ revoked). `process_label: refactorization`. |
| Padre | #185 done · A1–A3 @ `2026-08-20` |
| Sighting | PPR #210 · CID `4c2dfd1d-393d-4411-8956-d596ff0eef9c` · re-revocación @ `05:25:41Z` |

### Estado empírico (corte 2026-08-28)

| Clave | Valor |
|-------|--------|
| Cerbero | `revoked.feature` · `entity_type: process` · `since: 2026-08-28T05:25:41Z` |
| Radamanto | `degraded` · 8 samples KO · rate 0 |

Dictamen: A1 sin poda = recidiva. **No** inventar A2/A3 nuevos (#185 en `main`).

## D2 — Laudos Mayeuta (A1)

| ID | Decisión |
|----|----------|
| **L-REHAB-INST** | Instancia `.SddIA/`. No versionar en PR. |
| **L-CERBERO** | DELETE `revoked.feature`. |
| **L-RESET-ABS** | Laudo #210 · `samples: []`. |
| **L-STOP** | Planning ejecutado; T1–T5 en rama. |

## D3 — Criterios (A1)

| ID | Criterio |
|----|----------|
| AC-A1 | `feature` ∉ `revoked`/`permanent`; stats `healthy`. |
| AC-GIT-CLEAN | Instancia ausente del diff. |
