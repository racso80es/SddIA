---
feature_name: bug-fix-fracture-1d4115c57471
created: "2026-08-29"
process: bug-fix
base: main
scope: dirty-worktree-no-system-fracture
branch_name: fix/bug-fix-fracture-1d4115c57471
persist_ref: docs/fixes/bug-fix-fracture-1d4115c57471
pbi_ref: docs/todos/pending/[FIX] bug-fix — fractura sistémica (1d4115c57471).md
document_id: PBI-FIX-FRACTURE-1d4115c57471
execution_id: "9b0ac29e-b064-4e87-a41c-ecfd7d66525a"
---

# Especificación — fractura `1d4115c57471` (dirty-worktree ≠ colapso)

## Problema

`workspace_init` aborta ante paths dirty fuera de `persist_ref`/`pbi_ref` (`L-DIRTY-INIT`, `F-DIRTY-WORKTREE`). El abort es **correcto**.

El defecto es el **fan-out ontológico**: `emit_workspace_init_fracture` escribe `System_Fracture_Detected` en `eda_bus.pending`. Cúmulo materializa PBI Kintsugi y Mayeuta lo trata como colapso de proceso oficial. Higiene pre-flight se colapsa con fallo de runtime (fetch/checkout/cápsula).

Incidente `1d4115c57471`: ciclo `bug-fix` lanzado con 18 paths de genoma/core/interfaces sucios. El guard protegió la rama. El PBI es ruido.

## Cambio requerido

| Área | Artefacto | Vía |
|------|-----------|-----|
| Emisión de fractura | `workspace_init.rs` — `dirty_paths_outside_scope` / `emit_workspace_init_fracture` | parche motor (fuera de DA-2) |
| Telemetría (opcional) | `eda_fractal.telemetry` — evento de guard, no dominio | motor |
| Tests | abort dirty sin fichero `System_Fracture_Detected` en pending | mismo PR |

### Contrato

| Condición | Abort `Err` | `System_Fracture_Detected` | PBI Kintsugi |
|-----------|-------------|----------------------------|--------------|
| Dirty fuera de scope (`F-DIRTY-WORKTREE`) | Sí | **No** (hoy: Sí — defecto) | **No** |
| `SDDIA_LAB_ALLOW_DIRTY=1` | No | No | No |
| `SDDIA_LAB_SKIP_GIT` / `git_required=false` | No (salta git) | No | No |
| Fallo fetch/checkout/git-manager | Sí (`Err` de cápsula) | Sin cambio: **no** emite hoy; no añadir emisión en este ciclo | — |

`error: dirty-worktree` + lista de paths se conserva. Escape `SDDIA_LAB_ALLOW_DIRTY` intacto.

Si se emite señal de auditoría: **solo** familia `telemetry` (p. ej. `Workspace_Init_Guard_Abort` con `friction_id: F-DIRTY-WORKTREE`). Prohibido promover higiene a `event_family: domain`.

### Genoma

No mutar `SddIA/events/domain/system-fracture-detected.md` en este ciclo (DA-2). El contrato del evento sigue describiendo colapso real. Ausencia de emisión = discriminación.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA-1 | Dirty ajeno → `Err` con prefijo `dirty-worktree:` y lista de paths |
| CA-2 | Ese abort **no** escribe `System_Fracture_Detected` en `eda_bus.pending` |
| CA-3 | Fallo fetch/checkout **no** gana emisión nueva de fractura (paridad actual: solo `Err` de cápsula) |
| CA-4 | `SDDIA_LAB_ALLOW_DIRTY=1` sigue saltando el guard |
| CA-5 | Test unitario: dirty ajeno → error + pending sin evento de fractura |
| CA-6 | Cascada `spec`/`plan`/`implementation`/`execution`/`validacion` APTO; PBI en `docs/todos/done/` en el mismo PR |

## Fuera de alcance

- Rediseñar `L-DIRTY-INIT` (el guard permanece).
- Taxonomía genérica `severity` en el contrato del evento (kaizen mayor; requiere `entity-manager`).
- Reabrir `delivery-close-cycle` sobre fases `simulated` (PBI `c51acf014c0f`; PR #230 es síntoma).
- Mutación de genoma (`directories.events`).
