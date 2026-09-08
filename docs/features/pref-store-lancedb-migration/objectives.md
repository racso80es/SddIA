---
feature_name: pref-store-lancedb-migration
created: "2026-09-08"
process: feature
branch_name: feat/pref-store-lancedb-migration
persist_ref: docs/features/pref-store-lancedb-migration
pbi_ref: docs/todos/done/[ARQUITECTURA] Migración del Store de Preferencias de Usuario a LanceDB.md
execution_id: "338b68e4-de96-4303-93a7-d0c0e40acb9c"
document_id: PBI-PREF-STORE-LANCEDB-MIGRATION
pbi_uuid: "c79e6f1a-821b-4d7a-9a84-0b1e32d56a77"
pbi_version: "1.2.0"
status: in-progress
mayeuta_verdict: ok
---

# Objetivos — pref-store-lancedb-migration

## Misión

Materializar el puerto hexagonal `UserPreferenceStore` y un adaptador LanceDB host nativo para preferencias, en coexistencia con el store JSON vigente. JSON permanece SSOT de escritura. LanceDB es réplica + KNN. Cero reglas semánticas nuevas en triaje.

## Punto objetivo

> **O-PREF-LDB:** Trait compilado e implementado por JSON y LanceDB. Tabla `user_preferences` bajo `{paths.vectorStore}/lancedb/`. Migración idempotente sin borrar JSON. Interruptor = existencia de tabla. Tests de filtro, tombstone, KNN ordenado y dim 384. `email_triage` verde sin cablear LanceDB en ese handler.

## Alcance

| Dentro | Fuera |
|--------|-------|
| Campo `embedding` opcional en `UserPreference` | MiniLM / red / embeddings no locales |
| Trait + `JsonUserPreferenceStore` + fachada libre | Mutar skill `user-preference-store` (DA-2) |
| Crate `sddia-infrastructure-lancedb-preferences` | Convertir LanceDB en SSOT de escritura |
| Ficha + `index.md` + workspace member | Reglas de triaje semántico |
| `migrate_json_to_lancedb` | WASI / `wasm32-wasip1` del driver |
| Resolución en `user_preference.rs` por existencia de tabla | Contaminar `ThoughtNode` / `core/memory` con el trait |
| Tests CA-1..CA-9 locales + CA-10 CI | Purga at-rest / crypto (`DEUDA-PREF-CRYPTO`) |

## Objetivos medibles

| ID | Objetivo | CA |
|----|----------|-----|
| **O1** | Retrocompat JSON | CA-1 |
| **O2** | Puerto hexagonal | CA-2, CA-11 |
| **O3** | Gobernanza adaptador | CA-3 |
| **O4** | Idempotencia + tombstone + KNN + dim | CA-4, CA-5, CA-6, CA-7 |
| **O5** | Migración y paridad de filtro | CA-8 |
| **O6** | Cero regresión triaje | CA-9 |
| **O7** | CI del PR verde antes de accept-pr | CA-10 |

## Orden de ejecución

1. **L0** Diseño (este sello) + commit planificación
2. **L1** Dominio/trait/JSON store
3. **L2** Adaptador LanceDB + ficha + índice
4. **L3** Migración + paridad filtro
5. **L4** Chokepoint `execute-process` + tests triaje
6. **L5** Evolution + implementation/execution
7. **L6** Cierre documental, DCC, CI, accept-pr

## Decisiones Mayeuta (sello)

- Laudo host: `host-nativo-lancedb-hashing-embed-ssot-vectorstore` (no inventar `A-HOST-NATIVE-LANCEDB`).
- `L-FAIL-POLICY` = consulta fail-open → bloque vacío si tabla existe y LanceDB falla. Sin tabla → JSON.
- `L-TOMBSTONE`: revoked se persiste; se excluye en query/KNN.
- Pin `lancedb = "=0.37.1"`. Tabla `user_preferences`. URI `{vectorStore}/lancedb/`.
- `user-preference-core` sin `lancedb` ni `sddia-core-memory`.
- Tekton ejecuta **después** del commit de planificación.

## Ley aplicada

- `features-documentation-pattern` v1.2.1 / `feature` v1.3.2
- `CONSTITUTION_CORE` — Triaje C/A/B
- `adapters-contract` v1.0.0 (adaptadores ∉ DA-2)
- DA-2: no mutar skill. DA-5/DA-6: fire-and-forget CLI; un finding CI → un parche → un push
- `L-ONTOLOGY-SPLIT` (cero campos en ThoughtNode; trait en `user-preference-core`)
- `L-CUMULO-PATH` / `L-NO-DLT-VALUE`
