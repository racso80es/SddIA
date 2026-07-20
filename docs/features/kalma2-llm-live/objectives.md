---
feature_name: kalma2-llm-live
created: "2026-07-20"
process: feature
branch_name: feat/kalma2-llm-live
persist_ref: docs/features/kalma2-llm-live
document_id: PBI-KALMA2-LLM-LIVE-V2
uuid: f0f1b1ec-4b79-47c6-85e2-a0ac2ca3164b
pbi_ref: docs/todos/pending/[FEATURE] kalma2-llm-live — ejecución real Cursor desde Kalma2 (f0f1b1ec).md
depends_on:
  - docs/features/kalma2-full-cycle
execution_id_init: 7c200ac9-7713-4352-8463-886391b81540
status: in_progress
---

# Objetivos — kalma2-llm-live

## Misión

Entregar interacción S+ Grade en Kalma2: streaming SSE de tokens LLM, barrera Rust (`mayeuta-llm`) sobre prótesis Python desechable (`kalma2-agent-runtime-cursor.py` / Foso Biológico), y enrutamiento determinista Chat vs Proceso — sin que el Core conozca Cursor ni que el LLM decida el flujo de negocio.

## Punto objetivo

1. **Hito 1:** `kalma2-bridge` expone `/api/chat` SSE con watchdog; colapso → `System_Fracture_Detected`.
2. **Hito 2:** `mayeuta-llm` orquesta subproceso inyectado y reenvía stdout como stream; sin `CLASSIFY_INTENT` como aduana.
3. **Hito 3:** `.py` dual-mode: chat-stream (tokens stdout) + AGENT_PHASE JSON intacto.
4. **Hito 4:** UI Bifurca Chat vs Forjar Proceso → `/api/chat` vs `/api/execute`.

## Criterios (AC1–AC5)

Ver PBI v2.2.0 §5: SSE desde stdout Python; kill → fractura; proceso no es texto libre; purga `.py` no rompe `cargo build --release`; AGENT_PHASE no regride.

## Restricciones

- Core Rust libre de deps Cursor/SQLite (Foso Python).
- No reabrir ECST / TQM / event-bus.
- Git vía `skill:git-manager`.
- Cascada `features-documentation-pattern`.
