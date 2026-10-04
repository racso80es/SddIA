---


feature_name: linear-hu-a-07-forge-pbi
---
# Especificación — forge-pbi + Linear

| Artefacto | Rol |
|-----------|-----|
| `SddIA/library/codexes/codex-software-engineering/process/forge-pbi.md` | Proceso dominio v1.1.3; context + `tracker-operations` |
| `SddIA/engine/execute-process/src/engine/handlers/forge_pbi.rs` | Fase tracker + sellado PBI |
| `SddIA/engine/execute-process/src/engine/project_binding.rs` | Labels tracker por tipo |
| `SddIA/engine/execute-process/src/engine/tracker_operations_gate.rs` | Gate `create_issue` para forge-pbi |
| `SddIA/engine/execute-process/src/forges/common.rs` | `patch_process_phases_update` acepta `process_context` |
