---
feature_name: eda-redb-lock
created: 2026-10-26
process: bug-fix
branch: fix/eda-redb-lock
tracker_ref: AUT-16
global: APTO
pbi_archived: false
checks:
  REDB-CA1: APTO
  REDB-CA2: APTO
  REDB-CA3: APTO
  REDB-CA4: APTO
  REDB-CA5: APTO
  REDB-CA6: APTO
git_changes:
- SddIA/sddia-event-store/
- SddIA/engine/execute-process/src/engine/accept_pr.rs
- SddIA/engine/execute-process/src/engine/tracker_pbi_meta.rs
- SddIA/engine/execute-process/src/engine/handlers/system_vitality.rs
- docs/fixes/eda-redb-lock/
validated_base_sha: d906cc8e843effdb991cecd3190080a2c073d738
validated_head_sha: f9c76b637a66d876cf6ef50b9b61b2d3370dfd88
validated_tree_sha: bf13effd91bccadc3c27848d8eca597a5cf0f70f
diff_digest: sha256:3cf67a5f01b7f3c84062c9ce2d78af3343894a421e207fdabb74360f99bd60e1
---

# Validación — AUT-16

`done_gate: linear`. No hay ficha en `docs/todos/done/`.

| Criterio | Evidencia |
| --- | --- |
| REDB-CA1 | `open_retries_until_holder_releases` |
| REDB-CA2 | `open_returns_locked_when_budget_exhausted` y `open_non_lock_error_does_not_retry` |
| REDB-CA3 | `t_redb_ca3_scan_presented_from_store_only`; `t_adu_ca7_scan_presented_processed_and_dead_letter` |
| REDB-CA4 | `redb_ca4_vitality_and_fracture_go_to_store` |
| REDB-CA5 | `redb_ca5_locked_store_is_green_topology` |
| REDB-CA6 | El predicado que usa `accept-pr` es el de CA3. El `accept-pr` vivo queda en el pull posterior al merge. |

Comando: `CARGO_TARGET_DIR=SddIA/target cargo test -p sddia-event-store --test store open_` y `cargo test -p execute-process --lib redb_ca` más `t_adu_ca7`. Todos en verde.