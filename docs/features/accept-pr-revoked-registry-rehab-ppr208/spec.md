---
feature_name: accept-pr-revoked-registry-rehab-ppr208
created: "2026-08-28"
process: refactorization
phase: design
agents: dedalo
base: main
scope: rehab-accept-pr-cerbero-a1
branch_name: refactor/accept-pr-revoked-registry-rehab-ppr208
persist_ref: docs/features/accept-pr-revoked-registry-rehab-ppr208
pbi_ref: docs/todos/pending/PBI-PPR-208-ACCEPT-PR-REVOKED-REGISTRY.md
document_id: PBI-PPR-208-ACCEPT-PR-REVOKED-REGISTRY
uuid: d4f8e2a1-6c39-4b7e-9a05-1f3c8d7e6b20
version_spec: "1.0.0"
status: dedalo_locked
ola: A1
olas:
  - A1
source_correlation_id: "4CMsk8z5Gx7mFQHc512m9FoJibvnr463cVyVcWz5imKm"
source_pr_url: https://github.com/racso80es/SddIA/pull/208
parent_pbi: docs/todos/done/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #203).md
incident_ref: "REVOKED_ENTITY_ALERT_ACCEPT_PR — abrupt_success_rate_drop since 2026-08-27T18:21:13Z"
---

# Spec — ola A1 accept-pr-revoked-registry-rehab-ppr208

## 1. Misión técnica

Saneamiento de instancia `accept-pr` (Yunque). **Cero** mutación de `SddIA/engine/`. Contratos A2 vigentes en `docs/features/accept-pr-anti-recurrence-ppr203/`.

## 2. Diagnóstico instancia

| Vector | Hecho |
|--------|--------|
| Cerbero | `revoked.accept-pr` since `2026-08-27T18:21:13Z`. |
| Stats | `degraded` · laudo fósil #203 @ `16:04:48Z` · mix 0/1 · rate 0,571 · `structure_valid: false`. |
| ≠ #203 | Incidente #203 since `12:31:30Z` **done**. |

## 3. Laudos Dedalo (A1)

| Ref | Decisión |
|-----|----------|
| **L-NO-ENGINE** | Prohibido tocar crate, umbrales, YAML `accept-pr.md`. |
| **L-REHAB-INST** | Locus Cúmulo revoked/stats. Fuera del diff git. |
| **L-CERBERO** | DELETE `revoked.accept-pr`. |
| **L-RESET-ABS** | Bucket raíz + laudo #208. |
| **L-SAMPLES** | Vaciar o ≤3 OK. |
| **L-ONTOLOGY** | `entity_type: process`. |
| **L-LATERAL** | Laterales revoked intactos. |
| **L-REUSE-203** | A2 sync fail_soft = código vigente. |
| **L-HANDOFF** | No merge PR #208 aquí. |
| **L-DOC** | Cascada + evolution UUID ciclo. |
| **L-STOP** | Planning only esta sesión. |

## 4. Contrato A1

```text
DELETE revoked["accept-pr"]
ASSERT permanent["accept-pr"] absent
stats["accept-pr"] := {
  status: healthy,
  recovery_attempts: 0,
  consecutive_success_count: 0,
  degraded_at: null,
  entity_type: process,
  structure_valid: true,
  rehab_laudo: "PBI-PPR-208-ACCEPT-PR-REVOKED-REGISTRY",
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
