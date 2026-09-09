---
feature_name: kalma2-aiua-perceptive-bridge
created: "2026-09-09"
process: feature
purpose: Estabilización Filtro A PBI v1.2.0; canal perceptivo WUI→Aiúa
version_clarify: "1.0.0"
execution_id: "f075b5ea-aac3-47cb-b5ba-a18c5431a7e5"
pbi_ref: docs/todos/pending/[NÚCLEO] Puente Perceptivo: Interacción Biológica con Tormentosa desde Kalma2 WUI.md
document_id: PBI-NUCLEO-PUENTE-PERCEPTIVO-KALMA2
pbi_uuid: "d7192a54-7389-4b68-b3d4-b91c0e35921a"
pbi_version: "1.2.0"
---

# Clarificación — kalma2-aiua-perceptive-bridge

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. `execution_id` `f075b5ea-aac3-47cb-b5ba-a18c5431a7e5`. Rama `feat/kalma2-aiua-perceptive-bridge`. Mayeuta…Argos: simulated / phase-barrier; relevo IDE.

## Decisiones

| ID | Laudo |
|----|-------|
| L-ATOMIC | Un POST `/api/aiua/interact` + una respuesta JSON. Cero SSE, WebSocket, tokens parciales, cola inversa. |
| L-BLIND | `aiua-stimulus-processing` permanece ciego a Kalma2. Mutación = WUI + `kalma2-bridge`. Cero genoma del latido. |
| L-BUTTON | Botón `#aiua-pulse`. `#chat` / `#forge` / `#sync-genome` / `Ctrl+Enter` intactos. Selector de interlocutor rechazado. |
| L-ENVELOPE | Stdout hijo = `OrchestratorEnvelope`. Puente extrae `data` y aplana al JSON WUI. Campo proceso = `exitCode`. |
| L-PROCESS | Parametrizar spawn (`process` + `inputs`). Camino Aiúa **nunca** llama `kalma2-interact`. Helper legacy de Mayeuta síncrono conserva el nombre actual. |
| L-PULSE | `#cognitive-pulse` agregado = Radamanto/SSE. Latido → `#status` + `appendProgressTrace` objeto PTC. Spans `#aiua-last-*` opcionales (SSE no los toca). |
| L-THOUGHT | `thought_id` = SHA-256 hex 64. Nunca UUID. |
| L-BUSY | `setBusy` deshabilita `#chat`, `#forge`, `#sync-genome`, `#aiua-pulse`. No el textarea. |
| L-TIMEOUT | `SDDIA_CLIENT_TIMEOUT_SECONDS` ≥ `SDDIA_GEMINI_HTTP_TIMEOUT_SECS` (env heredada). Cero env nueva. Cero `sleep`/retry post-acuse (DA-5). |
| L-FRACTURE | Fractura solo si falta ELF orquestador o el spawn falla. `attempted_action: aiua_interact`. Gemini/LanceDB/`success:false` → HTTP 5xx sanitizado, sin evento. |
| L-NO-SLUG | Cero model slug en WUI/bridge. Pintar `telemetry.model` tal cual. `tokens` objeto u omitir. |
| L-VOICE | Identidad 1ª persona = kaizen H-LLM-3. Este PBI renderiza `data.response` literal. |
| L-CI | `validacion.md` no `global: APTO` hasta `run_id` verde. `accept-pr` solo entonces. |

## Filtro A (no reintroducir)

- Cero Python/FastAPI/`sddia-client-bridge.py` en la pasarela.
- `interfaces/kalma2/README.MD` es fósil; SSOT = `main.rs` + `kalma2-bridge.sh`. README fuera de alcance.
- `#cognitive-pulse` no se pisa con telemetría del latido.
- `appendProgressTrace` exige objeto `{severity, source_agent, phase, message}`, no string.
- `depends_on` kaizen/genoma/Anatomía Motora/Poda = `related`, no prerrequisito.
