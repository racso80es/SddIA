---
feature_name: kaizen-rust-compile-audit
created: "2026-10-25"
process: bug-fix
items:
  - engine/mod.rs
  - email-watcher
  - executor.rs
  - done_gate.rs
  - sddia-qa/resolve.rs
  - lab_state.rs
  - iota-publish-relay
  - instance_health_verify.rs
  - tracker_stamp.rs
  - qa_attestation.rs
  - fracture_signatures.rs
  - pull_request_review.rs
  - enrich_fracture_pbi_kaizen.rs
  - phagocyte_recovered_fracture_pbis.rs
  - tracker_sync_replay.rs
  - sandbox.rs
---

# Implementación — AUT-3

El módulo AC-21 queda bajo `#[cfg(test)]`, en paridad con `linear_direct_cycle_e2e`. El arnés no se reescribe.

`catalog_for_enrich` llama `load_fracture_catalog` una vez. Las dos síntesis de `run` reciben `Some(&catalog)`. El `Err` de la carga clona `shared_default_catalog()` y `enrich_notice` lo añade a `message`. Un fichero ausente en un repo temporal sigue resolviendo el manifiesto del crate: esa carga es `Ok` y el mensaje no anuncia fallback. Un JSON inválido sí.

`resolve_replay_tracker` es el único sitio del fallback. Sin mock devuelve `replay: sin tracker y sin SDDIA_LAB_MOCK_OUTBOUND`.

`resolve_under_root` mapea el fallo de `canonicalize` a `SandboxError::Io` cuando `exists()` es verdadero.

`rel_under_pending` exige el directorio o el prefijo `pending/`. Un hermano `pending-evil` no pasa.

`resolve_refined_diagnosis` exige `scope_matches` además del AST.

`handoff_accept_pr` sigue siendo quien escribe `accept_pr_handoff`. El test F5 lo invoca con veredicto aprobado y sin `merge_already_done`.
