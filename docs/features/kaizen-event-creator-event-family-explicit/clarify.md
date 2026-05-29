---
feature_name: kaizen-event-creator-event-family-explicit
created: "2026-05-29"
process: feature
---

# Clarificación — D1.9 cerrado

## Laudo

| Decisión | Valor |
|----------|-------|
| Fallback `domain` en input de proceso | **Eliminado** |
| `effective_event_family` | Solo desde `process_inputs.event_family` trim; vacío → `ValueError` |
| Runtime laboratorio | `resolve_effective_event_family()` en `execute_process_capsules.py` |
| E2E lab eventos | `event_family: domain` en semilla (smoke local) |

## Disparador

Fase 3 (Aduana Universal) mergeada; emisores de instancia ya declaran familia en builders (`eda_bus_utils`). El gap restante era forja de **Clases** vía `event-creator` / `entity-manager`.

## Fuera de alcance

Envelope JSON de instancia sin `event_family` (inferencia por Clase en ruteo) — sin cambio en este Kaizen.
