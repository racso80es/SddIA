---
feature_name: kaizen-ci-workflow-optimization
created: "2026-08-29"
process: refactorization
phase: planning
agents: dedalo
phases:
  - T1-concurrency
  - T2-job-if
  - T3-evolution
  - T4-tekton-docs
  - T5-argos-archive
  - T6-delivery-close
branch_name: feat/kaizen-ci-workflow-optimization
persist_ref: docs/features/kaizen-ci-workflow-optimization
pbi_ref: docs/todos/pending/[ARQUITECTURA] Optimización termodinámica de flujos .md
document_id: PBI-KAIZEN-CI-WORKFLOW-OPTIMIZATION
uuid: d664b94d-3ce8-4b66-a4a7-0ff10570acf9
runtime_execution_id: "780fed96-4a4c-4c5d-a693-f926e7bd79fb"
---

# Plan — kaizen-ci-workflow-optimization

Blueprint Tekton. Contratos: `spec.md`. **Stop planning:** no ejecutar T1–T6 en esta sesión.

Init lab: `execution_id` `780fed96-4a4c-4c5d-a693-f926e7bd79fb` · vehículo `feature` · `process_label: refactorization` · relevo IDE.

## T1 — Concurrency (CA4 / L-CONC-EVENT)

En `.github/workflows/sddia-index-qa.yml`, inyectar el bloque §4.1 de `spec.md` a nivel de workflow. No alterar `on:` ni `permissions:`.

## T2 — `if:` jobs pesados (CA1–CA3 / L-HEAVY-IF / L-FORK-COMPOSE)

1. Añadir `if:` de §4.2 a `eda-bus-e2e-smoke`.
2. Reemplazar `if:` de `eda-iota-physical` por §4.3 (componer fork-guard).
3. Assert: jobs `sddia-index-integrity`, `eda-iota-smoke-simulate`, `wasi-runtime-smoke` sin `if:` nuevo.
4. Assert: step `IOTA_WALLET_SECRET` / *exit 0* intacto.

## T3 — Evolution

Entrada `SddIA/evolution/` UUID `d664b94d-3ce8-4b66-a4a7-0ff10570acf9` (una por ciclo). Texto: segregación push/PR + concurrency. Sin mutar `sddia-qa`.

## T4 — Documental Tekton

`implementation.md` + `execution.md` (frontmatter patrón; `items` / `items_applied`). Registrar `execution_id` de este init.

## T5 — Argos + archive

`validacion.md`: `global`, checks CA1–CA5, `git_changes`, `pbi_archived: true`, `branch: feat/kaizen-ci-workflow-optimization`. Mover PBI canónico a `docs/todos/done/` (mismo `document_id`). **L-DEDUP:** un path.

## T6 — DCC

`delivery-close-cycle` · `source_process: feature` (vehículo) / `process_label: refactorization` · `persist_ref` · `branch_name`. Git: `skill:git-manager`.

## Orden

```text
T1 → T2 → T3 → T4 → T5 → T6
```

## Delegaciones

| Fase | Cápsula |
|------|---------|
| YAML CI | Tekton escritura directa (`.github/` ∉ genoma DA-2) |
| Evolution | Tekton `directories.evolution` (registro, no entidad creator) |
| Git | `skill:git-manager` |
| PR | `action:execute-process` → `delivery-close-cycle` |

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Cancelar Aduana PR | **L-CONC-EVENT** |
| Perder fork-guard | **L-FORK-COMPOSE** + review diff del `if:` |
| Alucinar job `verify-tools-index` | Inventario D1 / spec §2 |
| Recortar `on:` por exceso | **L-PR-IF** |

## Fuera de este plan

Implementar T1–T6 en esta sesión; rehab `refactorization`; mutar `sddia-qa`.
