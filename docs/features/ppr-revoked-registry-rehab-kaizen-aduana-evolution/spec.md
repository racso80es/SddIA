---
feature_name: ppr-revoked-registry-rehab-kaizen-aduana-evolution
created: "2026-08-29"
process: refactorization
phase: design
agents: dedalo
base: main
scope: rehab-ppr-cerbero-a1-hollow-a2
branch_name: refactor/ppr-revoked-registry-rehab-kaizen-aduana-evolution
persist_ref: docs/features/ppr-revoked-registry-rehab-kaizen-aduana-evolution
pbi_ref: docs/todos/pending/PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL-PPR-REVOKED-REGISTRY.md
document_id: PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL-PPR-REVOKED-REGISTRY
uuid: c4e8f1a2-9b3d-4f7e-a6c1-2d8e5f0b3a71
version_spec: "1.0.0"
status: dedalo_locked
ola: A1
olas:
  - A1
  - A2
source_correlation_id: "8ZjTzcBwfFAVFQujfjGCJwJeJcj5pbB4SMHAD5bn5ybE"
parent_pbi: docs/todos/done/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #190).md
incident_ref: "REVOKED_ENTITY_ALERT_PULL_REQUEST_REVIEW — success_rate_below_threshold since 2026-08-28T10:10:42Z"
---

# Spec — ppr-revoked-registry-rehab-kaizen-aduana-evolution

## 1. Misión técnica

Saneamiento de instancia `pull-request-review` (Yunque A1). A2: poda de supervivencia para abortos de gobernanza — **solo si laudo**; no bloquea A1.

## 2. Diagnóstico

| Vector | Hecho |
|--------|--------|
| Cerbero | `revoked.pull-request-review` since `2026-08-28T10:10:42Z` · `success_rate_below_threshold`. |
| Stats | `degraded` · laudo fósil #190 @ `18:02:03Z` · 20 samples · rate 0.25. |
| Motor rate | `radamanto_batch_core.rs` `success_rate()` = ok/n sobre ventana completa. |
| Redención | `set_structure_valid(true)` es el único puente `degraded → pending_redemption`. |
| ≠ #190 | Incidente #190 (permanent+revoked) **done**; receta incompleta (sin `L-SAMPLES`). |
| Hipótesis F4 | 12 KO en 636–1301 ms vs OK 258–412 s. `failed_phase_code` ya en `thermodynamic.rs`. `is_survival_hollow` no poda F4. Evidencia de eventos: ausente. |

## 3. Laudos Dedalo

| Ref | Decisión |
|-----|----------|
| **L-REHAB-INST** | Locus Cúmulo `radamanto.revoked_entities` / `radamanto.stats`. Fuera del diff git. |
| **L-CERBERO** | DELETE nodo `revoked.pull-request-review`. |
| **L-RESET-ABS** | Bucket raíz según contrato A1 + laudo este `document_id`. |
| **L-SAMPLES** | `samples: []`. Sin esto, recidiva determinista. |
| **L-ONTOLOGY** | `entity_type: process`. |
| **L-LATERAL** | `bug-fix` y `refactorization` intactos. |
| **L-NO-THRESH** | `radamanto.thresholds.json` prohibido. |
| **L-NO-YAML** | `pull-request-review.md` / `phase_terminal.rs` / agregador terminal prohibidos. |
| **L-A2-HOLLOW** | Si laudo: `is_survival_hollow` retorna true cuando `failed_phase_code` ∈ {`FAIL_F4_RBAC`} **o** payload marque aborto de entidad revocada. Patrón `lab_hollow` / `detached_child`. |
| **L-A2-SPLIT** | Sin laudo A2 → PBI hijo; A1 cierra igual. |
| **L-DOC** | Cascada este persist_ref. Evolution UUID ciclo. |
| **L-STOP** | Planning only esta sesión. |
| **L-VEHICLE** | DCC `source_process: feature` / nota `process_label: refactorization`. |

## 4. Contrato A1

```text
DELETE revoked["pull-request-review"]
ASSERT permanent["pull-request-review"] absent
ASSERT revoked["bug-fix"] unchanged
ASSERT revoked["refactorization"] unchanged
stats["pull-request-review"] := {
  status: healthy,
  recovery_attempts: 0,
  consecutive_success_count: 0,
  degraded_at: null,
  entity_type: process,
  structure_valid: true,
  rehab_laudo: "PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL-PPR-REVOKED-REGISTRY",
  rehabilitated_at: <ISO A1>,
  samples: []
}
# no git-add .SddIA/cerbero|radamanto
```

## 5. Contrato A2 (condicionado)

```text
is_survival_hollow(payload) ⊇ {
  lab_hollow,
  detach,
  detached_child ∧ exit_code ≠ 0,
  cycle_phase ∈ {initialized, awaiting_agents},
  failed_phase_code == FAIL_F4_RBAC   # nuevo, si laudo
}
# tests t_a2_hollow_* ; t_survival_hollow_* preexistentes intactos
```

## 6. Mapa AC

| AC | Verificación |
|----|--------------|
| AC-A1-* | Contrato §4 + smoke `execution_id`. |
| AC-A2-* | Contrato §5 **o** PBI hijo. |
| AC-GIT-CLEAN | Diff PR sin instancia. |
| AC-NO-THRESH | Diff sin thresholds. |
| AC-DOC | Cascada + evolution. |

## 7. RBAC

`filesystem-ops` (instancia) · `source-control` (git-manager) · crate `execute-process` solo en A2 laudo. Sin forja genoma (DA-2).

## 8. Handoff Tekton

`plan.md` T1→T5 (+ T0 si A2). **No ejecutar** en esta sesión.
