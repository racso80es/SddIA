---
feature_name: gate-anti-anemia-entidades-dominio
created: "2026-11-26"
process: refactorization
persist_ref: docs/refactors/gate-anti-anemia-entidades-dominio
tracker_ref: AUT-125
---

# Implementation — gate anti-anemia

| Área | Touchpoints |
| --- | --- |
| Motor | `entity_no_anemia.rs`, `mod.rs` |
| QA CLI | `gate_entity_no_anemia.rs`, `main.rs` |
| Forja | `common.rs` (`patch_artifact_yaml_replacements`, `sanitize_action_index_description`, índice acciones), `factory.rs` |
| DCC/PPR | `phase_capsules.rs`, `pull_request_review.rs` |
| Genoma | Remediación masiva entidades dominio (actions, process, agents, events, daemons, skills, suites, codex) |
| Índice acciones | `rebuild_actions_index.py` + saneamiento celdas multilínea |
| Expediente | `remediate_bulk.py`, `audit-inventory.json`, CI workflow |
| Tests | `entity_no_anemia` (4 unit tests en `execute-process`) |