---
feature_name: ci-index-qa-execution-time
created: "2026-10-25"
process: bug-fix
base: main
tracker_ref: AUT-13
document_id: PBI-FIX-CI-INDEX-QA-EXECUTION-TIME
uuid: 9f4b2c81-5e7a-43d9-8a62-1c08e3d94f2b
branch_name: fix/ci-index-qa-execution-time
persist_ref: docs/fixes/ci-index-qa-execution-time
---
# Spec — AUT-13 (`sddia-index-qa` tiempo de ejecución)

## Misión

Reducir recompilaciones redundantes (LanceDB, labs Linear, artefactos WASI) y podar pasos pesados en `pull_request` sin romper los cinco jobs de `KNOWN_SDDIA_QA_JOBS`. Guardián de suite íntegra en `push` a `main` solo en `sddia-index-qa`. `sddia-installer-smoke` en workflow aparte con `paths` en PR y push (ver `clarify-installer-smoke.md`).

## Corte

| ID | Entrega |
|----|---------|
| C1 | `CARGO_PROFILE_*_DEBUG=0`, `DEV_STRIP=debuginfo`; artefacto `wasi-runtime` solo `.wasm` + bins nativos |
| C2 | Labs Linear con `cargo test -p execute-process --lib …` |
| C3 | Un solo `cargo test` LanceDB + caché `native-test-lance` |
| C4 | Job `changes` + condicionales `lance_memory` / `linear_e2e` |
| C5 | Workflow `sddia-installer-qa.yml` (`paths` en PR y push `main`); `prep-installer-lab-release.sh` acotado |

## Criterios

AC-1..AC-7 según cuerpo Linear AUT-13.
