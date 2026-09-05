---
feature_name: ppr-revoked-registry-rehab-restore-kaizen-ci-step
created: "2026-09-05"
process: refactorization
phase: planning
agents: dedalo
phases:
  - T1-instance-rehab
  - T2-evolution
  - T3-smoke-ppr
  - T4-argos
  - T5-doc-archive
  - T6-delivery-close
branch_name: refactor/ppr-revoked-registry-rehab-restore-kaizen-ci-step
persist_ref: docs/features/ppr-revoked-registry-rehab-restore-kaizen-ci-step
pbi_ref: docs/todos/pending/PBI-RESTORE-PBI-KAIZEN-CI-STEP-ARCHIVE-PPR-REVOKED-REGISTRY.md
document_id: PBI-RESTORE-PBI-KAIZEN-CI-STEP-ARCHIVE-PPR-REVOKED-REGISTRY
uuid: e2f8a1c4-7b3d-4e9f-a612-8c5d0b9e4f17
ola: A1
olas:
  - A1
runtime_execution_id: "4fe5d41e-5ebb-430c-96c9-3f3a31b0103b"
---

# Plan — ppr-revoked-registry-rehab-restore-kaizen-ci-step

Blueprint Tekton. Contratos: `spec.md`.

Init lab: `execution_id` `4fe5d41e-5ebb-430c-96c9-3f3a31b0103b` · vehículo `feature` · `process_label: refactorization` · relevo IDE.

## T0 — Planning (esta entrega, commit 1)

Cascada `clarify.md` / `objectives.md` / `spec.md` / `plan.md` + PBI v1.2.0. Commit vía `skill:git-manager` **antes** de mutar instancia.

## T1 — A1 instancia (CA1 / CA2 / CA3)

Locus Cúmulo: `.SddIA/cerbero/revoked_entities.json` / `.SddIA/radamanto/stats.json`. **Fuera del diff git.**

1. DELETE `revoked.pull-request-review`. Assert `permanent.pull-request-review` ausente.
2. Reset absoluto bucket raíz (**L-RESET-ABS** + **L-SAMPLES** + laudo este `document_id`).
3. Assert laterales `revoked.{bug-fix,delivery-close-cycle,entity-manager,feature,refactorization}` intactos (snapshot pre-T1).
4. Evidencia (campos/timestamp, no secretos) en `execution.md`.

## T2 — Documental + evolution

1. `implementation.md` + `execution.md`.
2. Entrada `directories.evolution` UUID `e2f8a1c4-7b3d-4e9f-a612-8c5d0b9e4f17` (contrato v1.1.2; `hash_integrity` vía `sddia-qa evolution-rehash`).
3. Assert diff: **no** `.SddIA/cerbero/` ni `.SddIA/radamanto/` ni umbrales.

## T3 — Smoke PPR (CA4)

Inyección `pull-request-review` detached. Flags: `SDDIA_AGENT_RELAY_IDE=1` · `SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF=1`. `execution_id` en `execution.md`. Post-acuse: entidad ∉ `revoked`. Sin join.

## T4 — Argos

`validacion.md`: checks CA1–CA5; CA6 = `PENDIENTE-CI` hasta run verde. `pbi_archived: false` hasta T5. `global` no APTO mientras CA6 sea gate.

## T5 — Archive PBI (post-CI verde, CA6)

Mover `docs/todos/pending/PBI-RESTORE-PBI-KAIZEN-CI-STEP-ARCHIVE-PPR-REVOKED-REGISTRY.md` → `docs/todos/done/`. `validacion.md`: `pbi_archived: true`, CA6 APTO con `run_id`/URL, `global: APTO`. Mismo PR.

## T6 — DCC (forja PR, post-T2, pre-T5)

`delivery-close-cycle` · `source_process: feature` / `process_label: refactorization` · `persist_ref` · `branch_name`. Git: `skill:git-manager`.

Orden operador:

```text
T0 commit planning → T1 → T2 → T3 → T4 (PENDIENTE-CI) → T6 PR → CI verde → T5 archive
```

## Delegaciones

| Tarea | Agente / cápsula |
|-------|------------------|
| T0–T2, T4–T5 | Tekton / Argos (relevo IDE) |
| T1 | filesystem instancia (no genoma) |
| T3 / T6 | `execute-process` |
| Git | `skill:git-manager` |
