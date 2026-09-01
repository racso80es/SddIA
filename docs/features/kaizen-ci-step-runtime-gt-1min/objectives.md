---
feature_name: kaizen-ci-step-runtime-gt-1min
created: "2026-09-01"
process: feature
branch_name: feat/kaizen-ci-step-runtime-gt-1min
persist_ref: docs/features/kaizen-ci-step-runtime-gt-1min
pbi_ref: docs/todos/pending/[KAIZEN] CI — optimizar steps >1 min (verify-compiled-capsules y LanceDB).md
document_id: PBI-KAIZEN-CI-STEP-RUNTIME-GT-1MIN
uuid: "530039c9-100b-413a-b3d5-ca632d83acc6"
phase: mayeuta-stabilization
agents: mayeuta
runtime_execution_id: "a13e2476-8474-49ef-ab2f-0d1fe915a21f"
---

# Objetivos — kaizen-ci-step-runtime-gt-1min

## Objetivo

Comprimir los steps >60 s de `.github/workflows/sddia-index-qa.yml` (job `sddia-index-integrity`) sin recortar cobertura del gate de bins ni de los 16 tests LanceDB/ingesta.

## Alcance

1. Cache nativa: key `native-integrity-*` con save solo en integrity; jobs IOTA `lookup-only`.
2. Un `cargo build --workspace` antes de las aduanas QA; `verify-compiled-capsules` = gate I/O. Presupuesto CA1 = suma de ambos steps.
3. Tests LanceDB: un `cargo test` multi-`-p` + `--test memory_evolution_ingest` (lib sin `cfg(test)` de 366 tests).
4. Evolution UUID `530039c9-100b-413a-b3d5-ca632d83acc6`. Cascada `persist_ref`. PBI a `done/` en el mismo PR.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | Suma `Build native workspace` + `verify-compiled-capsules` < 60 s, o techo < 50 % de 340 s justificado con cronómetro. Idem LanceDB step vs 361 s. |
| CA2 | Gate 29 bins `main.rs`. 16 tests (3+5+5+3) o mapeo integración explícito. |
| CA3 | A0 baseline 33477170741 SSOT; tabla de cierre en `validacion.md`. |
| CA4 | Decisión cache en `implementation.md` con números de un run. |
| CA5 | Job `sddia-index-integrity` < 8 min en `pull_request`. |
| CA6 | Hermanos verdes; physical skip-secret no es anclaje. |
| CA7 | Sin mutar `SddIA/tools/`. Evolution anclado al UUID. |

## Fuera de alcance

Segregación push/PR (done). Telemetría CI. MiniLM. Debilitar gate. `build-wasi-capsules.sh`. Polling DA-6.

## Ley aplicada

- `features-documentation-pattern` v1.2.1.
- `external-ai-constraints.md` DA-2/DA-5/DA-6.
- PBI v1.1.0 (diagnóstico verificado).
