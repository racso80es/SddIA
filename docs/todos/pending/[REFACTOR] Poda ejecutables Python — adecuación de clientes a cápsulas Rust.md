---
document_id: PBI-REFACTOR-PODA-PYTHON-RUST
title: "[REFACTOR] Poda ejecutables Python — adecuación de clientes a cápsulas Rust"
format: markdown
version: "2.0.0"
created: "2026-07-11"
updated: "2026-07-11"
status: "abierto"
priority: alta
process: refactorization
related:
  - README.md
  - SddIA/CONSTITUTION_CORE.md
  - SddIA/core/cumulo.paths.json
  - docs/todos/done/[ARQUITECTURA] Migración execute-process a Rust nativo (orquestador soberano).md
  - docs/todos/done/[FIX] Porte procesos residuales capsules bridge a Rust.md
  - docs/todos/done/[FIX] Porte route-domain-event core a Rust (eliminar route bridge).md
  - docs/features/poda-python-rust-clientes/
---

# [REFACTOR] Poda ejecutables Python — adecuación de clientes a cápsulas Rust

## Contexto

El Core mantenía **96 ficheros `.py`** pese a que todas las cápsulas (daemons, skills, tools) y el orquestador `execute-process` ya disponen de crate Rust. Las **Olas 1–3** cerraron la ruta caliente (engine, clientes, limbo). Persisten **~75 `.py`** en capa QA/aduana, **cores Python duplicados** respecto a portes Rust, **documentación genómica obsoleta** (`SddIA/skills/*.md` citando `.py`) y **un spawn Python residual** en `github-bridge-watcher` (DEBT-K2).

**Política v2.0.0:** el PBI exige **cero ficheros `.py`** en el repositorio (excl. dependencias de terceros bajo `.venv/` y `.tools/`) y **cero referencias operativas** a rutas o invocaciones Python. **Se asume rotura de compatibilidad** con clientes, scripts, hooks y documentación no adaptados.

## Inventario — Olas 1–3 (cerradas)

| Cliente | Dependencia Python | Estado |
|---------|--------------------|--------|
| `python_core.rs` | cores EDA Python | ✅ eliminado; portes Rust |
| `route_domain_core.rs` | `execute-action.py` | ✅ invocación nativa |
| `phase_capsules.rs` | `audit-entity-eda-coverage.py` | ✅ módulo Rust |
| `telegram_fallback.rs` | limbo `send-telegram-notification` | ✅ cápsula Rust |
| `sddia-run.sh` | `orchestrator_resolve.py` | ✅ bash + binario |
| `invoke.py` / `_exec_daemon.py` | Python launchers | ✅ eliminados |
| `SddIA/scripts/limbo/**` | 19+ `.py` fósil | ✅ purgado |

## Inventario — Olas 4–7 (pendientes)

| Bloque | Artefactos | Acción |
|--------|------------|--------|
| **Ola 4 — Cores QA duplicados** | 15 ficheros: `route_fractal_event_core.py`, `radamanto_batch_core.py`, `telemetry_compliance_audit_core.py`, `fix_tool_process_core.py`, `cerbero_governance_react_core.py`, `route_domain_event_core.py`, `execute-action.py`, `execute_process_core.py`, `telegram_*_core.py`, `daemon_*_core.py`, `governance_daemon_manager_core.py`, `kalma2_interact_core.py` | Eliminar tras verificar paridad Rust; tests/golden migrados a `cargo test` |
| **Ola 5 — Capa QA Python** | ~57 `.py` restantes en `SddIA/scripts/qa/` (tests, hooks, runners, utilidades), `scripts/qa/`, `scripts/migrate-local-constitutions-once.py` | Portar a Rust/shell, integrar en `execute-process --verify` / crates de aduana, o eliminar; incluye **DEBT-K2** (`github-bridge-watcher` → `github_bridge_process_pr.py`) |
| **Ola 6 — Documentación genómica** | `SddIA/skills/*.md` (`git-manager.py`, `shell-executor.py`, …), contratos/normas con rutas `.py` operativas, drift en `README`/`daemons-contract` | Actualizar SSOT vía forja: `implementation_path_ref` → crates + `SddIA/target/` |
| **Ola 7 — Gate cero-Python + cierre** | Verificación global, suite Rust, `validacion.md`, evolution, PBI → done | Argos + delivery-close-cycle |

## Fuera de alcance

- Contenido bajo `.venv/` y `.tools/` (dependencias de terceros, no SSOT del Core).
- Histórico inmutable en `SddIA/evolution/*.md` que cite `.py` en pasado (no reescribir; añadir evolution de cierre).

## Mandato

1. **Cero Python en el repo:** ningún fichero `.py` bajo control del proyecto salvo exclusiones de terceros.
2. **Cero referencias operativas a Python:** specs, contratos, normas, skills y daemons no citan rutas `.py` ni `python3` como delivery canónico.
3. **Rotura de compatibilidad asumida:** scripts, hooks, clientes externos y docs no actualizados quedan obsoletos; no se mantiene shim Python.
4. **Paridad JSON intacta:** contratos stdin/stdout (`capsule-json-io`) se preservan en los portes.
5. **Mutación de genoma vía forja** (`entity-manager` / proceso autorizado).

## Olas de entrega

```text
Ola 1 Engine ──► Ola 2 Clientes ──► Ola 3 Limbo/SSOT
      ──► Ola 4 Purga cores QA duplicados
      ──► Ola 5 Capa QA/aduana (hooks, tests, DEBT-K2)
      ──► Ola 6 Docs genómicas (skills, contratos)
      ──► Ola 7 Gate cero-Python + Argos + cierre
```

## Criterios de aceptación

### Olas 1–3 (runtime caliente)

- [x] `rg -l 'python3?' SddIA/engine SddIA/daemons SddIA/skills SddIA/tools` sin spawn en código (excl. comentarios/allowlist shell-executor).
- [x] `python_core.rs` eliminado; cores EDA en Rust con tests verdes.
- [x] Clientes (`sddia-run`, `invoke`, `_exec_daemon`) sin Python.
- [x] `SddIA/scripts/limbo/` no existe; `cumulo.paths.json` sin `scripts_limbo`.

### Olas 4–7 (cero-Python total)

- [ ] **CA10 — Sin ficheros `.py`:** `find . -name '*.py' -not -path './.venv/*' -not -path './.tools/*' -not -path './.git/*'` → vacío.
- [ ] **CA11 — Sin referencias operativas:** `rg -l '\.py|python3?' SddIA docs scripts --glob '!**/.venv/**' --glob '!**/.tools/**' --glob '!**/evolution/**' --glob '!**/features/**/poc-*'` → vacío en rutas operativas (contratos, normas, skills, daemons, README raíz).
- [ ] **CA12 — Cores QA eliminados:** los 15 cores listados en Ola 4 ausentes; paridad cubierta por tests Rust.
- [ ] **CA13 — Capa QA/aduana:** `SddIA/scripts/qa/` sin `.py`; hooks y verificadores operan vía Rust/shell o subcomandos `execute-process`.
- [ ] **CA14 — DEBT-K2 cerrado:** `github-bridge-watcher` sin spawn de `github_bridge_process_pr.py`.
- [ ] **CA15 — Docs skills alineadas:** `{name}.md` bajo `SddIA/skills/` apuntan a crates Rust / `compiled_capsules`, no a `.py`.
- [ ] **CA16 — Suite verde:** `cargo test` workspace + smokes EDA en verde tras poda total.
- [ ] **CA17 — Evolution:** registro UUID entidades afectadas por olas 4–7.

## Definición de Done

Un único PR mergeado en main con `validacion.md` APTO (`pbi_archived: true`, `global: APTO`) y este PBI movido a `docs/todos/done/` en la misma rama. **Done implica CA1–CA17 cumplidos**, no solo runtime caliente.
