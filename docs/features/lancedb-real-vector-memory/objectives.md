---
feature_name: lancedb-real-vector-memory
created: "2026-08-31"
process: feature
branch_name: feat/lancedb-real-vector-memory
persist_ref: docs/features/lancedb-real-vector-memory
pbi_ref: docs/todos/pending/[ARQUITECTURA] LanceDB — integración física real y memoria vectorial efectiva.md
document_id: PBI-CORE-LANCEDB-REAL-001
uuid: "3c8b1a72-6e4f-4d90-a2c5-7f1e8b3d9a46"
execution_id: "c4e7971d-7c67-4745-b1b4-eb8b3d84d652"
mayeuta_verdict: ok
laudo: host-nativo-lancedb-hashing-embed-ssot-vectorstore
---

# Objetivos — lancedb-real-vector-memory

## Misión

Sustituir la implementación nominal de LanceDB (stubs, JSON, vectores cero) por persistencia física durable y consultable: crate `lancedb` real, embeddings locales no constantes, una sola ruta de ingest, SSOT Cúmulo, tests de reapertura.

## Punto objetivo

> **O-LDB:** Tras `store` + drop de conexión + reopen sobre un directorio temporal, un `ThoughtNode` y un `EvolutionEvent` se recuperan por id, hijos, KNN ordenado e idempotencia de clave. El runtime `memory-evolution-ingest` escribe en tabla LanceDB, no en JSON.

## Alcance

| Dentro | Fuera |
|--------|-------|
| Crate `lancedb` + members workspace | Preferencias de usuario |
| Puertos thought (4 métodos) + evolution (store+read+KNN) | LanceDB remoto / multiusuario |
| Embeddings `sddia-local-hashing-v1` dim 384 | Reentrenamiento neural |
| Wiring ingest → `EvolutionStore` | UI vectorial |
| SSOT `paths.vectorStore` | Borrado automático JSON legado |
| Importador JSON explícito | Compatibilidad WASI del driver |
| CI tests sin red / sin store operador | Nuevo evento ECST |

## Objetivos medibles

| ID | Objetivo | Criterio (PBI) |
|----|----------|----------------|
| **O1** | Grafo de build honesto | LDB-CA1 |
| **O2** | Cero placeholders | LDB-CA2 |
| **O3** | Thought durable | LDB-CA3, LDB-CA4, LDB-CA5 |
| **O4** | Evolution durable + idempotente | LDB-CA6, LDB-CA7 |
| **O5** | Embeddings efectivos | LDB-CA8, LDB-CA9 |
| **O6** | Runtime único (no JSON inline) | LDB-CA10, LDB-CA11 |
| **O7** | Soberanía de ruta | LDB-CA12 |
| **O8** | CI aislado | LDB-CA13 |
| **O9** | Cicatriz + Done en un PR | LDB-CA14, LDB-CA15 |

## Orden de ejecución (sello)

1. **T0** members + dep `lancedb` + PoC compile
2. **T1** core: errores, puertos read, embedder
3. **T2** adaptador thought físico
4. **T3** adaptador evolution + importador JSON
5. **T4** SSOT `paths.vectorStore` + wiring ingest
6. **T5** tests de aceptación (10) + CI
7. **T6** fichas adaptador `active` + evolution
8. **T7** cierre documental + PR

## Decisiones Mayeuta (sello)

- Driver solo host nativo. WASI no es target del crate.
- Un Runtime Tokio por `open()` del adaptador; puertos sync.
- URI LanceDB = `{vectorStore}/lancedb/` (aislado de JSON legado y `user_preferences`).
- Embeddings hashing local determinista; no red en CI.
- Ingest pasa por puerto; JSON no es fallback.
- Tekton arranca **después** del commit de este plan.

## No objetivos

- Sustituir `user-preference-core`.
- MiniLM/ONNX en este PR.
- Purgar `.SddIA/vector_store/evolution/*.json` del operador.

## Ley aplicada

- `features-documentation-pattern` v1.2.1 / proceso `feature` v1.3.2
- `CONSTITUTION_CORE` — Triaje C/A/B
- DA-2/DA-3/DA-4/DA-5; DA-6 al primer check CI rojo
- Rutas vía `SddIA/core/cumulo.paths.json`
- Git vía `skill:git-manager`
- Cierre documental en rama: PBI → `docs/todos/done/` + `validacion.md` APTO en el mismo PR
