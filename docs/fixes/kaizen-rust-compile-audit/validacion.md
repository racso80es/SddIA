---
feature_name: kaizen-rust-compile-audit
created: "2026-10-25"
process: bug-fix
branch: fix/kaizen-rust-compile-audit
global: APTO
pbi_archived: false
tracker_ref: AUT-3
checks:
  KZ-CA1: APTO
  KZ-CA2: APTO
  KZ-CA3: APTO
  KZ-CA4: APTO
  KZ-CA5: APTO
  KZ-CA6: APTO
  KZ-CA7: APTO
  KZ-CA8: APTO
  KZ-CA9: APTO
  KZ-CA10: APTO
git_changes:
  - SddIA/daemons/email-watcher/src/main.rs
  - SddIA/daemons/iota-publish-relay/src/main.rs
  - SddIA/engine/execute-process/src/core/fracture_signatures.rs
  - SddIA/engine/execute-process/src/engine/done_gate.rs
  - SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs
  - SddIA/engine/execute-process/src/engine/executor.rs
  - SddIA/engine/execute-process/src/engine/handlers/instance_health_verify.rs
  - SddIA/engine/execute-process/src/engine/handlers/phagocyte_recovered_fracture_pbis.rs
  - SddIA/engine/execute-process/src/engine/handlers/tracker_backlog_query.rs
  - SddIA/engine/execute-process/src/engine/handlers/tracker_markdown_apply.rs
  - SddIA/engine/execute-process/src/engine/handlers/tracker_stamp.rs
  - SddIA/engine/execute-process/src/engine/handlers/tracker_sync_replay.rs
  - SddIA/engine/execute-process/src/engine/mod.rs
  - SddIA/engine/execute-process/src/engine/pull_request_review.rs
  - SddIA/engine/execute-process/src/engine/qa_attestation.rs
  - SddIA/tools/linear-tracker-adapter/src/lab_state.rs
  - SddIA/tools/sddia-qa/src/resolve.rs
  - SddIA/tools/sddia-workspace-server/src/sandbox.rs
  - docs/fixes/kaizen-rust-compile-audit/
---

# Validación — AUT-3

`cargo build -p execute-process -p sddia-qa -p linear-tracker-adapter -p sddia-workspace-server -p email-watcher -p iota-publish-relay` terminó en el perfil `dev`. Los warnings que quedan son ajenos al corte: `PathBuf` en `forge_pbi.rs` y `refine.rs`, y `dead_code` de `seal_markdown`, `signal_process_alive`, `phase_requires_git_sync`, `phase_has_git_manager_delegate`, `assert_git_isolation`. Ningún símbolo de §2.1 ni §2.2.

`cargo test -p execute-process --lib linear_done_gate_both_e2e -- --test-threads=1`: 3 verdes. El apply archivaba mal porque `update_issue_state` y `update_issue_description` llevaban `project_slug` y el adaptador de fichero buscaba en `docs/_fixtures/lab-done-gate-project-both`, mientras `resolve_markdown_by_tracker_ref` ya había encontrado el PBI en `docs/todos/pending` del Core. Esas dos mutaciones ya no llevan el slug: el slug sigue eligiendo el tracker remoto y la rama `tracker-sync`. AC-14, AC-15 y AC-16 siguen verdes. El módulo no entra en el `cargo build` sin tests.

Verdes del corte, un test por criterio: `enrich_uses_repo_catalog_and_announces_load_fallback`, `replay_fallback_only_with_lab_mock`, `canonicalize_failure_on_existing_path_is_io`, `parent_dir_is_escape`, `apply_documental_skips_outside_pending`, `refined_child_requires_scope`, `t_a3_no_true_pending_f5_convention`, `watermark_roundtrip_persists_last_uid`, `skipped_barrier_entry_marks_verification`, `fracture_corpus_regression`.

`forge_pbi.rs` no está en el diff. No contiene `split_frontmatter` ni `hu_parent_tracker_ref`. El alta lee `parent_ref` del input (`ac_0b_creates_tracker_ref_with_parent`). No se reintroducen.

Sin mutación de genoma. AUT-3 sigue en Backlog. Sin commit.
