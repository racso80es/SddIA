---
feature_name: core-git-isolation-lab-absolute
created: "2026-10-30"
process: bug-fix
branch_name: fix/core-git-isolation-lab-absolute
persist_ref: docs/fixes/core-git-isolation-lab-absolute
pbi_ref: AUT-6
tracker_ref: AUT-6
execution_id: "652511ad-2740-49f1-8ef5-ef9397fadefe"
---

# Especificación — `PROJECT_GIT_INVALID` con lab absoluto anidado (AUT-6)

## Problema

`project_binding::bind` con `project_slug: core` compara la raíz del repo con cada `project_root` del índice `.SddIA/projects/`. L-ISOLATION ya omite entradas **relativas**. Los tests AC de `refine.rs` / `forge_pbi.rs` registran labs con `project_root` **absoluto** (`_refine_lab_*`, `_forge_lab_*`). Si el cleanup del test no corre, la fila huérfana niega todo el Core-self.

## Cambio

En `assert_git_isolation_for_slug`, cuando el slug enlazado es `core`, omitir `other` si `core_nested_index_exempt`:

- Primer segmento bajo la raíz del Core: `_refine_lab_*` o `_forge_lab_*`.
- Ruta bajo `docs/_fixtures/` (absoluta anidada; las relativas ya las cubre L-ISOLATION).

## Criterios de aceptación

| ID | Verificación |
|----|----------------|
| AC-1 | `bind(core)` OK con índice `_refine_lab_*` de `project_root` absoluto anidado |
| AC-2 | `bind(core)` sigue fallando con cliente real anidado (no lab/fixture) |
| AC-3 | Proyecto absoluto fuera del repo sin falso positivo (sin cambio de contrato) |
| AC-4 | Tests `bind_core_tolerates_absolute_refine_lab_in_index` y `bind_core_still_rejects_absolute_nested_real_project` |

## Fuera de alcance

AUT-8 (fractura en deny), AUT-11 (WASM token), relajar aislamiento entre repos distintos, cambiar L-ROOT.
