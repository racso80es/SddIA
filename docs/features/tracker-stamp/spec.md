---
feature_name: tracker-stamp
created: "2026-10-02"
process: feature
---

# Especificación — tracker-stamp

## Proceso `tracker-stamp`

Suscriptor de eventos de ciclo de vida (p.ej. `Work_Initiated`, eventos PR) con `tracker_ref` opcional.

## Comportamiento

- Invoca `linear-tracker-adapter` (transición/comentario).
- Fail-soft: emite `Tracker_Sync_Failed` sin tumbar el bus.
- Correlación D7a: `tracker_ref` en payloads PR cuando aplica.
