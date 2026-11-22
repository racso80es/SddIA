---
feature_name: man-52-runtime-parity-live-organs
created: "2026-11-22"
process: feature
phases:
  - sddia-runtime-parity-9-organs-and-systemd-pid
  - validacion-runtime-parity-git-diff-trigger
  - sddia-qa-verify-compiled-capsules-freshness
  - sync-live-organs-script-and-runtime-recycle
  - release-sync-evolution-and-dcc
branch_name: feat/man-52-runtime-parity-live-organs
persist_ref: docs/features/man-52-runtime-parity-live-organs
tracker_ref: MAN-52
execution_id: "07a7307e-caf6-4a8c-b263-ad64910f48ea"
---

# Plan — man-52-runtime-parity-live-organs

| Tarea | Alcance |
|-------|---------|
| **T1** | Ampliar `SddIA/sddia-runtime-parity/src/lib.rs` a los 9 órganos (`LIVE_ORGANS`), `resolve_organ_crate_dir`, `merge_touches_live_organs`, `collect_organ_source_files`, inspección `systemd --user` `MainPID`/`/proc/<pid>/exe` y tests unitarios. |
| **T2** | Actualizar `SddIA/engine/execute-process/src/engine/validacion_runtime_parity.rs` y `pull_request_review.rs` para disparo por diff git de crates (`branch_diff_touches_live_organs`) sin tokens Markdown, más tests. |
| **T3** | Actualizar `SddIA/tools/sddia-qa/Cargo.toml` y `verify_compiled_capsules.rs` para validar frescura (testigo `.sha256` + `mtime` frente a fuentes en `LIVE_ORGANS`) y tests. |
| **T4** | Crear `SddIA/scripts/sync-live-organs.sh` y actualizar `runtime_recycle.rs` + `start-sddia.sh` para los 9 órganos y sus unidades `systemd --user`. |
| **T5** | Ejecutar `cargo test` de los crates afectados, `./SddIA/scripts/sync-live-organs.sh`, registrar evolución (`sddia-evolution-register`), `sddia-qa` gates, `validacion.md` APTO, `delivery-close-cycle`, iterar hasta CI verde y fin de ciclo. |
