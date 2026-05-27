---
feature_name: telemetria-reactiva-eda-fase4
created: "2026-05-27"
process: feature
items:
  - id: "4.0"
    touchpoint: "dlt-handoff-acta.md"
    proposal: "Acta ventana dual DLT D0.1 — Cúmulo PR/ECST intacto"
  - id: "4.E'"
    touchpoint: "SddIA/events/domain/tool-*.md, status-restored.md"
    proposal: "Clases ECST Self-Healing Tool_Degraded / Status_Restored / Tool_Deprecated"
  - id: "4.A"
    touchpoint: "SddIA/agents/radamanto.md, radamanto.instructions.json"
    proposal: "Contrato actuario + prohibición medición directa"
  - id: "4.B"
    touchpoint: "cumulo.paths.json v1.3.0, radamanto.thresholds.json"
    proposal: "Umbrales configurables SSOT"
  - id: "4.E"
    touchpoint: "radamanto-batch.md, radamanto_batch_core.py"
    proposal: "Sustituye stub; único emisor Status_Restored"
  - id: "4.C"
    touchpoint: "cerbero-governance-react, fix-tool-process, event-domain-subscriptions.json"
    proposal: "Suscripciones Cerbero + reparación"
  - id: "4.D"
    touchpoint: "fix_tool_process_core.py"
    proposal: "Sandbox estricto + Argos structure_valid sin redención"
  - id: "4.F"
    touchpoint: "test_radamanto_*.py, test_eda_fractal_bus.py"
    proposal: "Smoke Self-Healing + DLT dual"
---

# Implementación — Fase 4

| Paso | Archivos | Cambio |
|------|----------|--------|
| 4.0 | `dlt-handoff-acta.md` | Matriz Cúmulo vs Radamanto |
| 4.E′ | `events/domain/tool-*.md`, `status-restored.md`, `index.md` | Genoma Self-Healing |
| 4.A | `agents/radamanto.md`, `.instructions.json`, `index.md` | Contrato actuario |
| 4.B | `cumulo.paths.json` v1.3.0, `eda_bus_utils.load_radamanto_config` | SSOT umbrales |
| 4.E | `radamanto-batch.md`, `radamanto_batch_core.py`, `event-telemetry-subscriptions.json` | Batch real |
| 4.C | `cerbero-governance-react.md`, `fix-tool-process.md`, `event-domain-subscriptions.json` | Fan-out dominio |
| 4.D | `fix_tool_process_core.py`, `cerbero_governance_react_core.py` | Sandbox + RBAC |
| 4.F | `test_radamanto_self_healing.py`, `test_radamanto_dlt_tool_status.py` | QA |

Nuevos módulos: `radamanto_batch_core.py`, `cerbero_governance_react_core.py`, `fix_tool_process_core.py`.
