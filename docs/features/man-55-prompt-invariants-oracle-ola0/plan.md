---







feature_name: man-55-prompt-invariants-oracle-ola0
created: "2026-11-22"
process: feature
phases:
  - ola0-hermetic-baseline-and-red-oracles
  - sddia-qa-allowlist-ratchet-and-invariant-oracles
  - genome-norms-agents-templates-via-entity-manager
  - cursor-rules-and-operational-norm-sync
  - release-sync-evolution-and-dcc
branch_name: feat/man-55-prompt-invariants-oracle-ola0
persist_ref: docs/features/man-55-prompt-invariants-oracle-ola0
tracker_ref: MAN-55
execution_id: "cfd0a522-c6ca-401b-a817-679f9389c6b0"
---
# Plan — man-55-prompt-invariants-oracle-ola0

| Tarea | Alcance |
|-------|---------|
| **T0 (Ola 0)** | Registrar baseline hermético (`git status`, raíz limpia, `systemctl --user list-units 'sddia-*'`, baseline `env -u SDDIA_* cargo test -p sddia-qa`), escribir oráculos mecánicos de `AC-1..AC-4` en `sddia-qa` y capturar su ejecución en rojo en `oracle-red.log` antes de tocar genoma/reglas productivas. |
| **T1** | Implementar `gate_allowlist_ratchet.rs` + subcomando `gate-allowlist-ratchet` en `sddia-qa` (`AC-2` / `K-RATCH`). |
| **T2** | Actualizar entidades de genoma vía `./sddia-run.sh --process entity-manager` (`features-documentation-pattern`, `dedalo`, `tekton`, `argos`) verificando `handoff_hash_signature_new != handoff_hash_signature_old` (`AC-1`, `AC-2`, `AC-3`, `AC-4`). |
| **T3** | Actualizar `SddIA/templates/spec-template/spec.md`, `SddIA/norms/external-ai-constraints.md` y `.cursor/rules/task-closure-documental.mdc` con las invariantes `KZ-PRM-01..06` (`AC-1..AC-4`). |
| **T4** | Verificar tests verdes (`sddia-qa`, `execute-process`), compilar debug + `--release` (`./SddIA/scripts/sync-live-organs.sh`), reiniciar `systemctl --user restart 'sddia-*@*.service'`, re-ejecutar `eda_event_token_hygiene`, registrar evolución (`sddia-qa evolution-register`), emitir `validacion.md` APTO, `delivery-close-cycle`, iterar hasta CI verde y fin de ciclo. |
