---

feature_name: wasm-linear-tracker-auth-fallback
created: "2026-10-25"
process: bug-fix
branch: fix/wasm-linear-tracker-auth-fallback
global: APTO
pbi_archived: false
tracker_ref: AUT-11
pr_url: https://github.com/racso80es/SddIA/pull/391
checks:
  AC-1: APTO
  AC-2: APTO
  AC-3: APTO
  AC-4: APTO
git_changes:
  - SddIA/engine/execute-process/src/engine/capsules.rs
  - SddIA/engine/execute-process/src/engine/handlers/forge_pbi.rs
  - docs/fixes/wasm-linear-tracker-auth-fallback/
  - SddIA/evolution/7c4e9f12-8a3b-4d5e-9f0a-1b2c3d4e5f60.md
  - SddIA/evolution/Evolution_log.md
execution_id: "480762f7-058e-410d-841c-d3750e90b3bb"
---
# Validación — AUT-11

Evidencia local: tests `tool_invoke_error_message_surfaces_linear_auth_code` y suite `forge_pbi` en verde. Fallback WASM→nativo cubre AC-1/AC-2 cuando wasmtime está presente y el binario nativo del adapter existe bajo `SddIA/target`. Cierre tracker Linear pendiente de merge del PR.
