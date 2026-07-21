---
feature_name: kaizen-kalma2-feature-cycle-observability
created: "2026-07-21"
process: feature
purpose: Checklist entrega reproducible (O3) — member Cargo / lockfile
---

# Checklist — entrega reproducible (Cargo)

Antes de Argos APTO en features que toquen el workspace Rust:

1. Si se añade member en `SddIA/Cargo.toml` → actualizar `SddIA/Cargo.lock` en el mismo PR.
2. Evidencia obligatoria en `execution.md`:
   - `cargo check -p <crate> --locked`
   - `cargo test -p <crate> --locked` (o justificar 0 tests)
3. Prohibido declarar AC compile APTO sin `--locked` cuando el crate es member nuevo.
4. Inventariar `Cargo.lock` en `git_changes` de `validacion.md`.
