---
feature_name: ppr-revoked-registry-rehab-ppr190
created: "2026-08-26"
process: refactorization
phase: blueprint
agents: dedalo
phases: T0-T5
branch_name: refactor/ppr-revoked-registry-rehab-ppr190
persist_ref: docs/features/ppr-revoked-registry-rehab-ppr190
pbi_ref: docs/todos/pending/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #190).md
document_id: PBI-PPR-190-REVOKED-REGISTRY
uuid: e2b9a4f1-7c83-4d5e-9a16-0f8b3c5d7e21
olas:
  - A1
  - A2
---

# Plan — ppr-revoked-registry-rehab-ppr190

## T0 — Motor A2

1. `thermodynamic.rs`: PPR en `LIFECYCLE_PROCESSES`; tag `detached_child` si `SDDIA_DETACHED_EXECUTION_ID`.
2. `radamanto_batch_core.rs`: `is_survival_hollow` + tests.
3. `cargo test -p execute-process --lib hollow derive_ppr`.

## T1 — A1 instancia

1. Borrar `pull-request-review` de `permanent` y `revoked`.
2. Reset stats raíz PPR (healthy, laudo, ≤3 OK samples).
3. Evidencia en `execution.md`.

## T2 — Documental + evolution

1. Cascada `implementation.md` / `execution.md`.
2. `SddIA/evolution/e2b9a4f1-7c83-4d5e-9a16-0f8b3c5d7e21.md`.

## T3 — Argos → `validacion.md`

## T4 — PBI → `docs/todos/done/`

## T5 — DCC (posterior, fuera de esta sesión Tekton)
