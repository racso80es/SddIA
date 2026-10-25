---
feature_name: ci-index-qa-execution-time
created: "2026-10-25"
process: bug-fix
phases:
  - id: T1
    title: Labs Linear con --lib
  - id: T2
    title: Instalador acotado y workflow propio
  - id: T3
    title: debuginfo=0 y artefacto WASI ligero
  - id: T4
    title: Job changes + cargo test unificado + Linear condicional
  - id: T5
    title: Cascada documental y evolution
execution_id: "fe2e81d3-8e1b-42da-83d6-dee0a3661acf"
tracker_ref: AUT-13
---

# Plan — ci-index-qa-execution-time

| Id | Tarea | Archivos | Hecho |
|----|--------|----------|--------|
| T1 | `--lib` en runners Linear de `sddia-qa`. | `SddIA/tools/sddia-qa/src/linear_direct_cycle_e2e_lab.rs`, `linear_done_gate_both_e2e_lab.rs` | AC-1 |
| T2 | Prep `-p execute-process`; workflow `sddia-installer-qa.yml` con `paths:`; quitar smoke del índice. | `SddIA/scripts/qa/prep-installer-lab-release.sh`, `.github/workflows/sddia-installer-qa.yml`, `.github/workflows/sddia-index-qa.yml` | AC-2 |
| T3 | Perfiles debug/strip en `env:`; stage solo `*.wasm` + bins. | `.github/workflows/sddia-index-qa.yml` | AC-3 |
| T4 | Job `changes` + `classify-sddia-index-qa-changes.sh`; un `cargo test` en integrity; Linear gated; caché `native-test-lance`. | `.github/workflows/sddia-index-qa.yml`, `SddIA/scripts/qa/classify-sddia-index-qa-changes.sh` | AC-4, AC-5, AC-6 |
| T5 | `implementation.md` / `execution.md`; hito evolution; handoff Argos (`validacion.md`). | `docs/fixes/ci-index-qa-execution-time/*`, `SddIA/evolution/*` | AC-7 |

Nota Tekton: C1–C5 ya forjados en rama; T5 y acta de commits/`execution.md` son el residual obligatorio antes de Verificación.
