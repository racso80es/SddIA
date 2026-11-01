---
feature_name: kaizen-disk-thermodynamics-retention
created: "2026-11-01"
process: feature
items:
  - L1-crate-jail-lock
  - L2-slice-target
  - L3-slice-events-fs
  - L4-slice-scratch-dist
  - L5-action-purge-repo-retention
branch_name: Fix_Events_Multiple
persist_ref: docs/features/kaizen-disk-thermodynamics-retention
tracker_ref: AUT-49
pbi_uuid: "0d2c4bf9-8837-4842-89f2-5a0f874b203d"
---

# Implementación — kaizen-disk-thermodynamics-retention

| Path | Cambio |
|------|--------|
| `SddIA/tools/repo-retention-purger.md` | Forja tool (`e9f12a4f-…`), capability `repo_retention_purger`. |
| `SddIA/tools/repo-retention-purger/` | Cápsula Rust: jail, lock, slices A–D, plan/apply + digest sha256. |
| `SddIA/tools/index.md` | Fila catálogo. |
| `SddIA/actions/purge-repo-retention.md` | Forja action (`e4b84b41-…`), dos tiempos. |
| `SddIA/actions/index.md` | Fila catálogo. |
| `SddIA/engine/execute-process/src/engine/purge_repo_retention.rs` | Handler nativo → `repo-retention-purger`. |
| `SddIA/engine/execute-process/src/engine/actions.rs` | Dispatch `purge-repo-retention`. |
| `SddIA/engine/execute-process/Cargo.toml` | Dep path `repo-retention-purger`. |
| `docs/features/kaizen-disk-thermodynamics-retention/` | Cascada planificación + L6. |

Sin mutación de `ephemeral-cache-purger` / `VETO_PREFIXES`. Sin escribir en `events.redb`.
