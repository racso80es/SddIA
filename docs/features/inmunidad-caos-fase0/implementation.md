---
feature_name: inmunidad-caos-fase0
created: "2026-05-28"
process: feature
items:
  - id: "0.A"
    touchpoint: "Barrido SddIA/ + scripts/qa/"
    proposal: "Inventario H01–H28 en impact-analysis.md"
  - id: "0.B"
    touchpoint: "Contraste PBI Fases 1–5"
    proposal: "Clasificación gap (a–d) por hallazgo"
  - id: "0.C"
    touchpoint: "PBI v2.1.0 + clarify.md"
    proposal: "Decisiones D0.1–D0.9 y subtareas inline"
  - id: "0.D"
    touchpoint: "docs/features/inmunidad-caos-fase0/"
    proposal: "impact-analysis.md, spec, plan, clarify, execution"
---

# Implementación — Fase 0 (solo documentación)

Sin mutación de código productivo Core. Entregables exclusivamente bajo `persist_ref` y refinamiento del PBI maestro en `docs/todos/pending/`.

| Área explorada | Evidencia |
|----------------|-----------|
| SSOT rutas | `cumulo.paths.json` v1.4.0 |
| Entity lifecycle | `entity-manager.md`, `sync-entity-index.md` |
| Tools / RBAC | `tools/`, `tools-contract.md`, `execution-contexts.md`, `tekton.md` |
| Sandbox | `fix_tool_process_core.py`, `radamanto.sandbox_root` |
| Runtime | `execute_process_capsules.py`, `workspace_utils.py` |
| EDA | `event-*-subscriptions.json`, `telemetry-compliance-audit.md` |
| Radamanto | `radamanto.md`, `event-domain-subscriptions.json` |
