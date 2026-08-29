---
feature_name: ppr-anti-recurrence-hollow-a2-kaizen-aduana-evolution
created: "2026-08-29"
process: refactorization
phase: planning
agents: dedalo
phases:
  - T0-hollow-a2
  - T2-evolution
  - T3-argos
  - T4-doc-archive
  - T5-delivery-close
branch_name: refactor/ppr-anti-recurrence-hollow-a2-kaizen-aduana-evolution
persist_ref: docs/features/ppr-anti-recurrence-hollow-a2-kaizen-aduana-evolution
pbi_ref: docs/todos/pending/PBI-PPR-ANTI-RECURRENCE-HOLLOW-A2-KAIZEN-ADUANA-EVOLUTION.md
document_id: PBI-PPR-ANTI-RECURRENCE-HOLLOW-A2-KAIZEN-ADUANA-EVOLUTION
uuid: 18bacf31-9223-4b07-853e-a66c0d6c3ebd
ola: A2
---

# Plan — ppr-anti-recurrence-hollow-a2-kaizen-aduana-evolution

Parent A1: `docs/features/ppr-revoked-registry-rehab-kaizen-aduana-evolution/` (**done** PR #220).

## T0 — Motor hollow (AC-A2-DISCRIM / AC-A2-TESTS)

1. Documentar T0 empírico en `execution.md` (eventos purgados; perfil KO histórico del parent).
2. `radamanto_batch_core.rs`: extender `is_survival_hollow` per **L-A2-HOLLOW**.
3. Tests `t_a2_hollow_*`. Assert podas preexistentes intactas.
4. `cargo test -p execute-process --lib`.

## T2 — Documental + evolution

1. `implementation.md` + `execution.md`.
2. Evolution UUID `18bacf31-9223-4b07-853e-a66c0d6c3ebd`.

## T3 — Argos

`validacion.md`: AC-A2-*, `pbi_archived: true`, rama coherente.

## T4 — Archive PBI

`docs/todos/pending/` → `docs/todos/done/` mismo `document_id`.

## T5 — DCC

`delivery-close-cycle` · vehículo `feature` · `process_label: refactorization`.

## Orden

```text
T0 → T2 → T3 → T4 → T5
```
