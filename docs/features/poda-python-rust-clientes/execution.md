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
  - ola-5-capa-qa-cero-py
  - ola-6-docs-genoma
  - ola-7-verify-binaries
  - ola-8-cierre
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

# Verificación binarios (Ola 7)
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo build --workspace
SddIA/target/debug/sddia-qa verify-compiled-capsules
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
| 15 cores QA + `execute-action.py` eliminados | ✅ |
| `cargo test -p execute-process --lib` | ✅ 51/51 |

## Evidencias Ola 5

| Check | Resultado |
|-------|-----------|
| `sddia-qa` binario | ✅ |
| `SddIA/scripts/qa/**/*.py` | ✅ 0 |
| Gate O11 | ✅ 0 `.py` |

## Evidencias Ola 6

| Check | Resultado |
|-------|-----------|
| Skills/normas/README sin refs `.py` operativas | ✅ |
| `event-watcher` / `telegram-watcher` sin `python_bin` | ✅ |

## Evidencias Ola 7

| Check | Resultado |
|-------|-----------|
| `cargo build --workspace` | ✅ |
| `sddia-qa verify-compiled-capsules` | ✅ 24/24 |
| CI gate binarios | ✅ workflow actualizado |

## Evidencias Ola 8

| Check | Resultado |
|-------|-----------|
| Gate O11 | ✅ 0 `.py` |
| Gate O12 | ✅ genoma operativo limpio |
| `cargo test -p execute-process --lib` | ✅ 51/51 |
| `validacion.md` APTO | ✅ |
| PBI → `docs/todos/done/` | ✅ |

## Notas

- Compilar con `CARGO_TARGET_DIR=$PWD/target` para que clientes resuelvan el binario correcto.
- **Done documental:** olas 1–8 cerradas; PR pendiente.

## Gate verificación binarios (Ola 7)

```bash
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo build --workspace
SddIA/target/debug/sddia-qa verify-compiled-capsules [--json]
```

## Gate cero-Python (Ola 8)

```bash
find . -name '*.py' -not -path './.venv/*' -not -path './.tools/*' -not -path './.git/*'
SddIA/target/debug/sddia-qa verify-tools-index
SddIA/target/debug/sddia-qa verify-process-integrity
```
