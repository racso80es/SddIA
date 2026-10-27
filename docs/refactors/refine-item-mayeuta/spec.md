---
feature_name: refine-item-mayeuta
created: "2026-10-27"
process: refactorization
tracker_ref: AUT-19
execution_id: "51caf6b7-c386-447b-a380-edf5e8c97d73"
---

# Especificación — refine-item

## Handler

`SddIA/engine/execute-process/src/engine/handlers/refine_item/`:

- **Preparación:** `fetch_issue`, `ensure_canonical_markdown` (cabecera YAML mínima si falta), workspace `.SddIA/workspaces/refine-item/{execution_id}/`.
- **Mayeuta:** `invoke_agent_phase` → `verdict.json`, `item.refined.md`, `clarify.md` (runtime o relevo IDE).
- **DoR:** `dor.rs` por tipo (`hu`, `pbi`, `fix`, `kaizen`, `deuda`, `spike`).
- **Sellado:** condicional; `PROCESS_RENAMED` en `refine-pbi` / `refine-hu`.

## Pendiente (misma entrega)

- Forja genoma: `refine-item.md` (códice), norma `backlog-item-refinement`, baja `refine-pbi`/`refine-hu`, emisores ECST, `codex` 1.1.0.
- `create_issue` para `hu` sin `tracker_ref` en `refine-item`.
- Consumidores `.cursorrules`, hooks, `barcelonaxplorer`.
