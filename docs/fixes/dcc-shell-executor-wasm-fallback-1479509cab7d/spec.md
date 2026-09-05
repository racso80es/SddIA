---
feature_name: dcc-shell-executor-wasm-fallback-1479509cab7d
created: "2026-09-05"
process: bug-fix
base: main
scope: dcc-shell-executor-wasm-fallback
branch_name: fix/dcc-shell-executor-wasm-fallback-1479509cab7d
persist_ref: docs/fixes/dcc-shell-executor-wasm-fallback-1479509cab7d
pbi_ref: docs/todos/pending/[FIX] delivery-close-cycle — fractura sistémica (1479509cab7d).md
document_id: PBI-FIX-FRACTURE-1479509cab7d
uuid: "ca61b900-e474-4ebb-a623-4baf8ffd5f22"
execution_id: "4fef455f-9155-42e2-b39d-f5085e167607"
fracture_hash: 1479509cab7d
---

# Especificación — fractura `1479509cab7d` (centinela `shell-executor`)

## Problema

`delivery-close-cycle` fase **Apertura en forja** colapsó con traza:

```text
shell-executor wasm fallback marker
```

Hash `sha256(traza)[:12] = 1479509cab7d` (verificado). `emit_dcc_phase_fractures` abrió Kintsugi. Receta WASI/lab ≠ colapso ontológico (`A-COMPILE-RECIPE-NO-KINTSUGI`, paridad Ola 3 `ca3d901fdc9a`).

PBI v1.0.0 committed incrustó verbatim el cubo Mayeuta `is_remote_branch_absent_trace` (`Head sha can't be blank`). Ese predicado **no casa** el centinela: contaminación de asignación, no de matching. `halt_after_push` ya existe desde `b2698e1`. Prohibido reimplementarlo.

## Defectos (motor `execute-process`)

| ID | Defecto | Archivo |
|----|---------|---------|
| D1 | Tras fallo WASM, si `resolve_capsule_native` es `None`, `invoke_shell_executor` re-ejecuta WASM y fuga el centinela | `capsules.rs` |
| D2 | `run_shell` convierte marcadores a centinela también en vía nativa | `capsules.rs` |
| D3 | `dcc_lab_binary_missing_trace` no casa el centinela (el literal canónico Ola 3 **sí** está cubierto) | `delivery_close.rs` |
| D4 | Mayeuta sin cubo para el centinela | `enrich_fracture_pbi_kaizen.rs` |

## Cambio requerido

1. Decisión pura (testeable sin wasmtime): WASM fallido + nativo ausente → error canónico Ola 3 + sufijo WASI; **no** re-ejecutar WASM. Vía nativa: nunca emitir el centinela.
2. Añadir el centinela a `dcc_lab_binary_missing_trace`. No `fail_soft`. No friction_id nuevo.
3. Cubo Mayeuta `refactor_tool` antes del catch-all. Cero texto head-sha. El cubo head-sha permanece para **su** traza.

## Fuera de alcance

- `phase_capsules.rs` / `classify_delivery_error` / `F-DCC-LAB-BINARY-MISSING` (inventado).
- Genoma DA-2. Reabrir `halt_after_push`. Compilar `shell-executor` como único remedio.

## Criterios de aceptación

PBI §5: **KZ-DCC-CA1** … **KZ-DCC-CA5**.
