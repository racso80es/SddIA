---
feature_name: aut-111-sigterm-orphan-lock
created: "2026-11-23"
updated: "2026-11-23"
process: feature
branch_name: feat/aut-111-sigterm-orphan-lock
persist_ref: docs/features/aut-111-sigterm-orphan-lock
pbi_ref: AUT-111
tracker_ref: AUT-111
execution_id: "a4111dfb-8fd3-4792-b2a8-b43f89fe369a"
items:
  - runtime-sigterm-shared
  - wire-six-sentinels
  - audit-orphan-grace
---

# Implementación — aut-111-sigterm-orphan-lock

## Touchpoints aplicados

### 1. Runtime compartido (`SddIA/sddia-daemon-runtime`)
- `SddIA/sddia-daemon-runtime/Cargo.toml`: `ctrlc = { version = "3.4", features = ["termination"] }` para interceptar `SIGTERM` y `SIGINT` en Linux/Unix.
- `SddIA/sddia-daemon-runtime/src/termination.rs`: API `install_termination_handlers()`, `request_shutdown()`, `shutdown_requested()`, `clear_shutdown_request()`, `sleep_interruptible(Duration) -> bool` (rodajas de 200ms para despertar antes de `TimeoutStopSec=10`).
- `SddIA/sddia-daemon-runtime/src/lib.rs`: re-export de helpers de terminación y método `DaemonRuntime::graceful_shutdown(&mut self, top: &BusTopology)` (emisión best-effort `status: "shutting_down"` + `shutdown()` con borrado de `.lock`).

### 2. Los 6 binarios en alcance (`D-2`)
- `SddIA/daemons/telegram-watcher/src/main.rs`: `install_termination_handlers()`, bucle y keepalive interrumpibles, `POLL_TIMEOUT = 5` (margen bajo `TimeoutStopSec=10`), `graceful_shutdown(&top)`.
- `SddIA/daemons/event-watcher/src/main.rs`: `install_termination_handlers()`, `invoke_named_probe` con `spawn()` + `try_wait()` interrumpible ante `shutdown_requested()`, bucle y keepalive interrumpibles, `graceful_shutdown(&top)`.
- `SddIA/daemons/github-bridge-watcher/src/main.rs`: `install_termination_handlers()`, bucle y keepalive con `sleep_interruptible`, `graceful_shutdown(&top)`.
- `SddIA/daemons/email-watcher/Cargo.toml` y `src/main.rs`: migrado a `sddia_daemon_runtime::{install_termination_handlers, shutdown_requested, sleep_interruptible}` y `graceful_shutdown(&top)`.
- `SddIA/daemons/iota-publish-relay/Cargo.toml` y `src/main.rs`: migrado a `sddia_daemon_runtime::{install_termination_handlers, shutdown_requested, sleep_interruptible}` y `graceful_shutdown(&top)`.
- `SddIA/interfaces/kalma2-bridge/Cargo.toml` y `src/main.rs`: `install_termination_handlers()`, bucle HTTP con `server.recv_timeout(500ms)` y keepalive interrumpible, `graceful_shutdown(&top)`.

### 3. Gracia y confirmación en `daemon-heartbeat-audit`
- `SddIA/daemons/heartbeat-audit.thresholds.json`: añadidos `orphan_lock_confirm_sweeps: 2` y `orphan_lock_grace_seconds: 45`.
- `SddIA/engine/execute-process/src/engine/handlers/heartbeat_audit_thresholds.rs`: parseo, defaults y tests (`defaults_include_orphan_grace`).
- `SddIA/engine/execute-process/src/engine/handlers/daemon_heartbeat.rs`: `handle_dead_pid_lock` + `clear_orphan_suspect`:
  - `status == "shutting_down"` → `remove_lock` y `classification: "clean_shutdown"` sin fractura.
  - Primer muestreo con `!pid_alive(pid)` → `classification: "orphan_suspect"`, sin emitir `System_Fracture_Detected`.
  - Confirmación al alcanzar `orphan_lock_confirm_sweeps` (≥ 2) o `orphan_lock_grace_seconds` (≥ 45s).
  - Limpieza automática de sospecha si el lock desaparece o el PID revive.
