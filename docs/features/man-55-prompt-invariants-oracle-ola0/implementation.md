---

feature_name: man-55-prompt-invariants-oracle-ola0
created: "2026-11-22"
process: feature
branch_name: feat/man-55-prompt-invariants-oracle-ola0
persist_ref: docs/features/man-55-prompt-invariants-oracle-ola0
tracker_ref: MAN-55
items:
  - gate_allowlist_ratchet_and_prompt_oracles
  - entity_manager_norms_and_agents_update
  - templates_and_cursor_rules_update
execution_id: "95aafb28-50d1-4f8c-bdfa-574f696d168f"
---
# Implementación — man-55-prompt-invariants-oracle-ola0

## Touchpoints propuestos

- `SddIA/tools/sddia-qa/src/gate_allowlist_ratchet.rs` (nuevo): gate ratchet de cardinalidad de allowlists (`VIRTUAL_PATH_ALLOW`, `SCAN_EXCLUDE`, `INTEGRATION_SRC`, `allowlists/*.toml`) vs `origin/main` con excepción solo por `RATCHET_LAUDO` en `clarify.md`.
- `SddIA/tools/sddia-qa/src/gate_prompt_invariants.rs` (nuevo): oráculos mecánicos de `AC-1..AC-4` verificando cláusulas `KZ-PRM-01..06` en plantillas, normas, agentes y reglas Cursor.
- `SddIA/tools/sddia-qa/src/main.rs`: registro de módulos y subcomando `gate-allowlist-ratchet [--json] [--base <ref>]`.
- `SddIA/library/norms/features-documentation-pattern.md` (vía `entity-manager`): cláusulas `KZ-PRM-01` y `KZ-PRM-03`.
- `SddIA/agents/dedalo.md`, `SddIA/agents/tekton.md`, `SddIA/agents/argos.md` (vía `entity-manager`): cláusulas `KZ-PRM-01..06`.
- `SddIA/templates/spec-template/spec.md`, `SddIA/norms/external-ai-constraints.md`, `.cursor/rules/task-closure-documental.mdc`: difusión normativa y de touchpoint IDE.
