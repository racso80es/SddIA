---






feature_name: aut-111-sigterm-orphan-lock
created: "2026-11-23"
updated: "2026-11-23"
process: feature
branch_name: feat/aut-111-sigterm-orphan-lock
persist_ref: docs/features/aut-111-sigterm-orphan-lock
pbi_ref: AUT-111
tracker_ref: AUT-111
items_applied:
  - ola0-oracle-red-baseline
  - runtime-sigterm-shared
  - wire-six-sentinels
  - audit-orphan-grace
  - release-sync-and-gates
  - corpus-validacion-cierre
execution_id: "a5f95b61-ab09-4659-8554-403502c4a239"
---
# Ejecución — aut-111-sigterm-orphan-lock

## Baseline Ola 0 (hermético)

- Rama base `main` limpia (`git status --porcelain` vacío previo a `workspace-init`).
- Raíz física limpia: ausencia de `.events`, `orchestration`, `telemetry` y `_lab`.
- `systemctl --user list-units 'sddia-*'`: 6 servicios activos (`email-watcher`, `event-watcher`, `github-bridge-watcher`, `iota-publish-relay`, `kalma2-bridge`, `telegram-watcher`).
- Captura en rojo Ola 0 registrada en [oracle-red.log](file:///home/racso/Proyectos/SddIA/docs/features/aut-111-sigterm-orphan-lock/oracle-red.log) (evidencia de `lock huérfano` en reinicio systemd `2026-11-22 19:12:06` + `E0063` inicial en `daemon_heartbeat.rs`).

## Verificación ejecutada

1. **Tests unitarios (`cargo test`, entorno limpio `env -u SDDIA_EVENT_STORE -u SDDIA_EXECUTE_PROCESS_BIN`):**
   - `sddia-daemon-runtime`: 24 passed, 0 failed (incl. `graceful_shutdown_removes_lock_and_emits_shutting_down`, `request_shutdown_flag_visible`, `sleep_interruptible_returns_true_when_requested`).
   - `email-watcher`: 21 passed, 0 failed.
   - `iota-publish-relay`: 6 passed, 0 failed.
   - `kalma2-bridge`: 51 passed, 0 failed.
   - `telegram-watcher`: 2 passed, 0 failed.
   - `event-watcher` y `github-bridge-watcher`: compilación y harness `0 failed`.
   - `execute-process::daemon_heartbeat`: 16 passed, 0 failed (incl. `orphan_lock_dead_pid_emits_fracture_once`, `orphan_lock_clears_suspect_when_lock_removed`, `orphan_lock_shutting_down_cleans_lock_without_fracture`).
   - `execute-process::heartbeat_audit_thresholds`: 3 passed, 0 failed.
   - `execute-process::eda_event_token_hygiene`: 9 passed, 0 failed.
2. **Clippy (`--no-deps -- -D warnings`):** 0 warnings en los 7 crates de runtime/centinelas.
3. **Paridad build/host (`./SddIA/scripts/sync-live-organs.sh`):** 9/9 órganos `--release` compilados y sellados (`missing_count: 0`, `stale_count: 0`, `success: true`).
4. **Reinicio controlado en vivo (`systemctl --user restart` de los 6 servicios):**
   - Los 6 servicios arrancaron con nuevos PIDs y `journalctl --user` confirmó **0** mensajes `lock huérfano` tras el reinicio completo.
5. **Gates `sddia-qa`:**
   - `verify-tools-index`: OK
   - `verify-process-integrity`: OK
   - `audit-eda-coverage --scan --json`: `indexed_entities: 147`, `orphan_count: 0`
   - `gate-event-manager-port --json`: `ok: true`, `violation_count: 0`
   - `gate-eda-unit-no-real-backend --json`: `ok: true`, `violation_count: 0`
   - `gate-allowlist-ratchet --json`: `ok: true`, `violations: []`
   - `gate-evolution --json --range --sync-base`: `EVOL_OK` (`success: true`)
