---
feature_name: kaizen-disk-thermodynamics-syslog
created: "2026-09-13"
process: feature
items:
  - silence-hot-path
  - unit-tests
  - systemd-rate-limit
  - smoke-factory-grep
branch_name: feat/kaizen-disk-thermodynamics-syslog
persist_ref: docs/features/kaizen-disk-thermodynamics-syslog
execution_id: "e3bbb4f8-b27c-4833-9750-d855b504da8b"
document_id: PBI-KAIZEN-DISK-THERMODYNAMICS-SYSLOG
---

# Implementación — kaizen-disk-thermodynamics-syslog

| Path | Cambio |
|------|--------|
| `SddIA/daemons/event-watcher/src/main.rs` | Skip hot path: `continue` sin I/O. `watcher_skip_emits_hot_path_trace` → `false`. Tests `#[cfg(test)]` (6). |
| `SddIA/templates/systemd/sddia-daemon@.service.template` | `LogRateLimitIntervalSec=30s`, `LogRateLimitBurst=500` |
| `.SddIA/systemd/sddia-event-watcher@.service` | Mismas claves (unidad instancia versionada) |
| `SddIA/scripts/qa/test-instance-root-resolver.sh` | grep de ambas claves en la fábrica |

`Cargo.toml` del watcher intacto. Purger / `purge_sandbox_cache.rs` no tocados. `log_route_outcome` intacto.
