---
feature_name: interaction-audit-session-karma2
created: "2026-10-28"
process: feature
branch_name: feat/interaction-audit-session-karma2
---

# Implementación

| Área | Cambio |
| --- | --- |
| `llm_interaction_audit.rs` | `InteractionRow` karma2; tabla v2; frontmatter `uuid` v4; `maybe_record_process_peaje` |
| `thermodynamic.rs` | Peaje registra fila `process` por ciclo lifecycle |
| `handlers/llm_interaction_audit.rs` | `record_interaction` exige `entity_type`, `entity_id`, `invoked_by`, `timestamp` |

Legacy `rol`/`tipo`/`resumen` retirados del handler.
