---
feature_name: kaizen-ci-workflow-optimization
created: "2026-08-29"
process: refactorization
branch_name: feat/kaizen-ci-workflow-optimization
persist_ref: docs/features/kaizen-ci-workflow-optimization
pbi_ref: docs/todos/pending/[ARQUITECTURA] Optimización termodinámica de flujos .md
document_id: PBI-KAIZEN-CI-WORKFLOW-OPTIMIZATION
uuid: d664b94d-3ce8-4b66-a4a7-0ff10570acf9
phase: mayeuta-stabilization
agents: mayeuta
runtime_execution_id: "780fed96-4a4c-4c5d-a693-f926e7bd79fb"
---

# Objetivos — kaizen-ci-workflow-optimization

## Objetivo

Bifurcar la línea de montaje de `sddia-index-qa.yml`: *Feedback Inmediato* en `push` a `feat/**`/`fix/**`; *Aduana de Integración* en `pull_request`; *Guardián* en `push` a `main`. Cancelar runs `push` obsoletos sin abortar auditorías de PR.

## Alcance

1. `if:` en `eda-bus-e2e-smoke` y `eda-iota-physical` (componer fork-guard existente).
2. `concurrency` a nivel workflow, grupo por evento, cancel solo en `push`.
3. Evolution UUID `d664b94d-3ce8-4b66-a4a7-0ff10570acf9`.
4. Sin cambiar lógica de `sddia-qa` ni de cápsulas.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | Push a `feat/**`/`fix/**` con PR abierto no ejecuta `eda-bus-e2e-smoke` ni `eda-iota-physical` bajo evento `push`. |
| CA2 | `pull_request` (base `main`) ejecuta E2E + físico; físico respeta fork-guard y omite anclaje si falta `IOTA_WALLET_SECRET`. |
| CA3 | `push` a `main` ejecuta el conjunto completo de jobs. |
| CA4 | Dos `push` <1 min a la misma rama de trabajo cancelan el primero; no cancelan `pull_request` en curso. |
| CA5 | Diff de implementación acotado a `.github/workflows/sddia-index-qa.yml` + evolution + cascada `persist_ref` / PBI. |

## Fuera de alcance

- Recortar `on.pull_request.branches` (`feat/**`, `fix/**`).
- Fragmentar el workflow en varios archivos.
- Mutar genoma `SddIA/tools|skills|process|…`.
- Ejecutar implementación CI en la sesión de planning.

## Restricciones

- Vehículo `feature` + `process_label: refactorization`. Rama `feat/kaizen-ci-workflow-optimization`.
- Init lab `execution_id` `780fed96-4a4c-4c5d-a693-f926e7bd79fb`.
- Git: `skill:git-manager`.
- Cuerpo = `refined_requirements` Dedalo.

## Ley aplicada

- `features-documentation-pattern` v1.2.1.
- `SddIA/norms/external-ai-constraints.md` (DA-2/DA-5; YAML CI no es genoma Cúmulo).
- PBI v1.1.0 (inventario de jobs verificado).
