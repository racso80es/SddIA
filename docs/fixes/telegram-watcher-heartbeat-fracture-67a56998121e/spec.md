---
feature_name: telegram-watcher-heartbeat-fracture-67a56998121e
created: "2026-06-20"
process: bug-fix
base: main
scope: telegram-watcher
version_spec: "1.0.0"
incident_ref: "System_Fracture_Detected — 67a56998121e"
---

# Especificación — Heartbeat durante long-poll Telegram

## Diagnóstico (causa raíz)

| Síntoma | Evidencia |
|---------|-----------|
| `System_Fracture_Detected` | 5653 ciclos omitidos; `last_heartbeat=2026-06-16T10:45:34Z` |
| HTTP 409 en logs | Conflicto `getUpdates` (instancia duplicada o webhook activo) |
| Heartbeat starvation | `centinela.tick()` solo tras `getUpdates` bloqueante (timeout=30s); intervalo heartbeat=30s |

El auditor Argos (`daemon-heartbeat-audit`) calcula `missed_cycles = elapsed / interval`. Con el hilo principal bloqueado ≥30s sin emitir `Daemon_Heartbeat`, un PID vivo supera el umbral=3 y dispara fractura sistémica.

## Corrección

### H1 — Keepalive asíncrono

Hilo auxiliar en `run_loop` que invoca `centinela.tick()` cada **10s** (rate-limit interno del runtime emite cada 30s). Garantiza latido durante long-poll.

### H2 — Margen de poll

Reducir `POLL_TIMEOUT` de 30s a **25s** para acortar ventana de bloqueo del hilo principal.

### H3 — Resiliencia 409

- `deleteWebhook` en bootstrap (idempotente).
- Ante HTTP 409 en `getUpdates`: log explícito + backoff 5s antes del siguiente ciclo.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | `cargo build -p telegram-watcher` sin errores |
| CA2 | `--once` exit 2 sin credenciales (comportamiento existente) |
| CA3 | Con credenciales lab: ciclo `--once` emite heartbeat en telemetry |
| CA4 | Hilo keepalive activo solo en modo continuo (no `--once`) |
| CA5 | Logs 409 no detienen el bucle; backoff aplicado |

## Alcance prohibido

- No mutar `telegram-watcher.md` (genoma) salvo bump de versión documental post-merge.
- No tocar Capa 0 Python legacy en `scripts/limbo/`.
