---

feature_name: wasm-linear-tracker-auth-fallback
created: "2026-10-25"
process: bug-fix
base: main
tracker_ref: AUT-11
document_id: PBI-ab4c1ddb
uuid: ab4c1ddb-0a29-446d-b331-9ae9f6689467
branch_name: fix/wasm-linear-tracker-auth-fallback
persist_ref: docs/fixes/wasm-linear-tracker-auth-fallback
execution_id: "480762f7-058e-410d-841c-d3750e90b3bb"
---
# Spec — AUT-11 (WASM linear-tracker-adapter sin `LINEAR_API_TOKEN`)

## Misión

`invoke_tool_for_process` prefiere WASM; wasmtime solo reenvía `SDDIA_*`, no `LINEAR_API_TOKEN`. El adapter devuelve `LINEAR_AUTH_MISSING` en envelope v2 (`error.code`), pero el motor no activaba fallback nativo ni propagaba el código (mensaje plano `tool failed`). `forge-pbi` acusaba `success: true` con fase tracker en `warn` sin `tracker_ref`.

## Corte

| ID | Entrega |
|----|---------|
| C1 | Marcadores WASM→nativo incluyen códigos Linear de auth; blob de fallo lee `error.code` objeto |
| C2 | `tool_invoke_error_message` expone `LINEAR_AUTH_MISSING` (y equivalentes) al caller |
| C3 | `forge-pbi` falla (`success: false`) si tracker Linear está configurado, `create_issue` se invocó y no hay `tracker_ref` |

## Criterios (Linear AUT-11)

- AC-1: `forge-pbi` con `provider: linear` y token en bóveda obtiene `tracker_ref` sin `--prefer-native` (fallback nativo tras fallo WASM).
- AC-2: Si WASM no autentica, fallback nativo o error visible `LINEAR_AUTH_MISSING` en envelope.
- AC-3: Test unitario de extracción de código y marcadores de fallback.
- AC-4: Prohibido fail-soft `success: true` sin `tracker_ref` cuando el registro tracker era obligatorio.

## Fuera de alcance

No inyectar `LINEAR_API_TOKEN` en el módulo WASM. Sin cambios al contrato GraphQL del adapter.
