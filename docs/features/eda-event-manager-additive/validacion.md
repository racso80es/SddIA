---
feature_name: eda-event-manager-additive
created: 2026-11-06
process: feature
phase: Verificación
branch: feat/eda-event-manager-additive
branch_name: feat/eda-event-manager-additive
persist_ref: docs/features/eda-event-manager-additive
pbi_ref: MAN-32
tracker_ref: MAN-32
execution_id: 5513eab2-a948-4e5b-9476-3ff561266b95
global: APTO
pbi_archived: false
checks:
  AC-1-hot-path-clean: APTO
  AC-2-genoma-contract: APTO
  AC-3-suite-tripartite: APTO
  AC-4-external-capsule-e2e: APTO
  cargo-test-event-manager-crates: APTO
git_changes:
- SddIA/Cargo.toml
- SddIA/sddia-core/
- SddIA/library/norms/capability-contracts/eda.event_manager.v1.schema.json
- SddIA/library/norms/capability-taxonomy.md
- SddIA/library/norms/index.md
- SddIA/core/capability-bindings.md
- SddIA/infrastructure/adapters/
- docs/features/eda-event-manager-additive/
validated_base_sha: 54dcdfc746a518a5ada4cc1c6d261b25d1b889b8
validated_head_sha: 1ed4be2418bbdec6f4fdde7eda01b7daabc1bfee
validated_tree_sha: d18685dde77b8082ae3f0cdad7118eb22529d76a
diff_digest: sha256:fac7f60281ec83390f6bf2ab9b8098b2997e36c9f4b10ecb4919bb07853e5a4f
---

# Validación — MAN-32

## Criterios

- **AC-1:** `git diff` sin rutas bajo `SddIA/engine/execute-process/`, daemons event-watcher, `SddIA/sddia-event-store/`.
- **AC-2:** Schema + taxonomía + binding multi-rol presentes y coherentes con `spec.md`.
- **AC-3:** `cargo test -p sddia-core -p sddia-infrastructure-redb-event -p sddia-infrastructure-fs-event` → ok (suite compartida en los tres).
- **AC-4:** `cargo test -p sddia-infrastructure-external-capsule-event` → ok (incl. E2E dummy + `CARGO_BIN_EXE_eda-event-manager-capsule-dummy`).

## Evidencia local

```bash
cd SddIA && cargo test -p sddia-core -p sddia-infrastructure-redb-event -p sddia-infrastructure-fs-event -p sddia-infrastructure-external-capsule-event
```

Comando ejecutado con **ok** en el entorno de cierre (2026-11-06).

## Nota

`pbi_archived: false` — tracker Linear (`done_gate: linear`); sin archivo en `docs/todos/done/`.