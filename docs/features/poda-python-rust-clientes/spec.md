---
feature_name: poda-python-rust-clientes
created: "2026-07-11"
updated: "2026-07-11"
process: feature
branch_name: feat/poda-python-rust-clientes
persist_ref: docs/features/poda-python-rust-clientes
pbi_ref: docs/todos/pending/[REFACTOR] Poda ejecutables Python — adecuación de clientes a cápsulas Rust.md
impacts_doc: true
pbi_version: "2.0.0"
---

# Especificación técnica — poda-python-rust-clientes

## 1. Contexto

Tras `migracion-execute-process-rust`, `kaizen-rust-capsule-structure` y **Olas 1–3** de esta feature, el runtime caliente opera en Rust. Persisten **~75 ficheros `.py`** (capa QA), **15 cores QA duplicados** respecto a portes Rust, **documentación genómica obsoleta** y **DEBT-K2** en `github-bridge-watcher`.

**PBI v2.0.0** extiende el alcance a **cero Python en el repositorio** y **cero referencias operativas** a rutas o invocaciones `.py`. **Rotura de compatibilidad** con clientes no adaptados: asumida.

## 2. Alcance técnico

### 2.1 Olas 1–3 — Runtime caliente ✅

| Área | Acción | Estado |
|------|--------|--------|
| Engine `execute-process` | Portes cores; eliminar `python_core.rs` | ✅ |
| Clientes | `sddia-run`, `invoke`, `_exec_daemon` → Rust | ✅ |
| Limbo + Cúmulo | Purga `scripts/limbo`; retirar `scripts_limbo` | ✅ |

### 2.2 Ola 4 — Cores QA duplicados

| Python (eliminar) | Rust (SSOT) |
|-------------------|-------------|
| `route_fractal_event_core.py` | `route_fractal_core.rs` |
| `radamanto_batch_core.py` | `radamanto_batch_core.rs` |
| `telemetry_compliance_audit_core.py` | `telemetry_compliance_core.rs` |
| `fix_tool_process_core.py` | `fix_tool_process_core.rs` |
| `cerbero_governance_react_core.py` | `cerbero_governance_react_core.rs` |
| `route_domain_event_core.py` | `route_domain_core.rs` |
| `execute-action.py` | `actions.rs` + handlers nativos |
| `execute_process_core.py` | `execute-process` crate |
| `telegram_notify_core.py`, `telegram_gateway_core.py`, `telegram_fallback_responder_core.py` | cápsulas + engine |
| `daemon_*_core.py`, `governance_daemon_manager_core.py` | `sddia-daemon-runtime` + daemons crates |
| `kalma2_interact_core.py` | porte o eliminación con PoC |

### 2.3 Ola 5 — Capa QA/aduana

| Ruta | Acción |
|------|--------|
| `SddIA/scripts/qa/**/*.py` | Eliminar tras porte a Rust/shell o integración en orquestador |
| `SddIA/scripts/qa/git-hooks/*.py` | Gates nativos |
| `scripts/qa/verify-process-integrity.py` | Eliminar wrapper legacy |
| `scripts/migrate-local-constitutions-once.py` | One-shot: eliminar o portar |
| `github-bridge-watcher` | Portar lógica `github_bridge_process_pr.py` a Rust (DEBT-K2) |

### 2.4 Ola 6 — Documentación genómica

| Ruta | Acción |
|------|--------|
| `SddIA/skills/*.md` | Delivery → crate Rust + `compiled_capsules`; retirar `python .../foo.py` |
| Contratos/normas operativos | Sin rutas `.py` canónicas |
| `README.md`, índices daemons/skills | Alinear comandos de invocación |

### 2.5 Ola 7 — Gate global

Verificación mecánica O11/O12 antes de Argos.

## 3. Fuera de alcance

- `.venv/`, `.tools/` (dependencias de terceros).
- Reescritura histórica de `SddIA/evolution/*.md` (solo registro de cierre).

## 4. Criterios de aceptación

| ID | Criterio | Ola |
|----|----------|-----|
| CA1 | Cero spawn `python3` en engine/daemons/skills/tools | 1–3 ✅ |
| CA2 | `python_core.rs` ausente | 1–3 ✅ |
| CA3 | Tests unitarios Rust cores portados | 1–3 ✅ |
| CA4 | `telegram_fallback` → cápsula Rust | 1–3 ✅ |
| CA5 | `sddia-run.sh` sin `PYTHONPATH` | 1–3 ✅ |
| CA6 | `scripts/limbo/` ausente | 1–3 ✅ |
| CA7 | Cúmulo sin `scripts_limbo` | 1–3 ✅ |
| CA8 | Smoke EDA en verde | 7 |
| CA9 | Evolution registrada | 3 ✅ parcial; 7 completo |
| **CA10** | **Cero ficheros `.py` en repo** (excl. `.venv/`, `.tools/`) | 7 |
| **CA11** | **Cero referencias operativas a `.py` / `python3`** en genoma operativo | 6–7 |
| **CA12** | Cores QA duplicados eliminados | 4 |
| **CA13** | `SddIA/scripts/qa/` sin `.py` | 5 |
| **CA14** | `github-bridge-watcher` sin Python | 5 |
| **CA15** | `SddIA/skills/*.md` alineados a Rust | 6 |
| **CA16** | `cargo test` workspace + smokes verdes | 7 |
| **CA17** | Evolution olas 4–7 | 7 |

## 5. Política de compatibilidad

- No se mantienen shims Python para hooks, clientes externos ni scripts de lab legacy.
- Consumidores deben migrar a binarios Rust, wrappers shell/bat o subcomandos `execute-process`.
- Contratos JSON stdin/stdout se preservan; cambia solo el substrato de ejecución.

## 6. Impacto en documentación

- Feature: `docs/features/poda-python-rust-clientes/`
- PBI v2.0.0 → `docs/todos/done/` en cierre final (Ola 7)
- **No declarar Done** hasta CA10–CA11 cumplidos
