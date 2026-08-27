---
document_id: PBI-CORE-LANCEDB-REAL-001
title: "[ARQUITECTURA] LanceDB — integración física real y memoria vectorial efectiva"
format: markdown
version: "1.0.0"
created: "2026-08-27"
updated: "2026-08-27"
status: pending
priority: alta
process: feature
type: feature
dispatch: false
uuid: 3c8b1a72-6e4f-4d90-a2c5-7f1e8b3d9a46
suggested_branch: feat/lancedb-real-vector-memory
persist_ref_suggested: docs/features/lancedb-real-vector-memory
depends_on:
  - PBI-CORE-VECTOR-001-V3
  - PBI-CORE-GRAPH-002
  - PBI-CORE-EVOLUTION-003-V2
source_audits:
  - docs/features/memoria-vectorial/plan.md
  - docs/features/memoria-vectorial/implementation.md
  - docs/features/telemetria-activa-domain-entity-updated/spec.md
  - docs/features/memoria-preferencias-usuario/clarify.md
related:
  - SddIA/core/memory/
  - SddIA/infrastructure/adapters/lancedb_evolution_repo/
  - SddIA/infrastructure/adapters/lancedb_thought_repo/
  - .SddIA/vector_store/
---

# [ARQUITECTURA] LanceDB — integración física real y memoria vectorial efectiva

## Mandato

Sustituir la implementación nominal de LanceDB por una integración física, durable y consultable. El resultado debe usar el motor LanceDB real para persistir y recuperar memoria vectorial; no se considerarán implementación LanceDB los stubs, vectores constantes ni archivos JSON presentados bajo nombres de adaptador LanceDB.

## Estado objetivo frente al estado actual

| Área | Estado actual | Estado requerido |
|------|---------------|------------------|
| Dependencia física | Los `Cargo.toml` no declaran el crate `lancedb` | Dependencia LanceDB real, versión compatible obtenida mediante Cargo |
| Grafo de pensamiento | Operaciones placeholder: `Ok(())`, `None`, `vec![]` | Escritura, lectura exacta, consulta por `parent_id` y KNN reales |
| Evolución | Un JSON por evento en `.SddIA/vector_store/evolution/` | Tabla LanceDB durable con upsert idempotente y consulta verificable |
| Embeddings | `LocalSemanticInference` devuelve 384 ceros | Embeddings locales no constantes, dimensionalidad validada y errores explícitos |
| Compilación | Adaptadores fuera de los miembros del workspace principal | Adaptadores incluidos en el grafo de build/test soportado |
| Reapertura | Sin prueba de cierre y reapertura del store | Datos recuperables tras destruir conexión/proceso y reabrir |

## Objetivo medible

Almacenar un `ThoughtNode` y un `EvolutionEvent`, cerrar toda conexión, reabrir `.SddIA/vector_store/` y demostrar:

1. recuperación exacta por identificador;
2. recuperación de hijos por `parent_id`;
3. búsqueda KNN con orden de distancia verificable;
4. idempotencia ante reingesta del mismo identificador;
5. persistencia de metadata, polaridad y embedding sin pérdida;
6. ausencia de JSON como SSOT alternativo o fallback silencioso.

## Decisiones arquitectónicas obligatorias

### 1. Frontera host nativa

LanceDB y su acceso al filesystem residirán en infraestructura/adaptadores ejecutados por el host nativo. No se impondrá compatibilidad `wasm32-wasip1` ficticia al driver. Las cápsulas WASI, cuando participen, intercambiarán JSON por stdin/stdout conforme a `SddIA/norms/capsule-json-io.md`; no abrirán directamente la base.

Durante la fase `clarify` se documentará con una prueba mínima de compilación la matriz real de targets del crate LanceDB seleccionado. Si LanceDB soporta WASI de forma efectiva, podrá añadirse ese target; la integración nativa seguirá siendo el mínimo obligatorio.

### 2. Hexágono agnóstico

`SddIA/core/memory/` conservará modelos y puertos sin importar tipos de LanceDB, Arrow o runtime concreto. Los detalles de conexión, tablas, esquemas, índices y conversión Arrow pertenecerán a `SddIA/infrastructure/adapters/`.

Si la API de LanceDB obliga a asincronía, la feature debe definir una frontera async coherente de extremo a extremo. Queda prohibido crear un runtime Tokio por operación o esconder bloqueos dentro de los puertos.

### 3. Esquema y colecciones

Como mínimo:

| Dominio | Tabla/colección lógica | Clave idempotente | Vector |
|---------|-------------------------|-------------------|--------|
| Pensamiento | `thought_graph_collection` | `node_id` | `embedding` |
| Evolución | `evolution` | `id` | `embedding` |

Los esquemas deben declarar dimensionalidad del vector, campos requeridos, tipos de metadata y política de evolución de schema. Una dimensión incompatible debe producir error tipado; no truncado, padding ni sustitución por ceros.

### 4. Configuración y soberanía

- La raíz física se resolverá por configuración inyectada y SSOT Cúmulo; `.SddIA/vector_store/` será el valor local por defecto, no una ruta dispersa en lógica de dominio.
- El almacenamiento permanecerá local y gitignored.
- No habrá servicios vectoriales remotos ni credenciales obligatorias.
- Los tests usarán directorios temporales aislados y no tocarán el store real del operador.

### 5. Embeddings efectivos

La búsqueda vectorial solo será aceptable con embeddings locales reales y deterministas para una misma entrada/modelo. Debe sustituirse el vector constante de `LocalSemanticInference` o desacoplarse mediante un proveedor local funcional.

El modelo, dimensión, normalización y versión se registrarán en metadata. Los fallos de carga o inferencia deberán propagarse; queda prohibido degradar silenciosamente a vectores cero.

## Alcance

### Dentro

- Integración del crate LanceDB real y sus dependencias.
- Inclusión explícita de los crates adaptadores en workspace/build/CI.
- Implementación completa de `ThoughtGraphRepository`.
- Persistencia y lectura verificable de `EvolutionStore`; ampliar el puerto con consultas mínimas si actualmente impide probar el resultado.
- Creación/apertura idempotente de tablas y validación de schemas.
- Embedding local efectivo para los flujos cubiertos.
- Wiring host del flujo `memory-evolution-ingest` al adaptador físico.
- Migración controlada de los JSON evolution existentes, si existen en el store local, mediante importador explícito e idempotente.
- Telemetría y errores operativos sin exponer contenido sensible.
- Documentación de decisión tecnológica y límites de LanceDB.

### Fuera

- Memoria de preferencias de usuario: conserva su PBI y modelo propios; no reutilizar `ThoughtNode` como perfil.
- Servicio LanceDB remoto o multiusuario.
- Sincronización entre instalaciones.
- UI de exploración vectorial.
- Reentrenamiento de modelos.
- Borrado automático de los JSON anteriores antes de validar migración.

## Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| LDB-CA1 | Los manifiestos declaran `lancedb` y los adaptadores compilan dentro del grafo soportado | `cargo check`/build selectivo documentado |
| LDB-CA2 | No quedan métodos placeholder en los dos adaptadores ni comentarios que prometan integración futura | búsqueda estática + revisión |
| LDB-CA3 | `store_thought` y recuperación por `node_id` sobreviven a cierre y reapertura | test de integración con directorio temporal |
| LDB-CA4 | `get_children(parent_id)` devuelve únicamente hijos directos persistidos | test de integración |
| LDB-CA5 | KNN devuelve vecinos reales en orden verificable y respeta `limit` | fixture con vectores de distancia conocida |
| LDB-CA6 | `EvolutionEvent` queda persistido en una tabla LanceDB y es recuperable con metadata íntegra | test write → drop → reopen → read |
| LDB-CA7 | Reingestar el mismo ID no duplica registros ni destruye el registro válido | test de idempotencia |
| LDB-CA8 | Embeddings no constantes: textos distintos producen vectores válidos; misma entrada/modelo produce resultado estable dentro de tolerancia | test de inferencia local |
| LDB-CA9 | Dimensión inválida, schema incompatible y store corrupto devuelven errores observables | tests negativos |
| LDB-CA10 | El runtime `memory-evolution-ingest` usa el adaptador real; un evento causal aparece en LanceDB tras reapertura | smoke E2E |
| LDB-CA11 | Ningún archivo JSON actúa como fallback o SSOT oculto tras activar LanceDB | inspección del store + prueba de fallo explícito |
| LDB-CA12 | Store path y configuración se inyectan sin nombres de proyecto cliente en Core | revisión arquitectónica |
| LDB-CA13 | CI ejecuta tests unitarios e integración sin red y sin mutar `.SddIA/vector_store/` del operador | workflow/log APTO |
| LDB-CA14 | La decisión LanceDB queda registrada con contexto, alternativas, restricciones, consecuencias y UUID en evolución | artefacto `SddIA/evolution/` |
| LDB-CA15 | Cierre documental en un único PR | `validacion.md` APTO, `pbi_archived: true`, PBI movido a `done/` |

## Pruebas mínimas

1. `thought_roundtrip_after_reopen`.
2. `thought_children_filtered_by_parent`.
3. `thought_knn_orders_known_vectors`.
4. `evolution_roundtrip_after_reopen`.
5. `duplicate_ids_are_idempotent`.
6. `wrong_vector_dimension_is_rejected`.
7. `schema_mismatch_is_rejected`.
8. `embedding_is_nonzero_nonconstant_and_repeatable`.
9. `memory_evolution_ingest_persists_to_lancedb`.
10. `json_fallback_is_not_used`.

## Migración y reversibilidad

1. Detectar registros JSON legados sin modificarlos.
2. Importarlos a una tabla temporal o destino vacío con identidad determinista.
3. Verificar conteo, IDs y hash/metadata de cada registro.
4. Conmutar el wiring al store LanceDB solo tras prueba de reapertura.
5. Mantener rollback por configuración durante la rama, pero el fallback deberá ser explícito y visible; no automático.
6. La retirada o archivo de JSON legados requiere validación positiva y decisión documentada.

## Riesgos y mitigaciones

| Riesgo | Mitigación |
|--------|------------|
| Incompatibilidad LanceDB ↔ WASI | Driver en frontera host nativa; PoC de targets en `clarify.md` |
| APIs async contaminan dominio | Puerto async explícito y runtime único en composición host |
| Dependencia Arrow aumenta build/CI | Build selectivo, caché y medición antes/después |
| Embeddings incompatibles entre versiones | Metadata de modelo/dimensión/normalización y política de reindexado |
| Corrupción o schema drift | Apertura validada, errores tipados y backup antes de migración |
| Falso verde por mocks | Tests de integración inspeccionan tablas y reabren conexión |
| Pérdida de datos JSON | Migración idempotente, reconciliación y retirada posterior |

## Touchpoints previstos

| Área | Ruta |
|------|------|
| Puertos/modelos/servicios | `SddIA/core/memory/` |
| Adaptador evolución | `SddIA/infrastructure/adapters/lancedb_evolution_repo/` |
| Adaptador pensamiento | `SddIA/infrastructure/adapters/lancedb_thought_repo/` |
| Composición runtime | handler nativo de `memory-evolution-ingest` |
| Workspace y dependencias | manifiestos Cargo aplicables |
| Persistencia local | `.SddIA/vector_store/` |
| Cicatriz de decisión | `SddIA/evolution/{uuid}.md` vía proceso autorizado |
| Evidencia | `docs/features/lancedb-real-vector-memory/` |

## Definición de Done

Done = un único PR mergeado en `main`, implementación LanceDB física probada tras reapertura, embeddings efectivos, integración E2E, `validacion.md` con `global: APTO` y `pbi_archived: true`, y este PBI movido a `docs/todos/done/` en la misma rama.
