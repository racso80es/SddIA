---
feature_name: kaizen-ci-workflow-optimization
created: "2026-08-29"
process: refactorization
phase: execution
agents: tekton
items_applied:
  - T1-concurrency
  - T2-job-if
  - T3-evolution
  - T4-tekton-docs
  - T5-argos-archive
branch_name: feat/kaizen-ci-workflow-optimization
persist_ref: docs/features/kaizen-ci-workflow-optimization
pbi_ref: docs/todos/done/PBI-KAIZEN-CI-WORKFLOW-OPTIMIZATION.md
document_id: PBI-KAIZEN-CI-WORKFLOW-OPTIMIZATION
uuid: d664b94d-3ce8-4b66-a4a7-0ff10570acf9
runtime_execution_id: "780fed96-4a4c-4c5d-a693-f926e7bd79fb"
---

# Execution — kaizen-ci-workflow-optimization

## T1 — Concurrency

Bloque inyectado tras `on:` en `sddia-index-qa.yml`:

```yaml
concurrency:
  group: ${{ github.workflow }}-${{ github.event_name }}-${{ github.ref }}
  cancel-in-progress: ${{ github.event_name == 'push' }}
```

## T2 — Jobs pesados

| Job | `if:` |
|-----|-------|
| `eda-bus-e2e-smoke` | `pull_request \|\| refs/heads/main` |
| `eda-iota-physical` | (PR ∨ main) ∧ fork-guard legado |

Jobs `sddia-index-integrity`, `eda-iota-smoke-simulate`, `wasi-runtime-smoke`: sin `if:` nuevo. Step `IOTA_WALLET_SECRET` / *exit 0*: intacto.

## T3 — Evolution

UUID `d664b94d-3ce8-4b66-a4a7-0ff10570acf9` · rehash vía `sddia-qa evolution-rehash`.

## T4–T5 — Documental

Cascada completa + PBI → `docs/todos/done/PBI-KAIZEN-CI-WORKFLOW-OPTIMIZATION.md`.

## T6 — DCC

| Campo | Valor |
|-------|--------|
| `execution_id` | `18a0f24b-888a-4649-a30a-f7a24133c1b8` |
| Aduanas pre-push | evolution · EDA · índices — OK |
| Push DCC | KO — PAT sin scope `workflow` en `.github/workflows/` |
| Recuperación | `gh auth setup-git` + `git push -u origin HEAD` |
| PR | https://github.com/racso80es/SddIA/pull/227 (`gh pr create`) |
| ECST Presentación | `PullRequest_Presented` `1aa8b666-fdf3-4874-bde8-7dca3c26d6ab` vía `emit-pr-presented-event` |

## CI empírico (PR #227)

| Evento | Run | E2E | Físico | Fast-fail (3 jobs) |
|--------|-----|-----|--------|---------------------|
| `push` | [33258097811](https://github.com/racso80es/SddIA/actions/runs/33258097811) | SKIPPED | SKIPPED | SUCCESS |
| `pull_request` | [33258099388](https://github.com/racso80es/SddIA/actions/runs/33258099388) | SUCCESS | SUCCESS | SUCCESS |
