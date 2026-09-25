---
feature_name: kaizen-ci-chronic-wasi-runtime-smoke
created: "2026-09-25"
process: feature
branch_name: feat/kaizen-ci-chronic-wasi-runtime-smoke
persist_ref: docs/features/kaizen-ci-chronic-wasi-runtime-smoke
pbi_ref: docs/todos/pending/[KAIZEN] CI crónica — wasi-runtime-smoke.md
document_id: PBI-KAIZEN-CI-CHRONIC-WASI-RUNTIME-SMOKE
uuid: "30a026cf-8227-4315-a521-53ec34a8cc0a"
phase: mayeuta-stabilization
execution_id: "915ff612-6aad-4d82-b92a-6d455d2cfe3d"
---

# Objetivos — kaizen-ci-chronic-wasi-runtime-smoke

## Objetivo

Dejar conforme el universo evolution para que el job `wasi-runtime-smoke` deje de fallar en `evolution gate (universe)` por el placeholder `sha256:pending-pr-292` del registro `83d6eb73-0936-4acb-9f1e-5d519987f1ab`.

## Alcance

1. `sddia-qa evolution-rehash --id 83d6eb73-0936-4acb-9f1e-5d519987f1ab`.
2. `sddia-qa gate-evolution --json --all` con `exitCode: 0` antes del push.
3. Cascada en `docs/features/kaizen-ci-chronic-wasi-runtime-smoke/`. PBI v1.1.0.
4. CA3: run `pull_request` de `sddia-index-qa` con `wasi-runtime-smoke` en success. Cierre documental después de ese verde.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | `hash_integrity` = `sha256:` + 64 hex. Sin `pending-pr-292`. |
| CA2 | Universo evolution local en 0. |
| CA3 | Job `wasi-runtime-smoke` success en el PR. Sin run verde, no hay `global: APTO`. |
| CA4 | Los otros cuatro jobs del mismo run no fallan por este diff. |
| CA5 | Sin mutar `build-wasi-capsules.sh` ni el ledger Radamanto. |

## Fuera de alcance

Exclusión WASI de `thought-graph-access`. Evolución ausente de `8ea6be51`. Dedup del ledger. Split del job. Reverso de `f3b45954`. Polling DA-6.

## Ley aplicada

- `features-documentation-pattern` v1.2.1.
- `external-ai-constraints.md` DA-2 / DA-5 / DA-6.
- PBI v1.1.0.
