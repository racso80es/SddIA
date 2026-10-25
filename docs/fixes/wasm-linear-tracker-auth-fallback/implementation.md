---
feature_name: wasm-linear-tracker-auth-fallback
created: "2026-10-25"
process: bug-fix
tracker_ref: AUT-11
---
# Implementación — AUT-11

## `SddIA/engine/execute-process/src/engine/capsules.rs`

- `capsule_failure_blob`: agrega `message`, `error` string u objeto (`code` + `message`).
- `WASM_NATIVE_FALLBACK_MARKERS`: `linear_auth_missing`, `linear_auth_rejected`, `linear_api_token`.
- Fallback WASM→nativo usa el blob completo (detecta `LINEAR_AUTH_MISSING` desde WASM).
- `tool_invoke_error_message`: preferencia `error.code` para `invoke_tool_for_process`.
- Test `tool_invoke_error_message_surfaces_linear_auth_code`.

## `SddIA/engine/execute-process/src/engine/handlers/forge_pbi.rs`

- Tras fase tracker: si manifiesto tiene tracker, `create_issue` invocado y sin `tracker_ref` → envelope `success: false`, `error` = código (`LINEAR_AUTH_MISSING` u otro), fase `Registro en tracker` = `failed`.
