---
feature_name: aut-135-refine-item-execution-report
created: "2026-11-27"
process: bug-fix
phase: design
branch_name: fix/aut-135-refine-item-execution-report
persist_ref: docs/fixes/aut-135-refine-item-execution-report
pbi_ref: AUT-135
document_id: PBI-a54a09c6
uuid: 25eecbe2-3707-4dc5-8a43-952eae5dbc1f
tracker_ref: AUT-135
historia_ref: AUT-131
execution_id: "68aebeb8-47ed-4a62-842e-13ad36b1643c"
---
# Spec — AUT-135 execution_report fases honestas (refine-item)

## Misión

`refine-item` debe emitir `execution_report.phases` alineado con `data.status` y el progreso real; prohibido el literal fijo de cuatro fases `executed` en `envelope_status`.

## AC (oráculo)

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-1 | Cada fase refleja outcome real | Unit tests `phases_for_terminal` + inspección envelope |
| AC-2 | `awaiting_agents`: fases posteriores no `executed` | Test fixture Mayeuta simulated |
| AC-3 | `needs_clarification` / `conflict`: sin fases fantasmas `executed` | Tests conflict / DoR fallido |
| AC-4 | `refined`: sellado handler sin fingir agente Argos | Campo `seal_mode: handler_deterministic` |
| AC-5 | Paridad testigos AUT-134 execution_ids | Regresión estructural informe |
| AC-6 | `cargo test -p execute-process` verde | CI paquete motor |

## Superficie

- `SddIA/engine/execute-process/src/engine/handlers/refine_item/run.rs`: `envelope_status`, helper `phases_for_terminal`, call sites.

## Diseño (estabilizado)

1. Centralizar construcción de fases en `phases_for_terminal(status, mayeuta, dor, sellado)`.
2. Mayeuta: `executed` | `awaiting_agents` | `simulated` | `skipped` (reentrada con `verdict.json` previo).
3. Validación DoR: solo si se leyó `verdict.json`; `executed` si `refined`, `needs_clarification` si DoR/veredicto incompleto.
4. Sellado Argos: solo si `remote` y camino `refined`; `handler: refine-item`, `seal_mode: handler_deterministic`.