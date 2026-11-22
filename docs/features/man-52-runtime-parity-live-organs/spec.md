---
feature_name: man-52-runtime-parity-live-organs
created: "2026-11-22"
process: feature
base: main
scope: core
tracker_ref: MAN-52
execution_id: "07a7307e-caf6-4a8c-b263-ad64910f48ea"
---

# Especificación técnica — man-52-runtime-parity-live-organs

## 1. `SddIA/sddia-runtime-parity/src/lib.rs`

- **`LIVE_ORGANS`:**
  ```rust
  pub const LIVE_ORGANS: &[&str] = &[
      "execute-process",
      "kalma2-bridge",
      "event-watcher",
      "github-bridge-watcher",
      "email-watcher",
      "telegram-watcher",
      "iota-publish-relay",
      "sddia-workspace-server",
      "sddia-qa",
  ];
  ```
- **`resolve_organ_crate_dir(repo, name)`:**
  - `execute-process` → `SddIA/engine/execute-process`
  - `kalma2-bridge` → `SddIA/interfaces/kalma2-bridge` (fallback `SddIA/engine/kalma2-bridge`)
  - `event-watcher`, `github-bridge-watcher`, `email-watcher`, `telegram-watcher`, `iota-publish-relay` → `SddIA/daemons/<name>`
  - `sddia-workspace-server`, `sddia-qa` → `SddIA/tools/<name>`
- **Paridad de proceso `systemd --user` (`MainPID`/`exe`):**
  - Para órganos con servicio `systemd --user` (`kalma2-bridge`, `event-watcher`, `github-bridge-watcher`, `email-watcher`, `telegram-watcher`, `iota-publish-relay`, `sddia-workspace-server`):
  - Resolver instancia escapada del `repo` (`systemd-escape -p <repo>`, con fallback determinista).
  - Consultar `systemctl --user show sddia-<organ>@<esc>.service -p ActiveState -p MainPID --value`.
  - Si `ActiveState == "active"` y `MainPID > 0`, inspeccionar `/proc/<pid>/exe`:
    - Si `exe` termina en ` (deleted)` o no existe → `{organ}:systemd_live_deleted:pid={pid}`.
    - Si `sha256_file_hex(&exe) != witness.elf_sha256` → `{organ}:systemd_live_drift:pid={pid}`.
  - Si la unidad no existe o está inactiva → sin error de proceso (D-R2).
- **`merge_touches_live_organs(changed_paths)`:**
  - Detecta cambios bajo cualquiera de los 9 directorios de crate, `SddIA/sddia-`, `SddIA/ecosystem-health/`, `SddIA/core/memory/`, `SddIA/user-preference-core/`, `SddIA/Cargo.toml`, `SddIA/Cargo.lock`.
- **Helper público `collect_organ_source_files(repo, name)`:** reutilizable por `sddia-qa` para comprobación de frescura.

## 2. `SddIA/engine/execute-process/src/engine/validacion_runtime_parity.rs` y `pull_request_review.rs`

- Nueva función `branch_diff_touches_live_organs(repo: &Path) -> bool` que obtiene los archivos modificados de la rama (`git diff --name-only origin/main...HEAD`, fallback `main...HEAD`, fallback `HEAD~1..HEAD`) y delega en `sddia_runtime_parity::merge_touches_live_organs(&paths)`.
- `persist_requires_runtime_parity_smoke(repo, _persist_ref)` (o `requires_runtime_parity_gate(repo)`) pasa a evaluarse por `branch_diff_touches_live_organs(repo)` (D-R3), eliminando la dependencia de tokens Markdown.
- `pull_request_review.rs` invoca el gate cuando `branch_diff_touches_live_organs(repo)` es `true`.

## 3. `SddIA/tools/sddia-qa/src/verify_compiled_capsules.rs`

- Añadir dependencia `sddia-runtime-parity` en `SddIA/tools/sddia-qa/Cargo.toml`.
- Al localizar cada binario en `found`:
  - Si existe testigo `<bin>.sha256`: validar `witness.source_sha256 == compute_organ_source_digest(repo, name)` y `witness.elf_sha256 == sha256_file_hex(&bin)`. Si diverge → añadir a `stale[]` y marcar `ok = false` (exit `1`).
  - Si NO existe testigo `<bin>.sha256` y `name ∈ LIVE_ORGANS`: comprobar si algún fichero fuente de `collect_organ_source_files(repo, name)` tiene `mtime > bin.mtime`. Si es más reciente → añadir a `stale[]` (`source_newer_than_binary`) y marcar `ok = false` (exit `1`).
  - Mantener compatibilidad con CI (`sddia-index-qa.yml`), donde `cargo build` acaba de compilar `target/debug/*` sin `.sha256`.

## 4. Ritual canónico pre-cierre (`SddIA/scripts/sync-live-organs.sh` y `runtime_recycle.rs`)

- Nuevo script `SddIA/scripts/sync-live-organs.sh`:
  1. Compila en `--release` los 9 órganos de `LIVE_ORGANS`.
  2. Sella testigos `.sha256` vía `execute-process --seal-capsules` (`write_genome: false`, `write_witness: true`).
  3. Reinicia (`systemctl --user try-restart`) las unidades `sddia-<organ>@<esc>.service` activas del workspace.
  4. Verifica paridad final ejecutando `sddia-qa` / `assess_runtime_binary_parity`.
- Actualizar `SddIA/engine/execute-process/src/engine/runtime_recycle.rs` para compilar, sellar y reiniciar los 9 órganos / unidades `systemd`.
- Actualizar `start-sddia.sh` si procede para incluir los órganos restantes en `--seal-capsules`.
