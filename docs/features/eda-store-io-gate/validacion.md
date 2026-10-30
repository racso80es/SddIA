---
feature_name: eda-store-io-gate
branch: Kaicen_Auditoria_Eventos_261030
tracker_ref: AUT-56
kaizen_ref: docs/audits/kaizen-eda-store-io-gate-261030.md
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
  TM-1: APTO
  TM-2: APTO
  TM-3: APTO
  TM-4: APTO
  TM-5: APTO
  TM-6: APTO
  TM-7: APTO
  TM-8: parcial
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
- SddIA/engine/execute-process/tests/common/mod.rs
- SddIA/engine/execute-process/tests/eda_store_io_matrix_pending.rs
- SddIA/engine/execute-process/tests/eda_store_io_matrix_fractal.rs
- docs/features/eda-store-io-gate/plan-test-matrix.md
- docs/features/eda-store-io-gate/execution-test-matrix.md
- docs/audits/eda-doc-drift-eda-store-io-gate-261030.md
- docs/features/eda-store-io-gate/plan-doc-drift-261030.md
- docs/features/eda-store-io-gate/execution-doc-drift.md
- docs/features/eda-store-io-gate/gen-test-matrix-table.sh
- docs/features/eda-store-io-gate/_lab/gen_test_matrix_table.py
- SddIA/engine/execute-process/src/engine/entity_manager.rs
- SddIA/engine/execute-process/tests/common/mod.rs
- SddIA/engine/execute-process/tests/eda_store_io_matrix_pending.rs
- SddIA/engine/execute-process/tests/eda_store_io_matrix_fractal.rs
- docs/features/eda-store-io-gate/spec-test-matrix.md
- SddIA/actions/emit-pr-presented-event.md
- SddIA/actions/emit-pr-audited-event.md
- SddIA/actions/emit-pr-resealed-event.md
- SddIA/actions/emit-pr-merged-event.md
- SddIA/actions/index.md
- SddIA/daemons/event-watcher.md
- SddIA/events/index.md
- .gitignore
- docs/audits/kaizen-eda-store-io-gate-261030.md
- docs/audits/kaizen-auditoria-eventos-261030.md
doc_drift:
  P0: APTO
  P1: APTO
  P2: APTO
  P3: APTO
  P4: APTO
validated_base_sha: d906cc8e843effdb991cecd3190080a2c073d738
validated_head_sha: f9c76b637a66d876cf6ef50b9b61b2d3370dfd88
validated_tree_sha: bf13effd91bccadc3c27848d8eca597a5cf0f70f
diff_digest: sha256:3cf67a5f01b7f3c84062c9ce2d78af3343894a421e207fdabb74360f99bd60e1
---

# Validación

## Deriva documental (plan-doc-drift)

| Fase | Estado |
|------|--------|
| P0 | APTO |
| P1 | APTO (forja acciones + event-watcher) |
| P2 | APTO (codex + events/index) |
| P3 | APTO |
| P4 | APTO (gen-test-matrix-table + TM-8 smoke) |

| Prueba | Resultado |
|--------|-----------|
| `cargo test -p execute-process --test eda_store_io_matrix_pending` | 3 passed |
| `cargo test -p execute-process --test eda_store_io_matrix_fractal` | 5 passed |
| `cargo test -p sddia-event-store --test gate` | 7 passed (incluye publish redb/filesystem) |
| `cargo test -p execute-process --test eda_store_io_matrix_pending tm8_presented` | 1 passed (TM-8 fan-out) |
| `orphan_lock_dead_pid_emits_fracture_once` (filesystem) | passed |
| `ac_r6_emit_pending_non_blocking` / `ac_r6_drain_resolved_ecst_ack` | passed |

TM-8: `tm8_presented_has_pending_subscriber_rows_after_stamp` + tabla § Cobertura generada (`gen-test-matrix-table.sh`); fan-out exhaustivo por tipo sigue **parcial**.

AC-W4: installer publica con `--publish-ecst`; sin binario y modo `redb` no escribe `.events/`.

Tracker Linear: issue kaizen roll-up **[AUT-56](https://linear.app/oscar-perez/issue/AUT-56/kaizen-gate-io-bus-eda-y-cierre-rama-261030)** (hijo **AUT-20**). `pbi_archived` no aplica hasta issue en `done`. Este `global: APTO` cubre la verificación local de la rama, no el cierre del issue.