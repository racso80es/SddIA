---
feature_name: memoria-preferencias-usuario
created: "2026-08-27"
process: feature
base: main
scope: memoria-preferencias-usuario
version_spec: "1.0.0"
document_id: PBI-ARQ-CONSCIENCIA-UNIVERSAL
uuid: "7ad2ef99-4c50-4b6d-9cc1-313d3338bb1b"
persist_ref: docs/features/memoria-preferencias-usuario
branch_name: feat/memoria-preferencias-usuario
execution_id: "56eb29e0-e2f5-46d1-90c6-48b918a1af8a"
dedalo_verdict: ok
laudos:
  - L-ONTOLOGY-SPLIT
  - L-NO-QUERY-EVENT
  - L-CAP-GOVERNED
  - L-PILOT-CHANNELS
  - L-FAIL-POLICY
  - L-NO-DLT-VALUE
  - L-TOMBSTONE
  - L-CUMULO-PATH
---

# Especificación — memoria-preferencias-usuario

## 1. Decisiones Dedalo

| ID | Decisión | Rationale |
|----|----------|-----------|
| **L-ONTOLOGY-SPLIT** | Entidad `UserPreference` + trait `UserPreferenceStore` en `directories` memory Core. Cero campos en `ThoughtNode`. | CA-02; H6 |
| **L-NO-QUERY-EVENT** | Lectura síncrona por cápsula. Prohibido `Query_Subgraph` / ECST de consulta. | Latencia, opt-in, H7 |
| **L-CAP-GOVERNED** | Alta previa de `memory:pref-write` y `memory:pref-query` en taxonomía + schemas + `capability-bindings`. | AC-NO-INVENT |
| **L-PILOT-CHANNELS** | Escritura: `kalma2-bridge` → emisor → `user-preference-ingest`. Lectura: `telegram-fallback-responder` declara `memory:pref-query`. | CA-08; D2 |
| **L-FAIL-POLICY** | Escritura **fail-closed**. Consulta piloto **fail-open** → bloque de contexto vacío (nunca “permitir todo”). Proceso no opt-in: **cero** llamada. | CA-09, CA-14 |
| **L-NO-DLT-VALUE** | `User_Preference_Changed` sin suscriptor IOTA. Payload: IDs + operación + ámbito + `sensitivity`; FORBIDDEN `value`, cuerpos, identificadores en claro de terceros. | CA-11 |
| **L-TOMBSTONE** | `REVOKE` = revisión `status: revoked`. `PURGE` = borrado físico de valor/embedding/derivados; operación de privacidad, no de triaje. | CA-05, CA-06 |
| **L-CUMULO-PATH** | Nueva clave SSOT (propuesta) `paths.userPreferencesStore`: `.SddIA/vector_store/user_preferences/`. Mutación `cumulo.paths.json` vía Cúmulo/proceso autorizado, no hardcode en agentes. | CA-13 |

## 2. Circuito

```
kalma2-bridge (intención explícita)
  → action emit-user-preference-change-requested
  → ECST User_Preference_Change_Requested  (eda_fractal.domain / pending según emisor)
  → process user-preference-ingest
       ├─ Mayeuta: PreferenceProposal | IGNORE
       ├─ validación autoridad / contradicción / sensibilidad
       └─ skill:user-preference-store (memory:pref-write)
            → persistencia durable
            → emit User_Preference_Changed (metadatos no sensibles)

telegram-fallback-responder (opt-in)
  → DI memory:pref-query
  → skill:user-preference-store QUERY
  → envelope.request.user_preference_context (schema versionado)
  → fail-open: {} si store caído
```

## 3. Modelo `UserPreference`

Identidad lógica `preference_id` = SHA-256 canónico de `scope_type | scope_id | subject_kind | subject_key | predicate` (bytes UTF-8, separador `0x1f`). `revision_id` = SHA-256 de `preference_id | payload_canonico | ts`.

| Campo | Tipo / enum | Regla |
|-------|-------------|-------|
| `preference_id` | hex SHA-256 | Estable ante reescrituras de `value` |
| `revision_id` | hex SHA-256 | Inmutable por revisión |
| `subject_kind` | `person` \| `topic` \| `project` \| `channel` | Vocabulario cerrado MVP |
| `subject_key` | string | Clave normalizada (hash de identificador de canal, no PII en eventos) |
| `predicate` | `priority` \| `mute` \| `attention_window` | Cerrado MVP; alta = mutación de spec |
| `value` | JSON tipado por predicado | Ver §3.1 |
| `scope_type` | `global` \| `domain` \| `project` \| `channel` | Más específico gana |
| `scope_id` | string \| null | Obligatorio si no `global` |
| `status` | `proposed` \| `active` \| `revoked` \| `superseded` | |
| `authority` | `explicit_user` \| `inferred` | `inferred` no puede pasar a `active` sin confirmación |
| `sensitivity` | `internal` \| `personal` | `personal` no sale en logs/ECST |
| `valid_from` / `valid_until` | ISO-8601 opc. | |
| `supersedes` | `revision_id` \| null | |
| `provenance` | `{channel, emitter, causal_event_id, at}` | Sin cuerpo de mensaje |
| `embedding` | `Option<Vec<f32>>` | Solo ranking post-filtro; no autoridad |

### 3.1 Valores por predicado

| Predicado | `value` |
|-----------|---------|
| `priority` | `{ "level": "max"\|"high"\|"normal"\|"low" }` |
| `mute` | `{ "muted": true, "until": "<iso>\|null" }` |
| `attention_window` | `{ "dow": [0-6], "hours_local": { "start": "HH:MM", "end": "HH:MM" } }` |

### 3.2 Operaciones

`IGNORE` | `PROPOSE` | `ACTIVATE` | `SUPERSEDE` | `REVOKE` | `PURGE` — semántica PBI §5.1.

Precedencia (CA-10): tombstone vigente → ámbito más específico → `explicit_user` > `inferred` → `revision` más reciente. Empate no resuelto → no activar; quedar `proposed` o conflicto explícito en resultado de write.

## 4. Puerto y adaptador

```
UserPreferenceStore
  put_revision(UserPreference) -> Result
  get_active(preference_id) -> Option<UserPreference>  // excluye revoked/superseded
  query(QuerySpec) -> Vec<UserPreference>              // filtros duros luego KNN opcional
  purge(preference_id) -> Result                       // CA-06
```

`QuerySpec`: `subject_key?`, `predicate?`, `scope`, `max_results` (default 8, max 32), `include_proposed: false` para consumidores.

Adaptador MVP: persistencia **archivo JSON durable** bajo la clave Cúmulo (un fichero por `revision_id` + índice `preference_id` → revisión head). LanceDB opcional **después** de que el path JSON pase reapertura (evitar deuda del placeholder thought repo). Tests: write → drop handle → read.

## 5. Capacidades (alta gobernada — T1)

| id | contract | Provider |
|----|----------|----------|
| `memory:pref-write` | `memory.pref_write` | `skill:user-preference-store` |
| `memory:pref-query` | `memory.pref_query` | `skill:user-preference-store` |

Schemas bajo `directories.capability_contracts`. Bindings en `capability_di.bindings`.

Envelope write `request`: `{ "op": "PUT\|REVOKE\|PURGE", "revision": {…} }`.  
Envelope query `request`: `{ "op": "QUERY", "spec": {…} }`.  
Salida: `capsule-json-io` v2.0; `result.preferences` sin elevar `sensitivity: personal` a stdout de telemetría CLI.

## 6. Eventos (T2, entity-manager)

### 6.1 `User_Preference_Change_Requested`

REQUIRED: `operation` (`propose`\|`activate`\|`revoke`\|`purge`), `channel`, `utterance_ref` (id corto / hash; **no** texto largo).  
OPTIONAL: `subject_hint`, `predicate_hint`, `source_event_id`.  
FORBIDDEN: `body`, `snippet`, `raw_email`, `value` (el valor lo destila el ingest).  
Emisor: acción `emit-user-preference-change-requested` invocada por `kalma2-bridge`.

### 6.2 `User_Preference_Changed`

REQUIRED: `preference_id`, `revision_id`, `operation`, `scope_type`, `status`.  
OPTIONAL: `predicate`, `sensitivity`.  
FORBIDDEN: `value`, PII, cuerpos.  
Emisor: `user-preference-ingest`. Suscriptores MVP: ninguno (o solo logging no persistente). **No** IOTA.

## 7. Amenaza de privacidad (mínimo)

| Activo | Amenaza | Control MVP |
|--------|---------|-------------|
| Store local | Lectura de disco del host | Gitignore; permisos de directorio de instancia; **DEUDA-PREF-CRYPTO** cifrado at-rest |
| Bus ECST | Fuga a DLT / logs | FORBIDDEN value; cero IOTA |
| Consulta DI | Context poisoning / overshare | Filtro status/authority/scope **antes** de embedding; presupuesto `max_results`; consumidores opt-in |
| Exportación | Derecho de inspección | Op `EXPORT` en cápsula → JSON redactable en workspace; CA-15 |
| Purga | Residuos índice/embedding | `PURGE` borra revisiones + índice; test de ausencia post-purga |

Consentimiento: datos del Vértice sobre **terceros** (p. ej. prioridad de un remitente) se tratan `sensitivity: personal`; `subject_key` es hash, no dirección en claro en ECST.

## 8. Mutaciones previstas (Tekton, no esta sesión)

| ID | Artefacto | Vía |
|----|-----------|-----|
| M1 | `capability-taxonomy` + 2 schemas + `capability-bindings` | `entity-manager` / Write atómico norma + sello |
| M2 | `cumulo.paths.json` `paths.userPreferencesStore` | proceso/Cúmulo autorizado |
| M3 | `core/memory` modelo + puerto + tests | in-ciclo crate |
| M4 | skill `user-preference-store` | `entity-manager` create skill |
| M5 | events + action emit | `entity-manager` |
| M6 | process `user-preference-ingest` | `entity-manager` create process |
| M7 | `kalma2-bridge` disparo explícito | in-ciclo UI/API |
| M8 | `telegram-fallback-responder` `requires_capability` query | `entity-manager` update process |
| M9 | `event-domain-subscriptions.json` ingest | SSOT in-ciclo (sin DLT) |

## 9. RBAC ejecutor

`target_executor_rbac` esperado para Tekton: `ecosystem-evolution`, `filesystem-ops`, `knowledge-management`. Skill store: `context` `knowledge-management` o `filesystem-ops` (alinear YAML en forja). Cerbero: no grant global de `memory:pref-query`.
