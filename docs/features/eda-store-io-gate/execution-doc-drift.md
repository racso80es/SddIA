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

## Comandos forja

Una invocación por entidad; acuse JSON = éxito (DA-5).
