---
feature_name: telemetria-reactiva-eda-fase2
created: "2026-05-27"
process: feature
items:
  - id: "2.A"
    touchpoint: "SddIA/process/process-contract.md, SddIA/process/*.md"
    proposal: "process-contract v1.4.0 + workspace_template en todos los procesos"
  - id: "2.B"
    touchpoint: "SddIA/scripts/qa/workspace_utils.py, execute_process_capsules.py"
    proposal: "bootstrap_process_workspace + materialize antes de fases"
  - id: "2.C"
    touchpoint: "execute_process_capsules.py"
    proposal: "sync_workspace_context en execute_phase"
  - id: "2.D"
    touchpoint: "cumulo.paths.json, eda_bus_utils, route_domain_event_core, norms"
    proposal: "paths.workspacesRoot + resolución Cúmulo persist_ref"
  - id: "2.E"
    touchpoint: "SddIA/process/workspace-smoke.md"
    proposal: "Smoke AC2.1 + .gitignore workspaces"
---

# Implementación — Fase 2

| Paso | Archivos | Cambio |
|------|----------|--------|
| 2.A | `process-contract.md` v1.4.0, 18× `SddIA/process/*.md` | `workspace_template` obligatorio |
| 2.B | `workspace_utils.py`, `execute_process_capsules.py` | Instanciación UUID + mkdir pre-fases |
| 2.C | `execute_process_capsules.py` | `sync_workspace_context` en delegaciones |
| 2.D | `cumulo.paths.json`, `eda_bus_utils.py`, `route_domain_event_core.py`, 3 normas | SSOT + sin fallback hardcodeado PR review |
| 2.E | `workspace-smoke.md`, `.gitignore` | Proceso no-SW + workspaces no versionados |

Nuevo módulo: `SddIA/scripts/qa/workspace_utils.py` (resolución Cúmulo, materialización, bootstrap).
