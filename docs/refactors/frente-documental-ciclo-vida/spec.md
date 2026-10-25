---
feature_name: frente-documental-ciclo-vida
created: "2026-10-25"
process: refactorization
tracker_ref: AUT-5
execution_id: "c3a91f2e-7d4b-4e1a-9f06-8b2d4e5f1a0c"
---

# Spec — frontmatter del ítem

## Cambios

| Componente | Antes | Después |
|------------|-------|---------|
| `forge_pbi` | `status: pending` | `tracker_column: backlog`, `refinement: pending` |
| `refine-pbi` / `refine-hu` | `status: refinado/a` | `tracker_column: todo`, `refinement: refined`, `refined` |
| `tracker-stamp` (Linear) | solo columna remota | + parche de `tracker_column` en `description` |
| `tracker-stamp` (file) | no-op | parche en markdown local vía `pbi_ref` / `persist_ref` |

Módulo compartido: `SddIA/engine/execute-process/src/engine/item_lifecycle_frontmatter.rs`.

## Fuera de corte

Renombrar buckets `docs/todos/*`. Cambiar `persist_ref` por transición de columna. AUT-6 (fricciones forge/WASM).
