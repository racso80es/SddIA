---
feature_name: capsula-binario-fosil-release-stale
created: "2026-08-28"
process: bug-fix
branch_name: fix/capsula-binario-fosil-release-stale
persist_ref: docs/fixes/capsula-binario-fosil-release-stale
execution_id: "13161205-2a2a-4320-9953-554e18a1f7c5"
phases_done:
  - l0-porcelain-unescape
  - l1-phase-fracture-emit
  - l2-source-digest-core
  - l4-resolve-anchor-gated
  - l6-call-sites-trace-purge
---

# Implementation — anclaje de ejecución (parcial)

## L0 — Porcelain (CA1/CA2)

- Nuevo `engine/git_porcelain.rs`: `unescape_git_cquoted_path`, `porcelain_path_from_line`.
- `workspace_init.rs` consume el módulo compartido (elimina `replace('\\', "/")` sobre escapes).
- `phase_capsules.rs` delega en el mismo módulo (sin duplicar 70 líneas).
- Test: `path_in_scope_accepts_pbi_ref_with_unicode_after_unescape`.

## L1 — Fractura en precondición (CA3)

- `emit_workspace_init_fracture` → `materialize_pending_domain_event` con `friction_id: F-DIRTY-WORKTREE`.
- El `Err` de fase se mantiene; el bus recibe el evento.

## L2 — Digest (CA9 parcial)

- `engine/capsule_digest.rs`: digest determinista por crate (`Cargo.toml`, `build.rs`, `src/**`).
- `compute_crate_source_digest_with_workspace` para política B (lockfile completo).

## L4 — Aduana anclada (CA5/CA6, gate apagado por defecto)

- `capsule_paths.rs`: `resolve_capsule_native_anchored`, testigos `.sha256`, lectura de `source_sha256` del genoma.
- Activación: `SDDIA_CAPSULE_ANCHOR=1`. Sin flag → comportamiento legacy (primer perfil existente).
- `capsules.rs`: propaga `capsule-stale-hash` cuando el ancla está activo.

## L6 — Traza factual + purga (CA12/CA13)

- `classify_batch_anchor_friction`: relay / fósil / contrato-entrada / genérico.
- Eliminado bloque `#region agent log` con ruta absoluta de host.

## Pendiente (plan L5–L8)

- Backfill `source_sha256` en genomas vía `entity-manager`.
- `start-sddia.sh`: build multi-cápsula + testigos.
- Rebuild release del parque + `SDDIA_CAPSULE_ANCHOR=1` en runtime.
- Drenaje `.SddIA/dlt/reanchor-queue/`.
- Digest política A (`cargo metadata` + lockfile recortado).
- `validacion.md` + cierre documental.
