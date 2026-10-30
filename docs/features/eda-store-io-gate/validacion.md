---
feature_name: eda-store-io-gate
branch: Kaicen_Auditoria_Eventos_261030
global: APTO
pbi_archived: false
checks:
  AC-W1: APTO
  AC-W2: APTO
  AC-W3: APTO
  AC-R1: APTO
  AC-R2: APTO
  AC-F1: APTO
  AC-F2: APTO
  AC-N1: APTO
git_changes:
  - SddIA/sddia-event-store/src/gate.rs
  - SddIA/sddia-event-store/src/lib.rs
  - SddIA/sddia-event-store/tests/gate.rs
  - SddIA/engine/execute-process/src/engine/handlers/daemon_heartbeat.rs
  - SddIA/engine/execute-process/src/engine/capability_di_reactor.rs
  - SddIA/engine/execute-process/src/engine/eda_bus.rs
  - SddIA/engine/execute-process/src/engine/tracker_pbi_meta.rs
  - SddIA/engine/execute-process/src/engine/accept_pr.rs
  - SddIA/engine/execute-process/src/engine/route_fractal_core.rs
  - SddIA/engine/execute-process/src/engine/event_store_query.rs
  - SddIA/engine/execute-process/src/main.rs
  - SddIA/daemons/event-watcher/src/main.rs
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - SddIA/interfaces/kalma2-bridge/Cargo.toml
  - SddIA/infrastructure/adapters/lancedb_thought_repo/src/lib.rs
  - SddIA/infrastructure/adapters/lancedb_thought_repo/Cargo.toml
  - SddIA/tools/sddia-qa/src/iota_ci_smoke.rs
  - SddIA/tools/sddia-qa/Cargo.toml
  - sddia-installer.sh
---

# Validación

| Prueba | Resultado |
|--------|-----------|
| `cargo test -p sddia-event-store --test gate` | 7 passed (incluye publish redb/filesystem) |
| `orphan_lock_redb_does_not_write_pending_json` | passed |
| `orphan_lock_dead_pid_emits_fracture_once` (filesystem) | passed |
| `ac_r6_emit_pending_non_blocking` / `ac_r6_drain_resolved_ecst_ack` | passed |

AC-W4: installer publica con `--publish-ecst`; sin binario y modo `redb` no escribe `.events/`.

Tracker Linear: `pbi_archived` no aplica hasta issue en `done`. Este `global: APTO` cubre la verificación local de la rama, no el cierre del issue.
