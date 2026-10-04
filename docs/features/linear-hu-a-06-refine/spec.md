---


feature_name: linear-hu-a-06-refine
execution_id: "f1a3412c-c178-4b24-a034-8cc8f53c2407"
---
# Especificación — refine-hu / refine-pbi

| Artefacto | Rol |
|-----------|-----|
| `SddIA/process/refine-pbi.md` | Proceso Core; context knowledge-management + filesystem-ops |
| `SddIA/process/refine-hu.md` | Proceso Core; + tracker-operations |
| `SddIA/engine/execute-process/src/engine/handlers/refine.rs` | Handlers `run_pbi` / `run_hu` |
| `SddIA/engine/execute-process/src/engine/cerbero_di_rbac.rs` | Políticas desde `context` escalar comma-separated |
| `SddIA/engine/execute-process/src/engine/mod.rs` | Dispatch nativo |
