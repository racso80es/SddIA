---
feature_name: accept-pr-revoked-registry-rehab-ppr208
created: "2026-08-28"
purpose: Estabilización Mayeuta — ola A1 PBI-PPR-208-ACCEPT-PR-REVOKED-REGISTRY (Yunque instancia accept-pr)
process: refactorization
phase: mayeuta-stabilization
agents: mayeuta
branch_name: refactor/accept-pr-revoked-registry-rehab-ppr208
persist_ref: docs/features/accept-pr-revoked-registry-rehab-ppr208
pbi_ref: docs/todos/pending/PBI-PPR-208-ACCEPT-PR-REVOKED-REGISTRY.md
document_id: PBI-PPR-208-ACCEPT-PR-REVOKED-REGISTRY
uuid: d4f8e2a1-6c39-4b7e-9a05-1f3c8d7e6b20
source_correlation_id: "4CMsk8z5Gx7mFQHc512m9FoJibvnr463cVyVcWz5imKm"
source_pr_url: https://github.com/racso80es/SddIA/pull/208
feature_ref: docs/features/kaizen-aduana-dlt-relay-supervisado
parent_pbi: docs/todos/done/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #203).md
incident_ref: "REVOKED_ENTITY_ALERT_ACCEPT_PR — accept-pr re-revoked post-rehab #203 (abrupt_success_rate_drop since 2026-08-27T18:21:13Z)"
ola: A1
olas:
  - A1
runtime_execution_id: "e1de4691-5b6f-495b-85ff-b6a52dcd11c4"
---

# Clarificación — ola A1 accept-pr-revoked-registry-rehab-ppr208

Transcript Mayeuta. **Qué / por qué.** Sin touchpoints de motor (#203 A2 ya en `main`).

## D0 — Semilla

| Vector | Hecho |
|--------|--------|
| PBI | `PBI-PPR-208-ACCEPT-PR-REVOKED-REGISTRY` · uuid `d4f8e2a1-…` · path canónico `docs/todos/pending/PBI-PPR-208-ACCEPT-PR-REVOKED-REGISTRY.md` |
| Ola | **A1** este `persist_ref`. A2 #203 = fail_soft sync post-merge — **no** reabrir. |
| Vehículo CLI | `--process feature` (`refactorization` ∈ revoked). `process_label: refactorization`. Lab-skip git/archive/DCC. |
| Padre | #203 done · rehab A1 @ `2026-08-27T16:04:48Z` |
| Sighting | PPR #208 · CID `4CMsk8z5Gx7mFQHc512m9FoJibvnr463cVyVcWz5imKm` · lateral `04ea6960-…` · re-revocación @ `18:21:13Z` (~2h post-rehab). Affirm Cosecha #210 CID `4c2dfd1d-…`. |

### Estado empírico (corte 2026-08-28)

| Clave | Valor |
|-------|--------|
| Cerbero | `revoked.accept-pr` · `entity_type: process` · `abrupt_success_rate_drop` · `since: 2026-08-27T18:21:13Z`. `permanent` ausente. |
| Radamanto | `degraded` · `recovery_attempts: 1` · `degraded_at: 18:21:13Z` · `rehab_laudo` fósil **#203** · `rehabilitated_at: 16:04:48Z` · `structure_valid: false` · samples mix exit 0/1 (7: 0,0,1,0,1,1,0) · rate **0,571** < 0,70 · n≥3. |

Dictamen: umbrales 1.1.0 intactos. Mix en ventana n≥3 reabre el vector. A1 sin poda = recidiva. A2 motor #203 cubre sync post-merge; este episodio = ventana sucia + laudo fósil, no nuevo payload.

## D1 — Misión

Rehabilitar `accept-pr` en Cerbero/Radamanto (Yunque). Merge/handoff soberano PR #208 fuera (L-HANDOFF-F5) hasta post-A1.

## D2 — Laudos Mayeuta (A1)

| ID | Decisión |
|----|----------|
| **L-UNIFY** | Un ciclo, un `persist_ref`, un PR. |
| **L-WAVES** | Solo **A1**. A2 = reutilizar #203. |
| **L-REHAB-INST** | Instancia `.SddIA/`. No versionar en PR. |
| **L-CERBERO** | DELETE `revoked.accept-pr`. Assert `permanent` ausente. |
| **L-STATS** | Reset **solo** bucket raíz `accept-pr`. |
| **L-RESET-ABS** | `healthy` · `recovery_attempts: 0` · `consecutive_success_count: 0` · `degraded_at: null` · `structure_valid: true` · `rehab_laudo: PBI-PPR-208-ACCEPT-PR-REVOKED-REGISTRY` · `rehabilitated_at` ISO A1. Sustituir fósiles #203. |
| **L-SAMPLES** | `samples: []` (preferente) o ≤3 últimos OK runtime. |
| **L-ONTOLOGY** | `entity_type: process`. |
| **L-LATERAL** | `revoked.{bug-fix,feature,refactorization}` intactos. |
| **L-HANDOFF** | No merge PR #208 en este ciclo. |
| **L-OUT** | Rehab `refactorization`; Kalma2 #136; umbrales; genoma. |
| **L-STOP** | Forja se detiene en `plan.md`. |

## D3 — Criterios (A1)

| ID | Criterio |
|----|----------|
| AC-A1 | `accept-pr` ∉ `revoked` ni `permanent`; stats `healthy`; laudo #208; samples podados; `structure_valid: true`. |
| AC-GIT-CLEAN | Instancia ausente del diff. |
| AC-ONTO | `entity_type: process`. |
| AC-DOC | Cascada A1 este `persist_ref`. |
