---
feature_name: feature-revoked-registry-rehab-ppr210
created: "2026-08-28"
process: refactorization
phase: planning
agents: dedalo
phases:
  - T0-assert-185
  - T1-instance-rehab
  - T2-evolution
  - T3-argos
  - T4-doc-archive
  - T5-delivery-close
branch_name: refactor/feature-revoked-registry-rehab-ppr210
persist_ref: docs/features/feature-revoked-registry-rehab-ppr210
pbi_ref: docs/todos/pending/[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #210).md
document_id: PBI-PPR-210-FEATURE-REVOKED-REGISTRY
uuid: f8b2c3d4-5e6f-7a89-0b1c-2d3e4f5a6b7c
ola: A1
olas:
  - A1
---

# Plan — ola A1 feature-revoked-registry-rehab-ppr210

Blueprint Tekton. Contratos: `spec.md`. **Stop planning:** no ejecutar T0–T5 en esta sesión.

Init lab: `execution_id` `532a36c1-d46e-4c49-82ec-dbfc2ea50315` · vehículo `feature` · `SDDIA_LAB_SKIP_GIT`.

## T0 — Assert motor #185 (read-only)

1. Confirmar en disco: fail-soft padre DCC (`phase_capsules` / `residual_runner`) + gate hollow (`radamanto_batch_core` / `thermodynamic` `cycle_phase`).
2. Si **FAIL**: ABORT A1-only; no improvisar A2 en este persist_ref. Escalado = laudo humano.
3. Si PASS: continuar T1. **Prohibido** mutar engine en el PR A1.

## T1 — A1 instancia (AC-A1 / AC-ONTO / AC-GIT-CLEAN)

Locus Cúmulo `radamanto.revoked_entities` / `radamanto.stats`. **Fuera del diff git.**

1. DELETE `revoked.feature`. Assert `permanent.feature` ausente.
2. Reset absoluto bucket raíz `feature` (laudo #210, `samples: []`).
3. Assert laterales intactos.
4. Evidencia en `execution.md`.

## T2 — Documental + evolution

1. `implementation.md` + `execution.md`.
2. Evolution UUID `f8b2c3d4-5e6f-7a89-0b1c-2d3e4f5a6b7c`.
3. Assert diff sin instancia ni umbrales ni engine.

## T3 — Argos

`validacion.md` APTO + checks AC-* + `pbi_archived: true` + `branch: refactor/feature-revoked-registry-rehab-ppr210`.

## T4 — Archive PBI

Mover `docs/todos/pending/[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #210).md` → `docs/todos/done/`.

## T5 — DCC

`delivery-close-cycle` · `source_process: feature` · `process_label: refactorization` · este `persist_ref` / `branch_name`.

Post-rehab: no despachar `feature` productivo con ventana KO residual.

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

Reabrir A2/A3; rehab laterales; ejecución esta sesión.
