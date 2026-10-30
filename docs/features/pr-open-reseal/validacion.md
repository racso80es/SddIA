---
feature_name: pr-open-reseal
created: "2026-10-30"
process: feature
phase: Verificación
branch: feat/pr-open-reseal
global: NO_APTO
tracker_ref: AUT-42
pbi_archived: false
execution_id: ""
approval_status: integrity_failure
checks:
  AC-PREPUSH-RESEAL: NO_EVIDENCIA_CAPSULA
  AC-QA-LOCAL: NO_EVIDENCIA_CAPSULA
  cascade-documental: INCOMPLETA
  capsules-qa: BLOCKED
  git-evidence: BLOCKED
  injection-contract: INCOMPLETE
git_changes:
  - docs/features/pr-open-reseal/
  - SddIA/engine/execute-process/src/engine/diff_seal.rs
  - SddIA/tools/sddia-qa/src/reseal_diff_validacion.rs
  - SddIA/tools/sddia-qa/src/main.rs
---

# Validación — AUT-42 (Argos)

## approval_status: integrity_failure

Inyección kalma2 con `process: ?` / `phase: ?` / `persist_ref` vacío / `execution_id` vacío / `branch_name` vacío / `pbi_ref` vacío / `correlation_id` vacío.

Contrato **inferido** (no inyectado) desde `objectives.md` + snapshot git conversacional: `feature` / `Verificación` / `docs/features/pr-open-reseal` / `feat/pr-open-reseal` / AUT-42.

Peaje Shell **Rejected** (status + `./sddia-run.sh --tool git-manager`). GetDynamicTools: sin namespace sddia/kalma. Sin stdout de `git-manager` ni `sddia-qa reseal-diff-validacion` / QA pre-push.

## Evidencia de cápsulas

| Cápsula | Resultado |
|---------|-----------|
| `./sddia-run.sh --tool git-manager` | Shell Rejected — sin stdout |
| `sddia-qa reseal-diff-validacion` | No invocado — peaje Shell ausente |
| Local_QA_Requested (full/docs-only) | Sin acuse JSON ni evento correlacionado |

**L-ARGOS-SYNTHESIS:** lectura estática ≠ evidencia de cápsula. `global: NO_APTO`. Tracker `provider: linear` — Argos no materializa TODOs bajo `docs/todos/`.

## Hallazgos estáticos (descubrimiento, no veredicto APTO)

1. Cascada bajo persist inferido: existen `objectives.md`, `validacion.md`; **faltan** `clarify.md`, `spec.md`, `plan.md`, `implementation.md`, `execution.md`.
2. Código WIP visible en snapshot: `diff_seal::reseal_validacion_for_branch`, módulo `reseal_diff_validacion` cableado en `sddia-qa` — existencia ≠ prueba de ejecución.
3. Colateral: `docs/todos/pending/[FIX] delivery-close-cycle — fractura sistémica (c499eec7fb3f).md` (Cúmulo/Kintsugi). Con `tracker.provider: linear`, cola local no es vía canónica de backlog; Argos no toca ese fichero.

## acceptance_criteria — estado

1. Pre-push PR OPEN: reseal + QA en el mismo push, sin `delivery-close-cycle` — **sin stdout cápsula**.
2. Sello `validacion.md` con digest coherente post-reseal — **este artefacto sin `validated_*` / `diff_digest`**.

## correction_blueprint_md

```yaml
name: reentrada-verificacion-aut-42
intent: Reinyectar Argos con peaje Shell/MCP y campos explícitos; no inventar APTO.
delegates_to:
  - tool:git-manager
  - action:execute-process
  - agent:argos
phases:
  - name: Evidencia-git
    intent: status/diff/log vía skill:git-manager (JSON stdin sddia-run)
    delegates_to:
      - tool:git-manager
  - name: Completar-cascada
    intent: Si faltan clarify/spec/plan/implementation/execution, Dedalo/Tekton antes de re-auditoría
    delegates_to:
      - agent:tekton
  - name: Verify-reseal
    intent: sddia-qa reseal-diff-validacion --json + prueba pre-push PR OPEN sin DCC
    delegates_to:
      - action:execute-process
  - name: Sello-validacion
    intent: Argos reescribe validacion.md solo tras stdout físico
    delegates_to:
      - agent:argos
```

Desbloqueo: habilitar Shell; reinyectar con `process: feature`, `phase: Verificación`, `persist_ref: docs/features/pr-open-reseal`, `branch_name: feat/pr-open-reseal`, `pbi_ref: AUT-42`, `execution_id` real.
