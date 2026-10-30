---
feature_name: eda-store-io-gate
artifact: doc-drift
created: "2026-10-30"
items_applied:
  - docs P0/P3 alineados
  - entity-manager P1 acciones + event-watcher
  - entity-manager P2 codex procesos
  - events/index.md runtime bullet (índice cumulo; sin creator de índice)
  - entity_manager.rs action_inputs/outputs/orchestration_logic en update
---

# Ejecución — deriva documental

## Scripts lab que escriben `.events/pending`

| Script | Nota |
|--------|------|
| `SddIA/scripts/qa/test-merge-thermo-02-net-prune.sh` | Lab; prefijar `SDDIA_EVENT_STORE=filesystem` al invocar |
| `SddIA/scripts/tools/kalma2-sse-fracture-smoke.sh` | Lee pending; no emisor principal del gate |

`hook_common.sh` solo resuelve rutas por defecto; no es emisor ECST.

## P4 — Matriz generada

```bash
./docs/features/eda-store-io-gate/gen-test-matrix-table.sh
cd SddIA && CARGO_TARGET_DIR=target cargo test -p execute-process --test eda_store_io_matrix_pending tm8_presented -- --test-threads=1
```

Salida: bloque `MATRIX-COVERAGE` en `spec-test-matrix.md`. Test `tm8_presented_has_pending_subscriber_rows_after_stamp` (TM-8 no exhaustivo).
