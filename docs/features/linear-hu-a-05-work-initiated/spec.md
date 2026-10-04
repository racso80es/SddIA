---
feature_name: linear-hu-a-05-work-initiated
created: "2026-10-04"
process: feature
---

# Especificación — Work_Initiated en Ejecución

| Componente | Comportamiento |
|------------|----------------|
| `workspace_init.rs` | Ya no emite `Work_Initiated`; `work_initiated` en resultado = `null`. |
| `executor.rs` / `residual_runner.rs` | `maybe_emit_work_initiated_on_tekton_entry` antes de fase agente con `agent:tekton`. |
| Payload | `branch`, `persist_ref`, `source_process`; opcionales `project_slug`, `pbi_ref`, `tracker_ref`. |
| Barrera diseño | Ciclos que paran antes de Ejecución no emiten (R-1). |
