---

feature_name: man-40-eda-consumers-logical-seal
created: 2026-11-09
updated: 2026-11-09
process: feature
branch: feat/man-40-eda-consumers-logical-seal
branch_name: feat/man-40-eda-consumers-logical-seal
persist_ref: docs/features/man-40-eda-consumers-logical-seal
pbi_ref: MAN-40
tracker_ref: MAN-40
document_id: PBI-1b004e99
global: APTO
pbi_archived: false
checks:
  CA1_SELLO_LOGICO: APTO
  CA2_TOPOLOGIA: APTO
  CA3_WATCHER_DEFAULT: APTO
  CA4_GATE_MAN40: APTO
  CARGO_DAEMON_TEST: APTO
  CARGO_EXECUTE_PROCESS: PENDIENTE-CI
  TECH_FORMAL_EXECUTE_PROCESS: APTO
git_changes:
- SddIA/sddia-daemon-runtime/src/github_bridge.rs
- SddIA/engine/execute-process/src/engine/eda_bus_topology.rs
- SddIA/engine/execute-process/src/engine/route_domain_core.rs
- SddIA/daemons/event-watcher/src/main.rs
- SddIA/tools/event-bus-audit/src/main.rs
- SddIA/tools/repo-retention-purger/src/plan.rs
- SddIA/tools/repo-retention-purger/src/slice_events.rs
- SddIA/interfaces/kalma2-bridge/src/eda_productive.rs
- SddIA/tools/sddia-qa/src/gate_event_manager_port.rs
- docs/features/man-40-eda-consumers-logical-seal/
validated_base_sha: 9caa03433c8a5b9e207760d313568a1588c5957d
validated_head_sha: 272699136634519860ff06ce0a21d80d853fc982
validated_tree_sha: fedab16dcb2697d50b03020a7c7b5535e547313a
diff_digest: sha256:e6c890c354e56705ff355648ad17e87832d0bc79b20b55499e19fc86df5e2993
execution_id: "953409f1-b3df-42d5-a8d0-69466a366e7d"
---
# Validación — man-40-eda-consumers-logical-seal

## Veredicto

**global: APTO** — delta MAN-40 (sello lógico + veto path Redb en consumidores) con evidencia local. CI GitHub pendiente hasta push (`PENDIENTE-CI` en suite execute-process).

## Criterios

| ID | Resultado | Evidencia |
|----|-----------|-----------|
| CA-1 | APTO | `materialize_stamps_catalog_plan` ok; retorno `.events/domain/…` |
| CA-2 | APTO | Witness/header/route sin `runtime_db_path` como sello |
| CA-3 | APTO | Watcher default sin watch fractal FS; claim `port` |
| CA-4 | APTO | `gate-event-manager-port` → 0 violaciones |
| AC-6 gate unitario | APTO | Paridad MAN-35 gate verde post-regla MAN-40 |

## Gaps honestos

- T4 scripts/installer y `forge-doc-drift` lab: fuera del diff (spec § alcance residual).
- `claim_store_fractal`: sin implementación puerto (familias fractal).