---
feature_name: kalma2-canal-telemetria-progreso
created: "2026-08-15"
process: feature
purpose: Estabilización Mayeuta — canal asíncrono de trazas de progreso Kalma2 (ontología y alcance)
branch_name: feat/kalma2-canal-telemetria-progreso
persist_ref: docs/features/kalma2-canal-telemetria-progreso
pbi_ref: docs/todos/pending/[OPERATIVO] PBI: Canal Asíncrono de Telemetría de Progreso y Observabilidad Activa para Interfaces Externas (Kalma2).md
document_id: PBI-KALMA2-CANAL-TELEMETRIA-PROGRESO
uuid: c8f4a2e1-9d3b-4f67-a1c5-8e2b6d09f4a7
status: blueprint_locked
mayeuta_verdict: ok
dedalo_verdict: ok
---

# Clarificación — kalma2-canal-telemetria-progreso

Transcript Mayeuta (2026-08-15). Semilla v0 «Por Refinar» (PBI OPERATIVO Kalma2) → requisitos estabilizados para handoff Dedalo. Dedalo (2026-08-15) cerró D7 en `spec.md` / `plan.md` (laudo `C-ephemeral-progress-leaf`).

Fuentes: PBI operativo; `SddIA/core/cumulo.paths.json`; `SddIA/events/events-contract.md` v1.1.0; códice `SddIA/events/telemetry/`; laudos `kalma2-event-bus-integration` / PBI-044; Kaizen adyacente `PBI-KAIZEN-PEC-SUBSCRIBERS-CIRCUIT-AUDIT` (no fusionar).

---

## D0 — Apertura formal

| Pregunta | Decisión |
|----------|----------|
| Proceso | `feature` |
| `feature_name` | `kalma2-canal-telemetria-progreso` |
| Rama | `feat/kalma2-canal-telemetria-progreso` |
| `persist_ref` | `docs/features/kalma2-canal-telemetria-progreso` |
| `document_id` | `PBI-KALMA2-CANAL-TELEMETRIA-PROGRESO` |
| Fase | Estabilización de Requisitos (Mayeuta) |
| Intención estable | Canal **fire-and-forget** de **trazas de progreso** para interfaces externas durante `execute-process`, sin acoplar el Core a clientes ni contaminar dominio/peaje |

---

## D1 — Triaje de incongruencias (I1–I8)

| ID | Afirmación PBI / semilla | Hecho SSOT / precedente | Laudo Mayeuta |
|----|--------------------------|-------------------------|---------------|
| **I1** | Escritura en `.SddIA/events/telemetry/` | `eda_fractal.telemetry` = `./.events/telemetry`; `.SddIA/events` = `eda_instance.customization` (Vía C), **no** bus | **Prohibido** documentar o emitir progreso bajo `.SddIA/events/`. El bus fractal vive en `./.events/…`. |
| **I2** | Schema en `.SddIA/library/norms/capability-contracts/telemetry.trace.schema.json` | `directories.capability_contracts` = `SddIA/library/norms/capability-contracts/`; Clases de telemetría de dominio = ECST en `SddIA/events/telemetry/` | **Rechazado** como capability-contract y como path bajo `.SddIA/`. Contrato de progreso ≠ contrato de capacidad. |
| **I3** | Envelope `{phase, severity, source_agent, message}` | ECST exige `event_type`, `emitter_agent`, `payload` (+ `event_id`, `timestamp`) | Envelope PBI **no es ECST**. Si Dedalo elige Clase ECST, el semántico de progreso va en `payload`; si elige canal efímero no-ECST, el schema propio **no** se hace pasar por ECST. |
| **I4** | Reutilizar familia `telemetry` / mismo directorio fractal | Familia actual = `Raw_Execution_Finished` + `Daemon_Heartbeat`; emisores CLI; consumidores `radamanto-batch` + `telemetry-compliance-audit` (+ heartbeat audit) | **Colisión ontológica.** Progreso UI ≠ peaje termodinámico / compliance. **Prohibido** mezclar trazas de progreso con la familia `telemetry` existente ni con su fan-out. |
| **I5** | «Sustituir» polling `GET /api/status` por SSE de trazas | `kalma2-event-bus-integration` / PBI-044: `status` = proyección de correlación/terminal; SSE/WS fuera de alcance v1 de ese lazo | **Overreach rechazado.** `GET /api/status` = **veredicto terminal**; trazas = **progreso intermedio**. Canales **complementarios**, no sustitutivos. |
| **I6** | SSE genérico en bridge = bus de progreso | `interfaces/kalma2` ya usa `text/event-stream` para **chat LLM** | No reutilizar el SSE de chat como bus de progreso. Endpoint/flujo de progreso, si existe, es **capacidad distinta**. |
| **I7** | Latencia WUI &lt;100 ms como invariante | Criterio de experiencia de interfaz | Aceptable como **AC de interfaz Kalma2**; **no** invariante del Core ni del peaje. Fallo de latencia UI ≠ fallo de `execute-process`. |
| **I8** | Sintoma 404 `GET /api/status` post-purge PEC | Kaizen `PBI-KAIZEN-PEC-SUBSCRIBERS-CIRCUIT-AUDIT` (suscriptores PEC / circuito EDA) | **Adyacente, no fusionar.** Este feature no liquida suscriptores PEC ni el 404 de status. |

---

## D2 — Laudo de canal (ontología)

Tres opciones triadas:

| Opción | Descripción | Veredicto |
|--------|-------------|-----------|
| A — Reusar `eda_fractal.telemetry` + familia `telemetry` | Misma hoja FS y códice peaje/heartbeat | **Rechazada** (I4) |
| B — Clase ECST nueva en familia `telemetry` (u otra familia existente de peaje) | Sigue el fan-out / semántica de peaje | **Rechazada** mientras el consumidor sea Radamanto/compliance |
| C — **Canal efímero distinto** | Señal volátil fire-and-forget de progreso UI/orquestación visible; sin anclaje DLT; sin consenso de dominio; sin peaje | **Adoptada (requisito estable)** |

**Definición operativa del canal (qué / por qué, no el cómo Dedalo):**

1. **Qué:** cápsulas de traza de progreso ligadas a una ejecución (`process_id` / `correlation_id` según laudo Dedalo), emitidas en chispazos de fase, descartables sin reintento bloqueante.
2. **Por qué separado:** Ceguera espacial del orquestador (no conoce WUI/bridge); integridad del bus de dominio; pureza del peaje termodinámico.
3. **Dónde (restricciones, no forja):** Dedalo propone leaf/ruta vía topología Cúmulo; Mayeuta fija **límites negativos**: no `.SddIA/events/`; no contaminar `eda_fractal.domain`; no reutilizar consumidores de familia `telemetry` peaje. Si hace falta hoja nueva en `eda_fractal` o familia ECST nueva (p. ej. `progress`), eso es **mutación de topología/genoma** fuera de jurisdicción Mayeuta — proceso/entidad-manager posteriores.
4. **Higiene:** sweeper/poda de huérfanos sigue siendo requisito; no acumular residuos post-cierre de ejecución.

---

## D3 — Alcance estabilizado (dentro / fuera)

| Dentro | Fuera |
|--------|-------|
| Contrato semántico de traza de progreso (campos de intención: fase, severidad, agente emisor lógico, mensaje, metadatos) bajo ontología del canal C | Convertir progreso en evento de dominio o peaje |
| Emisión fire-and-forget desde orquestación de fases sin bloquear ni reintentar por fallo de receptor | Sustituir `GET /api/status` / liquidar Kaizen PEC |
| Difusión hacia interfaces externas (bridge como **lector/proyector**, no emisor de negocio) | Reutilizar SSE de chat LLM como bus de progreso |
| Consola WUI cromática consumiendo el canal de progreso **además** del sondeo/veredicto terminal | Latencia &lt;100 ms como gate del Core |
| Circuito de poda de trazas huérfanas/cerradas | Autenticación, TLS, historial conversacional durable |

---

## D4 — Criterios de aceptación estabilizados (entrada Dedalo)

| ID | Criterio | Notas |
|----|----------|-------|
| **AC1** | Desacople absoluto: bridge caído/apagado ⇒ `execute-process` completa sin error ni reintento bloqueante por telemetría de progreso | Innegociable (Ceguera / fire-and-forget) |
| **AC2** | Integridad: ninguna traza de progreso escribe o exige consenso en `eda_fractal.domain` ni anclaje DLT | I1/I4 |
| **AC3** | Ontología: trazas de progreso no entran al fan-out peaje (`route-telemetry` → Radamanto/compliance) | I4 |
| **AC4** | Dual-canal UI: veredicto terminal permanece en proyección tipo `GET /api/status`; progreso es canal adicional | I5 |
| **AC5** | Latencia de refresco WUI &lt;100 ms = AC de interfaz, no invariante Core | I7 |
| **AC6** | Higiene: sin acumulación residual de trazas tras poda alineada a ejecuciones cerradas/expiradas | Sweeper |
| **AC7** | Compilación/higiene Rust del motor tocado: sin panics/warnings en el perímetro del cambio | Grade S+ PBI, acotado a touchpoints Dedalo |

---

## D5 — Campos semánticos (intención; forma exacta = Dedalo)

Intención de payload (nombres provisionales; Dedalo homologa a ECST o schema efímero):

| Concepto | Rol |
|----------|-----|
| Identidad de traza | UUID de la cápsula |
| Identidad de ejecución | `process_id` y/o `correlation_id` alineado al lazo Kalma2 existente |
| Tiempo | ISO-8601 UTC |
| Fase | Spec \| Clarify \| Plan \| Implementation \| Validation \| Closure (u homologación al ciclo `feature`) |
| Severidad | info \| warn \| error \| kaizen_alert |
| Agente lógico | cerbero \| mayeuta \| dedalo \| tekton \| argos \| cumulo (u otros indexados) |
| Mensaje | Texto legible del hito |
| Metadatos | Objeto extensible opcional |

Si canal = ECST nuevo: raíz ECST canónica; conceptos anteriores **dentro de `payload`**; `emitter_agent` ≠ confundir con badge UI `source_agent` salvo homologación explícita.

---

## D6 — Dependencias y no-objetivos explícitos

| Ítem | Relación |
|------|----------|
| `kalma2-event-bus-integration` / PBI-044 | Precedente: poll status = veredicto; no invalidar |
| Kaizen PEC suscriptores | Paralelo; **no** absorbe este feature |
| Familia `telemetry` peaje | Contigua; **aislamiento obligatorio** |

---

## D7 — Veredicto Mayeuta

**ok** — alcance y ontología estabilizados; incongruencias I1–I8 laudadadas; handoff a Dedalo con canal **efímero distinto** (opción C) y dualidad status/progreso.

Abiertos para Dedalo (no bloquean el *qué*): leaf exacto en topología Cúmulo; si Clase ECST nueva vs schema no-ECST; nombre de `event_type` / endpoint bridge; mecanismo FS-watch concreto.

### Cierre Dedalo (2026-08-15)

| Abierto | Resolución |
|---------|------------|
| Leaf | `eda_fractal.progress` → `./.events/progress` |
| Forma | PTC no-ECST (`progress-trace-contract.md` bajo `library_norms`) |
| Endpoint | `GET /api/progress/stream?correlation_id=` |
| FS | Replay + watch por subdir `correlation_id` |
