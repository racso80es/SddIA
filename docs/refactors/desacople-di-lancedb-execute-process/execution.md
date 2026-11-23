---

feature_name: desacople-di-lancedb-execute-process
created: "2026-11-23"
process: refactorization
branch_name: refactor/desacople-di-lancedb-execute-process
persist_ref: docs/refactors/desacople-di-lancedb-execute-process
tracker_ref: AUT-119
items_applied:
  - "T1: Purga Redb en lancedb_thought_repo y allowlist"
  - "T2: Creación de satélite lancedb-memory-adapter + 4 tests"
  - "T3: Desacople de execute-process + tests memory_evolution_ingest, user_preference, aiua_stimulus"
  - "T4: Retirada thought-graph-access + SSOT (cumulo.paths, bindings, taxonomy, eda-coverage, tools/index)"
  - "T5: default-members en SddIA/Cargo.toml + CI/scripts"
  - "T6: Registro SddIA/evolution/bab6e051-04bd-4d83-b73e-267189430e37.md + gates sddia-qa"
execution_id: "9d87f216-2e4f-4ade-8c8d-820e265f62b0"
---
# Registro de Ejecución — desacople-di-lancedb-execute-process (`AUT-119`)

## Baseline Ola 0
- `systemctl --user list-units 'sddia-*'`: 6 unidades activas (`email-watcher`, `event-watcher`, `github-bridge-watcher`, `iota-publish-relay`, `kalma2-bridge`, `telegram-watcher`).
- Raíz del repo: ausentes `.events/`, `orchestration/`, `telemetry/`, `pending/`, `domain/`, `dead-letter/`, `progress/`, `_lab/`.
- Oráculo rojo capturado en `oracle-red.log`: `cargo tree -p execute-process` mostraba `sddia-infrastructure-lancedb-evolution` → `lancedb v0.37.1` → `arrow v58.4.0` / `tokio`; `default-members` ausente en `SddIA/Cargo.toml`.

## Verificación ejecutada
- `cargo tree --manifest-path SddIA/Cargo.toml -p execute-process -e normal | grep -E "lancedb|lance|arrow|datafusion|tokio"` → **0 coincidencias** (AC-1).
- `RUSTFLAGS="-Dwarnings" cargo test -p execute-process --test memory_evolution_ingest` → **3 passed, 0 failed**.
- `RUSTFLAGS="-Dwarnings" cargo test -p execute-process --lib user_preference` → **2 passed, 0 failed**.
- `RUSTFLAGS="-Dwarnings" cargo test -p execute-process --lib aiua_stimulus` → **11 passed, 0 failed** (con binario real `lancedb-memory-adapter`).
- `RUSTFLAGS="-Dwarnings" cargo check -p user-preference-store --target wasm32-wasip1` → **OK (4.81s)**.
- `RUSTFLAGS="-Dwarnings" cargo test -p sddia-infrastructure-lancedb-thought` → **5 passed, 0 failed**.
- `RUSTFLAGS="-Dwarnings" cargo test -p lancedb-memory-adapter` → **4 passed, 0 failed**.
- `sddia-qa verify-tools-index` → **OK**.
- `sddia-qa verify-process-integrity` → **OK**.
- `sddia-qa gate-eda-unit-no-real-backend --json` → `ok: true, violation_count: 0`.
- `sddia-qa gate-no-env-bypass --json` → `ok: true, violation_count: 0`.
- `sddia-qa gate-event-manager-port --json` → `ok: true, violation_count: 0`.
- `sddia-qa audit-eda-coverage --scan --json` → `indexed_entities: 147, orphan_count: 0`.
- `sddia-qa gate-evolution --json --range --sync-base` → `EVOL_OK`.
