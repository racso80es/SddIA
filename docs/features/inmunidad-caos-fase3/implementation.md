---
feature_name: inmunidad-caos-fase3
created: "2026-05-29"
process: feature
items:
  - id: "3.A"
    touchpoint: "cumulo.paths.json, suite-creator, entity-manager, sync-entity-index"
    proposal: "Genoma ED Suite — 9.ª clase entity-manager"
  - id: "3.B"
    touchpoint: "SddIA/suites/suites-contract.md, index.md"
    proposal: "Contrato Suite + catálogo"
  - id: "3.C"
    touchpoint: "execute-suite.md, workspace_utils, execute_process_capsules"
    proposal: "Orquestador con sub-workspaces aislados"
  - id: "3.D"
    touchpoint: "compile_survival_manifest"
    proposal: "Manifiesto Argos post-nodos"
  - id: "3.E"
    touchpoint: "suites/core-full-stress.md"
    proposal: "Códice de Asedio — 3 procesos Fase 2"
  - id: "3.F"
    touchpoint: "test_execute_suite.py, eda-coverage.json, smoke fixture"
    proposal: "Regresión AC3.x + cobertura EDA"
---

# Implementación — Fase 3

| ID | Artefacto | Estado |
|----|-----------|--------|
| 3.A | Genoma ED Suite (SSOT + creators) | ✅ |
| 3.B | `suites-contract` + índice | ✅ |
| 3.C | `execute-suite` + `materialize_child_workspace` | ✅ |
| 3.D | `survival-manifest.md` | ✅ |
| 3.E | `core-full-stress` | ✅ |
| 3.F | Tests + EDA coverage | ✅ |
