---
document_id: PBI-FIX-FRACTURE-c339de406e29
title: "[FIX] delivery-close-cycle — fractura sistémica (UNIFICADA en c51acf014c0f)"
format: markdown
version: "2.0.0"
created: "2026-08-29"
updated: "2026-08-29"
status: "unificado"
priority: alta
process: bug-fix
fracture_hash: c339de406e29
fracture_process: delivery-close-cycle
friction_id: F-DCC-ADUANA-EVOLUTION-ESCALA-COLAPSO
incident_ref: "System_Fracture_Detected — c339de406e29"
superseded_by: PBI-FIX-FRACTURE-c51acf014c0f
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - SddIA/engine/execute-process/src/engine/delivery_close.rs
  - docs/todos/pending/[FIX] delivery-close-cycle — barrera de fase simulated (c51acf014c0f).md
---

# [FIX] delivery-close-cycle — fractura sistémica (UNIFICADA)

> **Estado: unificada en `c51acf014c0f` (§F4b).** No trabajar por separado. Este stub conserva la trazabilidad del hash `c339de406e29`; las acciones viven en el PBI destino.

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `delivery-close-cycle` |
| Emisor | `execute-process` |
| Acción intentada | `Aduana evolution` |
| Traza | `diff material sin evolution correlacionada` |

## Motivo de unificación

Misma familia de bucle que `c51acf014c0f` (recurrencia `wasi-runtime-smoke` / evolution gate). Mecanismo real auditado (2026-08-29):

- La fase «Aduana evolution» de `delivery-close-cycle` **bloquea correctamente** con `EVOL_MATERIAL_UNREGISTERED` (`status: blocked`; test `evolution_phase_blocks_unregistered_material_ca12`).
- `emit_dcc_phase_fractures` (`delivery_close.rs:271-298`) escala **todo** `blocked`/`failed` a `System_Fracture_Detected` → materializa este PBI de ruido. Antipatrón idéntico a `F-DIRTY-WORKTREE` (`1d4115c57471`, resuelto): guard determinista escalado a colapso.
- El diagnóstico auto-generado original («recursión hook» → guarda `SDDIA_HOOK_DELIVERY_CLOSE` + `SDDIA_SKIP_HOOKS=1`) es **erróneo/moot**: esa guarda ya existe (`in_delivery_close_cycle`, `6d64bcc7` §3.7).

## Acción necesaria (trasladada a `c51acf014c0f` §F4b)

Discriminar gate-block de colapso en `emit_dcc_phase_fractures`: un `blocked` determinista de aduana (`Aduana evolution`/`Aduana EDA`) **no** emite `System_Fracture_Detected` ni PBI Kintsugi; conserva veredicto accionable (o degrada a `telemetry`).

## Criterio de cierre

- [x] Diagnóstico corregido (no es «recursión hook»)
- [x] Acción necesaria trasladada a `c51acf014c0f` (§F4b + criterio de cierre)
- [x] Unificada; sin trabajo independiente pendiente
