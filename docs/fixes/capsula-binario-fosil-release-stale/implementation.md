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
  - l3-witness-write
  - l4-resolve-anchor-gated
  - l5-genome-em
  - l6-call-sites-trace-purge
  - l7-rebuild-then-hard-gate
  - l8-drain-dlt-start
---

# Implementation — anclaje de ejecución

## L0 — Porcelain (CA1/CA2)

- Nuevo `engine/git_porcelain.rs`: `unescape_git_cquoted_path`, `porcelain_path_from_line`.
- `workspace_init.rs` consume el módulo compartido (elimina `replace('\\', "/")` sobre escapes).
- `phase_capsules.rs` delega en el mismo módulo (sin duplicar 70 líneas).
- Test: `path_in_scope_accepts_pbi_ref_with_unicode_after_unescape`.

## L1 — Fractura en precondición (CA3)

- `emit_workspace_init_fracture` → `materialize_pending_domain_event` con `friction_id: F-DIRTY-WORKTREE`.
- El `Err` de fase se mantiene; el bus recibe el evento.

## L2 — Digest (CA9)

- `engine/capsule_digest.rs`: digest por crate; `compute_bundle_source_digest` con path-deps (paridad bundle).
- `compute_crate_source_digest_with_workspace` — política B.

## L3 — Testigo ELF (CA8)

- `write_capsule_witness` en `capsule_paths.rs`.
- `build-release-bundle.sh`: `iota-immutable-publisher` en `CONSUMER_BINS`.
- `start-sddia.sh`: build release multi-cápsula + `--seal-capsules` post-build.

## L4 — Aduana anclada (CA5/CA6)

- `resolve_capsule_native_anchored`, testigos `.sha256`, lectura de `source_sha256` del genoma.
- Activación: `SDDIA_CAPSULE_ANCHOR=1`. Sin flag → legacy.

## L5 — Genoma `source_sha256` (CA4/CA7)

- `engine/capsule_seal.rs`: inventario, sellado batch, CLI `--seal-capsules`.
- `forges/common.rs`: `patch_genome_source_sha256` (parche quirúrgico).
- `entity-manager`: `lifecycle_operation: seal-anchor` delega en `capsule_seal`.

## L6 — Traza factual + purga (CA12/CA13)

- `classify_batch_anchor_friction`: relay / fósil / contrato-entrada / genérico.
- Eliminado bloque `#region agent log` con ruta absoluta de host.

## L7/L8 — Ops

Ver `execution.md` (rebuild release, sellado, cola DLT purgada).

## Pendiente cierre

- `validacion.md` APTO + PBI → `docs/todos/done/`.
- Backfill genomas restantes de tools indexados (batch `--seal-capsules` sin filtro `names`).
- Digest política A estricta (`cargo metadata` + lockfile recortado) — follow-up.
