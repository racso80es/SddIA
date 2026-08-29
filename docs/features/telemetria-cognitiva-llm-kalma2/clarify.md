---
feature_name: telemetria-cognitiva-llm-kalma2
created: "2026-08-29"
process: feature
purpose: Estabilización Mayeuta — telemetría cognitiva LLM y exposición Kalma2
branch_name: feat/telemetria-cognitiva-llm-kalma2
persist_ref: docs/features/telemetria-cognitiva-llm-kalma2
pbi_ref: docs/todos/pending/[ARQUITECTURA] Telemetría Cognitiva: Captura de métricas LLM y exposición en Kalma2.md
document_id: PBI-TELEMETRY-LLM-COGNITIVE-METRICS-KALMA2
uuid: a1535038-8db5-4351-8a81-cfa5586b8c5b
execution_id: "0cdb618e-51e4-461b-9d14-469a5363257b"
status: blueprint_locked
mayeuta_verdict: ok
---

# Clarificación — telemetria-cognitiva-llm-kalma2

Transcript Mayeuta (2026-08-29). Semilla PBI v1.2.0 (`refinement_status: refined`, DD-1…DD-4) → requisitos estables para Dedalo.

Fuentes: PBI; `SddIA/core/cumulo.paths.json`; `SddIA/events/telemetry/raw-execution-finished.md`; `SddIA/core/event-telemetry-subscriptions.json`; `thermodynamic.rs`; `mayeuta-llm`; `kalma2-bridge`; `radamanto-batch`.

---

## D0 — Apertura formal

| Pregunta | Decisión |
|----------|----------|
| Proceso | `feature` |
| `feature_name` | `telemetria-cognitiva-llm-kalma2` |
| Rama | `feat/telemetria-cognitiva-llm-kalma2` |
| `persist_ref` | `docs/features/telemetria-cognitiva-llm-kalma2` |
| `document_id` | `PBI-TELEMETRY-LLM-COGNITIVE-METRICS-KALMA2` |
| Fase | Estabilización de Requisitos (Mayeuta) + relevo Dedalo en este ciclo |
| Intención estable | Observabilidad de coste LLM (tokens, modelo, latencia) sin tercer nombre, sin hard fail, sin polling WUI, sin gobernanza Cerbero en el umbral de cuota ordinaria |

---

## D1 — Decisiones PBI (vinculantes, no reabrir)

| ID | Laudo | Efecto |
|----|-------|--------|
| **DD-1** | SSOT público = `telemetry_receipt` (`prompt_tokens`, `completion_tokens`, + `llm_model`, `tier`, `provider_latency_ms`) | Prohibido `cognitive_metrics`. `thermodynamic_cost` interno se mapea en Aduana |
| **DD-2** | Sin `usage` → ceros + `cognitive-degraded: true` | Observabilidad no tumba negocio |
| **DD-3** | Vivo = SSE Rust broadcast; pull = snapshot opcional | No polling de `stats.json` desde WUI inerte |
| **DD-4** | Cuota `max_tokens_per_minute` Nivel 1 = alerta visual; Nivel 2 = `Domain_Entity_Degraded` solo excesos críticos | Radamanto no “bloquea Cerbero” en el caso ordinario |

---

## D2 — Hechos SSOT (triage de inexactitudes residuales)

| ID | Afirmación | Hecho | Laudo Mayeuta |
|----|------------|-------|---------------|
| **I1** | Evento marca fan-out “Reservado Fase 3.C” | `event-telemetry-subscriptions.json` ya enruta `Raw_Execution_Finished` → `radamanto-batch` + `telemetry-compliance-audit` | Pipeline **vivo**. El comentario del evento está **fósil**. Dedalo actualiza el códice, no “inventa” suscripción |
| **I2** | El Peaje “intercepta” `telemetry_receipt` | `thermodynamic.rs` emite REF con `asset_id`/`exit_code`/`duration_ms`/`process_name`; **no** copia receipt ni `capsule_id` | Hueco real de Aduana. El mapeo DD-1 **aún no existe** |
| **I3** | Toda invocación LLM cruza el CLI | `POST /api/chat` → `kalma2-bridge` → `mayeuta-llm` **STREAM** (stdout línea a línea, **sin envelope**). No hay Peaje ni REF | Hueco de canal. El pulso visible en Kalma2 chat **no** entra hoy al bus. Dedalo debe laudar puente STREAM ≠ Aduana (ver D3) |
| **I4** | `mayeuta-llm` usa envelope `capsule-json-io` | Emite `{success, data, error}` (y STREAM sin JSON) | No exigir v2 en STREAM. Receipt en SYNTHESIZE/CLASSIFY va en el JSON de salida; STREAM usa canal lateral (Dedalo) |
| **I5** | `capsule-json-io.md` es SSOT del receipt | Norma = envelope genérico | Fuera de mutación salvo referencia cruzada (PBI) |
| **I6** | Umbral cognitivo = fila más en thresholds | Hoy solo calidad + `latency_ms_p95_threshold` | Clase **rate** nueva. Genoma `radamanto.thresholds.json` vía `entity-manager` |

---

## D3 — Dos caminos de captura (requisito estable)

| Camino | Entrada | ¿Cruza Peaje? | Qué exige Mayeuta |
|--------|---------|---------------|-------------------|
| **A — Aduana** | Cápsula vía `execute-process` (SYNTHESIZE/CLASSIFY, `thermodynamic_cost` de handlers como `email_triage`) | Sí, al cierre de proceso | Receipt en stdout/state → REF en `./.events/telemetry/` |
| **B — STREAM** | `POST /api/chat` / `mayeuta-llm` STREAM | **No** | No hard fail. No emitir ECST desde el bridge (emisores autorizados: CLI). Dedalo elige sidecar de instancia + ingesta Radamanto, **sin** tercer schema |

Prohibido: que el bridge escriba `Raw_Execution_Finished` en `eda_fractal.telemetry`.

---

## D4 — Alcance estabilizado

| Dentro | Fuera |
|--------|-------|
| Contrato REF + schema default + compliance de campos cognitivos | Mutar `capsule-json-io.md` como SSOT |
| Mapeo Aduana `thermodynamic_cost` → `telemetry_receipt` | Python / `sddia-client-bridge.py` |
| `mayeuta-llm` SYNTHESIZE/CLASSIFY + STREAM degradado | Hard fail por métrica ausente |
| Agregación Radamanto + umbral rate escalonado | Revocación Cerbero en Nivel 1 |
| SSE broadcast + snapshot pull + widget WUI | Sustituir `/api/progress/stream` o `/api/status` |
| Actualizar códice evento (fósil Fase 3.C) | Nuevo nombre `cognitive_metrics` |

---

## D5 — Criterios de aceptación (heredados PBI, precisados)

1. Evento + `DEFAULT_TELEMETRY_SCHEMA` documentan campos DD-1; no aparece `cognitive_metrics`.
2. CLI mapea `thermodynamic_cost` → `telemetry_receipt` en REF.
3. `mayeuta-llm` sin usage: ceros + `cognitive-degraded`; no exit≠0 por eso.
4. REF cognitivo llega a `./.events/telemetry/`.
5. `radamanto-batch` consolida sin romper `entities`/calidad/latencia.
6. Nivel 1 = alerta WUI; Nivel 2 = Degraded solo crítico.
7. `GET /api/telemetry/stream` (SSE broadcast) + `GET /api/telemetry/cognitive` opcional.
8. WUI pulso + alerta cuota; Despertador Inerte intacto.

---

## D6 — Abierto para Dedalo (cerrado en spec)

- Mecánica exacta STREAM (sidecar Cúmulo vs stderr parseado por bridge).
- Dónde acumula Radamanto (`stats.json` vs bloque hermano).
- Umbrales numéricos Nivel 1 / Nivel 2 (valores; no la semántica).
