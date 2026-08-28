---
feature_name: accept-pr-revoked-registry-rehab-ppr208
created: "2026-08-28"
purpose: Estabilización Mayeuta — ola A1 PBI-PPR-208-ACCEPT-PR-REVOKED-REGISTRY (Yunque instancia accept-pr)
process: refactorization
phase: mayeuta-stabilization
agents: mayeuta
branch_name: refactor/accept-pr-revoked-registry-rehab-ppr208
persist_ref: docs/features/accept-pr-revoked-registry-rehab-ppr208
pbi_ref: docs/todos/pending/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #210).md
document_id: PBI-PPR-208-ACCEPT-PR-REVOKED-REGISTRY
uuid: d4f8e2a1-6c39-4b7e-9a05-1f3c8d7e6b20
source_correlation_id: "4CMsk8z5Gx7mFQHc512m9FoJibvnr463cVyVcWz5imKm"
source_pr_url: https://github.com/racso80es/SddIA/pull/208
feature_ref: docs/fixes/kaizen-aduana-dlt-relay-supervisado
parent_pbi: docs/todos/done/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #203).md
incident_ref: "REVOKED_ENTITY_ALERT_BUG_FIX — accept-pr re-revoked post-rehab #203 (abrupt_success_rate_drop since 2026-08-28T18:21:13Z)"
ola: A1
olas:
  - A1
runtime_execution_id: "e1de4691-5b6f-495b-85ff-b6a52dcd11c4"
---

# Clarificación — ola A1 accept-pr-revoked-registry-rehab-ppr208

Transcript Mayeuta. **Qué / por qué.** Sin touchpoints de motor.

## D0 — Semilla

| Vector | Hecho |
|--------|--------|
| PBI | `PBI-PPR-208-ACCEPT-PR-REVOKED-REGISTRY` · uuid `e7a1b2c3-…` · harvested |
| Ola | **A1** este `persist_ref`. Un PR. Stop planning esta sesión. |
| Vehículo CLI | `--process feature` (`refactorization` ∈ revoked · PPR #186 fuera). `process_label: refactorization`. Lab: `SDDIA_LAB_SKIP_GIT` (dirty-tree `main`) + skip archive/DCC + relevo IDE. |
| Padre | #203 done · rehab A1 @ `2026-08-27T11:45:00Z` |
| Sighting | PPR #210 · CID `4CMsk8z5Gx7mFQHc512m9FoJibvnr463cVyVcWz5imKm` · `PullRequest_Presented` @ `05:59:44Z` · re-revocación @ `18:21:13Z` |

### Estado empírico (corte 2026-08-28)

| Clave | Valor |
|-------|--------|
| Cerbero | `revoked.accept-pr` · `entity_type: process` · `abrupt_success_rate_drop` · `since: 2026-08-28T18:21:13Z`. `permanent.accept-pr` ausente. Laterales `revoked.{accept-pr,feature,refactorization}` **intactos**. |
| Radamanto | `degraded` · `recovery_attempts: 1` · `degraded_at: 18:21:13Z` · `rehab_laudo` fósil **#203** · `rehabilitated_at: 2026-08-27T11:45:00Z` · `structure_valid: false` · 4 samples **todos** `exit_code: 1` (70 ms / 57 ms / 108771 ms / 171799 ms) · rate **0** < `process: 0,70` · n≥3. |

Dictamen: umbrales 1.1.0 **intactos**. Ventana KO pura (incl. corridas largas) dispara el vector. A1 sin poda = recidiva inmediata. Semilla **no** aporta vector motor nuevo vs #203 (`L-TYPE-VERIFY` PASS histórico). Anti-recurrencia = reset absoluto + laudo #210 (sustituir fósil).

## D1 — Misión

Rehabilitar `accept-pr` en instancia Cerbero/Radamanto (Yunque). Prohibido despachar `accept-pr` como vehículo de este ciclo.

## D2 — Laudos Mayeuta (A1)

| ID | Decisión |
|----|----------|
| **L-UNIFY** | Un ciclo, un `persist_ref`, un PR. |
| **L-WAVES** | Solo **A1**. Prohibido inventar A2/A3. Hollow #185 y tipología #203 se reutilizan. |
| **L-REHAB-INST** | A1 = `.SddIA/` (Cúmulo `radamanto.revoked_entities` / `radamanto.stats`). Evidencia `execution.md`. **No** versionar en el PR. |
| **L-CERBERO** | DELETE `revoked.accept-pr`. Assert `permanent` ausente. |
| **L-STATS** | Reset **solo** bucket raíz `accept-pr`. |
| **L-RESET-ABS** | `healthy` · `recovery_attempts: 0` · `consecutive_success_count: 0` · `degraded_at: null` · `structure_valid: true` · `rehab_laudo: PBI-PPR-208-ACCEPT-PR-REVOKED-REGISTRY` · `rehabilitated_at` ISO A1. Sustituir fósiles #203. |
| **L-SAMPLES** | `samples: []` (preferente). Poda de los 4 KO actuales. |
| **L-ONTOLOGY** | `entity_type: process`. |
| **L-LATERAL** | `revoked.{accept-pr,feature,refactorization}` intactos (olas hermanas). |
| **L-VEHICLE** | Init vía `feature` + `process_label: refactorization`. Git formal en ejecución (T1+) cuando worktree limpio; esta sesión lab-skip git. |
| **L-OUT** | Rehab laterales; residual Kalma2; umbrales; genoma `{name}.md`; purga copia stale FIX pending. |
| **L-DOC** | Cascada patrón este `persist_ref`. Cierre PBI en rama **tras** ejecución (fuera de esta sesión). |
| **L-STOP** | Forja se detiene en `plan.md`. |

## D3 — Criterios (A1)

| ID | Criterio |
|----|----------|
| AC-A1 | `accept-pr` ∉ `revoked` ni `permanent`; stats `healthy`; laudo #210; samples `[]`; evidencia `execution.md`. |
| AC-GIT-CLEAN | Instancia ausente del diff del PR. |
| AC-ONTO | `entity_type: process`. |
| AC-DOC | Frontmatter patrón; no `pbi_archived` hasta archive. |
