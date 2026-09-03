---
feature_name: antigravity-connectors
created: "2026-09-03"
process: feature
items:
  - tool-gemini-http-infer
  - skill-antigravity-cli-executor
  - retire-http-skill-jules
  - outbound-lab-gemini
---

# Implementation — antigravity-connectors

| Artefacto | Rol |
|-----------|-----|
| `SddIA/tools/gemini-http-infer.md` + crate | Tool create `entity-manager` uuid `7a8da3ad-4916-4ee3-8407-aa1ecdc7ecba` |
| `SddIA/skills/antigravity-cli-executor.md` + crate | Forja nativa (update Jules regeneró UUID) + sello `create` `d8b07e6f-1cc0-4b6f-a789-02ade10471f5` |
| `SddIA/skills/antigravity-http-connector.md` | Delete `entity-manager` uuid Jules `b548b894-…` |
| `SddIA/sddia-io/src/outbound_lab.rs` | `lab_mock_gemini_url` |
| `SddIA/scripts/qa/build-wasi-capsules.sh` | Exclude nativos CLI + HTTP tool |
| `SddIA/core/eda-coverage.json` | Sellos Domain_Entity_* |

Cero `provides`. Cero taxonomía. HTTP: `ureq` + header `x-goog-api-key`. CLI: argv print-mode, stdin null.
