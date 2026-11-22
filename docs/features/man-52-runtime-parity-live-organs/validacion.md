---
feature_name: man-52-runtime-parity-live-organs
created: "2026-11-22"
process: feature
branch: feat/man-52-runtime-parity-live-organs
persist_ref: docs/features/man-52-runtime-parity-live-organs
tracker_ref: MAN-52
execution_id: "07a7307e-caf6-4a8c-b263-ad64910f48ea"
global: APTO
checks:
  sddia_runtime_parity_tests: APTO
  validacion_runtime_parity_tests: APTO
  verify_compiled_capsules_tests: APTO
  sync_live_organs_release: APTO
  verify_compiled_capsules_live: APTO
  gate_evolution: APTO
git_changes:
  - SddIA/Cargo.lock
  - SddIA/sddia-runtime-parity/src/lib.rs
  - SddIA/engine/execute-process/src/engine/validacion_runtime_parity.rs
  - SddIA/engine/execute-process/src/engine/capsule_seal.rs
  - SddIA/engine/execute-process/src/engine/runtime_recycle.rs
  - SddIA/tools/sddia-qa/Cargo.toml
  - SddIA/tools/sddia-qa/src/verify_compiled_capsules.rs
  - SddIA/scripts/sync-live-organs.sh
  - SddIA/evolution/Evolution_log.md
  - SddIA/evolution/8e47108c-1d40-423c-9985-d80d64d5c1c2.md
---

# Validación — man-52-runtime-parity-live-organs

Veredicto global: **APTO**

## Cobertura de criterios (MAN-52)

| Criterio | Evidencia | Estado |
|----------|-----------|--------|
| **AC-1 (`K-ORGANS`)** | `LIVE_ORGANS` = 9 órganos; test `live_organs_covers_all_nine_organs` + `merge_touch_detects_all_organ_paths` | APTO |
| **AC-2 (`K-PROC`)** | `active_systemd_main_pid` + `check_live_exe_against_witness` en `sddia-runtime-parity`; tests `parse_systemd_show_active_pid_handles_active_and_inactive` y `check_live_exe_detects_deleted_and_drift` | APTO |
| **AC-3 (`K-TRIGGER`)** | `branch_diff_touches_live_organs` en `validacion_runtime_parity.rs`; tests `triggers_on_live_organ_crate_diff_without_markdown_mention` y `does_not_trigger_on_docs_only_diff_even_with_token` | APTO |
| **AC-4 (`K-SYNC`)** | `./SddIA/scripts/sync-live-organs.sh` ejecutado: 9/9 `--release` + 9/9 `.sha256` + `systemctl --user try-restart` + `verify-compiled-capsules --json` `success: true` | APTO |
| **AC-5 (`K-CAP`)** | `check_binary_freshness` en `verify_compiled_capsules.rs` + test `check_binary_freshness_detects_stale_witness_and_newer_source` | APTO |
| **AC-6 (`K-REGRESS`)** | Todos los tests unitarios y gates locales verdes | APTO |
