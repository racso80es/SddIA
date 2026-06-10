---
feature_name: fix-tool-process-entity-type-scope
created: "2026-06-10"
process: feature
branch_name: feat/fix-tool-process-entity-type-scope-4531057036477780961
persist_ref: docs/features/fix-tool-process-entity-type-scope
---

# Plan — fix-tool-process-entity-type-scope

## Fases

1. **Gate estricto** — retirar `"skill"` del allowlist en `fix_tool_process_core.py`.
2. **Documentación proceso** — nota gate estricto en `fix-tool-process.md`.
3. **Tests** — migrar fixtures `skill:*` → `tool:*` en `test_radamanto_self_healing.py`.
4. **Kaizen** — mover PBI a `docs/todos/done/`.
5. **Validación** — unittest + `verify-process-integrity`; aduana `pull-request-review`.
