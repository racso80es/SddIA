---
feature_name: fracture-provider-sink
created: 2026-10-30
process: refactorization
phase: Verificación
branch: refactor/fracture-provider-sink
global: APTO
tracker_ref: AUT-30
execution_id: 478e05bc-92e4-47e8-a473-fe6a4ac1c4db
pbi_archived: false
checks:
  materialize-enrich-port-deny: APTO
  event-watcher-check: APTO
git_changes:
- SddIA/engine/execute-process/src/engine/materialize_fracture_pbi.rs
- SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs
- SddIA/engine/execute-process/src/engine/tracker_lifecycle_port.rs
- SddIA/engine/execute-process/src/engine/project_binding.rs
- SddIA/engine/execute-process/src/engine/route_domain_core.rs
- SddIA/engine/execute-process/src/engine/domain_authority.rs
- SddIA/engine/execute-process/src/engine/mod.rs
- SddIA/engine/execute-process/src/engine/tracker_operations_gate.rs
- SddIA/daemons/event-watcher/src/main.rs
- SddIA/tools/linear-tracker-adapter/src/main.rs
- SddIA/tools/linear-tracker-adapter/src/lab_state.rs
- SddIA/actions/materialize-fracture-pbi.md
- SddIA/events/domain/fracture-pbi-materialized.md
- SddIA/events/domain/system-fracture-detected.md
- SddIA/events/domain/index.md
- SddIA/core/event-domain-subscriptions.json
- SddIA/core/event-subscriptions.json
- docs/features/fracture-provider-sink/
validated_base_sha: df1ac9b65767e0d6c96cfc22f1ed48d9416f5665
validated_head_sha: d58ff322dfa17aa04f08f222e010231b30678f84
validated_tree_sha: a767c0be49b19a5a373bc535267328e3e5808a2d
diff_digest: sha256:297724825071002491443a0f10f90ba09b04b810cf27f5707a577273d0d9cbf9
---

# Validación — AUT-30

## Evidencia

`SDDIA_EVENT_STORE=filesystem cargo test -p execute-process --lib -- materialize_fracture enrich_fracture resolver_file emit_fracture_on_deny`: 40 passed.

`cargo check -p event-watcher`: Finished.

Criterios vigentes del issue, contrastados con el código:

1. Sin slug, este repo resuelve el índice auto-hospedado linear y el alta no crea markdown. Cubierto en temp por `self_hosted_linear_ignores_orphan_markdown`.
2. Un `.md` huérfano no impide la issue. Una issue abierta con el mismo `fracture_hash` no se duplica.
3. `Unconfigured` no escribe backlog.
4. El deny deja `fracture_deposited` y no traga el error de escritura. El watcher deja rastro si `events.redb` no abre.

## Límite

No se abrió PR en este relevo. `pbi_archived` queda false: el cierre linear es el estado Done del issue, en el mismo PR.