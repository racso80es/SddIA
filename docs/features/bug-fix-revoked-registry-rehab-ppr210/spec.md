---
feature_name: bug-fix-revoked-registry-rehab-ppr210
created: "2026-08-28"
process: refactorization
phase: design
agents: dedalo
base: main
scope: rehab-bug-fix-cerbero-a1
branch_name: refactor/bug-fix-revoked-registry-rehab-ppr210
persist_ref: docs/features/bug-fix-revoked-registry-rehab-ppr210
pbi_ref: docs/todos/pending/[ARQUITECTURA] bug-fix — rehabilitación revoked_entities (PPR #210).md
document_id: PBI-PPR-210-BUG-FIX-REVOKED-REGISTRY
uuid: e7a1b2c3-4d5e-6f78-9a0b-1c2d3e4f5a6b
version_spec: "1.0.0"
status: dedalo_locked
ola: A1
olas:
  - A1
source_correlation_id: "4c2dfd1d-393d-4411-8956-d596ff0eef9c"
source_pr_url: https://github.com/racso80es/SddIA/pull/210
parent_pbi: docs/todos/done/[ARQUITECTURA] bug-fix — rehabilitación revoked_entities (PPR #194).md
incident_ref: "REVOKED_ENTITY_ALERT_BUG_FIX — abrupt_success_rate_drop since 2026-08-28T05:32:55Z"
---

# Spec — ola A1 bug-fix-revoked-registry-rehab-ppr210

## 1. Misión técnica

Saneamiento de instancia `bug-fix` (Yunque). **Cero** mutación de `SddIA/engine/`.

## 2. Diagnóstico instancia

| Vector | Hecho |
|--------|--------|
| Cerbero | `revoked.bug-fix` since `2026-08-28T05:32:55Z`. |
| Stats | `degraded` · laudo fósil #194 @ `11:45:00Z` · 4 samples KO · rate 0. |
| ≠ #194 | Incidente #194 since `2026-08-16T16:09:32Z` **done**. |
| Tipología | Motor `bug-fix`→`process` (#194 L-TYPE-VERIFY PASS). No escalar A2. |

## 3. Laudos Dedalo (A1)

| Ref | Decisión |
|-----|----------|
| **L-NO-ENGINE** | Prohibido tocar crate `execute-process`, umbrales, YAML `bug-fix.md`. |
| **L-REHAB-INST** | Locus Cúmulo `radamanto.revoked_entities` / `radamanto.stats`. Fuera del diff git. |
| **L-CERBERO** | DELETE nodo `revoked.bug-fix`. |
| **L-RESET-ABS** | Bucket raíz según `objectives.md` + laudo #210. |
| **L-SAMPLES** | `samples: []`. |
| **L-ONTOLOGY** | `entity_type: process`. |
| **L-LATERAL** | Laterales revoked intactos. |
| **L-NO-A2** | Sin vector motor nuevo. |
| **L-DOC** | `implementation.md` + `execution.md` en este persist_ref. Evolution UUID ciclo. |
| **L-STOP** | Planning only esta sesión. |

## 4. Contrato A1

```text
DELETE revoked["bug-fix"]
ASSERT permanent["bug-fix"] absent
stats["bug-fix"] := {
  status: healthy,
  recovery_attempts: 0,
  consecutive_success_count: 0,
  degraded_at: null,
  entity_type: process,
  structure_valid: true,
  rehab_laudo: "PBI-PPR-210-BUG-FIX-REVOKED-REGISTRY",
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
