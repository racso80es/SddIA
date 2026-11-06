---

feature_name: rust-warning-gate
created: "2026-11-06"
process: refactorization
persist_ref: docs/refactors/rust-warning-gate
document_id: PBI-a1b2c3d4
---

# Plan — rust-warning-gate

## Fases

1. **Normativa y CI**  
   Añadir norma kernel `SddIA/norms/rust-warning-gate.md`, propagar `RUSTFLAGS=-Dwarnings` en `.github/workflows/sddia-index-qa.yml` y paridad en `SddIA/.cargo/config.toml`.

2. **Higiene del workspace**  
   Corregir warnings `rustc` en crates afectados (execute-process, tools, skills WASI) hasta `cargo build --workspace`, release de ignición, `build-wasi-capsules.sh` y tests relevantes.

3. **Evolución y sellos**  
   Registrar en `SddIA/evolution/` con `relacionado` que cubra paths tocados; mantener `hash_integrity`; sellar `docs/refactors/rust-warning-gate/validacion.md` (AUT-40) antes de merge.

4. **Cierre**  
   Issue en Linear en Done según `tracker.provider`; sin ficheros nuevos en `docs/todos/pending` ni `done/`.

## Riesgos

- Tiempo de CI y jobs pesados con `-Dwarnings` en todo el workspace.  
- Desalineación `hash_integrity` / `diff_digest` si se muta material sin actualizar evolution o re-sello.

## Verificación

- `sddia-qa verify-diff-digest` y `gate-evolution` en paridad con CI.  
- Workflow `sddia-index-qa` verde en el PR.
