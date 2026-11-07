---
feature_name: eda-event-manager-redb-cutover
created: 2026-11-07
process: refactorization
phase: Verificación
branch: refactor/eda-event-manager-redb-cutover
branch_name: refactor/eda-event-manager-redb-cutover
persist_ref: docs/features/eda-event-manager-redb-cutover
pbi_ref: MAN-33
tracker_ref: MAN-33
execution_id: a3ab115d-9ebb-4679-8749-d3db2a216ea0
global: APTO
pbi_archived: false
checks:
  AC-build-ci-crates: APTO
  AC-workspace-build: APTO
  AC-event-manager-unit: APTO
  AC-qa-aduana-smokes: APTO
  AC-T5-inventory-closure: PENDIENTE
git_changes:
- SddIA/sddia-event-manager-host/
- SddIA/sddia-core/src/ports/event_manager/
- SddIA/engine/execute-process/
- SddIA/daemons/event-watcher/
- SddIA/tools/sddia-qa/
- docs/features/eda-event-manager-redb-cutover/
validated_base_sha: b5de29c291ed27ed59ce7a8d3513d5291c50eea5
---

# Validación — MAN-33 (corte intermedio)

Cutover del camino caliente y smoke QA al puerto Redb vía host. **T5** (tools restantes y retiro de `sddia-event-store`) sigue abierto; ver `implementation.md`.

## Evidencia

Build/tests ejecutados en rama con `TMPDIR` bajo el repo y target `SddIA/target` (2026-11-07).

## Linear

`pbi_archived: false` — cierre de issue al merge con `state_map.done`.
