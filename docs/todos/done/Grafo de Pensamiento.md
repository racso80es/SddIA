---
document_id: PBI-CORE-GRAPH-002
title: "[ARQUITECTURA] PBI: Instanciación del Contrapoder Interno (Grafo de Pensamiento Espacial)"
format: markdown
version: "2.0.0"
created: "2026-06-04"
refined: "2026-06-11"
status: done
priority: arquitectura-core
closed: "2026-06-11"
depends_on: PBI-CORE-VECTOR-001-V3
active_feature: docs/features/grafo-pensamiento
merged_pr: 81
merge_commit: 82c360c
origin: docs/todos/kitchen/Grafo de Pensamiento.md
---

# [ARQUITECTURA] PBI: Instanciación del Contrapoder Interno (Grafo de Pensamiento Espacial) — v2

| Campo | Valor |
|-------|-------|
| **ID** | `PBI-CORE-GRAPH-002` |
| **Estatus** | ✅ Done — implementado y validado |
| **Dependencia** | `PBI-CORE-VECTOR-001-V3` (satisfecha) |
| **Feature** | [`docs/features/grafo-pensamiento/`](../../features/grafo-pensamiento/) |
| **Validación** | [`validacion.md`](../../features/grafo-pensamiento/validacion.md) — APTO |
| **Merge** | PR #81 → `82c360c` |
| **Origen kitchen** | `docs/todos/kitchen/Grafo de Pensamiento.md` |

## 1. Identificación y Estatus del Activo

* **Código de Item:** PBI-CORE-GRAPH-002
* **Dependencia Crítica:** `PBI-CORE-VECTOR-001-V3` (Motor de Memoria Vectorial y Aduana Semántica) — ✅ satisfecha.
* **Naturaleza:** Arquitectura SddIA / Seguridad Cognitiva / Topología de Memoria
* **Entorno:** Cúmulo Core / Motor Vectorial Local
* **Grado de Alineación:** S+ Grade

## 2. Objetivo Estratégico

Dotar al Nodo de Control (Tormentosa) de un contrapoder interno e independiente, inmune al "Secuestro Semántico". Transmutar el flujo lineal de generación LLM en un **Grafo de Pensamiento Espacial** persistido en la Base de Datos Vectorial (BDV). Esto permite aplicar la duda metódica autónoma, explorar caminos paralelos y auto-corregir derivas lógicas antes de emitir cualquier resultado al Vértice Biológico.

## 3. Requisitos Arquitectónicos (Física Interna)

* **Triaje Predictivo Vectorial (Paso 0):** Antes de consumir *tokens* desarrollando un borrador, el sistema genera el *embedding* de la hipótesis inicial y busca en la BDV colisiones con "nodos de fallo" históricos. Si el camino es semánticamente idéntico a un error previo, se poda instantáneamente.
* **Borrador Ciego (Desacople del Output):** El proceso de ramificación del pensamiento ocurre en la oscuridad operativa. Ningún nodo inmaduro o en fase de evaluación (Triaje) cruza la frontera de la Aduana Universal hacia el entorno del usuario.
* **Autopoiesis (Retroceso Autónomo):** Capacidad del motor para detectar una colisión con la Constitución IA (Filtro B) o con la integridad lógica (Filtro A), bloquear esa ruta y utilizar el `Parent_ID` para retroceder autónomamente al último nodo matriz válido sin requerir la intervención del humano.
* **Telemetría de Fricción Vectorizada:** Registro innegociable de las ramas podadas por fallos estructurales. Estos "pensamientos descartados" se vectorizan y guardan en la BDV etiquetados como ruido/falla, sirviendo como anticuerpos para futuras evaluaciones.

## 4. Anatomía del Nodo de Pensamiento (Grafo Vectorial)

Esquema implementado en `SddIA/core/memory/src/models/thought_node.rs`:

| Campo | Regla |
|-------|-------|
| `node_id` | SHA-256(`parent_id` + `content` + `friction_trace`) |
| `parent_id` | Topología jerárquica |
| `content` | Hipótesis, borrador o fragmento de código |
| `metadata` | Estatus de Triaje (A/B), estado `ACTIVE` / `PRUNED` |
| `friction_trace` | Contraargumento superado o motivo de la poda |
| `embedding` | Coordenadas espaciales (BDV LanceDB) |

## 5. Arquitectura de Implementación (Vía S+ Grade)

* **Inferencia Nativa (Rust):** Análisis de similitud semántica y vectorización local vía puerto `EmbeddingGenerator` de Memoria Vectorial.
* **Persistencia Espacial en LanceDB:** Colección `thought_graph_collection` bajo `.SddIA/vector_store/`. Navegación por `parent_id` y saltos espaciales (KNN).

## 6. Criterios de Aceptación (Definition of Done)

| ID | Criterio | Estado |
|----|----------|--------|
| GP-CA1 | Dependencia LanceDB satisfecha | ✅ |
| GP-CA2 | Almacenamiento dual (topológico + KNN) | ✅ |
| GP-CA3 | Aislamiento de borrador (sin fuga stdout) | ✅ |
| GP-CA4 | Retroalimentación inmunológica (`status: PRUNED`) | ✅ |
| GP-CA5 | Paridad documental | ✅ |
| GP-EDA1 | Evento `Thought_Persisted` + suscripción | ✅ |

## 7. Evidencia de implementación

| Artefacto | Ruta |
|-----------|------|
| Modelo | `SddIA/core/memory/src/models/thought_node.rs` |
| Triaje | `SddIA/core/memory/src/services/thought_triage.rs` |
| Puerto | `SddIA/core/memory/src/ports.rs` → `ThoughtGraphRepository` |
| Adaptador | `SddIA/infrastructure/adapters/lancedb_thought_repo/` |
| Evento ECST | `SddIA/events/domain/thought-persisted.md` |
