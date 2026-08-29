---
feature_name: ppr-revoked-registry-rehab-kaizen-aduana-evolution
created: "2026-08-29"
purpose: Estabilización Mayeuta — PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL-PPR-REVOKED-REGISTRY (rehab PPR + A2 condicionada)
process: refactorization
phase: mayeuta-stabilization
agents: mayeuta
branch_name: refactor/ppr-revoked-registry-rehab-kaizen-aduana-evolution
persist_ref: docs/features/ppr-revoked-registry-rehab-kaizen-aduana-evolution
pbi_ref: docs/todos/pending/PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL-PPR-REVOKED-REGISTRY.md
document_id: PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL-PPR-REVOKED-REGISTRY
uuid: c4e8f1a2-9b3d-4f7e-a6c1-2d8e5f0b3a71
source_correlation_id: "8ZjTzcBwfFAVFQujfjGCJwJeJcj5pbB4SMHAD5bn5ybE"
feature_ref: docs/fixes/kaizen-aduana-evolution-local
parent_pbi: docs/todos/done/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #190).md
incident_ref: "REVOKED_ENTITY_ALERT_PULL_REQUEST_REVIEW — re-revoked post-rehab #190 (success_rate_below_threshold since 2026-08-28T10:10:42Z)"
ola: A1
olas:
  - A1
  - A2
runtime_execution_id: "aa0d1244-043a-421f-9b60-efb76c4985ca"
---

# Clarificación — ppr-revoked-registry-rehab-kaizen-aduana-evolution

Transcript Mayeuta. **Qué / por qué.** Touchpoints de motor solo como hipótesis A2 (no ejecución esta sesión).

## D0 — Semilla

| Vector | Hecho |
|--------|--------|
| PBI | `PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL-PPR-REVOKED-REGISTRY` · uuid `c4e8f1a2-…` · `refinement_status: dedalo_ready` |
| Olas | **A1** Yunque instancia (innegociable). **A2** motor condicionada a laudo (PBI hijo si se recorta). |
| Vehículo CLI | `--process feature` (`refactorization` ∈ revoked · PPR #186 fuera). `process_label: refactorization`. Relé IDE + skip archive/DCC. |
| Padre | #190 done · rehab @ `2026-08-26T18:02:03Z` **sin poda de samples** |
| Sighting | Cosecha `kaizen-aduana-evolution-local` · CID `8ZjTzcBwfFAVFQujfjGCJwJeJcj5pbB4SMHAD5bn5ybE` · F4 `FAIL_F4_RBAC` · Handoff prohibido |

### Estado empírico (corte 2026-08-29T04:33:37Z)

| Clave | Valor |
|-------|--------|
| Cerbero | `revoked.pull-request-review` · `process` · `success_rate_below_threshold` · `since: 2026-08-28T10:10:42Z`. `permanent` vacío. Laterales `revoked.{bug-fix,refactorization}` **intactos**. |
| Radamanto | `degraded` · `recovery_attempts: 1` · `structure_valid: false` · laudo fósil **#190** · `rehabilitated_at: 2026-08-26T18:02:03Z` · 20 samples (5/15) · rate **0.25** < `process: 0.70` |
| Recidiva | Determinista: `success_rate()` evalúa ventana completa. Receta #208/#210 (`samples: []`) **no** aplicada en #190. |

Dictamen: umbrales 1.1.0 **intactos**. A1 sin `L-SAMPLES` = recidiva inmediata. Hipótesis A2 (KO cortos 636–1301 ms = aborto F4) **no concluyente** — eventos `Raw_Execution_Finished` purgados.

## D1 — Misión

Rehabilitar `pull-request-review` en instancia Cerbero/Radamanto (Yunque) con reset absoluto de samples y laudo de este `document_id`. No despachar `refactorization` ni `pull-request-review` como vehículo de este ciclo.

## D2 — Decisiones

| ID | Decisión |
|----|----------|
| D-VEHICLE | CLI `feature` + `process_label: refactorization`. |
| D-A1 | DELETE `revoked.pull-request-review` + reset bucket stats (contrato spec). |
| D-LATERAL | No tocar `bug-fix` ni `refactorization`. `bug-fix` re-revocado @ `16:18:17Z` = seed ajena. |
| D-A2 | Diseño en este persist_ref; ejecución solo si laudo. Si no, PBI hijo. |
| D-STOP | Esta sesión = Mayeuta + Dedalo + commit. Prohibido T1–T5. |

## D3 — Fuera

Rehab laterales; umbrales; merge del ciclo `kaizen-aduana-evolution-local`; residual Shell/`git-manager` (#136).
