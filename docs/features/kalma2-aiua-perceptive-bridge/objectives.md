---
feature_name: kalma2-aiua-perceptive-bridge
created: "2026-09-09"
process: feature
branch_name: feat/kalma2-aiua-perceptive-bridge
persist_ref: docs/features/kalma2-aiua-perceptive-bridge
pbi_ref: docs/todos/pending/[NÚCLEO] Puente Perceptivo: Interacción Biológica con Tormentosa desde Kalma2 WUI.md
execution_id: "f075b5ea-aac3-47cb-b5ba-a18c5431a7e5"
document_id: PBI-NUCLEO-PUENTE-PERCEPTIVO-KALMA2
pbi_uuid: "d7192a54-7389-4b68-b3d4-b91c0e35921a"
pbi_version: "1.2.0"
---

# Objetivos — kalma2-aiua-perceptive-bridge

## Misión

Tender el canal perceptivo atómico (estímulo → sobre → render) desde Kalma2 WUI hacia Tormentosa (`aiua-stimulus-processing`) sin acoplar el genoma del latido a la epidermis.

## Alcance

1. Botón `#aiua-pulse` + `enviarAiuaStimulus()` en `interfaces/kalma2/`. Estado cero. `setBusy` cubre el botón nuevo.
2. `POST /api/aiua/interact` en `kalma2-bridge`: spawn parametrizado `--process aiua-stimulus-processing`; parseo `OrchestratorEnvelope`; JSON WUI plano.
3. Timeout puente ≥ timeout Gemini (env heredada). Fractura solo colapso protésico (ELF/spawn).
4. Tests estáticos/unitarios del puente + `cargo check`. Smoke lab-mock documentado, no gate de CI.

## Fuera de gate

Mutar `aiua-stimulus-processing.md`, acciones, `gemini-http-infer`, `aiua_core.md`. Function calling. Poda LanceDB. Thinking HIGH / prefacio identidad. Unificar Mayeuta. SSE tokens Aiúa. Hijack `#cognitive-pulse`. README fósil Kalma2. Cerbero/Karma2Token en el puente.

## Ley aplicada

- PBI v1.2.0 Filtro A.
- `features-documentation-pattern` v1.2.1: un PR; `validacion.md` APTO solo con CI verde (`run_id`).
- DA-2: cero mutación de genoma (`process`/`actions`/`tools`).
- Git vía `git-manager`. DCC abre PR. `accept-pr` solo post-CI verde.

## Criterios

| ID | Criterio |
|----|----------|
| CA-1 | Cero Python/FastAPI. Ruta nueva solo en `kalma2-bridge`. `cargo check` verde. |
| CA-2 | Cada envío `{prompt}` autocontenido. Cero `localStorage` / historial Aiúa. |
| CA-3 | `setBusy` deshabilita `#chat`, `#forge`, `#sync-genome`, `#aiua-pulse`. |
| CA-4 | `POST /api/aiua/interact` spawnea `aiua-stimulus-processing`. Parsea envelope (`data` + `exitCode`). JSON WUI plano. Nunca el helper cableado a `kalma2-interact`. |
| CA-5 | Lab-mock: `response` no vacío (prefijo `lab-mock:` basta). Live opcional. |
| CA-6 | `thought_id` 64 hex + telemetría en `#status` y/o `#progress-console` vía PTC. Pulso agregado intacto. |
| CA-7 | Mayeuta SSE, forge, sync, `Ctrl+Enter`, inbox, espejo salud: intactos. |
| CA-8 | Timeout puente ≥ `SDDIA_GEMINI_HTTP_TIMEOUT_SECS`. Cero polling post-acuse. |
| CA-9 | Cero mención Kalma2 en proceso/acciones/`thought-graph-access`. |
| CA-10 | Un PR. `validacion.md` APTO, `pbi_archived: true`. PBI en `docs/todos/done/`. |
| CA-CI | Checks GitHub del PR verdes (`run_id`). |
