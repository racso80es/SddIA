---
feature_name: kaizen-start-sddia-ignicion
created: "2026-06-19"
process: feature
branch_name: feat/kaizen-start-sddia-ignicion
uuid: aad51b93-198b-4cf7-b6a8-195d7f988fb5
---

# Implementación — Kaizen start-sddia ignición ecosistema

## Touchpoints

| # | Artefacto | Cambio |
|---|-----------|--------|
| 1 | `start-sddia.sh` | Rutas `SddIA/scripts/daemons/`; funciones `_start_daemon`, `_wait_http`; `cleanup` con `pkill`; health check Kalma2 |
| 2 | `start-sddia.md` | Proceso documentado (`id: start-sddia`, `type: process`, v1.0.0) |
| 3 | `docs/features/kaizen-start-sddia-ignicion/` | Objetivos, spec, implementation, execution |
| 4 | PBI | `docs/todos/pending/[Kaizen] start-sddia — ignición ecosistema Centinelas y Kalma2.md` |

## Detalle técnico — `start-sddia.sh`

### Secuencia

1. `REPO_ROOT` + `cd`.
2. Bucle centinelas obligatorios → opcionales vía `_start_daemon`.
3. Gate: mínimo 2 centinelas (watcher + sweeper).
4. `python3 .SddIA/client/sddia-client-bridge.py &`.
5. `_wait_http` hasta 30 × 0,5 s.
6. `wait` bloqueante; `trap cleanup SIGINT SIGTERM`.

### Separación genoma / instancia

```
Genoma:  SddIA/scripts/daemons/*.sh  → _run_daemon.sh → binario Rust
Instancia: .SddIA/client/sddia-client-bridge.py
           .SddIA/daemons/{status,state,logs}
```

## Validación ejecutada (2026-06-19)

| Check | Resultado |
|-------|-----------|
| event-watcher | ACTIVO |
| event-sweeper | ACTIVO |
| telegram-watcher | ACTIVO |
| github-bridge-watcher | ACTIVO |
| Kalma2 GET / | HTTP 200 |
| SIGTERM cleanup | Apagado centinelas + bridge |

## Observaciones post-validación

- Stdout de centinelas muy verboso por backlog EDA pendiente (`execute-process.py` ausente tras migración Rust).
- No incluido en este diff: `sddia-run.sh` (wrapper orchestrator) permanece fuera del alcance Kaizen.
