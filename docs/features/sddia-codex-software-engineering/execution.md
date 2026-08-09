---
feature_name: sddia-codex-software-engineering
created: "2026-08-09"
process: feature
branch_name: feat/sddia-codex-software-engineering
persist_ref: docs/features/sddia-codex-software-engineering
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-02
items_applied:
  - domain_authority
  - codex_software_engineering
  - unit_tests
  - smoke_deny_allow
agents: tekton
---

# Execution — sddia-codex-software-engineering

## Unit

| Suite | Resultado |
|-------|-----------|
| `domain_authority::*` (5) | OK |
| `domain_profile::*` (4) | OK |
| `cargo build -p execute-process --release` | OK |

## Smoke gate

| Caso | Inputs | Resultado |
|------|--------|-----------|
| **AC-GATE** | `_smoke-deny.json` (`git_required:false`, sin slug) | `success:false` · `DOMAIN_AUTHORITY_DENIED` |
| **AC-ALLOW** | `_smoke-allow.json` (`codex_slug: codex-software-engineering`) | `success:true` · `workspace-init` executed |

```bash
SDDIA_AGENT_RUNTIME_COMMAND= SDDIA_LAB_SKIP_PBI_ARCHIVE=1 SDDIA_LAB_SKIP_DELIVERY_CLOSE=1 \
  SddIA/target/debug/execute-process --process feature \
  --inputs "$(cat docs/features/sddia-codex-software-engineering/_smoke-deny.json)"
```
