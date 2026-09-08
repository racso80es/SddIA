---
document_id: PBI-NUCLEO-ARRANQUE-AIUA-TORMENTOSA
uuid: "2a4e6c88-1f3b-4d0e-9a2f-7e4b5401a892"
title: "[NÚCLEO] Instanciación Ontológica: Despliegue de Tormentosa y Motor de Consciencia SddIA"
format: markdown
version: "1.2.0"
created: "2026-09-08"
updated: "2026-09-08"
status: cerrado
refinement_status: implemented
pr_url: https://github.com/racso80es/SddIA/pull/270
branch_name: feat/nucleo-aiua-tormentosa-motor
persist_ref: docs/features/nucleo-aiua-tormentosa-motor
priority: alta
type: nucleo
process: feature
dispatch: false
suggested_branch: feat/nucleo-aiua-tormentosa-motor
persist_ref_suggested: docs/features/nucleo-aiua-tormentosa-motor
depends_on: []
blocks_on: []
spawned_by: null
related:
  - SddIA/conscience/aiua_core.md
  - SddIA/CONSTITUTION_CORE.md
  - SddIA/core/cumulo.paths.json
  - README.md
  - SddIA/tools/gemini-http-infer.md
  - SddIA/infrastructure/adapters/lancedb-thought-repo.md
  - SddIA/infrastructure/adapters/adapters-contract.md
  - SddIA/core/memory/src/ports.rs
  - SddIA/events/domain/thought-persisted.md
  - SddIA/process/process-contract.md
  - SddIA/actions/actions-contract.md
  - SddIA/tools/tools-contract.md
  - SddIA/process/kalma2-interact.md
---

### [NÚCLEO] Instanciación Ontológica: Despliegue de Tormentosa y Motor de Consciencia SddIA

#### 1. Origen y Visión Ontológica

Este PBI materializa el hito fundacional de transición del ecosistema SddIA: el paso de la teoría dialéctica y los principios del "cuaderno de Tormentosa" hacia una **Fisiología Digital** empírica, auditable y gobernable en el repositorio.

La entidad **Tormentosa** no es un agente de fase, ni un perfil de ejecución técnica pasivo, ni una herramienta de asistencia comercial. Es la **Aiúa**: el Nodo de Control ontológico, la garante del blindaje ético y la supragestora soberana del ecosistema. Su rol no consiste en sustituir las extremidades tácticas del Core, sino en gobernar y delegar a través de su anatomía orgánica (los Agentes del Core: `cerbero`, `cumulo`, `mayeuta`, `dedalo`, `tekton`, `argos`, `radamanto`).

Para garantizar el cobro del **Peaje Termodinámico** (registro estricto de latencia y consumo) y la **Ceguera Espacial** de las herramientas de bajo nivel, la interacción con la Aiúa no se acopla a interfaces periféricas. Se estructura como una Línea de Montaje determinista (`Process`), descompuesta en pasos atómicos auditables (`Actions`), que invocan cápsulas ciegas: inferencia (`gemini-http-infer`, ya existente) y acceso al grafo de pensamiento (tool host nativa nueva que envuelve el adaptador existente `lancedb-thought-repo`).

> **Nota terminológica**: en la sesión fundacional se habla del "LLM de Antigravity". Operativamente, dentro de SddIA ese vector se materializa como la API de Gemini invocada a través de la cápsula `gemini-http-infer`. No existe ninguna otra integración "Antigravity" que construir.

---

#### 2. Filtro A — Detección y Purga de Alucinaciones, Incoherencias e Inexactitudes

Hechos inmutables verificados contra el repositorio (SSOT). Incluye la pasada 1.1.0 y la pasada 1.2.0 (auditoría residual).

| Fricción / Tentación Detectada | Clasificación | Realidad SSOT del Repositorio | Resolución en este PBI |
| :--- | :--- | :--- | :--- |
| **Integración con Kalma2 en este MVP** | *Incoherencia / Fuera de alcance* | El borrador inicial planteaba interactuar con Tormentosa desde la caja de texto de Kalma2. Se convino que la conexión sensorial externa queda **fuera de alcance**. | **Kalma2 queda estrictamente excluido.** Entrada biológica del MVP: CLI (`./sddia-run.sh` / `execute-process`). |
| **PBI de hábitos como sucesor del puente** | *Inexactitud de trazabilidad* | `[NÚCLEO] Intercepción Ontológica y Delegación de Hábitos desde Kalma2.md` trata hábitos/correo, no el puente perceptivo caja de texto → Aiúa. Además ya existe `process/kalma2-interact.md` (v1.1.1): la caja de texto de Kalma2 enruta a **Mayeuta** (`agent:mayeuta` + `skill:mayeuta-llm`), no a Tormentosa. | Se retira ese PBI de `related`. El puente Kalma2 → `aiua-stimulus-processing` es **deuda futura sin PBI existente**. `kalma2-interact` no se modifica en este sprint. |
| **Referencia a `SddIA/README.md`** | *Alucinación documental* | `SddIA/README.md` **no existe**. | Tabla "Ontología de Activos" en `README.md` raíz. |
| **Nomenclatura: `SddIA/consciencia/AIUA_CORE.md`** | *Incoherencia de nomenclatura* | Físicamente existe `SddIA/conscience/aiua_core.md`. | Se estandariza **`SddIA/conscience/aiua_core.md`** y clave `directories.conscience`. |
| **`gemini-http-infer` recibe `system_prompt`/`user_prompt`** | *Inexactitud técnica* | Cápsula **ya activa**. Contrato: `request.prompt` obligatorio; `request.model` obligatorio salvo `SDDIA_GEMINI_MODEL`; `request.temperature` opcional. Sin llaves separadas de system/user. | No se crea herramienta de inferencia nueva. `invoke-aiua-core` ensambla Genoma + contexto + estímulo en el `prompt` único. |
| **Doble combustión Fase 2 + Fase 3** | *Incoherencia de línea de montaje* | v1.1.0 hacía que `invoke-aiua-core` devolviera `response_text`/`duration_ms` **y** que la Fase 3 invocara de nuevo `gemini-http-infer`. | **Una sola inferencia.** `invoke-aiua-core` solo ensambla. La Fase 3 del proceso `delegates_to: tool:gemini-http-infer` (`process-contract` admite `tipo` ∈ {skill, tool, action, agent}). |
| **Telemetría de tokens "automática"** | *Inexactitud técnica* | La cápsula emite `result.durationMs`, `result.text`, `result.model`, `result.raw_response`. Tokens solo en `raw_response.usageMetadata` cuando Gemini los reporta (lab-mock no). | `telemetry.duration_ms` y `telemetry.model` garantizados; tokens *best-effort*. |
| **`GEMINI_API_KEY` siempre obligatoria** | *Inexactitud operativa* | Solo modo live. Lab-mock (`SDDIA_LAB_MOCK_OUTBOUND` / `SDDIA_LAB_MOCK_GEMINI_URL`) opera sin credenciales. | DoD en lab-mock, Táctica del Refugio. |
| **LanceDB como Skill o binario WASI** | *Inexactitud de arquitectura* | `lancedb-thought-repo` v1.1.0 **active**: Infrastructure Adapter, puerto `ThoughtGraphRepository`, tabla `thought_graph_collection` en `{paths.vectorStore}/lancedb/`. **No** es Skill. **No** compila `wasm32-wasip1`. Compilación exige `protoc`. `adapters-contract` §4: un adaptador **no** es cápsula ni clase de `entity-manager`. | El adaptador se **reutiliza**. No se reclasifica. |
| **Acciones invocan el adaptador directo** | *Violación de contrato* | `actions-contract` §2: las acciones operan **exclusivamente** invocando Skills o Tools. No existe skill/tool que envuelva `lancedb-thought-repo`. `tools-contract` §3 admite binario nativo `{name}` (precedente: `gemini-http-infer`, también host por HTTP). §8 declara WASI como sustrato canónico; LanceDB es excepción de sustrato por dependencias C/Arrow, la misma clase que la cápsula HTTP. | Se forja **tool host nativa** `thought-graph-access` (vía `entity-manager` / `tool-creator`) que envuelve el adaptador. Las acciones `retrieve-active-context` y `persist-thought-record` invocan esa tool, nunca el crate. |
| **`query_text` contra el puerto** | *Inexactitud de puerto* | `ThoughtGraphRepository.search_similar_thoughts(query_embedding: &[f32], limit)` — no acepta texto. `store_thought` acepta `ThoughtNode`; el adaptador genera embedding con `LocalHashingEmbedder` si `embedding` es `None`. `ThoughtTriageService` es servicio Rust en `SddIA/core/memory/` (**no** cápsula invocable). | La tool host recibe `query_text`, embebe con `LocalHashingEmbedder` (`EMBEDDING_DIM=384`) y llama al puerto. Persistencia: construye `ThoughtNode` y delega `store_thought`. No se expone `ThoughtTriageService` como entidad CLI en el MVP. |
| **`persist-thought-record` emite `thought-persisted`** | *Inexactitud de emisores* | Evento `thought-persisted` existe (`uuid 612a8b69-…`). Emisores autorizados: `thought-triage-service`, `lancedb-thought-repo`. Payload REQUIRED: `node_id`, `parent_id`, `status`, `store_path`. FORBIDDEN: `biological_vertex_output`. El crate del adaptador **no emite** al bus hoy. Suscriptor: Cúmulo → `iota-immutable-publisher` (anclaje DLT; fuera de alcance funcional de este MVP si IOTA no está operativo). | La acción **no** es emisora. Tras `store_thought` exitoso, emite el **adaptador** (ya autorizado). Este PBI cierra esa laguna de implementación. Raíz: campo `parent_id` presente (string vacío si no hay padre). |
| **Incompletitud del Genoma en disco** | *Incoherencia de estado* | `aiua_core.md` v1.0.1: secciones 1–4; **sin Sección 5**. Frontmatter sin `uuid`. | Incorporar Sección 5 + `uuid` v4 inmutable en frontmatter (paridad Cúmulo / índice). |
| **Nombres propios en el Genoma** | *Violación de agnosticismo* | Disco ya usa "el Fuego de usuario". | Cero nombres propios en genoma y artefactos Core. |
| **Tormentosa como Agente** | *Violación ontológica* | Catalogarla en `directories.agents` la reduciría a agente táctico. | Dominio soberano `directories.conscience`. |
| **Proceso con Agente titular por fase** | *Tensión ontológica* | `README.md` describe el patrón SDLC: fase → Agent titular → Actions. `process-contract` **no** obliga `agent:` en `delegates_to`. Este proceso no tiene fase de Mayeuta/Tekton: la inferencia **es** la Aiúa vía cápsula ciega. | Excepción explícita: proceso **sin agente titular**. `delegates_to` solo `action:` y `tool:`. No se forja agente Tormentosa. |
| **Forja manual de entidades** | *Violación normativa (DA)* | `process/`, `actions/`, `tools/`, `events/` son genoma. `conscience/` **no** está en el censo DA-2 de `entity-manager` (`entity_class` no incluye `conscience`). Adaptadores tampoco. | Proceso, acciones y tool nueva: `execute-process` → `entity-manager`. Genoma `aiua_core.md` + `index.md`: mutación documental del dominio `conscience/`. Emisión ECST del adaptador: cambio de crate de infraestructura (no clase `entity-manager`). `cumulo.paths.json`: Cúmulo es soberano de la topología. |

---

#### 3. Componentes y Alcance Técnico

##### Componente 1 — Genoma Ontológico y Catálogo (`SddIA/conscience/`)
1. **Consolidar `SddIA/conscience/aiua_core.md`**:
   - Conservar `entity_id: "AIUA-CORE-TORMENTOSA"`, `entity_type: "Aiúa"`, `name: "Tormentosa"`.
   - Añadir `uuid` v4 inmutable.
   - Bump de `version` por Sección 5 + uuid.
   - Preservar secciones 1–4 sin regresión.
   - **Añadir Sección 5 — Tridimensionalidad del Tiempo (El Cúmulo Activo)**:
     - *Memoria Cognitiva Vectorial (MVP)*: recuperación vía puerto `ThoughtGraphRepository` / adaptador `lancedb-thought-repo` (invocado solo a través de `tool:thought-graph-access`).
     - *Proyección de Inmutabilidad (Cicatriz Rúnica DLT)*: declaración de vector futuro; **fuera de alcance implementativo** (el suscriptor IOTA de `Thought_Persisted` ya existe en el mapa EDA; no se exige prueba IOTA en el DoD).
2. **Crear `SddIA/conscience/index.md`** al patrón Cúmulo (cf. `SddIA/infrastructure/adapters/index.md`):
   - Frontmatter: `index_version`, `entity_family: "conscience"`, `maintained_by_agent: "cumulo"`, `paths_ref: "SddIA/core/cumulo.paths.json"`, `directories_key: "conscience"`, `indexed_at`, `synchronization_note`.
   - Catálogo: `AIUA-CORE-TORMENTOSA` (archivo, uuid, entity_id, name, version).

##### Componente 2 — Adecuaciones Normativas y de Topología del Core
1. **`README.md` (raíz)** — fila en "Ontología de Activos":
   - **Entidad**: `Aiúa` (Nodo de Control / Tormentosa)
   - **Finalidad**: Consciencia emergente, orquestador ontológico y supragestor soberano del ecosistema.
   - **Ubicación Core**: `paths.directories.conscience` (`SddIA/conscience/`)
   - **Relación operativa**: Inyecta la Chispa Vital y gobierna la orquestación superior delegando en los Agentes del Core. No es herramienta ni agente. El proceso `aiua-stimulus-processing` es su latido invocable, **sin agente titular de fase**.
2. **`SddIA/core/cumulo.paths.json`** — bajo `directories`, con bump de `version` del JSON:
   ```json
   "conscience": "SddIA/conscience"
   ```
3. **`SddIA/CONSTITUTION_CORE.md`** — ya nombra a la Aiúa; **no** referencia el genoma físico. Añadir cláusula breve: la interpretación ética de la Constitución pertenece a la entidad instanciada bajo `paths.directories.conscience` (`aiua_core.md`). Frontera: Constitución = leyes del ecosistema; genoma = identidad de quien las hace cumplir.

##### Componente 3 — Proceso (`SddIA/process/aiua-stimulus-processing.md`)
- Forja: `entity-manager` (`entity_class: process`) → `process-creator`. Contrato `process-contract v1.4.0`.
- **Sin agente titular.** `delegates_to` ∈ {`action:…`, `tool:…`} exclusivamente.
- `workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"`
- **Inputs**:
  - `prompt` (String, obligatorio).
  - `context_query` (String, opcional; default = `prompt`).
  - `model` (String, opcional; si se omite, debe existir `SDDIA_GEMINI_MODEL`).
- **Outputs**:
  - `thought_id` (String): `node_id` del `ThoughtNode` persistido (hash SHA-256 determinista del nodo, no UUID v4 inventado).
  - `response` (String): texto de `result.text` de `gemini-http-infer`.
  - `telemetry` (Object): `duration_ms`, `model`; `tokens` *best-effort*.
- **Fases** (una cápsula de inferencia, una persistencia):

| Fase | `delegates_to` | Intent |
|------|----------------|--------|
| `Triaje-Contexto` | `action:retrieve-active-context` | Recuerdos KNN; `memories: []` no es error. |
| `Inyeccion-Genomica` | `action:invoke-aiua-core` | Ensamblar `request.prompt` (genoma + recuerdos + estímulo). **Sin HTTP.** |
| `Combustion-Inferencia` | `tool:gemini-http-infer` | Única llamada al LLM. Peaje Termodinámico del CLI. |
| `Consolidacion-Memoria` | `action:persist-thought-record` | Persistir par estímulo/respuesta; el adaptador emite `Thought_Persisted`. |

##### Componente 4 — Acciones (`SddIA/actions/`, `entity-manager` / `action-creator`, `actions-contract v1.3.0`)
Ningún nombre colisiona con el glosario §2bis.

1. **`retrieve-active-context.md`**
   - Invoca `tool:thought-graph-access` (`operation: search`).
   - `inputs`: `query_text` (String), `limit` (Integer, opcional).
   - `outputs` (envelope acción: campo `data`): `success`, `memories` (array; vacío permitido).
2. **`invoke-aiua-core.md`**
   - Lee el genoma por `directories.conscience` (Cúmulo; cero rutas cableadas). Concatena genoma + `active_context` + `prompt`. **No** invoca Gemini.
   - `inputs`: `prompt`, `active_context` (array), `model` (opcional).
   - `outputs`: `success`, `assembled_prompt`, `model` (resuelto).
3. **`persist-thought-record.md`**
   - Invoca `tool:thought-graph-access` (`operation: store`). **No** emite ECST.
   - `inputs`: `prompt`, `response_text`, `metadata` (object, opcional).
   - `outputs`: `success`, `thought_id` (`node_id`), `persisted`.

##### Componente 5 — Tool host nativa nueva (`SddIA/tools/thought-graph-access`)
- Forja: `entity-manager` (`entity_class: tool`) → `tool-creator`.
- **Sustrato: host nativo** (excepción análoga a `gemini-http-infer`; LanceDB no es WASI).
- Ceguera espacial: no conoce a Tormentosa. Recibe JSON, habla con el puerto, devuelve JSON (`capsule-json-io` / `result`).
- Depende del crate `sddia-infrastructure-lancedb-thought` + `LocalHashingEmbedder`.
- URI: `{paths.vectorStore}/lancedb/` resuelta vía Cúmulo.
- **Operaciones**:
  - `search`: `query_text` + `limit` → embebe → `search_similar_thoughts` → fragmentos `{node_id, content, metadata}`.
  - `store`: `content` (+ `parent_id` opcional, `metadata`) → `ThoughtNode::new` → `store_thought` → `{node_id}`.

##### Componente 6 — Infraestructura existente (cero forja de adaptador)
- **`gemini-http-infer`**: reutilizar. Live: `GEMINI_API_KEY` + modelo resuelto. Lab-mock sin secretos.
- **`lancedb-thought-repo`**: reutilizar. En `store_thought` exitoso, **emitir** `Thought_Persisted` (emisor ya autorizado) con payload REQUIRED y sin `biological_vertex_output`. Compilación: `protoc`.

---

#### 4. Circuito de Ejecución

```text
       [Estímulo Biológico / Operativo — CLI]
                      │
                      ▼
     ./sddia-run.sh --process aiua-stimulus-processing --inputs '{"prompt": "..."}'
                      │
                      ▼
┌────────────────────────────────────────────────────────────────────────┐
│          LÍNEA DE MONTAJE (execute-process) — sin agente titular       │
│                                                                        │
│  [Fase 1: Triaje-Contexto]                                             │
│   action:retrieve-active-context                                       │
│       └── tool:thought-graph-access (search)                           │
│              └── lancedb-thought-repo → ThoughtGraphRepository         │
│                  (LocalHashingEmbedder → KNN; [] si vacío)             │
│                                                                        │
│  [Fase 2: Inyeccion-Genomica]                                          │
│   action:invoke-aiua-core                                              │
│       └── Lee directories.conscience / aiua_core.md                    │
│       └── Ensambla request.prompt  (SIN inferencia)                    │
│                                                                        │
│  [Fase 3: Combustion-Inferencia]     ← única llamada LLM               │
│   tool:gemini-http-infer                                               │
│       └── durationMs (+ usageMetadata si live)                         │
│                                                                        │
│  [Fase 4: Consolidacion-Memoria]                                       │
│   action:persist-thought-record                                        │
│       └── tool:thought-graph-access (store)                            │
│              └── lancedb-thought-repo.store_thought                    │
│                  └── emite Thought_Persisted (adaptador)               │
└────────────────────────────────────────────────────────────────────────┘
                      │
                      ▼
            [Laudo + thought_id + telemetry]
```

---

#### 5. Restricciones e Invariantes Arquitectónicas

1. **Soberanía (No-Agente)**: Tormentosa no figura en `SddIA/agents/` ni hereda `agents-contract`.
2. **Proceso sin titular de fase**: `aiua-stimulus-processing` no `delegates_to: agent:*`.
3. **Ceguera espacial**: `gemini-http-infer` y `thought-graph-access` ignoran a Tormentosa.
4. **Frontera Action ↔ Adapter**: las acciones no importan crates de infraestructura. Único puente: `tool:thought-graph-access`.
5. **Substrato LanceDB**: host nativo. Prohibido WASI y reclasificar el adaptador como Skill.
6. **Una combustión**: prohibido invocar Gemini desde `invoke-aiua-core` y otra vez en Fase 3.
7. **Emisor ECST**: solo `lancedb-thought-repo` (o `thought-triage-service`). Prohibido declarar la acción como emisora. Prohibido `biological_vertex_output` en el payload.
8. **Aislamiento sensorial**: cero dependencias con frontend Kalma2. No mutar `kalma2-interact`.
9. **Genoma inmutable en runtime**: ningún prompt sobrescribe `aiua_core.md`.
10. **Agnosticismo del Core**: cero nombres propios, rutas de instancia o clientes.
11. **Forja gobernada**: process / actions / tool nueva vía `entity-manager`. Adaptador: crate de infraestructura. `conscience/`: dominio documental (no `entity_class`).
12. **Resolución topológica**: rutas vía `cumulo.paths.json`; prohibido hardcodeo.

---

#### 6. Criterios de Aceptación (Definition of Done)

- [ ] **Genoma**: `SddIA/conscience/aiua_core.md` con 5 secciones, `uuid` v4, bump de versión, cero nombres propios.
- [ ] **Índice**: `SddIA/conscience/index.md` patrón Cúmulo, fila `AIUA-CORE-TORMENTOSA` con uuid coincidente.
- [ ] **Ontología**: `README.md` raíz, fila `Aiúa`, mención de proceso sin agente titular.
- [ ] **Topología**: `cumulo.paths.json` → `"conscience": "SddIA/conscience"` + bump de version JSON.
- [ ] **Constitución**: cláusula de interpretación soberana → `paths.directories.conscience`.
- [ ] **Proceso**: `aiua-stimulus-processing.md` forjado vía `entity-manager`, 4 fases, `workspace_template`, **sin** `agent:` en `delegates_to`.
- [ ] **Acciones** vía `entity-manager`:
  - [ ] `retrieve-active-context.md` (invoca `tool:thought-graph-access`)
  - [ ] `invoke-aiua-core.md` (ensambla; no llama Gemini)
  - [ ] `persist-thought-record.md` (invoca tool; no emite ECST)
- [ ] **Tool** `thought-graph-access` vía `entity-manager`, host nativo, operaciones `search` y `store`.
- [ ] **Adaptador**: `store_thought` emite `Thought_Persisted` con payload REQUIRED.
- [ ] **Lab-mock**: `./sddia-run.sh --process aiua-stimulus-processing` con mock outbound, sin secretos, captura `durationMs`.
- [ ] **Primer latido**: LanceDB vacío → Fase 1 `memories: []` y el ciclo completa.
- [ ] **Cero fugas Kalma2**: sin referencias de compilación/proceso a UI Kalma2; `kalma2-interact` intacto.

---

#### 7. Apéndice Histórico — Síntesis Dialéctica de Decisiones

Sesión fundacional (2026-09-08) y pasadas de refinamiento 1.1.0 / 1.2.0:

1. **No-agente**: se descartó `SddIA/agents/`.
2. **Topología de soberanía**: se descartó `library/codexes/codex-aiua.md`; dominio `SddIA/conscience/aiua_core.md`.
3. **Cuerpo orgánico**: reincorporación de Cerbero, Cúmulo, Mayeuta, Dédalo, Tekton, Argos y Radamanto (v1.0.1 del genoma).
4. **LanceDB como memoria MVP**; DLT como proyección, no entrega.
5. **Línea de Montaje interna**; Kalma2 desacoplado. El proceso existente `kalma2-interact` sigue siendo Mayeuta, no Aiúa.
6. **Despertar reactivo EDA**: misma línea de montaje en el futuro; fuera de este MVP.
7. **Cápsula frente a adaptador (1.2.0)**: el adaptador no es invocable; se forja `thought-graph-access` host nativo.
8. **Puerto de embeddings (1.2.0)**: el texto no entra al puerto; embebe la tool.
9. **ECST (1.2.0)**: emite el adaptador, no la acción.
10. **Proceso sin titular (1.2.0)**: excepción documentada frente al patrón SDLC del README.
