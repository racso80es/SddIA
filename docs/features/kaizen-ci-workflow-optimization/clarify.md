---
feature_name: kaizen-ci-workflow-optimization
created: "2026-08-29"
process: refactorization
purpose: Estabilización Mayeuta — duplicación termodinámica sddia-index-qa
version_clarify: "1.0.0"
execution_id: "780fed96-4a4c-4c5d-a693-f926e7bd79fb"
pbi_ref: docs/todos/pending/[ARQUITECTURA] Optimización termodinámica de flujos .md
document_id: PBI-KAIZEN-CI-WORKFLOW-OPTIMIZATION
uuid: d664b94d-3ce8-4b66-a4a7-0ff10570acf9
---

# Clarificación — kaizen-ci-workflow-optimization

Transcript Mayeuta. Semilla: PBI `PBI-KAIZEN-CI-WORKFLOW-OPTIMIZATION` v1.1.0. Init lab `execution_id` `780fed96-4a4c-4c5d-a693-f926e7bd79fb`.

## D0 — Apertura

| Pregunta | Decisión |
|----------|----------|
| Vehículo CLI | `--process feature` (`refactorization` ∈ revoked). `process_label: refactorization`. Relé IDE + skip archive/DCC. |
| `feature_name` | `kaizen-ci-workflow-optimization` |
| Rama | `feat/kaizen-ci-workflow-optimization` (prefijo conservado; `canonicalize` no lo pisa) |
| `persist_ref` | `docs/features/kaizen-ci-workflow-optimization` |
| Superficie | Solo `.github/workflows/sddia-index-qa.yml` + evolution + cascada documental |
| Stop planning | Esta sesión: clarify / objectives / spec / plan + commit. Prohibido mutar el YAML de CI. |

## D1 — Jobs reales (anti-alucinación)

Jobs del genoma, no steps:

1. `sddia-index-integrity` (incluye step `verify-tools-index`)
2. `eda-iota-smoke-simulate`
3. `wasi-runtime-smoke`
4. `eda-bus-e2e-smoke`
5. `eda-iota-physical`

## D2 — Duplicación real

`on.push` y `on.pull_request` comparten globs `main`, `feat/**`, `fix/**`. Un push a rama con PR abierto dispara **dos** runs completos. `eda-iota-physical` ya tiene fork-guard + *exit 0* sin `IOTA_WALLET_SECRET`; no se reescribe esa semántica, se **compone**.

## D3 — Condicionales vs fragmentar YAML

**L-SINGLE-FILE:** un solo workflow. Condicionales `if:` en jobs pesados. No partir en dos `.yml`.

## D4 — `on.pull_request` hacia `feat/**` / `fix/**`

El disparador actual incluye PR cuyo *base* es `feat/**` o `fix/**`. CA2 exige aduana al PR contra `main`. **L-PR-IF:** no recortar `on:` en esta ola; `if:` `github.event_name == 'pull_request' || github.ref == 'refs/heads/main'` cubre cualquier PR (incl. base `feat/**`) y el guardián de `main`. Recortar `on.pull_request.branches` queda fuera.

## D5 — Concurrency

**L-CONC-EVENT:** grupo `${{ github.workflow }}-${{ github.event_name }}-${{ github.ref }}`; `cancel-in-progress` solo si `event_name == 'push'`. Prohibido un grupo único workflow+ref (cancelaría la Aduana del PR).

## D6 — Fuera de alcance

- Mutar `sddia-qa`, cápsulas, `Cargo.toml`.
- Rehabilitar `refactorization` en Cerbero.
- Implementar T1–Tn en esta sesión.
