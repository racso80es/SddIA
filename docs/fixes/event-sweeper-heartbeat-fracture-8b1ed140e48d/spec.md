---
feature_name: event-sweeper-heartbeat-fracture-8b1ed140e48d
created: "2026-06-20"
process: bug-fix
base: main
scope: event-sweeper
version_spec: "1.0.0"
incident_ref: "System_Fracture_Detected — 8b1ed140e48d"
---

# Especificación — Heartbeat durante sweep EDA

## Diagnóstico (causa raíz)

| Síntoma | Evidencia |
|---------|-----------|
| `System_Fracture_Detected` | 3 ciclos omitidos (`8b1ed140e48d`, `ff0989e5b8c0`) |
| Proceso | `event-sweeper` + `daemon-heartbeat-audit` |
| Heartbeat starvation | `centinela.tick()` solo tras `sweep_once()`; barrido largo sobre `pending/` bloquea el hilo ≥30s |

**No relacionado** con `docs/fixes/event-pending-sweeper/` (cierre automático post-route); aquí el fallo es telemetría `Daemon_Heartbeat`.

## Corrección

### H1 — Keepalive asíncrono

Hilo auxiliar en modo continuo: `tick()` cada 10s mientras el bucle principal ejecuta `sweep_once`.

### H2 — Tick explícito pre-sleep

Mantener `tick()` al cierre de ciclo como refuerzo (rate-limit runtime).

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | `cargo build -p event-sweeper` OK |
| CA2 | `event-sweeper --once --json` exit 0 |
| CA3 | Keepalive solo en modo continuo |
| CA4 | `daemon-heartbeat-audit` sweep OK |
| CA5 | PBIs `8b1ed140e48d` y `ff0989e5b8c0` en `done/` |
