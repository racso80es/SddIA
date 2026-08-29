---
document_id: PBI-TELEMETRY-LLM-COGNITIVE-METRICS-KALMA2
title: "[ARQUITECTURA] Telemetría Cognitiva: Captura de métricas LLM y exposición en Kalma2"
format: markdown
version: "1.0.0"
created: "2026-08-29"
status: pending
refinement_status: requires_clarification
priority: alta
process: feature
executor_vehicle: feature
type: architecture
dispatch: false
related:
  - SddIA/norms/capsule-json-io.md
  - SddIA/agents/radamanto.thresholds.json
  - .SddIA/client/sddia-client-bridge.py
  - interfaces/kalma2/index.html
---

# [ARQUITECTURA] Telemetría Cognitiva: Captura de métricas LLM y exposición en Kalma2

## Mandato
Dotar al ecosistema SddIA de observabilidad sobre el consumo de recursos cognitivos externos (LLMs). El objetivo es extraer métricas de uso (tokens, latencia, modelo) desde el punto de ejecución (cápsulas), propagarlas a través de la Aduana Universal (CLI) y el bus de telemetría, y consumirlas a través del Puente Físico para su visualización táctica en la interfaz Kalma2 (WUI).

## 1. Contexto Arquitectónico (La Fricción Actual)
Actualmente, el sistema SddIA orquesta ejecuciones a través del CLI y emite eventos `Raw_Execution_Finished`, pero carece de un desglose específico sobre el coste cognitivo cuando se invoca a un LLM. El Vértice Biológico opera a ciegas respecto al consumo de tokens, tiempos de inferencia y modelos utilizados por las distintas skills/tools, impidiendo la detección temprana de anomalías térmicas (ej. alucinaciones costosas o latencias inaceptables).

## 2. Superficie de Impacto y Línea de Montaje (A Refinar)

### Ola A1: Contrato y Emisión (Las Cápsulas y la Aduana)
1. **Actualización de `capsule-json-io.md`:** Ampliar el esquema opcional `telemetry_receipt` para aceptar un sub-objeto `cognitive_metrics` (campos: `llm_model`, `tier`, `prompt_tokens`, `completion_tokens`, `provider_latency_ms`).
2. **Inyección en Cápsulas:** Modificar las tools/skills que interactúan con LLMs para que extraigan estos metadatos de las respuestas de la API y los adjunten en su `stdout` JSON.
3. **Propagación del CLI:** El orquestador (Peaje Termodinámico) debe interceptar este sub-objeto y adjuntarlo explícitamente en el evento `Raw_Execution_Finished` emitido hacia `./.events/telemetry/`.

### Ola A2: Consolidación (El Actuario)
1. **Suscripción de Radamanto:** Ampliar el procesador de Radamanto para que agregue y persista estas métricas en un nuevo bloque dentro de `.SddIA/radamanto/stats.json` (ej. consumo acumulado por modelo/hora).
2. **Nuevos Umbrales:** Definir en `radamanto.thresholds.json` los límites térmicos cognitivos (ej. `max_tokens_per_minute`). (Punto a clarificar: ¿Queremos que Cerbero bloquee si nos pasamos de cuota térmica?).

### Ola A3: Consumo y Capa Material (Kalma2)
1. **Ampliación del Puente Físico (`sddia-client-bridge.py`):** Crear un endpoint ligero (ej. `GET /api/telemetry/cognitive`) que exponga un resumen de estas métricas leyendo de la fuente consolidada de Radamanto.
2. **Visualización en Kalma2:** Implementar un widget o panel simple en `index.html` / `app.js` que consuma este endpoint y muestre de un vistazo el pulso cognitivo del sistema (tokens consumidos, último modelo usado, latencia media).

## 3. Puntos de Clarificación Abiertos (Filtro Antientrópico)
- **Táctica de Persistencia:** ¿Kalma2 debe consultar el estado consolidado a Radamanto, o debe suscribirse a los eventos puros de telemetría mediante SSE (Server-Sent Events) en el backend de Python para ver el consumo "en vivo"?
- **Bloqueo vs. Advertencia:** Si el consumo supera los umbrales cognitivos, ¿Radamanto debe degradar la tool (provocando un bloqueo de Cerbero), o simplemente levantar un log visual de advertencia en Kalma2?

## 4. Criterios de Aceptación (Borrador)
- [ ] `capsule-json-io.md` documenta formalmente el esquema `cognitive_metrics`.
- [ ] Al menos una cápsula LLM emite telemetría cognitiva validada por la Aduana.
- [ ] El endpoint del servidor local Python expone las métricas al frontend sin latencia destructiva.
- [ ] Kalma2 WUI renderiza de forma visual y comprensible el gasto de tokens y latencia sin romper su inercia de diseño (Despertador Inerte).
