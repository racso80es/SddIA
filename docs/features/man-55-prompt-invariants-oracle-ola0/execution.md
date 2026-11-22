---






feature_name: man-55-prompt-invariants-oracle-ola0
created: "2026-11-22"
process: feature
branch_name: feat/man-55-prompt-invariants-oracle-ola0
persist_ref: docs/features/man-55-prompt-invariants-oracle-ola0
tracker_ref: MAN-55
items_applied:
  - ola0-hermetic-baseline-and-red-oracles
  - sddia-qa-allowlist-ratchet-and-invariant-oracles
  - genome-norms-agents-templates-via-entity-manager
  - cursor-rules-and-operational-norm-sync
  - release-sync-evolution-and-dcc
execution_id: "be6ad381-b5a2-4450-b559-aedf31ec1694"
---
# Ejecución — man-55-prompt-invariants-oracle-ola0

## Baseline Ola 0 (hermético)

- `git status --porcelain`: vacío previo a la ejecución (`workspace-init` creó `docs/audits/MAN-55.md` y `docs/features/man-55-prompt-invariants-oracle-ola0/`).
- Raíz física limpia: ausencia de `.events`, `orchestration`, `telemetry` y `_lab`.
- `systemctl --user list-units 'sddia-*'`: 6 servicios activos (`email-watcher`, `event-watcher`, `github-bridge-watcher`, `iota-publish-relay`, `kalma2-bridge`, `telegram-watcher`).
- Baseline tests: `env -u SDDIA_EVENT_STORE -u SDDIA_EXECUTE_PROCESS_BIN cargo test --manifest-path SddIA/Cargo.toml -p sddia-qa`: 26 passed, 0 failed.
- Captura en rojo Ola 0: `oracle-red.log` (`gate_prompt_invariants::tests`: 0 passed, 4 failed).

## Tareas aplicadas

- **T1 (`sddia-qa`):** Implementado `gate_allowlist_ratchet.rs` (`gate-allowlist-ratchet [--json] [--base REF]`) y `gate_prompt_invariants.rs` en `SddIA/tools/sddia-qa/src/`.
- **T2 (Genoma vía `entity-manager`):**
  - `dedalo`: `9e60e48e-43be-463d-999e-d3dbd83924af` (`handoff_hash_signature_old: sha256:f8c44970...` → `new: sha256:736ede01...`).
  - `tekton`: `b3a4c5d6-7e8f-9a0b-1c2d-3e4f5a6b7c8d` (`handoff_hash_signature_old: sha256:34bf6269...` → `new: sha256:937b8526...`).
  - `argos`: `bd3b1d76-3734-4fbb-b447-ad5e4a5e4907` (`handoff_hash_signature_old: sha256:69bded91...` → `new: sha256:73746cf3...`).
  - `features-documentation-pattern`: `4c448c82-de41-460f-b24f-82a84fa5ed69` (`v1.3.2` → `v1.3.3`, `handoff_hash_signature_old: sha256:7a177423...` → `new: sha256:d87f2d74...`).
- **T3 (Plantillas y reglas IDE):** Actualizados `SddIA/templates/spec-template/spec.md`, `SddIA/norms/external-ai-constraints.md` (`DA-8`) y `.cursor/rules/task-closure-documental.mdc`.
- **T4 (Verificación y paridad host):**
  - `cargo test -p sddia-qa`: 34 passed, 0 failed.
  - `cargo clippy -p sddia-qa --no-deps -- -D warnings`: 0 warnings.
  - `./SddIA/scripts/sync-live-organs.sh`: 9/9 órganos `--release` sellados, 6/6 unidades `sddia-*@*.service` reiniciadas, `verify-compiled-capsules` (`missing: 0`, `stale: 0`, `success: true`).
  - `cargo test -p execute-process --lib eda_event_token_hygiene` (post-reinicio): 9 passed, 0 failed.
  - Gates `sddia-qa`: `verify-tools-index` OK, `verify-process-integrity` OK, `gate-event-manager-port` (`267` scanned, `0` violations), `gate-eda-unit-no-real-backend` (`241` scanned, `0` violations), `gate-allowlist-ratchet` (`ok: true`, `0` violations).
