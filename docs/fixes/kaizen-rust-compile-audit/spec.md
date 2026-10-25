---
feature_name: kaizen-rust-compile-audit
created: "2026-10-25"
process: bug-fix
base: main
scope: higiene de compilados Rust y gates que el warning señalaba
tracker_ref: AUT-3
---

# Spec — AUT-3 higiene de compilados

## Hecho

El issue [AUT-3](https://linear.app/oscar-perez/issue/AUT-3/kaizen-higiene-de-compilados-rust-warnings-catalogo-de-fractura-y) mezcla símbolos muertos en `cargo build` con gates que el compilador ya no consulta. El corte separa las dos clases.

Hechos del árbol que el corte no contradice:

- `mod linear_done_gate_both_e2e` entraba en el artefacto. `linear_direct_cycle_e2e` ya estaba bajo `#[cfg(test)]`.
- `split_frontmatter` / `hu_parent_tracker_ref` no están en `forge_pbi.rs`. El alta lee `parent_ref` del input. No se reintroducen.
- `use project_binding::{self, DoneGate, TrackerConfig}` en `done_gate.rs` sigue vivo en producción. El `self` que el warning nombra está en `tracker_backlog_query.rs`: solo el test lo usa (`DoneGate`). Ahí `self` queda bajo `#[cfg(test)]`.

## Corte

### Higiene

| Símbolo | Disposición |
|---|---|
| `mod linear_done_gate_both_e2e` | `#[cfg(test)]` |
| `load_last_uid` | `#[cfg(test)]` |
| `skipped_barrier_entry` | `#[cfg(test)]` |
| `str_field` en `done_gate.rs` | eliminada |
| `resolve_execute_process`, `repo_from_cwd` | eliminadas, y el `use` de `find_repo_root` |
| `lab_audit_append` | eliminada |
| `use project_binding::{self, TrackerConfig}` | `self` bajo `#[cfg(test)]` en `tracker_backlog_query.rs` |
| `SocketAddr` | fuera del `use` de `iota-publish-relay` |
| `let mut val` | `let val` |
| `let mut stamped = false` | `let stamped` (cada brazo inicializa una vez; `mut` vuelve a avisar) |
| `let mut doc` e `instance_requires_hmac` | `let doc`; la función se elimina |
| `catalog` en `signature_matches_*` | `_catalog` |
| `#[test]` duplicado | uno en `fracture_corpus_regression` |

### Gates

| ID | Disposición |
|---|---|
| KZ-N1 | `run` carga `load_fracture_catalog` una vez y lo pasa a las dos síntesis. Si la carga falla, usa `shared_default_catalog()` y lo dice en `message`. |
| KZ-N2 | El fallback de laboratorio del replay solo corre con `SDDIA_LAB_MOCK_OUTBOUND`. Sin mock y sin binding, error explícito. |
| KZ-N3 | `canonicalize` fallido sobre ruta existente → `SANDBOX_IO`. `..`, absoluto, prefijo y `starts_with` siguen en `Escape`. |
| KZ-N4 | `apply_documental` solo mueve un `rel_path` bajo `pending_rel`. |
| KZ-N5 | Un hijo de `refine` sustituye al padre si el AST coincide y `scope_matches`. |
| KZ-N6 | `t_a3_no_true_pending_f5_convention` ejecuta `handoff_accept_pr` sin merge. `f5_handoff_when_merge_absent` desaparece. |

## Fuera de corte

Unificar `str_field`. Reescribir AC-21. HMAC. `status: cerrado` del fagocito. `persist_store`. Warnings de crates no nombrados.
