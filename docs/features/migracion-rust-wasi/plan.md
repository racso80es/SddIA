---
feature_name: migracion-rust-wasi
created: "2026-06-11"
process: feature
branch_name: feat/migracion-rust-wasi-certificacion
persist_ref: docs/features/migracion-rust-wasi
agent_planificador: tekton
---

# Plan — migracion-rust-wasi (certificación)

## Fase 1: Inicialización ✅

- [x] `execute-process.py --process feature` → rama `feat/migracion-rust-wasi-certificacion`
- [x] `_init-feature.json`, `objectives.md`, `clarify.md`

## Fase 2: Análisis de impacto (Aduana preventiva)

- [ ] Inventariar referencias a `scripts/skills/*.py` en orquestador, procesos y CI
- [ ] Mapear scripts `.sh`/`.bat`/`.ps1` que invocan binarios nativos vs wasmtime
- [ ] Registrar touchpoints en `implementation.md`

## Fase 3: Poda ontológica

- [ ] Eliminar `scripts/skills/git-manager.py`, `cryptography-manager.py`, `bus-operator.py`, `shell-executor.py`
- [ ] Retirar funciones fallback Python en `execute_process_capsules.py`
- [ ] Verificar que `SDDIA_CI_REQUIRE_WASI=1` no regresa a Python

## Fase 4: Normativa y contratos

- [ ] Actualizar `README.md` (sustrato Rust/WASI único para cápsulas)
- [ ] Actualizar `skills-contract.md` y `tools-contract.md`
- [ ] Revisar `capsule-json-io.md` si aplica safety net de pánicos

## Fase 5: Build y validación

- [ ] `cargo build --workspace --target wasm32-wasip1` (resolver warnings críticos)
- [ ] Smoke local: `run-wasi-ci-smoke.py`
- [ ] `eda-bus-e2e-smoke` sin fallback
- [ ] Redactar `validacion.md` con `global: APTO`

## Fase 6: Cierre documental en rama

- [ ] Mover PBI a `docs/todos/done/`
- [ ] `pbi_archived: true` en `validacion.md`
- [ ] `delivery-close-cycle` → PR único
