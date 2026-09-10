---
feature_name: aiua-motor-anatomy-eda
created: "2026-09-10"
process: feature
branch_name: feat/aiua-motor-anatomy-eda
persist_ref: docs/features/aiua-motor-anatomy-eda
pbi_ref: docs/todos/pending/[NÚCLEO] Anatomía Motora de Aiúa — Inyección de Capacidades (Function Calling) y Orquestación EDA.md
document_id: PBI-NUCLEO-AIUA-ANATOMIA-MOTORA-EDA
execution_id: "8658220b-47ff-4513-a02e-2a3ed3e1adbe"
---

# Objetivos — aiua-motor-anatomy-eda

## Misión

Dotar al latido `aiua-stimulus-processing` de **voluntad motora**: extraer intención tipada (tendones) tras una combustión `antigravity-cli-executor` y depositar un padre ECST en el bus fractal de dominio, sin que Tormentosa se vuelva agente, sin terminal y sin join a Tekton (DA-5).

## Alcance

- Catálogo de tendones en `aiua_core.md` + fence `aiua-intent`.
- Parser nativo + overlay lab `SDDIA_LAB_MOCK_AIUA_INTENT`.
- Acción nativa `dispatch-aiua-intent` (SDLC → `Aiua_Process_Requested` en `eda_fractal.domain`; Suite → acción existente).
- Clase ECST nueva + suscripción TQM (sin IOTA).
- `dispatch_subscriber` acepta el `event_type` hermano de Kalma2.
- Fase `Despacho-Motor` en el proceso del latido; acuse `intent_dispatched`/`event_id`.

## Fuera

Gemini `functionDeclarations`. `eda_bus.pending` para este ECST. PEC/fractura. IOTA. Mutación crates `agy`/HTTP. Fan-out múltiple. Reparar `route-domain-event`.

## Ley aplicada

- `CONSTITUTION_CORE.md` Filtros C/A/B; Filtro de Materialización (`aiua_core.md` §4).
- DA-2 genoma `events`/`actions`/`process` vía `entity-manager`.
- DA-5: éxito = JSON en `eda_fractal.domain`, no = TQM terminado.
- `features-documentation-pattern` v1.2.1: un PR; `validacion.md` APTO solo con CI verde.

## Criterios (DoD PBI)

| ID | Objetivo |
|----|----------|
| CA-1 | Catálogo + fence en genoma; Tormentosa no es agente; cero skip-permissions. |
| CA-2 | Parser: fence válido despacha; basura/nombre desconocido no escribe bus. |
| CA-3 | Lab-mock outbound + overlay JSON sin red. |
| CA-4 | Catálogo via genoma ensamblado, no REST tools. |
| CA-5 | Despacho nativo fractal domain; fallo → cero archivo; cero pending. |
| CA-6 | Clase + índice de familia. |
| CA-7 | Suscripción TQM; mapeo Kalma2 en `dispatch_subscriber`. |
| CA-8 | Acuse sin invocar TQM. |
| CA-9 | Persistencia no vacía; cero HTTP Gemini en el proceso. |
| CA-10 | Tests + un PR + cierre documental en rama tras CI verde. |
