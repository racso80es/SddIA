# [Kaizen] delivery-close-cycle — fallback shell-executor nativo cuando WASI falla working_directory

**Fecha:** 2026-06-11  
**Origen:** feature `snapshot-friccion-laboratorio-jules` — `delivery-close-cycle` abortó al invocar `shell-executor.wasm`  
**Prioridad:** alta  
**Proceso sugerido:** bug-fix  

---

## Síntoma

```text
execute-process.py --process delivery-close-cycle
→ error: working_directory invalid: No such file or directory (os error 44)
```

Segundo intento (pr_body simplificado) reproduce el fallo en fase que invoca `invoke_shell_executor` (`gh` o `git diff`).

## Causa probable

- `shell-executor.wasm` ejecutado con `wasmtime run --dir=.` no puede `canonicalize()` rutas absolutas del host de forma fiable (mismo patrón que `git-manager.wasm` → fallback Python).
- No existe `_invoke_shell_executor_native` paralelo a `_invoke_git_manager_native`.

## Acción requerida

1. Añadir fallback nativo Python para `shell-executor` en `execute_process_capsules.py` cuando WASM falle (`working_directory invalid`, `executable not found on PATH`).
2. Auditar `capsule_delivery_impact_assessment`: usa `shell-executor` con `git diff` — **violación** de allowlist (`git` prohibido en shell-executor); migrar a `git-manager` o cápsula dedicada.
3. Smoke lab: `delivery-close-cycle` completo (push + `gh pr create`) sin `SDDIA_LAB_SIMULATE_GH_PR`.

## Workaround aplicado (2026-06-11)

Push + `gh pr create` + `gh pr merge` manual en feature `snapshot-friccion-laboratorio-jules`.

## Criterios de aceptación

- [ ] `delivery-close-cycle` exit 0 en laboratorio con `wasmtime` + fallback
- [ ] Sin invocar `git` vía `shell-executor`
- [ ] Documentado en `docs/features/` o fix bajo `docs/fixes/`
