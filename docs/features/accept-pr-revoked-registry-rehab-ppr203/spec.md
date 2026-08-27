---
feature_name: accept-pr-revoked-registry-rehab-ppr203
created: "2026-08-27"
process: refactorization
phase: design
agents: dedalo
base: main
scope: rehab-accept-pr-cerbero-a1
branch_name: refactor/accept-pr-revoked-registry-rehab-ppr203
persist_ref: docs/features/accept-pr-revoked-registry-rehab-ppr203
pbi_ref: docs/todos/pending/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #203).md
document_id: PBI-PPR-203-ACCEPT-PR-REVOKED-REGISTRY
uuid: b7e4a91c-2f5d-4c8b-9e1a-6d3f0a8b2c7e
version_spec: "1.0.0"
status: dedalo_locked
ola: A1
olas:
  - A1
source_correlation_id: "6237015f-0f8d-42ea-97ea-a44afac5318d"
source_pr_url: https://github.com/racso80es/SddIA/pull/203
parent_pbi: docs/todos/done/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #200).md
incident_ref: "REVOKED_ENTITY_ALERT_ACCEPT_PR — abrupt_success_rate_drop since 2026-08-27T12:31:30Z"
---

# Spec — ola A1 accept-pr-revoked-registry-rehab-ppr203

## 1. Misión técnica

Saneamiento de instancia `accept-pr` (Yunque). **Cero** mutación de `SddIA/engine/`. Contratos A2 en `docs/features/accept-pr-anti-recurrence-ppr203/spec.md`.

## 2. Diagnóstico instancia

| Vector | Hecho |
|--------|--------|
| Cerbero | `revoked.accept-pr` since `2026-08-27T12:31:30Z`. |
| Stats | `degraded` · laudo fósil #200 @ `12:00:00Z` · samples mix 0/1/0 · rate 0,667. |
| ≠ #200 | Incidente #200 since `11:31:15Z` **done**. |

## 3. Laudos Dedalo (A1)

| Ref | Decisión |
|-----|----------|
| **L-NO-ENGINE** | Prohibido tocar crate `execute-process`, umbrales, YAML `accept-pr.md`. |
| **L-REHAB-INST** | Locus Cúmulo `radamanto.revoked_entities` / `radamanto.stats`. Fuera del diff git. |
| **L-CERBERO** | DELETE nodo `revoked.accept-pr`. |
| **L-RESET-ABS** | Bucket raíz según `objectives.md` + laudo #203. |
| **L-SAMPLES** | Vaciar o ≤3 OK. Eliminar `5d6f7cb3-db47-41d7-aeb3-fed4e7b4ad4f`. |
| **L-ONTOLOGY** | `entity_type: process`. |
| **L-LATERAL** | `revoked.refactorization` intacto. |
| **L-DOC** | `implementation.md` + `execution.md` en este persist_ref. Evolution UUID ciclo. |

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
  rehab_laudo: "PBI-PPR-203-ACCEPT-PR-REVOKED-REGISTRY",
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

`plan.md` T1→T2. No A1 antes de merge A2 en el mismo host.
