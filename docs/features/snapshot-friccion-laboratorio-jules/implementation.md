---
feature_name: snapshot-friccion-laboratorio-jules
process: feature
created: "2026-06-11"
items:
  - scripts/skills/git-manager.py
  - SddIA/scripts/qa/execute_process_capsules.py
  - SddIA/norms/external-ai-constraints.md
  - .cursorrules
  - SddIA/skills/intent-transpiler.md
  - SddIA/skills/index.md
  - docs/todos/done/Snapshot_Friccion_Laboratorio_Jules.md
---

# Implementación — snapshot-friccion-laboratorio-jules

## Touchpoints

| Archivo | Cambio |
|---------|--------|
| `scripts/skills/git-manager.py` | Detección offline en `fetch`/`pull`/`push`; envelope `{ offline: true, exitCode: 0 }` |
| `SddIA/scripts/qa/execute_process_capsules.py` | `_git_manager_data_from_body`; `run_workspace_init` tolera fetch/pull offline |
| `SddIA/norms/external-ai-constraints.md` | v1.1.0 — DA-4, prefijo creator ampliado |
| `.cursorrules` | §8 — referencia DA-4 |
| `SddIA/skills/intent-transpiler.md` | Forja vía `entity-manager` + contrato LLM-native |
| `SddIA/skills/index.md` | Fila `intent-transpiler` |
| `docs/todos/done/Snapshot_Friccion_Laboratorio_Jules.md` | PBI archivado |

## Entidades EDA

| UUID | Entidad |
|------|---------|
| `4f0edfe0-4380-442b-962d-9e98f8ecf956` | skill `intent-transpiler` |
| `95b5ac3a-061f-458d-bfb6-69f91a1c1731` | norma `external-ai-constraints` (update v1.1.0) |
