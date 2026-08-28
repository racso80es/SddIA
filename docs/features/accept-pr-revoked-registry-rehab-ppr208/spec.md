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
pbi_ref: docs/todos/pending/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #210).md
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
incident_ref: "REVOKED_ENTITY_ALERT_BUG_FIX — abrupt_success_rate_drop since 2026-08-28T18:21:13Z"
---

# Spec — ola A1 accept-pr-revoked-registry-rehab-ppr208

## 1. Misión técnica

Saneamiento de instancia `accept-pr` (Yunque). **Cero** mutación de `SddIA/engine/`.

## 2. Diagnóstico instancia

| Vector | Hecho |
|--------|--------|
| Cerbero | `revoked.accept-pr` since `2026-08-28T18:21:13Z`. |
| Stats | `degraded` · laudo fósil #203 @ `11:45:00Z` · 4 samples KO · rate 0. |
| ≠ #203 | Incidente #203 since `2026-08-16T16:09:32Z` **done**. |
| Tipología | Motor `accept-pr`→`process` (#203 L-TYPE-VERIFY PASS). No escalar A2. |

## 3. Laudos Dedalo (A1)

| Ref | Decisión |
|-----|----------|
| **L-NO-ENGINE** | Prohibido tocar crate `execute-process`, umbrales, YAML `accept-pr.md`. |
| **L-REHAB-INST** | Locus Cúmulo `radamanto.revoked_entities` / `radamanto.stats`. Fuera del diff git. |
| **L-CERBERO** | DELETE nodo `revoked.accept-pr`. |
| **L-RESET-ABS** | Bucket raíz según `objectives.md` + laudo #210. |
| **L-SAMPLES** | `samples: []`. |
| **L-ONTOLOGY** | `entity_type: process`. |
| **L-LATERAL** | Laterales revoked intactos. |
| **L-NO-A2** | Sin vector motor nuevo. |
| **L-DOC** | `implementation.md` + `execution.md` en este persist_ref. Evolution UUID ciclo. |
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

`filesystem-ops` (instancia) · `source-control` (git-manager) · sin forja genoma.

## 7. Handoff Tekton

`plan.md` T1→T5. **No ejecutar** en esta sesión.
