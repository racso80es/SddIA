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
---

# Ejecución — poda-python-rust-clientes

## Comandos

```bash
# Compilar en SSOT (obligatorio: target bajo SddIA/target)
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo build -p execute-process
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo test -p execute-process --lib

# Smoke clientes (sin PYTHONPATH)
env -u PYTHONPATH ./sddia-run.sh --process capsule-invoke-smoke --inputs '{}'
echo '{"workspace_path":"'"$(pwd)/.SddIA/workspaces/smoke"'"}' | SddIA/scripts/tools/invoke.sh io-choke

# Purga limbo
test ! -d SddIA/scripts/limbo
rg scripts_limbo SddIA/core/cumulo.paths.json  # debe fallar (clave ausente)

# QA idempotencia watcher
cd SddIA/scripts/qa && python3 -m unittest test_bucle_fantasma_bus.py -v
```

## Evidencias Ola 1

| Check | Resultado |
|-------|-----------|
| `cargo build -p execute-process` | ✅ |
| `cargo test -p execute-process --lib` | ✅ 50/50 |
| `python_core.rs` eliminado | ✅ |
| Spawn `python3` en `SddIA/engine/` | ✅ 0 |
| CA telegram fallback limbo | ✅ fallback Python retirado |

## Evidencias Ola 2

| Check | Resultado |
|-------|-----------|
| `--tool io-choke` | ✅ |
| `invoke.sh io-choke` con `workspace_path` bajo repo | ✅ |
| `sddia-run.sh` sin `PYTHONPATH` | ✅ `capsule_invoked: true` |
| `--emit-shell-env bash` | ✅ |
| `invoke.py` / `_exec_daemon.py` eliminados | ✅ |

## Evidencias Ola 3

| Check | Resultado |
|-------|-----------|
| `SddIA/scripts/limbo/` ausente | ✅ |
| `cumulo.paths.json` sin `scripts_limbo` (v1.5.0) | ✅ |
| QA sin fallback limbo | ✅ |
| `test_bucle_fantasma_bus.py` | ✅ 4/4 |
| Evolution `b8e4f2a1-9c3d-4e7b-a1f0-6d2e8c9b4a7f` | ✅ |

## Evidencias Ola 4

| Check | Resultado |
|-------|-----------|
| 15 cores QA + `execute-action.py` eliminados | ✅ 0 `*_core.py` en QA |
| `--action emit-domain-mutation` (Rust) | ✅ |
| `run_process` delega procesos nativos | ✅ |
| `cargo test -p execute-process --lib` | ✅ 50/50 |
| Tests QA migrados (13 smoke) | ✅ |

## Notas

- Compilar con `CARGO_TARGET_DIR=$PWD/target` para que clientes resuelvan el binario correcto.
- **PBI v2.0.0:** Olas 1–3 cerradas; **Done** requiere Olas 4–7 (cero `.py`, cero referencias, rotura compatibilidad asumida).
- Inventario actual post-Ola 3: **~75 `.py`** en repo fuente (72 en QA + 3 misc).

## Gate pendiente (Ola 7)

```bash
# O11 — cero ficheros .py
find . -name '*.py' -not -path './.venv/*' -not -path './.tools/*' -not -path './.git/*'

# O12 — referencias operativas (objetivo final)
rg '\.py|python3?' SddIA docs scripts --glob '!**/.venv/**' --glob '!**/.tools/**'
```
