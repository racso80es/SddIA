---
feature_name: refine-item-tracker-ref-remote
created: "2026-10-29"
process: refactorization
branch_name: refactor/refine-item-tracker-ref-remote
persist_ref: docs/features/refine-item-tracker-ref-remote
pbi_ref: AUT-35
execution_id: "3876e61f-1e04-4140-abbd-702c0ebb416f"
---

# Implementación

| Archivo | Cambio |
|---------|--------|
| `SddIA/engine/execute-process/src/engine/handlers/refine_item/run.rs` | Resolución remota de `tracker_ref` desde `snap.identifier`; conflicto si FM diverge; eliminada alta HU remota |
