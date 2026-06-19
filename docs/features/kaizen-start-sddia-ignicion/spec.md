---
feature_name: kaizen-start-sddia-ignicion
created: "2026-06-19"
process: feature
base: main
scope: kaizen-start-sddia-ignicion
version_spec: "1.0.0"
uuid: aad51b93-198b-4cf7-b6a8-195d7f988fb5
---

# Especificación — Kaizen start-sddia ignición ecosistema

## Hito 1 — Corrección de rutas

| Antes (roto) | Después (SSOT) |
|--------------|----------------|
| `.SddIA/scripts/daemons/event-watcher.sh` | `SddIA/scripts/daemons/event-watcher.sh` |
| `.SddIA/scripts/daemons/event-sweeper.sh` | `SddIA/scripts/daemons/event-sweeper.sh` |
| `.SddIA/scripts/daemons/telegram-watcher.sh` | `SddIA/scripts/daemons/telegram-watcher.sh` |
| `.SddIA/scripts/daemons/github-bridge-watcher.sh` | `SddIA/scripts/daemons/github-bridge-watcher.sh` |
| `.SddIA/client/sddia-client-bridge.py` | Sin cambio (instancia) |

## Hito 2 — Endurecimiento del script

| Capacidad | Comportamiento |
|-----------|----------------|
| Ancla `REPO_ROOT` | `cd` al directorio del script al inicio |
| Centinelas obligatorios | `event-watcher`, `event-sweeper` — aborta si < 2 activos |
| Centinelas opcionales | `telegram-watcher`, `github-bridge-watcher` — omitidos si ausentes |
| Health Kalma2 | Espera HTTP en `SDDIA_CLIENT_PORT` (default 8765) |
| `cleanup` | `kill jobs`, `pkill -x` por centinela, `pkill -f` bridge; código de salida configurable |

## Hito 3 — Documentación

- `start-sddia.md`: proceso atómico con diagrama mermaid, uso, variables, diagnóstico.
- Referencia cruzada desde el encabezado del script.

## Criterios de aceptación

1. `./start-sddia.sh` imprime `Centinelas activos: 4/4` (o ≥ 2 si opcionales no aplican).
2. `curl -sf http://127.0.0.1:8765/` retorna HTTP 200.
3. `SIGTERM` al script apaga centinelas y puente sin procesos huérfanos.
4. Documentación Kaizen en `docs/features/kaizen-start-sddia-ignicion/`.
