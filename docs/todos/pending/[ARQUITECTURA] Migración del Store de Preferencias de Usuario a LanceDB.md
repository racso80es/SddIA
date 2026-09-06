---
document_id: PBI-PREF-STORE-LANCEDB-MIGRATION
title: "[ARQUITECTURA] Migración del Store de Preferencias de Usuario a LanceDB"
format: markdown
version: "1.0.0"
created: "2026-09-06"
status: "propuesta"
refinement_status: unrefined
priority: baja
type: architecture
process: feature
dispatch: false
suggested_branch: feat/pref-store-lancedb-migration
persist_ref_suggested: docs/features/pref-store-lancedb-migration
depends_on: []
related:
  - SddIA/user-preference-core/
  - SddIA/infrastructure/adapters/lancedb_evolution_repo/
  - SddIA/infrastructure/adapters/lancedb_thought_repo/
  - docs/features/memoria-preferencias-usuario/spec.md
spawned_by: PBI-EMAIL-TRIAGE-HEURISTIC
---

### [ARQUITECTURA] Migración del Store de Preferencias de Usuario a LanceDB

#### Origen

Deuda técnica identificada durante el refinamiento de `PBI-EMAIL-TRIAGE-HEURISTIC`. El adaptador MVP de preferencias de usuario (`user-preference-core`) persiste en ficheros JSON durables bajo `.SddIA/vector_store/user_preferences/`. Esta estrategia es funcional para volúmenes bajos, pero limita la capacidad de búsqueda semántica (KNN) necesaria para evolucionar la clasificación no-determinista del triaje de correo.

#### Contexto Arquitectónico

- **Precedente existente:** el ecosistema ya opera dos adaptadores LanceDB:
  - `sddia-infrastructure-lancedb-evolution` — tabla `evolution` para telemetría epigenética.
  - `sddia-infrastructure-lancedb-thought` — tabla `thought` para grafo de pensamiento.
- **Spec de referencia:** `memoria-preferencias-usuario/spec.md` §4 documenta explícitamente la deuda: *«LanceDB opcional **después** de que el path JSON pase reapertura (evitar deuda del placeholder thought repo)»*.
- **Trait existente:** `user-preference-core` define `put_revision`, `get_active`, `query(QuerySpec)`, `purge`. Un adaptador LanceDB implementaría el mismo trait.

#### Alcance Estimado

1. Crear crate `sddia-infrastructure-lancedb-preferences` siguiendo el patrón de `lancedb_evolution_repo`.
2. Implementar el trait `UserPreferenceStore` con tabla LanceDB (`preferences`) bajo `{paths.vectorStore}/lancedb/`.
3. Migrar la consulta `query(QuerySpec)` para soportar KNN sobre embeddings de preferencias (campo `embedding: Option<Vec<f32>>` ya existe en el modelo).
4. Estrategia de migración: leer JSON existente → ingestar en LanceDB → validar paridad → cutover.
5. Evaluar impacto en el contrato `memory:pref-query` / `memory:pref-write`.

#### Criterios de Aceptación (borrador)

- [ ] Nuevo crate compila y pasa tests unitarios con tabla LanceDB temporal.
- [ ] Consulta KNN devuelve preferencias semánticamente similares con recall ≥ 80% vs filtro duro equivalente.
- [ ] Migración de datos JSON → LanceDB es idempotente y reversible.
- [ ] `email-triage-gateway` opera sin regresión tras cutover.
