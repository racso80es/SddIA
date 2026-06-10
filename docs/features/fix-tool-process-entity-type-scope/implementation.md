---
feature_name: fix-tool-process-entity-type-scope
created: "2026-06-10"
process: feature
items:
  - gate-strict-entity-type-tool
  - update-radamanto-self-healing-tests
  - document-fix-tool-process-gate
  - archive-kaizen-pbi
---

# Implementación — touchpoints

| Archivo | Cambio |
|---------|--------|
| `SddIA/scripts/qa/fix_tool_process_core.py` | `entity_type != "tool"` → skip auditable |
| `SddIA/process/fix-tool-process.md` | Nota gate estricto |
| `SddIA/scripts/qa/test_radamanto_self_healing.py` | Fixtures `tool:lab-test`, `tool:doomed`, `tool:x` |
| `docs/todos/done/[Kaizen] fix-tool-process — entity_type tool y skill compat legacy.md` | PBI archivado |
| `docs/features/fix-tool-process-entity-type-scope/*` | Manifiesto feature |
