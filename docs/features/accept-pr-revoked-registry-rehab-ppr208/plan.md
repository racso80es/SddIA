---
feature_name: accept-pr-revoked-registry-rehab-ppr208
created: "2026-08-28"
process: refactorization
phase: planning
agents: dedalo
phases:
  - T0-assert-203
  - T1-instance-rehab
  - T2-evolution
  - T3-argos
  - T4-doc-archive
  - T5-delivery-close
branch_name: refactor/accept-pr-revoked-registry-rehab-ppr208
persist_ref: docs/features/accept-pr-revoked-registry-rehab-ppr208
pbi_ref: docs/todos/pending/PBI-PPR-208-ACCEPT-PR-REVOKED-REGISTRY.md
document_id: PBI-PPR-208-ACCEPT-PR-REVOKED-REGISTRY
uuid: d4f8e2a1-6c39-4b7e-9a05-1f3c8d7e6b20
ola: A1
olas:
  - A1
---

# Plan — ola A1 accept-pr-revoked-registry-rehab-ppr208

Blueprint Tekton. Contratos: `spec.md`. **Stop planning:** no ejecutar T0–T5 en esta sesión.

Init lab: `execution_id` `e1de4691-5b6f-495b-85ff-b6a52dcd11c4` · vehículo `feature` · `SDDIA_LAB_SKIP_GIT`.

## T0 — Assert motor #203 (read-only)

1. Confirmar fail_soft sync post-merge (`accept_pr.rs` / `residual_runner`) presente.
2. Si FAIL: ABORT; no improvisar A2 aquí.
3. Si PASS: T1. **Prohibido** mutar engine en el PR A1.

## T1 — A1 instancia (AC-A1 / AC-ONTO / AC-GIT-CLEAN)

Locus Cúmulo `radamanto.revoked_entities` / `radamanto.stats`. **Fuera del diff git.**

1. DELETE `revoked.accept-pr`. Assert `permanent.accept-pr` ausente.
2. Reset absoluto bucket raíz `accept-pr` (laudo #208, `samples: []`, `structure_valid: true`).
3. Assert laterales intactos.
4. Evidencia en `execution.md`.

**Orden host:** T0 PASS **antes** de T1.

## T2 — Documental + evolution

1. `implementation.md` + `execution.md`.
2. Evolution UUID `d4f8e2a1-6c39-4b7e-9a05-1f3c8d7e6b20`.
3. Assert diff sin instancia ni umbrales ni engine.

## T3 — Argos

`validacion.md` APTO + checks AC-* + `pbi_archived: true` + `branch: refactor/accept-pr-revoked-registry-rehab-ppr208`.

## T4 — Archive PBI

Mover `docs/todos/pending/PBI-PPR-208-ACCEPT-PR-REVOKED-REGISTRY.md` → `docs/todos/done/`.

## T5 — DCC

`delivery-close-cycle` · `source_process: feature` · `process_label: refactorization` · este `persist_ref` / `branch_name`.

Post-rehab: smoke handoff sin re-revocación inmediata. Merge PR #208 **fuera**.

## Orden

```text
T0 → T1 → T2 → T3 → T4 → T5
```

## Delegaciones

| Fase | Cápsula |
|------|---------|
| T0/T1/docs | Tekton `filesystem-ops` |
| Git | `skill:git-manager` |
| PR | `delivery-close-cycle` |

## Fuera de este plan

Reabrir A2 #203; rehab laterales #210; handoff PR #208; ejecución esta sesión.
