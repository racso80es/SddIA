---
feature_name: inmunidad-caos-fase2
created: "2026-05-29"
process: feature
items:
  - id: "2.0"
    touchpoint: "SddIA/agents/tekton.md, argos.md, index.md"
    proposal: "RBAC chaos-engineering + event-routing"
  - id: "2.A"
    touchpoint: "SddIA/process/audit-thermodynamic-toll-failsoft.md"
    proposal: "Proceso + handler io-choke fail-soft"
  - id: "2.B"
    touchpoint: "SddIA/process/audit-telemetry-compliance-breach.md"
    proposal: "Proceso + handler schema-corruptor breach"
  - id: "2.C"
    touchpoint: "SddIA/process/audit-sandbox-isolation-rbac.md"
    proposal: "Proceso + handler sandbox-breacher"
  - id: "2.D"
    touchpoint: "execute_process_capsules.py, eda_bus_utils.py"
    proposal: "run_chaos_audit_process + resolve tool telemetry"
  - id: "2.E"
    touchpoint: "process/index.md, test_chaos_audit_processes.py, smoke fixtures"
    proposal: "Catálogo + regresión AC2.x"
---

# Implementación — Fase 2

| ID | Artefacto | Estado |
|----|-----------|--------|
| 2.0 | RBAC Tekton/Argos | ✅ |
| 2.A | `audit-thermodynamic-toll-failsoft` | ✅ |
| 2.B | `audit-telemetry-compliance-breach` | ✅ |
| 2.C | `audit-sandbox-isolation-rbac` | ✅ |
| 2.D | Handlers + tests | ✅ |
| 2.E | Índice + fixtures smoke | ✅ |
