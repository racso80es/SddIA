---
feature_name: poda-python-rust-clientes
created: "2026-07-11"
process: feature
branch_name: feat/poda-python-rust-clientes
persist_ref: docs/features/poda-python-rust-clientes
items_applied:
  - ola-1-engine-cores
  - ola-2-clientes-shell
  - ola-3-purga-limbo
  - ola-4-cores-qa
  - ola-5-qa-parcial
  - ola-5-capa-qa-cero-py
  - ola-6-docs-genoma
---

# Implementación — poda-python-rust-clientes

## Ola 1 — engine `execute-process`

| Módulo | Acción |
|--------|--------|
| `engine/route_fractal_core.rs` | **Nuevo** — enrutador fractal EDA nativo |
| `engine/radamanto_batch_core.rs` | **Nuevo** — batch Radamanto |
| `engine/telemetry_compliance_core.rs` | **Nuevo** — auditoría cumplimiento termodinámico |
| `engine/fix_tool_process_core.rs` | **Nuevo** — sandbox fix-tool |
| `engine/cerbero_governance_react_core.rs` | **Nuevo** — reacción Cerbero RBAC |
| `engine/fractal_bus.rs` | **Nuevo** — utilidades bus fractal |
| `engine/telemetry_batch_stub.rs` | **Nuevo** — stub batch telemetría |
| `engine/eda_coverage.rs` | `scan_orphans` nativo (paridad `--scan`) |
| `engine/python_core.rs` | **Eliminado** |
| `engine/route_domain_core.rs` | `invoke_execute_action` → `capsules::invoke_action`; `dispatch_subscriber` exportado |
| `engine/phase_capsules.rs` | Gate EDA genómico sin `audit-entity-eda-coverage.py` |
| `engine/handlers/telegram_fallback.rs` | Eliminado fallback Python/limbo |
| `engine/handlers/daemon_heartbeat.rs` | `audit_telemetry_file` público para fractal |
| `engine/eda_bus_topology.rs` | `safe_remove_path` público |
| `engine/mod.rs` | Wiring módulos nuevos |
| `engine/residual_runner.rs` | Imports desde cores Rust |

## Ola 2 — clientes (shell / bat)

| Touchpoint | Acción |
|------------|--------|
| `sddia-run.sh` | Bash puro; delega en `execute-process` vía `sddia_shell_lib.sh` |
| `SddIA/scripts/common/sddia_shell_lib.sh` | **Nuevo** — resuelve binario, carga bóveda, helper daemon |
| `SddIA/scripts/tools/invoke.sh` + `invoke.bat` | **Nuevo** — `--tool` sobre `execute-process` |
| `SddIA/scripts/tools/*.sh` / `*.bat` | Wrappers actualizados → `invoke.sh` / `invoke.bat` |
| `SddIA/scripts/daemons/_exec_daemon.sh` | Bóveda bash; sin Python |
| `SddIA/scripts/daemons/_run_daemon.bat` | `--emit-shell-env bat` + binario nativo |
| `engine/execute-process/src/main.rs` | Flags `--tool`, `--prefer-native`, `--emit-shell-env [bash\|bat]` |
| `engine/execute-process/src/core/env.rs` | `load_hierarchical_env_merged()`, `emit_shell_env()` |
| `engine/execute-process/src/engine/capsules.rs` | `invoke_tool_capsule_json()` |
| `SddIA/scripts/tools/invoke.py` | **Eliminado** |
| `SddIA/scripts/daemons/_exec_daemon.py` | **Eliminado** |

## Ola 3 — purga limbo

| Touchpoint | Acción |
|------------|--------|
| `SddIA/scripts/limbo/` | **Eliminado** (daemons/skills/tools Python fósil) |
| `SddIA/core/cumulo.paths.json` | v1.5.0 — retirada `directories.scripts_limbo` |
| `SddIA/tools/tools-contract.md` | §3 sin referencia operativa a limbo |
| `SddIA/norms/skills-portability.md` | Checklist apunta a `execution_capsules` |
| `SddIA/scripts/qa/telegram_gateway_transmute.py` | **Nuevo** — helper transmute (ex-limbo) |
| `SddIA/scripts/qa/watcher_idempotency.py` | **Nuevo** — helper D3 idempotencia watcher |
| `SddIA/scripts/qa/*` | Retirados fallbacks limbo (telegram, iota, watcher) |
| `SddIA/evolution/b8e4f2a1-9c3d-4e7b-a1f0-6d2e8c9b4a7f.md` | Registro evolución Ola 3 |

## Ola 4 — purga cores QA duplicados ✅

| Touchpoint | Acción |
|------------|--------|
| 15 `*_core.py` + `execute-action.py` | **Eliminados** |
| `process_lab_shared.py` | Utilidades lab (ex-`execute_process_core`) |
| `rust_invoke.py` | Delegación `--process` / `--action` Rust |
| `telegram_lab.py` | Helpers Telegram (ex-notify/fallback cores) |
| `execute_process_capsules.py` | `RUST_DELEGATED_PROCESSES` + `invoke_rust_action` |
| `main.rs` | Flag `--action` (reemplaza `execute-action.py`) |
| Tests QA radamanto/EDA | Delegación Rust |

## Ola 5 — capa QA/aduana ✅

| Touchpoint | Acción |
|------------|--------|
| `tools/sddia-qa/` | **Nuevo** binario QA: verificadores + smokes CI |
| `engine/verify_process_integrity.rs` | Paridad VPI |
| `execute-process` CLI | `--verify-process-integrity`, `--audit-eda-coverage --scan [--json]` |
| `sddia-daemon-runtime/github_bridge.rs` | DEBT-K2 — `process_pr` nativo |
| `git-hooks/*.sh` | Hooks bash (sin Python) |
| `SddIA/scripts/qa/**/*.py` | **Eliminados** (0 ficheros) |
| `scripts/qa/verify-process-integrity.py` | **Eliminado** (wrapper raíz) |
| `scripts/migrate-local-constitutions-once.py` | **Eliminado** |
| PoC `_browser-func-test-kalma2.py` | **Eliminado** |
| `.github/workflows/sddia-index-qa.yml` | CI vía `sddia-qa` + Rust (sin Python) |
| Gate O11 (repo) | `find . -name '*.py'` excl. `.venv/.tools` → **0** |

## Ola 6 — documentación genómica ✅

| Touchpoint | Acción |
|------------|--------|
| `SddIA/skills/*.md` | Cápsulas Rust vía `compiled_capsules`; invocación `./sddia-run.sh --tool` |
| `SddIA/norms/external-ai-constraints.md` | v1.4.0 — SSOT `sddia_shell_lib.sh`; sin `orchestrator_resolve.py` |
| `SddIA/norms/git-operations.md`, `touchpoints-ia.md`, `execution-contexts.md` | Helpers bash / cápsulas Rust |
| `SddIA/process/*.md`, `actions/*.md`, `events/*.md` | Sin referencias operativas `.py` |
| `SddIA/daemons/*.md`, `daemons-contract.md` | Runtime `native-rust`; bridge IOTA nativo |
| `SddIA/library/norms/features-documentation-pattern.md` | Gates QA vía `sddia-qa` |
| `README.md` | Entrypoints bash/Rust; EDA con Centinelas nativos |
| `event-watcher`, `telegram-watcher` | Eliminado fallback `python_bin()` |
| Gate O12 (genoma operativo) | `SddIA/` excl. `evolution/` → **0 refs `.py` operativas** |

## Pendiente (Ola 7)
- [ ] O11: cero ficheros `.py` (excl. `.venv/`, `.tools/`)
- [ ] O12: cero referencias operativas
- [ ] Argos / `validacion.md` APTO
- [ ] PBI → `docs/todos/done/`
- [ ] PR + `delivery-close-cycle`
