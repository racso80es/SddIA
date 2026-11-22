---
feature_name: man-52-runtime-parity-live-organs
created: "2026-11-22"
process: feature
branch_name: feat/man-52-runtime-parity-live-organs
persist_ref: docs/features/man-52-runtime-parity-live-organs
tracker_ref: MAN-52
execution_id: "07a7307e-caf6-4a8c-b263-ad64910f48ea"
items_applied:
  - T1
  - T2
  - T3
  - T4
  - T5
---

# Ejecución — man-52-runtime-parity-live-organs

## Tareas aplicadas

- **T1 (`sddia-runtime-parity`):** Ampliado `LIVE_ORGANS` a los 9 órganos vivos (`execute-process`, `kalma2-bridge`, `event-watcher`, `github-bridge-watcher`, `email-watcher`, `telegram-watcher`, `iota-publish-relay`, `sddia-workspace-server`, `sddia-qa`). Añadida inspección `MainPID` / `/proc/<pid>/exe` para unidades `systemd --user` activas (`D-R2`), `resolve_organ_crate_dir`, `merge_touches_live_organs` y `collect_organ_source_files`.
- **T2 (`validacion_runtime_parity.rs`):** Activación del gate por diff git de crates de órganos vivos (`branch_diff_touches_live_organs`), eliminando la dependencia de tokens Markdown (`D-R3`).
- **T3 (`verify_compiled_capsules.rs`):** Comprobación de frescura (`check_binary_freshness`) mediante testigo `.sha256` (`elf_sha256` + `source_sha256`) y `mtime` frente a fuentes en `LIVE_ORGANS` (`H-NEW-07` / `K-CAP`).
- **T4 (`sync-live-organs.sh` / `runtime_recycle.rs` / `capsule_seal.rs`):** Script canónico `./SddIA/scripts/sync-live-organs.sh`, `inventory_indexed_capsules` alineado a `LIVE_ORGANS`, y `runtime_recycle.rs` ampliado a los 9 órganos y unidades `systemd --user`.
- **T5 (Verificación y evolución):**
  - `cargo test -p sddia-runtime-parity`: 4 passed, 0 failed.
  - `cargo test -p execute-process --lib validacion_runtime_parity`: 2 passed, 0 failed.
  - `cargo test -p sddia-qa verify_compiled_capsules`: 2 passed, 0 failed.
  - `./SddIA/scripts/sync-live-organs.sh`: 9/9 órganos compilados en `--release`, 9/9 testigos `.sha256` sellados (`count_ok: 9, count_err: 0`), unidades `systemd --user` activas reiniciadas, `sddia-qa verify-compiled-capsules --json` → `success: true`, `missing_count: 0`, `stale_count: 0`.
