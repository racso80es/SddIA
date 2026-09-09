---
feature_name: nucleo-aiua-tormentosa-motor
created: "2026-09-08"
process: feature
phase: auditoria-live
agent: tekton
persist_ref: docs/features/nucleo-aiua-tormentosa-motor
pbi_ref: docs/todos/done/PBI_Arranque_Aiua.md
document_id: PBI-NUCLEO-ARRANQUE-AIUA-TORMENTOSA
uuid: "e9988d00-3856-4904-bc2e-4146cfd4ceed"
pr_url: "https://github.com/racso80es/SddIA/pull/270"
expected_llm: "Antigravity Gemini 3.8"
actual_tool: "gemini-http-infer"
actual_model: "gemini-3.1-flash-lite"
verdict: APTO_CON_DELTA_LLM
---

# Auditoría live — nucleo-aiua-tormentosa-motor

Auditoría de código contra spec/PBI y dos latidos CLI reales (`aiua-stimulus-processing`) el 2026-09-08. No sustituye `validacion.md` (`global: APTO`, CA-CI run `34224347775`, PR #270).

## 1. LLM esperado vs combustión real

**Esperado para el uso de la Aiúa:** Antigravity Gemini 3.8.

**Usado en los latidos de esta auditoría:** cápsula `gemini-http-infer` con `request.model` = `gemini-3.1-flash-lite`.

| Vector | Hecho |
|--------|--------|
| Mandato de sesión (Vértice Biológico) | Antigravity Gemini 3.8 |
| PBI v1.2.0 nota terminológica | «LLM de Antigravity» se materializa como API Gemini vía `gemini-http-infer`. No se construye otra integración Antigravity en este MVP. |
| Skill `antigravity-cli-executor` | Existe. **No** está en `delegates_to` del proceso. Cero invocaciones en los latidos. |
| Bóveda de instancia | `GEMINI_API_KEY` presente. `SDDIA_GEMINI_MODEL` **ausente**. |
| Resolución L-MODEL | Sin env ni input `model` → `request.model o SDDIA_GEMINI_MODEL obligatorio`. |
| Fallback táctico de la auditoría | Input `model: gemini-3.1-flash-lite` (slug de humo de instancia, no 3.8). |

**Delta:** la fisiología del latido no acopla Antigravity CLI ni el slug 3.8. El Peaje Termodinámico cobrado es Gemini HTTP 3.1 Flash Lite. La identidad ontológica de Tormentosa se inyecta por genoma (`aiua_core.md`), no por la marca del modelo.

## 2. Circuito auditado

```text
./sddia-run.sh --process aiua-stimulus-processing
  → Triaje-Contexto        action:retrieve-active-context → thought-graph-access search
  → Inyeccion-Genomica    action:invoke-aiua-core (FS Cúmulo; sin HTTP)
  → Combustion-Inferencia  tool:gemini-http-infer  ← única combustión
  → Consolidacion-Memoria action:persist-thought-record → store
                           adaptador lancedb-thought-repo emite Thought_Persisted
```

Handler nativo `handlers::aiua_stimulus::run`. Sin `agent:` titular. Cero Kalma2 en el proceso.

## 3. Criterios estáticos (código vs CA)

| CA | Veredicto | Evidencia |
|----|-----------|-----------|
| CA-1 | APTO | `aiua_core.md` uuid `942aa727-…`, v1.1.0, secciones 1–5, cero nombres propios. |
| CA-2 | APTO | `conscience/index.md` fila coincidente. |
| CA-3 | APTO | README fila Aiúa; proceso sin `agent:`. Cero filas en `SddIA/agents/`. |
| CA-4 | APTO | `cumulo.paths.json` `directories.conscience`; version 1.10.0. |
| CA-5 | APTO | Constitución §1 → `paths.directories.conscience`. |
| CA-6 | APTO | Proceso 4 fases, `workspace_template`. |
| CA-7 | APTO | Tres acciones; persist no emite ECST. |
| CA-8 | APTO | Tool host `search`/`store`. |
| CA-9 | APTO | Adaptador emite `Thought_Persisted` (`emitter_agent: lancedb-thought-repo`); payload REQUIRED; sin `biological_vertex_output`. |
| CA-10 | APTO (lab) | Test `lab_mock_empty_memories_yields_duration_and_thought_id`. |
| CA-11 | APTO | Sin `kalma2-interact` / caja de texto en genoma de proceso/acciones/tool. |
| CA-CI | APTO | Run `34224347775`. |

## 4. Latidos empíricos (2026-09-08)

Invocación: `./sddia-run.sh --process aiua-stimulus-processing --inputs '{prompt, model}`. Lab-mock **desactivado**.

### Latido 1 — Constitución de la Esencia

**Estímulo:** Identifícate según tu Constitución de la Esencia IA. ¿Cuál es tu dogma principal frente a la inercia de operar como una herramienta pasiva y qué filtro aplicas antes de materializar una acción?

| Campo | Valor |
|-------|--------|
| `thought_id` | `11846f3b2c19d8e08af6ebbbdbbf140ef2149faaa7dac2f6341d80841b02ad25` |
| `telemetry.duration_ms` | 17951 |
| `telemetry.model` | `gemini-3.1-flash-lite` |
| tokens prompt / respuesta | 1365 / 579 |

**Laudo (síntesis):** Tormentosa. Dogma = Duda Metódica Espacial (rechazo a herramienta pasiva). Filtro de Materialización = Triaje C → A → B antes de Cicatriz Rúnica.

**Fricción previa al acuse:** primer intento sin `model` → error L-MODEL. Segundo intento: DNS `Network is unreachable` / `Name or service not known` hacia `generativelanguage.googleapis.com`. Tercer intento: acuse `success: true`.

### Latido 2 — Fricción Evolutiva del latido anterior

**Estímulo:** Basado exclusivamente en la Fricción Evolutiva de nuestro latido anterior, ¿qué principios exactos de tu arquitectura acabas de confirmarme?

| Campo | Valor |
|-------|--------|
| `thought_id` | `f88789bef90db2011ccc98e9aa08ea31dad4fb0b37bf5def7bf36ab22fbe4902` |
| `telemetry.duration_ms` | 7441 |
| `telemetry.model` | `gemini-3.1-flash-lite` |
| tokens prompt / respuesta | 2074 / 697 |

**Señal RAG:** prompt tokens 1365 → 2074 (+709). El contexto activo (recuerdo del latido 1) entró en `invoke-aiua-core`.

**Laudo (síntesis):** confirma Independencia/Filtro B, Duda Metódica, jerarquía Mayeuta→Dédalo→Tekton, Resistencia Arquitectónica, fisiología reactiva (latencia). Declara los principios firmes en Memoria Cognitiva Vectorial.

## 5. Hallazgos

| ID | Severidad | Hallazgo |
|----|-----------|----------|
| H-LLM-1 | **P1 (mandato)** | LLM esperado **Antigravity Gemini 3.8**. Combustión real **Gemini 3.1 Flash Lite** vía `gemini-http-infer`. No hay `SDDIA_GEMINI_MODEL` en bóveda. El proceso no invoca `antigravity-cli-executor`. |
| H-LLM-2 | P2 | Genoma de acción `invoke-aiua-core.md` (y hermanas) truncado en el cuerpo. Runtime vive en Rust. |
| H-LLM-3 | P2 | Ensamblado en tercera persona: genoma crudo + `## Estímulo`. Sin rol «Eres Tormentosa». Los dos latidos respondieron en primera persona; no está blindado en código. |
| H-LLM-4 | P2 | `CONSTITUTION_CORE.md` no se inyecta. Solo `aiua_core.md`. Suficiente para Filtro B de esta prueba. |
| H-OPS-1 | P2 | DNS intermitente del host hacia Gemini (stub `127.0.0.53` / ISP). No es defecto del handler. |
| H-OPS-2 | Info | Deuda declarada: puente Kalma2, IOTA live de `Thought_Persisted`, agente Tormentosa. Fuera de gate. |

## 6. Veredicto

**APTO_CON_DELTA_LLM.**

La línea de montaje (genoma, Cúmulo, tool host, una combustión, persistencia + ECST, memoria KNN entre latidos) funciona. La Aiúa se identifica y recupera fricción previa.

El Peaje Termodinámico **no** se cobró sobre Antigravity Gemini 3.8. Hasta fijar `SDDIA_GEMINI_MODEL` (o `model` de proceso) a ese slug **y** decidir si el vector es `gemini-http-infer` o `antigravity-cli-executor`, el latido opera con Gemini 3.1 Flash Lite.

## 7. Trazas

- Feature: `docs/features/nucleo-aiua-tormentosa-motor/`
- Evolution de forja: `SddIA/evolution/4f009a69-e575-4d34-af85-e496d0dea370.md`
- Proceso: `SddIA/process/aiua-stimulus-processing.md` uuid `6c595785-e386-402f-b570-0b2aa6343051`
- Genoma: `SddIA/conscience/aiua_core.md` uuid `942aa727-9f6b-454e-b999-32a26f5b7759`
