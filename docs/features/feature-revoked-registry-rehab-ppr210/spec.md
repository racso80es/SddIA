---
feature_name: feature-revoked-registry-rehab-ppr210
created: "2026-08-28"
process: refactorization
phase: design
agents: dedalo
base: main
scope: rehab-feature-cerbero-a1
branch_name: refactor/feature-revoked-registry-rehab-ppr210
persist_ref: docs/features/feature-revoked-registry-rehab-ppr210
pbi_ref: docs/todos/pending/[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #210).md
document_id: PBI-PPR-210-FEATURE-REVOKED-REGISTRY
uuid: f8b2c3d4-5e6f-7a89-0b1c-2d3e4f5a6b7c
version_spec: "1.0.0"
status: dedalo_locked
ola: A1
olas:
  - A1
source_correlation_id: "4c2dfd1d-393d-4411-8956-d596ff0eef9c"
source_pr_url: https://github.com/racso80es/SddIA/pull/210
parent_pbi: docs/todos/done/[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #185).md
incident_ref: "REVOKED_ENTITY_ALERT_FEATURE — abrupt_success_rate_drop since 2026-08-28T05:25:41Z"
---

# Spec — ola A1 feature-revoked-registry-rehab-ppr210

## 1. Misión técnica

Saneamiento de instancia `feature` (Yunque). **Cero** mutación de `SddIA/engine/` salvo aborto T0 (A2/A3 #185 ausentes — no esperado).

## 2. Diagnóstico instancia

| Vector | Hecho |
|--------|--------|
| Cerbero | `revoked.feature` since `2026-08-28T05:25:41Z`. `permanent.feature` ausente (≠ #185). |
| Stats | `degraded` · 8 samples KO · rate 0 · mix latencias cortas y ~999s. |
| ≠ #185 | #185 = `permanent` + recovery_attempts 4 · **done**. |
| Motor | A2 fail-soft padre DCC + A3 hollow ya en `main`. Samples actuales son `failed` reales → A1. |

## 3. Laudos Dedalo (A1)

| Ref | Decisión |
|-----|----------|
| **L-NO-ENGINE** | Prohibido tocar crate salvo T0 FAIL. |
| **L-REHAB-INST** | Locus Cúmulo revoked/stats. Fuera del diff git. |
| **L-CERBERO** | DELETE `revoked.feature`. |
| **L-RESET-ABS** | Bucket raíz + laudo #210. |
| **L-SAMPLES** | `samples: []`. |
| **L-ONTOLOGY** | `entity_type: process`. |
| **L-LATERAL** | Laterales revoked intactos. |
| **L-REUSE-185** | A2/A3 = código vigente; no duplicar. |
| **L-DOC** | Cascada + evolution UUID ciclo. |
| **L-STOP** | Planning only esta sesión. |

## 4. Contrato A1

```text
DELETE revoked["feature"]
ASSERT permanent["feature"] absent
stats["feature"] := {
  status: healthy,
  recovery_attempts: 0,
  consecutive_success_count: 0,
  degraded_at: null,
  entity_type: process,
  structure_valid: true,
  rehab_laudo: "PBI-PPR-210-FEATURE-REVOKED-REGISTRY",
  rehabilitated_at: <ISO A1>,
  samples: []
}
# no git-add .SddIA/cerbero|radamanto
```

## 5. Mapa AC

| AC | Verificación |
|----|--------------|
| AC-A1 | Claves Cerbero/stats según contrato. |
| AC-GIT-CLEAN | Diff PR sin instancia. |
| AC-ONTO | `entity_type: process`. |
| AC-DOC | Cascada A1. |

## 6. RBAC

`filesystem-ops` · `source-control` · sin forja genoma.

## 7. Handoff Tekton

`plan.md` T0→T5. **No ejecutar** en esta sesión.
