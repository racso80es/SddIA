---

feature_name: wasm-linear-tracker-auth-fallback
created: "2026-10-25"
process: bug-fix
tracker_ref: AUT-11
branch_name: fix/wasm-linear-tracker-auth-fallback
execution_id: "480762f7-058e-410d-841c-d3750e90b3bb"
---
# Ejecución — AUT-11

1. Rama `fix/wasm-linear-tracker-auth-fallback` desde `main`.
2. Parches motor `capsules` + `forge_pbi` según `implementation.md`.
3. `cargo test -p execute-process --lib tool_invoke_error_message`
4. `cargo test -p execute-process --lib forge_pbi`
5. Pendiente: PR + `delivery-close-cycle`; Linear `AUT-11` → Done tras merge (`done_gate: linear`).
