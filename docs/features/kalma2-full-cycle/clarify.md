---
feature_name: kalma2-full-cycle
created: "2026-07-20"
process: feature
purpose: Estabilización Mayeuta del PBI kalma2-full-cycle (arranque ≠ gestión completa)
---

# Clarificación — kalma2-full-cycle

Transcript Mayeuta (2026-07-20). Semilla PBI v1.0.0 → estabilización operativa.

## D0 — Apertura

| Pregunta | Decisión |
|----------|----------|
| Proceso | `feature` v1.3.0 |
| `feature_name` | `kalma2-full-cycle` |
| Rama | `feat/kalma2-full-cycle` |
| `persist_ref` | `docs/features/kalma2-full-cycle` |
| `document_id` | `PBI-KALMA2-FULL-CYCLE-RUNTIME` |
| Init lab | `execute-process feature` + `SDDIA_LAB_SKIP_PBI_ARCHIVE=1` + `SDDIA_LAB_SKIP_DELIVERY_CLOSE=1` → `execution_id` `956100c7-c03f-488b-af1e-2624f84bd0b0` |
| Evidencia | `event_id` `e022814f-fc3a-441f-88c5-d60cb5e47e48` · artefacto solo `objectives.md` |

## D1 — Título vs síntoma

| Borrador | Hecho | Decisión |
|----------|-------|----------|
| «Kalma2 no encola / Mayeuta alucina» | Emisión + TQM + init OK | **Descartado** (fósil de validación previa) |
| Expectativa «fix completo desde UI» | Solo arranque L2 + agentes `simulated` | **Núcleo** de esta feature |
| UI `completed` | PEC orquestador ≠ cierre de negocio | Slice **A** obligatorio |

## D2 — Topología ya entregada (no reabrir)

| Capacidad | Estado |
|-----------|--------|
| Bridge → `kalma2-interact` → evento | ✅ |
| TQM despacha hijo | ✅ `kalma2-process-dispatch` |
| Lazo poll `/api/status` | ✅ |
| Runtime Dedalo/Tekton/Argos post-init | ❌ |
| Semántica status honesta | ❌ |

## D3 — Rebanadas vinculantes

| Slice | Alcance | Orden |
|-------|---------|-------|
| **A** | `cycle_phase` en PEC + `project_status` + UI | 1 (esta iteración) |
| **B** | Runtime agentes post-init | 2 (diseño + stub/contrato; forja profunda puede diferirse) |
| **C** | Consumo cuerpo `pbi_ref` | 3 (con B o tras B) |

## D4 — Laudos propuestos (handoff Dedalo)

| Ref | Pregunta | Propuesta Mayeuta |
|-----|----------|-------------------|
| **L1** | ¿Qué estados UI nuevos? | `initialized` · `awaiting_agents` · conservar `pending`/`routed`/`completed`/`failed` |
| **L2** | ¿Cuándo `completed`? | Solo si `cycle_phase=completed` (sin fases `simulated` de agentes) |
| **L3** | Legacy PEC sin `cycle_phase` | Compat: proyectar `completed` como hoy |
| **L4** | ¿Derogar L2 process-dispatch ya? | **No** en slice A; solo honestidad. B condiciona full-cycle |
| **L5** | Runtime B | Preferir B1 (CLI/Agent SDK) con contrato; B2 evento handoff como evolución |

## D5 — Fuera

Remediación watchers/fracturas · IOTA · Cerbero en Kalma2 · restaurar bridge Python.
