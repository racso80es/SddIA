---
feature_name: inmunidad-caos-fase1
created: "2026-05-28"
process: feature
items:
  - id: "1.A"
    touchpoint: "SddIA/norms/execution-contexts.md"
    proposal: "§2.9 chaos-engineering; version 1.1.0"
  - id: "1.B"
    touchpoint: "SddIA/tools/tools-contract.md"
    proposal: "v1.3.0 §6 termodinámica declarativa"
  - id: "1.C"
    touchpoint: "SddIA/scripts/qa/chaos_workspace_utils.py"
    proposal: "assert_workspace_bound + touchpoints-ia.md"
  - id: "1.D"
    touchpoint: "SddIA/tools + scripts/tools"
    proposal: "io-choke, schema-corruptor, sandbox-breacher"
  - id: "1.E"
    touchpoint: "index.md + test_chaos_tools.py"
    proposal: "catálogo y regresión"
---

# Implementación — Fase 1

| ID | Artefacto | Estado |
|----|-----------|--------|
| 1.A | `execution-contexts.md` | ✅ |
| 1.B | `tools-contract.md` | ✅ |
| 1.C | `chaos_workspace_utils.py`, `touchpoints-ia.md` | ✅ |
| 1.D | 3 specs + 3 cápsulas Python | ✅ |
| 1.E | `index.md`, `test_chaos_tools.py` | ✅ |
