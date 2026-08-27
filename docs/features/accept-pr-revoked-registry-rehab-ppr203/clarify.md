---
feature_name: accept-pr-revoked-registry-rehab-ppr203
created: "2026-08-27"
purpose: Estabilización Mayeuta — ola A1 PBI-PPR-203-ACCEPT-PR-REVOKED-REGISTRY (Yunque Rúnico instancia accept-pr)
process: refactorization
phase: mayeuta-stabilization
agents: mayeuta
branch_name: refactor/accept-pr-revoked-registry-rehab-ppr203
persist_ref: docs/features/accept-pr-revoked-registry-rehab-ppr203
pbi_ref: docs/todos/pending/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #203).md
pbi_alias: docs/todos/pending/PBI-PPR-203-ACCEPT-PR-REVOKED-REGISTRY.md
document_id: PBI-PPR-203-ACCEPT-PR-REVOKED-REGISTRY
uuid: b7e4a91c-2f5d-4c8b-9e1a-6d3f0a8b2c7e
source_correlation_id: "6237015f-0f8d-42ea-97ea-a44afac5318d"
source_pr_url: https://github.com/racso80es/SddIA/pull/203
feature_ref: docs/features/emit-pr-audited-revoked-registry-rehab-ppr202
parent_pbi: docs/todos/done/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #200).md
incident_ref: "REVOKED_ENTITY_ALERT_ACCEPT_PR — accept-pr re-revoked post-rehab #200 / post-merge PR #203 (abrupt_success_rate_drop since 2026-08-27T12:31:30Z)"
ola: A1
olas:
  - A1
runtime_execution_id: "2363d1e8-8fd0-4863-93b7-33eea61087a3"
---

# Clarificación — ola A1 accept-pr-revoked-registry-rehab-ppr203

Transcript Mayeuta. **Qué / por qué.** Sin touchpoints de motor (jurisdicción ola A2).

## D0 — Semilla

| Vector | Hecho |
|--------|--------|
| PBI | Un `document_id` / un `uuid`. Canónico con corchetes; alias `PBI-PPR-203-ACCEPT-PR-REVOKED-REGISTRY.md` (Write Rejected histórico). **No** dos PBI. |
| Olas | A1 este `persist_ref`. A2 `docs/features/accept-pr-anti-recurrence-ppr203`. Misma rama. |
| Vehículo CLI | `--process feature` (`refactorization` ∈ `revoked` · PPR #186 fuera). `process_label: refactorization`. |
| Padre | #200 done · merge `42fff076…` · A1+A2 sello fail_soft. |
| Sighting | PPR #203 · CID `6237015f-…` · merge `120d741c33fe8c3e6e8b9fc423651c0f8768f446` @ `12:31:26Z` · re-revocación `12:31:30Z` (~4s). |

### Estado empírico (corte 2026-08-27)

| Clave | Valor |
|-------|--------|
| Cerbero | `revoked.accept-pr` · `entity_type: process` · `abrupt_success_rate_drop` · `since: 2026-08-27T12:31:30Z`. `permanent.accept-pr` ausente. Lateral `revoked.refactorization` intacto. |
| Radamanto | `degraded` · `recovery_attempts: 1` · `degraded_at: 12:31:30Z` · `rehab_laudo` fósil **#200** · `rehabilitated_at: 12:00:00Z` · `structure_valid: false` · samples 3 (0 / 1 / 0) · rate **0,667** < `process: 0,70` · n≥3. |

Dictamen: umbrales 1.1.0 **intactos**. Un KO en ventana de 3 dispara el vector. A1 sin poda = recidiva al primer sample KO residual.

## D1 — Misión

Rehabilitar `accept-pr` en instancia Cerbero/Radamanto (Yunque). Corte de re-muerte motor = **ola A2**.

## D2 — Laudos Mayeuta (A1)

| ID | Decisión |
|----|----------|
| **L-SPLIT** | Dos olas / dos `persist_ref` / **una rama** / un PBI. Prohibido mezclar mutación motor en A1. |
| **L-DEDUP** | Archive solo path canónico con corchetes; alias no es segundo PBI. |
| **L-REHAB-INST** | A1 = `.SddIA/` (Cúmulo `radamanto.revoked_entities` / `radamanto.stats`). Evidencia `execution.md`. **No** versionar en el PR. |
| **L-CERBERO** | DELETE `revoked.accept-pr`. Assert `permanent` ausente. Cerbero no tiene `healthy`. |
| **L-STATS** | Reset **solo** bucket raíz `accept-pr`. |
| **L-RESET-ABS** | `healthy` · `recovery_attempts: 0` · `consecutive_success_count: 0` · `degraded_at: null` · `structure_valid: true` · `rehab_laudo: PBI-PPR-203-ACCEPT-PR-REVOKED-REGISTRY` · `rehabilitated_at` ISO A1. Sustituir fósiles #200. |
| **L-SAMPLES** | `samples: []` (preferente) o ≤3 últimos OK runtime. Poda KO `5d6f7cb3…`. |
| **L-ONTOLOGY** | `entity_type: process`. |
| **L-ORDER** | A2 motor **antes** de A1 en el mismo host (anti 4s). A1 sola reabre el vector. |
| **L-OUT** | Rehab `refactorization`; residual Kalma2; reabrir A1 #202; umbrales; genoma `{name}.md`. |
| **L-DOC** | Cascada patrón este `persist_ref`. Cierre PBI en rama **tras** A2 (un PR). |

## D3 — Criterios (A1)

| ID | Criterio |
|----|----------|
| AC-A1 | `accept-pr` ∉ `revoked` ni `permanent`; stats raíz `healthy`; laudo #203; samples podados; evidencia `execution.md`. |
| AC-GIT-CLEAN | Instancia ausente del diff del PR. |
| AC-ONTO | `entity_type: process`. |
| AC-DOC | Frontmatter patrón; no `pbi_archived` hasta archive conjunto. |
