---
feature_name: telegram-watcher-heartbeat-fracture-67a56998121e
created: "2026-06-20"
process: bug-fix
version_implementation: "1.0.0"
---

# Implementación — Heartbeat durante long-poll Telegram

## Cambios

| Archivo | Cambio |
|---------|--------|
| `SddIA/daemons/telegram-watcher/src/main.rs` | Keepalive heartbeat, POLL_TIMEOUT=25, deleteWebhook, manejo 409 |

## Detalle técnico

1. **Hilo keepalive:** `spawn_heartbeat_worker` con `Arc<Mutex<DaemonRuntime>>` invoca `tick()` cada 10s en modo continuo.
2. **Bootstrap Telegram:** `deleteWebhook` antes del bucle para evitar conflicto webhook vs long-poll.
3. **409 Conflict:** detección explícita vía `ureq::Error::Status(409, _)` + backoff 5s.
4. **Poll timeout:** 25s (margen bajo intervalo heartbeat 30s).

## Sin cambios

- `telegram-watcher.md` (contrato daemon)
- Launchers `.sh` / `_run_daemon.sh`
- Proceso `daemon-heartbeat-audit`
