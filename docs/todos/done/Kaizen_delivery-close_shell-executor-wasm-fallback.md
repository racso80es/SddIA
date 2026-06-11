---
document_id: PBI-KAIZEN-DELIVERY-CLOSE-SHELL-EXECUTOR-WASM-FALLBACK
title: "[Kaizen] delivery-close-cycle — fallback shell-executor nativo cuando WASI falla working_directory"
format: markdown
version: "1.0.0"
created: "2026-06-11"
status: done
priority: alta
process: bug-fix
closed: "2026-06-11"
branch_name: fix/delivery-close-shell-executor-wasm-fallback
feature_ref: docs/fixes/kaizen-delivery-close-shell-executor-wasm-fallback
pr_url: https://github.com/racso80es/SddIA/pull/88
origin: docs/todos/pending/Kaizen_delivery-close_shell-executor-wasm-fallback.md
---

# [Kaizen] delivery-close-cycle — fallback shell-executor nativo cuando WASI falla working_directory

| Campo | Valor |
|-------|-------|
| **ID** | `PBI-KAIZEN-DELIVERY-CLOSE-SHELL-EXECUTOR-WASM-FALLBACK` |
| **Estatus** | ✅ Done (pre-merge) |
| **Fix** | [`docs/fixes/kaizen-delivery-close-shell-executor-wasm-fallback/`](../../fixes/kaizen-delivery-close-shell-executor-wasm-fallback/) |
| **Validación** | [`validacion.md`](../../fixes/kaizen-delivery-close-shell-executor-wasm-fallback/validacion.md) — APTO |
| **PR** | [#88](https://github.com/racso80es/SddIA/pull/88) |

## 1. Incidente

`delivery-close-cycle` abortaba al invocar `shell-executor.wasm`:

```text
working_directory invalid: No such file or directory (os error 44)
```

Origen: feature `snapshot-friccion-laboratorio-jules`.

## 2. Causa raíz

WASI con `wasmtime run --dir=.` no puede `canonicalize()` rutas absolutas del host. No existía fallback Python paralelo a `_invoke_git_manager_native`. Además, `capsule_delivery_impact_assessment` enrutaba `git diff` por `shell-executor` (violación anti-git).

## 3. Solución aplicada

| Cambio | Detalle |
|--------|---------|
| Fallback nativo | `_invoke_shell_executor_native` + `scripts/skills/shell-executor.py` |
| Anti-git | `_git_diff_name_only` → `git-manager diff_name_only` |
| Resiliencia gh | URL de PR existente extraída de `stderr` |

## 4. Criterios de aceptación

| ID | Criterio | Estado |
|----|----------|--------|
| KZ-CA1 | `delivery-close-cycle` exit 0 con wasmtime + fallback | ✅ |
| KZ-CA2 | Sin `git` vía `shell-executor` | ✅ |
| KZ-CA3 | Documentado en `docs/fixes/` | ✅ |
| KZ-CA4 | Paridad documental (`validacion.md` APTO, PBI archivado) | ✅ |
