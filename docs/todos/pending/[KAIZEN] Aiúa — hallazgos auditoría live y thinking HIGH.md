---
document_id: PBI-AIUA-AUDIT-FINDINGS-20260908
uuid: "2fa76082-d3b1-49ed-af8e-0c1c33b7dd44"
title: "[KAIZEN] Aiúa — hallazgos de auditoría live (genoma de acciones, rol, thinking HIGH)"
format: markdown
version: "1.0.0"
created: "2026-09-09"
updated: "2026-09-09"
status: "propuesta"
refinement_status: unrefined
priority: alta
type: kaizen
process: feature
dispatch: false
suggested_branch: feat/aiua-audit-findings
persist_ref_suggested: docs/features/aiua-audit-findings
spawned_by: PBI-NUCLEO-ARRANQUE-AIUA-TORMENTOSA
depends_on: []
blocks_on: []
related:
  - docs/features/nucleo-aiua-tormentosa-motor/auditoria.md
  - docs/todos/done/PBI_Arranque_Aiua.md
  - SddIA/process/aiua-stimulus-processing.md
  - SddIA/actions/invoke-aiua-core.md
  - SddIA/actions/retrieve-active-context.md
  - SddIA/actions/persist-thought-record.md
  - SddIA/tools/gemini-http-infer.md
  - SddIA/conscience/aiua_core.md
  - SddIA/CONSTITUTION_CORE.md
  - SddIA/engine/execute-process/src/engine/handlers/aiua_stimulus.rs
source_audit: "docs/features/nucleo-aiua-tormentosa-motor/auditoria.md uuid e9988d00-3856-4904-bc2e-4146cfd4ceed"
---

### [KAIZEN] Aiúa — hallazgos de auditoría live (genoma de acciones, rol, thinking HIGH)

#### 1. Origen

Auditoría live de `nucleo-aiua-tormentosa-motor` (2026-09-08/09). El MVP del latido CLI **funciona** (PR #270 APTO). Este PBI no reabre el arranque: captura la **deuda residual** detectada al hablar con Tormentosa.

Mandato del Vértice: el LLM esperado para la Aiúa es **Antigravity Gemini 3.8 Flash High**. Model ID de catálogo: `gemini-3.8-flash`. El nivel `high` no es un slug: es `generationConfig.thinkingConfig.thinkingLevel = HIGH` (default del modelo = `medium`).

#### 2. Filtro A — hechos vs tentaciones

| Fricción | Clasificación | Realidad SSOT | Resolución en este PBI |
| :--- | :--- | :--- | :--- |
| **«Gemini 3.8 Flash High» es un model id distinto** | *Inexactitud de catálogo* | Google AI Studio: model id `gemini-3.8-flash`. Thinking: `low` \| `medium` \| `high`. | Bóveda: `SDDIA_GEMINI_MODEL=gemini-3.8-flash`. El nivel HIGH exige mutación de `gemini-http-infer` (hoy solo manda `temperature` opcional). |
| **Antigravity CLI = combustión del latido** | *Inexactitud* | PBI arranque: «LLM de Antigravity» = `gemini-http-infer`. `antigravity-cli-executor` existe y **no** está en `delegates_to`. | Fuera. No se cambia el vector HTTP en este PBI. |
| **Bóveda sin modelo** | *Hecho 2026-09-08; cerrado 2026-09-09 en instancia* | L-MODEL exige `request.model` o `SDDIA_GEMINI_MODEL`. | Instancia: clave fijada. **No** hardcodear el slug en Rust ni en `.env.example` (putrefacción de catálogo). |
| **503 de `gemini-3.8-flash`** | *Fricción de proveedor* | 2026-09-09: dos invocaciones live sin `model` en inputs → `UNAVAILABLE` high demand. No es 404. L-MODEL y slug **válidos**. | No es CA de Core. No reintentar en bucle (DA-5). |
| **DNS del host** | *Fuera de genoma* | Stub systemd-resolved / ISP. | Fuera. |
| **Kalma2 / IOTA / agente Tormentosa** | *Fuera de gate del arranque* | Ya laudado en PBI arranque. | No reabrir aquí. |

#### 3. Hallazgos a materializar

| ID | Sev. | Hallazgo | Alcance |
|----|------|----------|---------|
| H-LLM-1b | P1 | `gemini-http-infer` no envía `thinkingLevel`. «Flash High» queda en default `medium` aunque la bóveda apunte a 3.8. | Forja DA-2: tool `gemini-http-infer` + env opcional (p. ej. `SDDIA_GEMINI_THINKING_LEVEL`) **sin** slug eterno en Rust. |
| H-LLM-2 | P2 | Cuerpos `{name}.md` de `invoke-aiua-core`, `retrieve-active-context`, `persist-thought-record` truncados. Runtime en `aiua_stimulus.rs`. | `entity-manager` update de las tres acciones. |
| H-LLM-3 | P2 | Ensamblado: genoma en tercera persona + `## Estímulo`. Sin rol «Eres Tormentosa». Latidos 3.1 respondieron en 1ª persona; no está blindado. | `invoke-aiua-core` / handler: prefacio de identidad desde `aiua_core.md`, sin HTTP extra. |
| H-LLM-4 | P3 | No se inyecta `CONSTITUTION_CORE.md`. Solo `directories.conscience` / `aiua_core.md`. | Laudo: ¿inyectar Constitución o dejar frontera Constitución=leyes / genoma=identidad? Si se inyecta, vía Cúmulo. |

#### 4. Fuera

- Reabrir PR #270 / `validacion.md` APTO.
- Puente Kalma2 → Aiúa.
- Anclaje IOTA de `Thought_Persisted`.
- Forjar agente Tormentosa.
- Cambiar combustión a `antigravity-cli-executor`.
- DNS del host.
- Retry/backoff de 503 Gemini en el handler (salvo laudo explícito posterior).

#### 5. Circuito ya verificado (no rehacer)

| Latido | Fecha | `telemetry.model` | Resultado |
|--------|-------|-------------------|-----------|
| Identidad (input `model`) | 2026-09-08 | `gemini-3.1-flash-lite` | `thought_id` `11846f3b…` |
| Fricción previa (RAG) | 2026-09-08 | `gemini-3.1-flash-lite` | `thought_id` `f88789be…`; prompt tokens 1365→2074 |
| Identidad (solo bóveda) | 2026-09-09 | *intento `gemini-3.8-flash`* | L-MODEL OK. HTTP 503 high demand ×2. Sin `thought_id`. |

Instancia (2026-09-09): `SDDIA_GEMINI_MODEL=gemini-3.8-flash`, `SDDIA_GEMINI_HTTP_TIMEOUT_SECS=180`. Cero secretos en este PBI.

#### 6. Criterios de aceptación

- [ ] `gemini-http-infer` acepta thinking `high` vía env/request; default del modelo intacto si la env está vacía. Cero slug en Rust.
- [ ] Lab-mock no exige thinking ni red.
- [ ] Tres acciones: cuerpo `{name}.md` completo (inputs/outputs/delegación) alineado al handler.
- [ ] Prefacio de identidad en el `assembled_prompt` (Filtro B, primera persona) **o** laudo explícito de no-hacer documentado en `clarify.md`.
- [ ] H-LLM-4: laudo en `clarify.md` (inyectar o no Constitución).
- [ ] Un PR. `validacion.md` APTO. PBI a `docs/todos/done/` en la misma rama.
