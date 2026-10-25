---
feature_name: ci-index-qa-execution-time
created: "2026-10-25"
process: bug-fix
branch: fix/ci-index-qa-execution-time
items_applied:
  - T1
  - T2
  - T3
  - T4
  - T5-docs
items_blocked:
  - T5-evolution-rehash
  - T5-commits-git-manager
tracker_ref: AUT-13
execution_id: "fe2e81d3-8e1b-42da-83d6-dee0a3661acf"
---

# Ejecución — AUT-13

| Tarea | Resultado |
|-------|-----------|
| T1 | Labs Linear con `--lib` en `linear_direct_cycle_e2e_lab.rs` y `linear_done_gate_both_e2e_lab.rs`. Verificado en FS. |
| T2 | Prep `-p execute-process`; workflow `sddia-installer-qa.yml`; smoke fuera de `sddia-index-qa`. Verificado en FS. |
| T3 | `CARGO_PROFILE_*_DEBUG=0` + strip; stage WASI sin `deps/`/`incremental/`. Verificado en FS. |
| T4 | Job `changes` + `classify-sddia-index-qa-changes.sh`; un `cargo test` integrity; Linear gated; caché `native-test-lance`. Verificado en FS. |
| T5 | `implementation.md` + este `execution.md` materializados. **Bloqueado:** Shell IDE rechaza `./sddia-run.sh --tool git-manager` y `sddia-qa evolution-rehash` → sin commits por tarea ni `hash_integrity` canónico. |

Rama (lectura `.git/HEAD`): `fix/ci-index-qa-execution-time`.

AC-1..AC-6: código en working tree alineado a spec C1–C5. AC-7 parcial: cascada docs sí; evolution anclado + commits git-manager = **PENDIENTE-SHELL**. Evidencia CI (run_id) = fase Argos / Actions.

## Cierre (merge)

- PR [#386](https://github.com/racso80es/SddIA/pull/386) → `15b2945b`.
- Laudo instalador: `paths` idénticos en `pull_request` y `push` `main` (`6943b166`); doc `clarify-installer-smoke.md`.
- Linear AUT-13: descripción actualizada (2026-10-25) con comportamiento `sddia-installer-smoke`.
