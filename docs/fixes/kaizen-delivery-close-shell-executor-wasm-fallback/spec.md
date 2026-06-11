---
feature_name: kaizen-delivery-close-shell-executor-wasm-fallback
created: "2026-06-11"
process: bug-fix
branch_name: fix/delivery-close-shell-executor-wasm-fallback
persist_ref: docs/fixes/kaizen-delivery-close-shell-executor-wasm-fallback
pbi_ref: docs/todos/pending/Kaizen_delivery-close_shell-executor-wasm-fallback.md
---

# Especificación — fallback shell-executor nativo (WASI cwd)

## Problema

`delivery-close-cycle` invoca `shell-executor.wasm` vía `wasmtime run --dir=.` para `gh pr create`. WASI no puede `canonicalize()` rutas absolutas del host → `working_directory invalid: No such file or directory (os error 44)`.

Además, `capsule_delivery_impact_assessment` enrutaba `git diff` por `shell-executor`, violando el invariante anti-git.

## Cambio

| Archivo | Modificación |
|---------|--------------|
| `SddIA/scripts/qa/execute_process_capsules.py` | `_invoke_shell_executor_native`, `_shell_executor_should_fallback`, `invoke_shell_executor()` con fallback; `_git_diff_name_only` → `git-manager` |
| `scripts/skills/shell-executor.py` | Cápsula nativa (paridad Rust + norma congelada) |
| `scripts/skills/git-manager.py` | Operación `diff_name_only` |

## Reglas de enrutamiento

1. Si `shell-executor.wasm` existe **y** `wasmtime` en PATH → intento WASI.
2. Si falla por `working_directory invalid`, `executable not found on PATH` u homólogos → `scripts/skills/shell-executor.py`.
3. Si wasm/wasmtime ausentes → fallback directo a Python.
4. `git diff --name-only` exclusivamente vía `git-manager` (`diff_name_only`).

## Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| KZ-CA1 | `invoke_shell_executor('gh', …)` exit 0 con wasmtime + fallback | smoke Python |
| KZ-CA2 | `delivery-close-cycle` exit 0 en lab (sin `SDDIA_LAB_SIMULATE_GH_PR`) | smoke proceso |
| KZ-CA3 | Sin `git` vía `shell-executor` | grep + `_git_diff_name_only` |
| KZ-CA4 | Paridad documental | `spec.md`, `implementation.md`, `validacion.md` |

## No objetivos

- Reinstalar `shell-executor.py` como ruta primaria (WASI sigue siendo preferente cuando funciona).
- Alterar contrato ECST ni fases de `delivery-close-cycle`.
