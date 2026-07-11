---
feature_name: event-bus-dead-letter-remediation
created: "2026-07-11"
process: bug-fix
items:
  - telegram_notify_core.py — invoke_tool_capsule_json SSOT
  - ecst_validation.py / ecst_validation.rs — límite sección FORBIDDEN
  - route_domain_event_core.py / route_domain_core.rs — lab skip + native capsules
  - iota_tool_invoke.py — sin fallback TS sin Node
  - eda_bus_utils.py — is_lab_simulated_pr_url
  - capsule_resolve.py — marcadores fallback WASI HTTP
---

# Implementación — Remediación dead-letters bus de eventos

## Touchpoints

| Archivo | Cambio |
|---------|--------|
| `SddIA/scripts/qa/telegram_notify_core.py` | `invoke_tool_capsule_json` (native) + fallback limbo Python |
| `SddIA/scripts/qa/ecst_validation.py` | `_section_block` corta en `\n## `; FORBIDDEN solo `- \`campo\`` |
| `SddIA/engine/execute-process/src/engine/ecst_validation.rs` | Paridad boundary + test `pull_request_audited_forbidden_*` |
| `SddIA/scripts/qa/eda_bus_utils.py` | `is_lab_simulated_pr_url()` |
| `SddIA/scripts/qa/route_domain_event_core.py` | Skip `skipped-lab-simulated` en precheck PR |
| `SddIA/engine/execute-process/src/engine/route_domain_core.rs` | Paridad lab skip; Telegram/IOTA `prefer_wasm=false` |
| `SddIA/engine/execute-process/src/engine/eda_bus_topology.rs` | `is_lab_simulated_pr_url()` Rust |
| `SddIA/scripts/qa/iota_tool_invoke.py` | Error nativo explícito; fallback TS solo con toolchain Node |
| `SddIA/scripts/qa/capsule_resolve.py` | Marcadores fallback WASI HTTP outbound |
| `SddIA/engine/execute-process/src/engine/capsules.rs` | Marcadores fallback WASI HTTP outbound |
| `SddIA/scripts/qa/test_eda_bus_v3plus.py` | Test `is_lab_simulated_pr_url` |

## Cadena implementada

1. **Telegram** — resolución SSOT vía cápsula Rust nativa (no ruta `scripts/tools/.../main.py`).
2. **ECST** — parser deja de absorber `## Invariantes` dentro de `### FORBIDDEN`.
3. **PR lab** — URLs `lab-simulated` / PR `#0` → `skipped-lab-simulated` (processed, no DL).
4. **IOTA** — Rust nativo primero; TS legacy solo si hay `node`/`npx` o `node_modules/.bin/ts-node`.
5. **Ops** — retirados testigos `ecst-gate` obsoletos; re-enrutados 9 pending estancados.
