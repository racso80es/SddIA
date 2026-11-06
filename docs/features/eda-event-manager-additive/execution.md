---
feature_name: eda-event-manager-additive
created: "2026-11-06"
process: feature
branch_name: feat/eda-event-manager-additive
persist_ref: docs/features/eda-event-manager-additive
pbi_ref: MAN-32
tracker_ref: MAN-32
execution_id: "5513eab2-a948-4e5b-9476-3ff561266b95"
document_id: PBI-EDA-001
---

# Ejecución — MAN-32

## Tekton (post planificación)

Implementación manual en workspace tras timeout de agent-runtime en fase «Ejecución» (primera pasada).

- Schema: `SddIA/library/norms/capability-contracts/eda.event_manager.v1.schema.json`
- Taxonomía/bindings: `SddIA/library/norms/capability-taxonomy.md` v1.0.10; `SddIA/core/capability-bindings.md` v1.8.0
- Shared kernel: `SddIA/sddia-core/src/ports/event_manager/` (`EventManagerPort`, `version_gate`, mock, contract suite, factory)
- Crates workspace: `redb_event_adapter`, `fs_event_adapter`, `external_capsule_event_adapter` (+ bin dummy + test `e2e_dummy.rs`)
- Fichas: `redb-event-adapter.md`, `fs-event-adapter.md`, `external-capsule-event-adapter.md`; `SddIA/infrastructure/adapters/index.md`

## Verificación local

```bash
cd SddIA && cargo test -p sddia-core -p sddia-infrastructure-redb-event -p sddia-infrastructure-fs-event -p sddia-infrastructure-external-capsule-event
```

AC-1: sin diff en `SddIA/engine/execute-process/`, `SddIA/daemons/` (event-watcher), `SddIA/sddia-event-store/` (comprobado con `git diff --name-only`).
