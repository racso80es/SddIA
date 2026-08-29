---
feature_name: ppr-anti-recurrence-hollow-a2-kaizen-aduana-evolution
created: "2026-08-29"
process: refactorization
phase: design
agents: dedalo
base: main
scope: hollow-a2-radamanto
branch_name: refactor/ppr-anti-recurrence-hollow-a2-kaizen-aduana-evolution
persist_ref: docs/features/ppr-anti-recurrence-hollow-a2-kaizen-aduana-evolution
pbi_ref: docs/todos/pending/PBI-PPR-ANTI-RECURRENCE-HOLLOW-A2-KAIZEN-ADUANA-EVOLUTION.md
document_id: PBI-PPR-ANTI-RECURRENCE-HOLLOW-A2-KAIZEN-ADUANA-EVOLUTION
uuid: 18bacf31-9223-4b07-853e-a66c0d6c3ebd
version_spec: "1.0.0"
status: dedalo_locked
ola: A2
parent_persist_ref: docs/features/ppr-revoked-registry-rehab-kaizen-aduana-evolution
---

# Spec — ppr-anti-recurrence-hollow-a2-kaizen-aduana-evolution

## Misión

Anti-recurrencia motor: cerrar bucle autoconfirmante revocado → KO → ratio → re-revocación, **solo** para denegaciones por entidad ya revocada que puntúan la misma entidad.

## Laudos

| Ref | Decisión |
|-----|----------|
| **L-A2-HOLLOW** | `is_survival_hollow` true si `failed_phase_code == CERBERO_ENTITY_REVOKED` **y** provider parseado de `failed_phase_error` == `target_entity_from_payload`. |
| **L-A2-NO-BLIND** | Prohibido podar `CERBERO_RBAC_DENIED` / `CERBERO_CONFIG_ERROR`. |
| **L-A2-T0** | Documentar replay empírico; si inconcluso, contract-first con riesgo explícito. |
| **L-NO-THRESH** | `radamanto.thresholds.json` prohibido. |
| **L-NO-YAML** | `pull-request-review.md` / `phase_terminal.rs` prohibidos. |
| **L-VEHICLE** | DCC `source_process: feature` / `process_label: refactorization`. |

## Contrato A2

```text
is_survival_hollow(payload) ⊇ {
  lab_hollow, detach, detached_child ∧ exit_code ≠ 0,
  cycle_phase ∈ {initialized, awaiting_agents},
  failed_phase_code == "CERBERO_ENTITY_REVOKED"
    ∧ revoked_provider(failed_phase_error) == target_entity_from_payload(payload)
}
# NUNCA: CERBERO_RBAC_DENIED, CERBERO_CONFIG_ERROR
```

Mensaje Cerbero (`cerbero_di_rbac.rs`): `proveedor '{provider}' revocado en revoked_entities`.

## AC

| AC | Verificación |
|----|--------------|
| AC-A2-DISCRIM | Contrato § + code review |
| AC-A2-TESTS | `t_a2_hollow_*` + regresión podas previas |
| AC-GIT-CLEAN | Sin `.SddIA/cerbero|radamanto` |
| AC-NO-THRESH | Diff sin thresholds |
| AC-DOC | persist_ref + evolution |
