---
feature_name: man-52-runtime-parity-live-organs
created: "2026-11-22"
process: feature
branch_name: feat/man-52-runtime-parity-live-organs
persist_ref: docs/features/man-52-runtime-parity-live-organs
tracker_ref: MAN-52
execution_id: "07a7307e-caf6-4a8c-b263-ad64910f48ea"
---

# Implementación — man-52-runtime-parity-live-organs

## Cambios realizados

1. **`SddIA/sddia-runtime-parity/src/lib.rs`:**
   - Ampliado `LIVE_ORGANS` a los 9 órganos vivos (`execute-process`, `kalma2-bridge`, `event-watcher`, `github-bridge-watcher`, `email-watcher`, `telegram-watcher`, `iota-publish-relay`, `sddia-workspace-server`, `sddia-qa`).
   - Añadido `SYSTEMD_DAEMON_ORGANS` y verificación de `MainPID` + `/proc/<pid>/exe` en `check_organ` cuando la unidad `sddia-<organ>@<esc>.service` está `active` (`D-R2`).
   - Ampliado `resolve_organ_crate_dir`, `merge_touches_live_organs` y expuesto `collect_organ_source_files` y `escape_systemd_path`.
2. **`SddIA/engine/execute-process/src/engine/validacion_runtime_parity.rs`:**
   - Sustituida la detección por tokens Markdown por `branch_diff_touches_live_organs` (diff git `origin/main...HEAD` / `main...HEAD` / `HEAD~1..HEAD` contra `merge_touches_live_organs`, `D-R3`).
3. **`SddIA/tools/sddia-qa/Cargo.toml` y `verify_compiled_capsules.rs`:**
   - Añadida dependencia `sddia-runtime-parity`.
   - Añadido `check_binary_freshness`: valida testigo `.sha256` (`elf_sha256` + `source_sha256` en `LIVE_ORGANS`) y, si no existe testigo en `LIVE_ORGANS`, comprueba `mtime` frente a fuentes (`H-NEW-07` / `K-CAP`).
4. **`SddIA/scripts/sync-live-organs.sh` y `SddIA/engine/execute-process/src/engine/runtime_recycle.rs`:**
   - Creado script canónico `sync-live-organs.sh` que recompila los 9 órganos en `--release`, sella testigos `.sha256`, reinicia las unidades `systemd --user` activas del workspace y verifica `verify-compiled-capsules --json`.
   - Actualizado `runtime_recycle.rs` para compilar, sellar y reiniciar los 9 órganos / unidades `systemd --user`.
