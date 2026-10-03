---
document_id: PBI-MERGE-THERMO-03-DELTA-PROFILE
uuid: "78b27ec1-dcc9-49a7-bbbc-21db02500007"
title: "[OPERATIVO] Aduana Git 03 — Triaje por delta y perfil docs-only"
format: markdown
version: "1.0.0"
status: done
created: "2026-10-03"
author: tekton
priority: alta
type: operativo
process: feature
dispatch: true
hu_order: 3
hu_order_total: 6
historia_ref: "docs/todos/historias/[OPERATIVO] Optimización Termodinámica de Aduana Git: Merge de Alta Eficiencia.md"
historia_document_id: HU-MERGE-THERMODYNAMICS
cola_ejecucion: docs/todos/done/
blocked_by:
  - PBI-MERGE-THERMO-02-NET-PRUNE
unblocks:
  - PBI-MERGE-THERMO-04-ATTESTATION
  - PBI-MERGE-THERMO-06-E2E-MEASURE
baseline_decisiones:
  - "D-1: aduana ligera síncrona suficiente; sin doble aduana LLM."
  - "qa_profile es input del proceso, no variable de entorno."
  - "README.md y SddIA/**/*.md y .SddIA/** son activos."
---

# Triaje por delta y perfil docs-only

HU `HU-MERGE-THERMODYNAMICS`, orden **03/06**. HU §2.1 y laudo D-1. AC-1, AC-8, AC-9 y la parte pre-commit de AC-5.

## 0. Filtro A

| Afirmación a evitar | Corrección |
|---|---|
| El bypass evita `cargo test` | Los hooks no ejecutan `cargo`. El bypass evita fases LLM de Argos y el handoff `accept-pr`. |
| `SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF` es el mecanismo de producto | Es atajo de laboratorio. El producto es el input `qa_profile`. La variable no amplía su alcance (AC-8). |
| `docs/`, `historias/` y `README.md` son pasivos | `historias/` ya está bajo `docs/`. `README.md` es parámetro de restricción y prefijo DIA: activo. |
| `git diff --name-only HEAD` | Sin rango refresca el índice (0,9 s medidos). El rango sale del stdin de pre-push. |

## 1. Intención

Un push cuyo árbol no toca genoma ni código paga solo el triaje documental determinista, en el hook y en el suscriptor posterior de `PullRequest_Presented`.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-1 | `delta_paths` usa `remote_sha..local_sha`. Ref nueva: `origin/main...local_sha`, o `main...local_sha` si no hay `origin/main`. |
| R-2 | `PASSIVE_PREFIXES`: `docs/` y `*.md` fuera de `SddIA/`, `.SddIA/` y `README.md`. El resto es activo. |
| R-3 | `GENOME_PREFIXES` vive en `hook_common.sh` (SSOT). Se amplía con `SddIA/engine/`, `SddIA/tools/`, `SddIA/scripts/`, `SddIA/core/`, `SddIA/norms/`. `pre_commit_gate.sh` la consume. |
| R-4 | Delta ⊆ pasivo → `Local_QA_Requested` con `qa_profile: docs-only` y `blocking: true`. Delta activo → `qa_profile: full`. |
| R-5 | `pull-request-review` con `docs-only` ejecuta el triaje documental determinista y omite fases LLM y el handoff `accept-pr`, con `note: skipped-by-profile`. |
| R-6 | `delivery-close-cycle` copia `qa_profile` al payload de `PullRequest_Presented`. El suscriptor `argos.pull-request-review` lo honra (AC-9). |
| R-7 | Contratos: `qa_profile` OPTIONAL en `local-qa-requested` y `pull-request-presented`; input en `pull-request-review` y propagación en `delivery-close-cycle`. Bumps vía `entity-manager`. Registro en `SddIA/evolution/`. |
| R-8 | `pre-commit`: `audit-eda-coverage --scan` solo si `staged_touches_genome`. `verify-process-integrity` siempre. Sigue bloqueando con huérfanos si el staged toca genoma (AC-5). |
| R-9 | `hook-timings` rellena `delta_class` (`passive` \| `active`). |

## 3. Plan

1. Clasificación en `hook_common.sh` y payload en `pre_push_gate.sh`.
2. Input `qa_profile` en el motor de `pull-request-review` y propagación en `delivery-close-cycle`.
3. Bumps de contratos por `entity-manager`.
4. Pre-commit condicional.
5. Tests de perfil, de herencia en `PullRequest_Presented` y de no regresión del pre-commit.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-1 | Rama nueva con delta pasivo completa `Local_QA_Requested` + `delivery-close-cycle` con `llm_phases: 0`. `total_ms` ≤ coste de push + `gh pr create` + 2 s. | E2E en repo temporal; `hook-timings` |
| AC-8 | `docs-only` es input declarado. `SDDIA_SKIP_HOOKS` y `SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF` no cambian de alcance. | Diff + `verify-process-integrity` |
| AC-9 | `PullRequest_Presented` con `qa_profile: docs-only` produce fases Argos `note: skipped-by-profile` y `llm_phases: 0`. | Test de bus EDA sobre `execution_report.json` |
| AC-5b | Pre-commit sin genoma en staged no ejecuta el scan. Con genoma y `orphan_count>0`, bloquea. `verify-process-integrity` siempre corre. | Test shell |
