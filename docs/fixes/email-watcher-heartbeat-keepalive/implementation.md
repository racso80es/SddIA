---
feature_name: email-watcher-heartbeat-keepalive
created: "2026-08-31"
process: bug-fix
version_implementation: "1.0.0"
items:
  - spawn_heartbeat_worker
  - run_loop_arc_mutex
  - once_no_keepalive
  - poll_once_stimulus_callback
---

# Implementación — Keepalive heartbeat `email-watcher`

## Cambios

| Archivo | Cambio |
|---------|--------|
| `SddIA/daemons/email-watcher/src/main.rs` | Hilo keepalive 10 s; `poll_once` sin retener mutex durante I/O IMAP; `--once` sin spawn |

## Detalle técnico

1. **Hilo keepalive:** `spawn_heartbeat_worker` con `Arc<Mutex<DaemonRuntime>>` invoca `tick()` cada 10 s en modo continuo. Presupuesto de fallos = 5 (pánico termodinámico). Paridad telegram/event/github/sweeper.
2. **Lock vs IMAP:** `poll_once` ya no toma `&mut DaemonRuntime`. Callback `on_stimulus` solo para `note_stimulus()`. El mutex no se sostiene durante connect/login/search/fetch. CA3: el worker puede latir aunque IMAP bloquee ≥90 s.
3. **Wait de poll:** `drop` implícito del guard antes del sleep de `SDDIA_EMAIL_POLL_SECONDS`. El hilo principal no hace `tick` en el wait.
4. **`--once`:** bootstrap → `poll_once` → `tick` → `shutdown`. Sin `thread::spawn`. Envelope JSON-IO intacto.

## Sin cambios

- `email-watcher.md` (contrato daemon; intervalo 30 s)
- Umbrales Argos (`missed_cycles_threshold`, `suspend_skew_seconds`)
- Timeout IMAP / `uid_search("ALL")`
- `iota-publish-relay`, emisores `daemon-heartbeat.md`
