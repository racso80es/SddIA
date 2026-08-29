---
document_id: PBI-TELEMETRY-LLM-COGNITIVE-METRICS-KALMA2
title: "[ARQUITECTURA] Telemetría Cognitiva: Captura de métricas LLM y exposición en Kalma2"
format: markdown
version: "1.2.0"
created: "2026-08-29"
refined: "2026-08-29"
clarified: "2026-08-29"
status: pending
refinement_status: refined
priority: alta
process: feature
executor_vehicle: feature
type: architecture
dispatch: false
related:
  - SddIA/events/telemetry/raw-execution-finished.md
  - SddIA/process/radamanto-batch.md
  - SddIA/agents/radamanto.thresholds.json
  - SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs
  - SddIA/engine/execute-process/src/engine/telemetry_compliance_core.rs
  - SddIA/engine/execute-process/src/engine/fractal_bus.rs
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - SddIA/skills/mayeuta-llm/src/main.rs
  - interfaces/kalma2/app.js
  - interfaces/kalma2/index.html
  - SddIA/core/cumulo.paths.json
---

# [ARQUITECTURA] Telemetría Cognitiva: Captura de métricas LLM y exposición en Kalma2

## Mandato
Dotar al ecosistema SddIA de observabilidad sobre el consumo de recursos cognitivos externos (LLMs): extraer métricas de uso (tokens, latencia, modelo) desde el punto de ejecución (cápsulas), propagarlas por la Aduana Universal (CLI, Peaje Termodinámico) y el bus fractal de telemetría, consolidarlas en Radamanto y exponerlas por el Puente Físico (`kalma2-bridge`, Rust) para su visualización táctica en Kalma2 (WUI).

## 0. Rectificación de Inexactitudes (Triaje Antientrópico previo)
La v1.0.0 de este PBI contenía código fósil/alucinaciones detectadas al validar contra `cumulo.paths.json` y el genoma real. Corregidas aquí:

1. **SSOT del contrato de telemetría mal atribuido.** `telemetry_receipt` **no** está definido en `SddIA/norms/capsule-json-io.md` (esa norma describe el envelope genérico `{meta, success, exitCode, result, …}`). El SSOT real de `telemetry_receipt` es:
   - Evento: `SddIA/events/telemetry/raw-execution-finished.md` → ya declara `telemetry_receipt` como campo **OPTIONAL** con `prompt_tokens`, `completion_tokens`, …
   - Esquema por defecto: `DEFAULT_TELEMETRY_SCHEMA = ["prompt_tokens", "completion_tokens"]` en `fractal_bus.rs`.
   - Validación: `telemetry_compliance_core.rs` (audita presencia/forma del `telemetry_receipt`).
2. **Puente Físico Python inexistente.** `.SddIA/client/sddia-client-bridge.py` **no existe**; los clientes Python fueron podados (feature `poda-python-rust-clientes`). El Puente Físico real es el binario **Rust** `SddIA/interfaces/kalma2-bridge/src/main.rs` (`kalma2-bridge`), sirviendo HTTP en `127.0.0.1:8765` con SSE ya operativo (`POST /api/chat`, `GET /api/progress/stream`). Proponer un backend Python viola el agnosticismo del Core (.cursorrules §4/§5) y la preferencia Rust para cápsulas.
3. **Nomenclatura de coste divergente.** En el stdout de las cápsulas la convención viva es `thermodynamic_cost { tokens_in, tokens_out, duration_ms }` (ver `email_triage.rs`), mientras el evento usa `telemetry_receipt { prompt_tokens, completion_tokens }`. Introducir un tercer nombre (`cognitive_metrics`) sin reconciliar es entropía. Este PBI debe **decidir un SSOT único** (ver §3).
4. **Rol de Radamanto mal descrito.** La consolidación no es una "suscripción" difusa: la ejecuta el proceso `radamanto-batch` (v1.1.1) → `radamanto_batch_core.rs`, que ya consume `Raw_Execution_Finished`, acumula en `.SddIA/radamanto/` (`stats.json`, `consumed.json` per `cumulo.paths.json`) y emite `Domain_Entity_{Degraded|Restored|Deprecated}` + `Domain_Entity_Telemetry_Captured`.
5. **Cadena de bloqueo Cerbero imprecisa.** Radamanto **no bloquea**; emite gobernanza (`Domain_Entity_Degraded/Deprecated`) que puede derivar en revocación en `.SddIA/cerbero/revoked_entities.json`, aplicada por Cerbero. La degradación es indirecta, vía bus.

## 1. Contexto Arquitectónico (La Fricción Actual)
El CLI emite `Raw_Execution_Finished` con Peaje Termodinámico (`asset_id`, `exit_code`, `duration_ms`, `process_name`) y admite ya un `telemetry_receipt` OPTIONAL con `prompt_tokens`/`completion_tokens`. Sin embargo:
- La única cápsula LLM viva, `mayeuta-llm` (`SddIA/skills/mayeuta-llm/src/main.rs`), **no emite hoy** tokens, modelo ni latencia: su salida es `{success, data, error}` y actúa como transductor a una prótesis CLI inyectada. La captura de tokens depende de que esa prótesis exponga `usage`; un transductor genérico puede no hacerlo (riesgo, ver §3).
- No hay agregación por modelo/tier ni umbrales de cuota cognitiva.
- Kalma2 no expone ningún panel de consumo cognitivo.
Resultado: el Vértice Biológico opera a ciegas respecto a coste de tokens, latencia de inferencia y modelo por skill/tool.

## 2. Superficie de Impacto y Línea de Montaje (A Refinar)

### Ola A1: Contrato y Emisión (Cápsulas y Aduana)
1. **Formalizar el esquema en el SSOT único `telemetry_receipt`** (DD-1): ampliar `SddIA/events/telemetry/raw-execution-finished.md` y `DEFAULT_TELEMETRY_SCHEMA` en `fractal_bus.rs` (+ validación en `telemetry_compliance_core.rs`) para que `telemetry_receipt` reutilice `prompt_tokens`/`completion_tokens` y sume `llm_model`, `tier`, `provider_latency_ms`. **No** tocar `capsule-json-io.md` salvo referencia cruzada. Se descarta `cognitive_metrics`.
2. **Inyección en la cápsula LLM:** modificar `mayeuta-llm` para extraer `usage`/modelo/latencia de la prótesis. Si la cápsula opera internamente con `thermodynamic_cost { tokens_in, tokens_out }`, **el CLI (Peaje Termodinámico) mapea** esos valores al esquema oficial del evento al cruzar la Aduana (DD-1).
3. **Tolerancia a fallos (DD-2):** si la prótesis omite `usage`, la cápsula inyecta valores a cero y activa `cognitive-degraded: true` (patrón `classification-degraded` de `email_triage.rs`). **Prohibido hard fail** por una métrica auxiliar de observabilidad: no debe colapsar la lógica de negocio.
4. **Propagación del CLI:** el Peaje Termodinámico traslada el sub-objeto al `telemetry_receipt` del `Raw_Execution_Finished` en `./.events/telemetry/`.

### Ola A2: Consolidación (El Actuario Radamanto)
1. **Agregación:** ampliar `radamanto_batch_core.rs` (proceso `radamanto-batch`) para acumular métricas cognitivas por modelo/tier/ventana en `.SddIA/radamanto/stats.json` (o `consumed.json`), sin romper los acumuladores actuales de calidad/latencia.
2. **Umbrales cognitivos escalonados (DD-4):** `radamanto.thresholds.json` v1.1.0 hoy sólo define calidad (`success_rate*`) y latencia (`latency_ms_p95_threshold`). `max_tokens_per_minute` es una **cuota temporal (rate)** nueva → bloque aparte con lógica de ventana deslizante. Escalado en dos niveles:
   - **Nivel 1 (defecto):** superar la cuota dispara **alerta visual** en Kalma2, sin gobernanza.
   - **Nivel 2 (excepcional):** sólo bucles anómalos o excesos críticos (estabilidad financiera/técnica) emiten `Domain_Entity_Degraded/Deprecated` (que Cerbero puede convertir en revocación en `.SddIA/cerbero/revoked_entities.json`).

### Ola A3: Consumo y Capa Material (Kalma2)
1. **Stream SSE en el Puente Físico Rust (DD-3):** añadir un endpoint SSE (ej. `GET /api/telemetry/stream`) en `SddIA/interfaces/kalma2-bridge/src/main.rs`, extendiendo la infraestructura SSE ya existente (`/api/progress/stream`, `/api/chat`) en vez de forzar polling de la interfaz inerte contra `.SddIA/radamanto/stats.json`. **Matiz técnico:** el SSE actual lee un directorio por `correlation_id`; la telemetría cognitiva es un **flujo broadcast** (no ligado a un ciclo), así que la fuente del stream debe ser el tail de `./.events/telemetry/` o el watch del consolidado de Radamanto, no un dir por correlación. Nada de Python.
2. **Fallback pull (opcional):** un `GET /api/telemetry/cognitive` (lectura puntual de `stats.json` vía `cumulo.paths.json`) puede coexistir para snapshot inicial/histórico; el vivo es SSE.
3. **Visualización en Kalma2:** widget/panel en `interfaces/kalma2/index.html` + `interfaces/kalma2/app.js` que consuma el stream y muestre el pulso cognitivo (tokens acumulados, último modelo, latencia media) + alerta de cuota (DD-4 Nivel 1), respetando la inercia de diseño (Despertador Inerte).

## 3. Decisiones de Diseño (Clarificaciones Resueltas)
- **DD-1 · SSOT de nomenclatura:** estándar único = `telemetry_receipt` del evento. Reutiliza `prompt_tokens`/`completion_tokens`, añade `llm_model`/`tier`/`provider_latency_ms`. `thermodynamic_cost` interno de cápsula se **mapea en el CLI** al cruzar la Aduana. Descartado `cognitive_metrics`. *(Verificado: `telemetry_receipt` es el SSOT real en `raw-execution-finished.md` + `fractal_bus.rs`.)*
- **DD-2 · Cápsula degradada:** sin `usage` → tokens a cero + `cognitive-degraded: true`. Nunca hard fail (observabilidad no bloquea negocio). *(Coherente con precedente `classification-degraded` en `email_triage.rs`.)*
- **DD-3 · Consumo Kalma2:** SSE sobre el bridge Rust (reutiliza infraestructura viva), no polling. Fuente broadcast (tail `./.events/telemetry/` o watch de Radamanto), no dir por `correlation_id`. *(Verificado: el bridge ya sirve SSE.)*
- **DD-4 · Umbrales y bloqueos:** cuota `max_tokens_per_minute` → Nivel 1 alerta visual; gobernanza (`Domain_Entity_Degraded` → Cerbero) reservada a bucles anómalos/excesos críticos.

## 3.1 Verificación Pendiente (no bloqueante para diseño)
- **Madurez del pipeline:** el evento marca instancias telemetry "(futuro)" y suscripción "Reservado Fase 3.C", pero `radamanto-batch` ya existe (Fase 4). Confirmar en implementación qué tramo está vivo end-to-end (emisión → routing → consolidación) antes de estimar esfuerzo.

## 4. Criterios de Aceptación
- [ ] **DD-1:** `raw-execution-finished.md` + `DEFAULT_TELEMETRY_SCHEMA` (`fractal_bus.rs`) documentan `telemetry_receipt` con `llm_model`/`tier`/`provider_latency_ms` sobre `prompt_tokens`/`completion_tokens`; `capsule-json-io.md` sólo referencia cruzada. No aparece `cognitive_metrics`.
- [ ] **DD-1:** el CLI mapea `thermodynamic_cost` interno de cápsula al `telemetry_receipt` oficial al cruzar la Aduana.
- [ ] **DD-2:** `mayeuta-llm` sin `usage` de la prótesis emite tokens a cero + `cognitive-degraded: true` y **no** falla duro; validado por `telemetry_compliance_core.rs`.
- [ ] `Raw_Execution_Finished` transporta el sub-objeto cognitivo hasta `./.events/telemetry/`.
- [ ] `radamanto-batch` (`radamanto_batch_core.rs`) consolida las métricas en `.SddIA/radamanto/stats.json` sin regresión de los acumuladores existentes.
- [ ] **DD-4:** superar `max_tokens_per_minute` (Nivel 1) genera alerta visual en Kalma2 sin gobernanza; sólo el exceso crítico (Nivel 2) emite `Domain_Entity_Degraded`.
- [ ] **DD-3:** el endpoint SSE `GET /api/telemetry/stream` del bridge **Rust** difunde telemetría en vivo (fuente broadcast, no por `correlation_id`); el `GET /api/telemetry/cognitive` (pull) queda como snapshot opcional.
- [ ] Kalma2 WUI renderiza tokens, latencia y alerta de cuota de forma comprensible sin romper su inercia de diseño (Despertador Inerte).
