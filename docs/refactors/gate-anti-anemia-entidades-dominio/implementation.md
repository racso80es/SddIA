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
| Motor | `entity_no_anemia.rs` (reglas por familia sin atajos, `split_frontmatter` estricto, `truncated_tail` mecánico, `classify_entity_path` por rango, `scan_changed` `base...HEAD`, `default_base_ref`), `mod.rs` |
| QA CLI | `gate_entity_no_anemia.rs` (`--base` estricto, JSON en errores), `main.rs` |
| Forja | `common.rs` (`patch_action_content_update` preserva frontmatter, `replace_yaml_key_block`, `sync_action_index_row` fila única, `sanitize_action_index_description` sin relleno, `with_file_rollback`), `factory.rs` (rollback en update de acción/agente/suite/evento/proceso solo-YAML) |
| Entity manager | `entity_manager.rs`: `update` de acción sin defaults inyectados |
| DCC/PPR | `phase_capsules.rs` (peaje in-process, `base_ref`), `pull_request_review.rs` (`anemia_triage`, `base_ref`, detalle en payload) |
| Genoma | Revertido a `main`; solo el testigo `switch-llm-provider` remediado vía `entity-manager` |
| CI | `sddia-index-qa.yml`: `gate-entity-no-anemia --if-touched --base origin/main --json` |
| Expediente | `audit-inventory.json` = inventario pre-remediación (KA-DOC-1) |
