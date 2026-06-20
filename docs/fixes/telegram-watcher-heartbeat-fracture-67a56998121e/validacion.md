---
feature_name: telegram-watcher-heartbeat-fracture-67a56998121e
created: "2026-06-20"
process: bug-fix
branch: fix/telegram-watcher-heartbeat-fracture-67a56998121e
global: APTO
pbi_archived: true
closed: "2026-06-20"
checks:
  CA1-build: pass
  CA2-once-no-creds: pass
  CA3-heartbeat-keepalive: pass
  CA4-409-resilience: pass
  CA5-audit-sweep: pass
git_changes:
  - SddIA/daemons/telegram-watcher/src/main.rs
  - docs/fixes/telegram-watcher-heartbeat-fracture-67a56998121e/
  - docs/todos/done/[FIX] telegram-watcher — fractura sistémica (67a56998121e).md
---

# Validación — telegram-watcher heartbeat fracture

**Veredicto global: APTO**

## Criterios de aceptación (spec.md)

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| CA1 | Build sin errores | ✅ | `cargo build -p telegram-watcher` exit 0 |
| CA2 | `--once` exit 2 sin credenciales | ✅ | smoke local |
| CA3 | Keepalive en modo continuo | ✅ | `spawn_heartbeat_worker` cada 10s |
| CA4 | Resiliencia 409 | ✅ | `Error::Status(409)` + backoff 5s + `deleteWebhook` bootstrap |
| CA5 | Audit sweep | ✅ | `daemon-heartbeat-audit` sweep OK |

## Causa raíz cerrada

Bloqueo síncrono de `getUpdates` (30s) sin latido intermedio → `missed_cycles ≥ 3` con PID vivo. Corregido con hilo keepalive desacoplado del long-poll.

## Cierre documental

| Paso | Estado |
|------|--------|
| PBI → `docs/todos/done/` | ✅ |
| `pbi_archived: true` | ✅ |
| PR único pre-merge | ⏳ |
