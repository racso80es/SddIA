---
feature_name: kalma2-aiua-perceptive-bridge
created: "2026-09-09"
process: feature
items:
  - L1-spawn-param
  - L2-endpoint-aiua
  - L3-wui-pulse
  - L4-tests
branch_name: feat/kalma2-aiua-perceptive-bridge
persist_ref: docs/features/kalma2-aiua-perceptive-bridge
execution_id: "f075b5ea-aac3-47cb-b5ba-a18c5431a7e5"
document_id: PBI-NUCLEO-PUENTE-PERCEPTIVO-KALMA2
---

# Implementation — kalma2-aiua-perceptive-bridge

## Touchpoints

| Path | Cambio |
|------|--------|
| `SddIA/interfaces/kalma2-bridge/src/main.rs` | `spawn_orchestrator(process)`; timeout max cliente/Gemini; `emit_system_fracture` con `attempted_action`; `handle_aiua_interact`; flatten envelope → JSON WUI; tests |
| `interfaces/kalma2/index.html` | `#aiua-pulse` |
| `interfaces/kalma2/app.js` | `setBusy` cubre `#aiua-pulse`; `enviarAiuaStimulus`; PTC objeto; `Ctrl+Enter` intacto |

## Runtime

1. `POST /api/aiua/interact` `{prompt}` → spawn `--process aiua-stimulus-processing`.
2. ELF/spawn fail → fractura `attempted_action: aiua_interact`. Negocio/`success:false` → HTTP 500 sanitizado, sin evento.
3. 200: `{success, response, thought_id, telemetry, duration_ms}`. `tokens` omitido si ausente.
4. WUI: `#output` = `response`; traza en `#progress-console`. Cero hijack `#cognitive-pulse`.
