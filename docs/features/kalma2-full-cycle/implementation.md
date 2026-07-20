---
feature_name: kalma2-full-cycle
created: "2026-07-20"
process: feature
items:
  - thermodynamic.rs cycle_phase
  - kalma2-bridge project_status
  - interfaces/kalma2 app.js + style.css
---

# Implementation — kalma2-full-cycle

## Touchpoints (Slice A — forjado)

| # | Path | Cambio |
|---|------|--------|
| 1 | `SddIA/engine/execute-process/src/engine/thermodynamic.rs` | `derive_cycle_phase` + emisión `cycle_phase` en PEC |
| 2 | `SddIA/interfaces/kalma2-bridge/src/main.rs` | `project_status` mapea `initialized` / `awaiting_agents`; orch expone `cycle_phase` |
| 3 | `interfaces/kalma2/app.js` | Poll terminal también en `initialized` / `awaiting_agents` |
| 4 | `interfaces/kalma2/style.css` | Colores status nuevos |

## Decisiones de forja

- L3 compat: PEC sin `cycle_phase` → UI `completed`.
- L4: no se derogó L2 de process-dispatch en esta entrega.
- Slices B/C: contrato en `spec.md`; forja diferida (deuda explícita).

## Pendiente

| Slice | Estado |
|-------|--------|
| B runtime agentes | Documentado; sin invocación física aún |
| C `pbi_body` | Documentado; sin lectura FS en TQM aún |
