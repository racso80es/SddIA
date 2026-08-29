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
| Hipótesis F4 | 12 KO en 636–1301 ms vs OK 258–412 s. Perfil de aborto temprano. Evidencia de eventos: ausente (purgados). |
| Nomenclatura | `FAIL_F4_RBAC` = etiqueta aduana Cosecha, **no** `failed_phase_code` motor. Motor: `CERBERO_ENTITY_REVOKED` / `CERBERO_RBAC_DENIED` / `CERBERO_CONFIG_ERROR` (`cerbero_di_rbac.rs::CerberoDiCode`). |
| Gate real | `validate_di_rbac` revoca por **provider** (`skill:`/`action:`/`tool:`), no por proceso puntuado. Lazo "PPR revocado → aborta sus runs" **no** sustentado por el código. |

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
| **L-A2-HOLLOW** | Si laudo + T0 confirma: `is_survival_hollow` true **solo** cuando `failed_phase_code == "CERBERO_ENTITY_REVOKED"` **y** provider revocado (de `failed_phase_error`) == entidad puntuada (`target_entity_from_payload`). Auto-referencial. Patrón `lab_hollow` / `detached_child`. |
| **L-A2-NO-BLIND** | Prohibido podar `CERBERO_RBAC_DENIED` / `CERBERO_CONFIG_ERROR`: violaciones/fallos legítimos deben degradar `success_rate`. |
| **L-A2-T0** | T0 bloqueante: reproducir empíricamente el `failed_phase_code` de las muestras KO de PPR antes de escribir poda. Si ≠ `CERBERO_ENTITY_REVOKED` auto-referencial, A2 no procede. |
| **L-A2-SPLIT** | Sin laudo A2 o sin confirmación T0 → PBI hijo; A1 cierra igual. |
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

## 5. Contrato A2 (condicionado a laudo + T0)

```text
is_survival_hollow(payload) ⊇ {
  lab_hollow,
  detach,
  detached_child ∧ exit_code ≠ 0,
  cycle_phase ∈ {initialized, awaiting_agents},
  # nuevo, si laudo + T0 confirma:
  failed_phase_code == "CERBERO_ENTITY_REVOKED"
    ∧ revoked_provider(failed_phase_error) == target_entity_from_payload(payload)
}
# NUNCA podar: CERBERO_RBAC_DENIED, CERBERO_CONFIG_ERROR
# tests: t_a2_hollow_entity_revoked_self (poda),
#        t_a2_hollow_rbac_denied_not_podado (no poda),
#        t_a2_hollow_revoked_other_provider_not_podado (no poda);
#        t_survival_hollow_* preexistentes intactos
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
