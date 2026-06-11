---
feature_name: migracion-rust-wasi
created: "2026-06-11"
process: feature
branch_name: feat/migracion-rust-wasi-certificacion
persist_ref: docs/features/migracion-rust-wasi
pbi_ref: docs/todos/pending/OPERATIVO-PBI-Migracion-Rust-WASI.md
continues_from: docs/features/wasi-poc-ignition
related:
  - docs/features/ci-wasi-runtime-validation
---

# Objetivos — migracion-rust-wasi

Certificar y cerrar la migración de cápsulas ejecutables (skills/tools) a Rust/WASI. La forja base ya está materializada en `main`; esta feature completa la poda, la paridad normativa y el cierre documental del PBI.

## Estado heredado (main)

| Ítem | Estado |
|------|--------|
| Cargo workspace (`SddIA/Cargo.toml`) | ✅ |
| Crate `sddia-io` | ✅ |
| 4 skills + 12 tools en Rust | ✅ |
| `cargo build --workspace --target wasm32-wasip1` | ✅ (warnings menores) |
| CI `wasi-runtime-smoke` (PR #84) | ✅ |
| PoC WASI (`wasi-poc-ignition`) | ✅ |

## Objetivos termodinámicos

| ID | Objetivo | Criterio de aceptación |
|----|----------|------------------------|
| O1 | Poda ontológica | Sin `scripts/skills/*.py` ni fallbacks Python en orquestador para cápsulas migradas |
| O2 | Contratos normativos | `skills-contract.md`, `tools-contract.md` y `README.md` declaran Rust/WASI como sustrato único de cápsulas |
| O3 | Invocación WASI | Orquestador invoca `.wasm` vía `wasmtime`; scripts `.sh`/`.bat` alineados |
| O4 | Safety net I/O | Pánicos/errores devuelven envelope JSON válido (`success: false`, `exitCode > 0`) |
| O5 | Validación E2E | Smoke CI + `eda-bus-e2e-smoke` verdes sin fallback Python |
| O6 | Cierre documental | `validacion.md` APTO + PBI en `docs/todos/done/` en la rama del PR |

## Alcance (manifiesto)

- **In:** certificación, purga de legado Python en cápsulas, actualización de contratos, `validacion.md`, archivo del PBI.
- **Out:** nuevas cápsulas, refactor del intérprete `execute-process.py` (orquestador lab), migración de adapters LanceDB.

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
- Cierre documental en un único PR (`features-documentation-pattern` v1.2.1).
