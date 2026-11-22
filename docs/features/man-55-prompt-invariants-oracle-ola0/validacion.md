---
feature_name: man-55-prompt-invariants-oracle-ola0
created: 2026-11-22
process: feature
branch: feat/man-55-prompt-invariants-oracle-ola0
persist_ref: docs/features/man-55-prompt-invariants-oracle-ola0
tracker_ref: MAN-55
execution_id: 6931ec92-babc-4ef7-a4d1-a28d5148d33c
global: APTO
checks:
  sddia_qa_tests: APTO
  sddia_qa_clippy: APTO
  gate_allowlist_ratchet: APTO
  eda_event_token_hygiene: APTO
  sync_live_organs_release: APTO
  verify_compiled_capsules_live: APTO
  qa_gates_suite: APTO
git_changes:
- .cursor/rules/task-closure-documental.mdc
- SddIA/agents/argos.md
- SddIA/agents/dedalo.md
- SddIA/agents/tekton.md
- SddIA/core/eda-coverage.json
- SddIA/evolution/9823b787-cf0b-48f3-bc9e-91fec7f1eb66.md
- SddIA/evolution/Evolution_log.md
- SddIA/library/norms/features-documentation-pattern.md
- SddIA/library/norms/index.md
- SddIA/norms/external-ai-constraints.md
- SddIA/templates/spec-template/spec.md
- SddIA/tools/sddia-qa/src/gate_allowlist_ratchet.rs
- SddIA/tools/sddia-qa/src/gate_prompt_invariants.rs
- SddIA/tools/sddia-qa/src/main.rs
---

# Validación — man-55-prompt-invariants-oracle-ola0

Veredicto global: **APTO**

## Cobertura de criterios (MAN-55)

| Criterio | Evidencia | Estado |
|----------|-----------|--------|
| **AC-1 (`K-ORA` / `KZ-PRM-01`)** | `spec-template/spec.md`, `features-documentation-pattern.md` (`v1.3.3`) y `dedalo.md` exigen oráculo mecánico ejecutable por AC y captura en rojo (`oracle-red.log`) en Ola 0; test `prompt_invariants_ac1_oracle_ola0_present` verde (tras captura en rojo `oracle-red.log`) | APTO |
| **AC-2 (`K-EVA` + `K-RATCH`)** | Cláusula anti-evasión `KZ-PRM-02` en `external-ai-constraints.md` (`DA-8`) y `tekton.md`; `gate_allowlist_ratchet.rs` + subcomando `gate-allowlist-ratchet`; tests `gate_allowlist_ratchet::tests` (4/4) y `prompt_invariants_ac2_anti_evasion_present` verdes | APTO |
| **AC-3 (`K-HERM` + `K-GEN`)** | Baseline hermético Ola 0 (`KZ-PRM-03`) en `features-documentation-pattern.md` y `external-ai-constraints.md`; protocolo estricto `entity-manager` (`KZ-PRM-04`, firmas old≠new verificadas en `dedalo`, `tekton`, `argos`, `features-documentation-pattern`); test `prompt_invariants_ac3_hermetic_and_entity_manager_present` verde | APTO |
| **AC-4 (`K-HOST` + `K-ROLE`)** | Paridad build/host (`KZ-PRM-05`) y separación Tekton/Argos (`KZ-PRM-06`) en `external-ai-constraints.md`, `tekton.md`, `argos.md` y `.cursor/rules/task-closure-documental.mdc`; `sync-live-organs.sh` (9/9 `--release`, 6 servicios reiniciados), `eda_event_token_hygiene` (9/9) y `prompt_invariants_ac4_host_parity_and_role_separation_present` verdes | APTO |
