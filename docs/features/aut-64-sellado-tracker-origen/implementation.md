---
feature_name: aut-64-sellado-tracker-origen
created: "2026-11-03"
process: refactorization
persist_ref: docs/features/aut-64-sellado-tracker-origen
pbi_ref: AUT-64
---

# Implementación — AUT-64

## Motor

- `SddIA/engine/execute-process/src/engine/handlers/tracker_stamp.rs`: sellado atómico, no-op defensivo en bus, `source_process` en outbound.
- Handlers forge/refine: llamada a sellado antes de materializar evento de dominio (forja) o inmediatamente tras descripción (refino).

## Core

- `SddIA/core/event-domain-subscriptions.json`: arrays vacíos en los cuatro eventos.

## Tests

- `tracker_stamp::tests` (incl. delegación a origen + `seal_*`).
- `linear_direct_cycle_e2e`: ciclo AC-7/AC-8 sin doble stamp.
