---
feature_name: aut-144-entity-uuid-migrate-forges
created: "2026-11-28"
process: bug-fix
branch_name: fix/aut-144-entity-uuid-migrate-forges
persist_ref: docs/fixes/aut-144-entity-uuid-migrate-forges
tracker_ref: AUT-144
execution_id: "d0b4e016-3133-4fdd-ba51-38cd393156cc"
---

# Ejecución — AUT-144

1. `SDDIA_ALLOW_STALE_RUNTIME=0 ./sddia-run.sh --process bug-fix` (`execution_id` `d0b4e016-3133-4fdd-ba51-38cd393156cc`) — workspace-init + Dedalo OK; Tekton subagent timeout 600s (`PROVIDER_TIMEOUT`).
2. Continuación IDE (Tekton): call sites migrate en cinco forjas + tests; `cargo build -p execute-process --release`.
3. `cargo test -p execute-process --lib migrate_entity_uuid suite_forge_entity_uuid_migrate sync_index_entity_uuid` — verde.
4. Reanudación `bug-fix` pendiente (fases Verificación / DCC).