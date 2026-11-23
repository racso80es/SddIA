---
feature_name: aut-111-sigterm-orphan-lock
created: "2026-11-23"
updated: "2026-11-23"
process: feature
phase: Verificación
agent: argos
branch: feat/aut-111-sigterm-orphan-lock
persist_ref: docs/features/aut-111-sigterm-orphan-lock
pbi_ref: AUT-111
tracker_provider: linear
tracker_ref: AUT-111
norm_ref: features-documentation-pattern@1.3.3
global: APTO
pbi_archived: false
checks:
  CA-1: APTO
  CA-2: APTO
  CA-3: APTO
  CA-4: APTO
  ROOT_TOPOLOGY_CLEAN: APTO
  SDDIA_QA_GATES: APTO
  SYNC_LIVE_ORGANS: APTO
git_changes:
  - SddIA/Cargo.lock
  - SddIA/daemons/email-watcher/Cargo.toml
  - SddIA/daemons/email-watcher/src/main.rs
  - SddIA/daemons/event-watcher/src/main.rs
  - SddIA/daemons/github-bridge-watcher/src/main.rs
  - SddIA/daemons/heartbeat-audit.thresholds.json
  - SddIA/daemons/iota-publish-relay/Cargo.toml
  - SddIA/daemons/iota-publish-relay/src/main.rs
  - SddIA/daemons/telegram-watcher/src/main.rs
  - SddIA/engine/execute-process/src/engine/handlers/daemon_heartbeat.rs
  - SddIA/engine/execute-process/src/engine/handlers/heartbeat_audit_thresholds.rs
  - SddIA/interfaces/kalma2-bridge/Cargo.toml
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - SddIA/sddia-daemon-runtime/Cargo.toml
  - SddIA/sddia-daemon-runtime/src/lib.rs
  - SddIA/sddia-daemon-runtime/src/termination.rs
  - docs/audits/AUT-111.md
  - docs/features/aut-111-sigterm-orphan-lock/
---

# Validación — aut-111-sigterm-orphan-lock

## Veredicto: `APTO`

| Criterio | Estado | Evidencia |
| -- | -- | -- |
| **CA-1** (Shutdown SIGTERM + borrado `.lock`) | `APTO` | `systemctl --user restart` sobre las 6 unidades tras `sync-live-organs.sh`: `0` mensajes `lock huérfano` en `journalctl --user`. |
| **CA-2** (Gracia/confirmación en `daemon-heartbeat-audit`) | `APTO` | `handle_dead_pid_lock` requiere `orphan_lock_confirm_sweeps: 2` o `orphan_lock_grace_seconds: 45` y depura locks `shutting_down` sin emitir fractura. |
| **CA-3** (Tests unitarios/integración) | `APTO` | `sddia-daemon-runtime` (24 passed), `email-watcher` (21), `iota-publish-relay` (6), `kalma2-bridge` (51), `telegram-watcher` (2), `daemon_heartbeat` (16), `heartbeat_audit_thresholds` (3), `eda_event_token_hygiene` (9). `clippy -D warnings` 0 warnings. |
| **CA-4** (Corpus + cierre Argos) | `APTO` | `F-ARGOS-ORPHAN-LOCK` ya catalogado en corpus; `validacion.md` sellado `APTO`; `AUT-111` en ciclo de cierre hacia `state_map.done` vía `accept-pr`. |
