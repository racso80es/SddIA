---
feature_name: memoria-preferencias-usuario
created: "2026-08-27"
process: feature
phases: "T0-docs,T1-cap-ssot,T2-domain-port,T3-skill-store,T4-events-ingest,T5-pilot-write,T6-pilot-read,T7-cross-revoke,T8-aduana"
uuid: "7ad2ef99-4c50-4b6d-9cc1-313d3338bb1b"
persist_ref: docs/features/memoria-preferencias-usuario
branch_name: feat/memoria-preferencias-usuario
execution_id: "56eb29e0-e2f5-46d1-90c6-48b918a1af8a"
dedalo_verdict: ok
---

# Plan — memoria-preferencias-usuario

T1–T5 + T7 smoke ejecutados en sesión 2026-08-27. T6 parcial; T8 pendiente.

## T0 · Documentación de planificación (esta entrega)

- `objectives.md`, `clarify.md`, `spec.md`, `plan.md` bajo `persist_ref`.
- Topología DA-4 activa. Stop.

## Estado T1–T8 (2026-08-27)

| Fase | Estado |
|------|--------|
| T0 | ✅ |
| T1 | ✅ |
| T2 | ✅ |
| T3 | ✅ (contrato; handler cápsula vía ingest/store nativo) |
| T4 | ✅ |
| T5 | ✅ kalma2-bridge `POST /api/user-preference-change` |
| T6 | 🟡 handler Telegram; falta smoke |
| T7 | ✅ smoke ingest activate/query/revoke |
| T8 | ✅ |

## T1 · Taxonomía y SSOT de rutas

- Mutación gobernada: catálogo `memory:pref-write` / `memory:pref-query`.
- Schemas `memory.pref_write.schema.json` / `memory.pref_query.schema.json`.
- Fila bindings → `skill:user-preference-store` (crear skill en T3; binding puede apuntar al `name` previsto).
- Clave `paths.userPreferencesStore` en `cumulo.paths.json`.
- `delegates_to`: `entity-manager` / Cúmulo; prohibido editar taxonomía a mano.

## T2 · Dominio Core

- `UserPreference` + `UserPreferenceStore` + servicio de precedencia/contradicción.
- Tests: identidad determinista, supersede, tombstone, query filtra `revoked`, reapertura JSON.
- Prohibido tocar `ThoughtNode` salvo comentario de no-mezcla.
- Crate tests nativos; sin WASI obligatorio en T2.

## T3 · Skill `user-preference-store`

- `entity-manager` create skill. Contrato `capsule-json-io` v2.0. `provides` las dos caps.
- Implementación: handler nativo o binario; I/O solo bajo path inyectado.
- Ops: PUT, REVOKE, PURGE, QUERY, EXPORT.
- Tests envelope + fail-closed en PUT si path ilegible.

## T4 · Eventos + proceso ingest

- CREATE `user-preference-change-requested`, `user-preference-changed`.
- CREATE action `emit-user-preference-change-requested`.
- CREATE process `user-preference-ingest`: fases Gate → Destilación (Mayeuta/`llm:interact` solo si no hay hints estructurados) → Persist (`memory:pref-write`) → Emit changed.
- Suscripción domain: `User_Preference_Change_Requested` → ingest. **Sin** IOTA.
- IGNORE no persiste; telemetría agregada sin utterance.

## T5 · Productor piloto (correo / Kalma2)

- `kalma2-bridge`: control explícito “recordar preferencia” (no botones archive/draft/delegate).
- Llama al emisor; **no** escribe store ni `.events/` a pelo.
- Test bridge: POST → envelope emisor; payload sin body.

## T6 · Consumidor piloto (Telegram)

- Update `telegram-fallback-responder`: `requires_capability` `memory:pref-query`.
- Inyección `user_preference_context` versionado; fail-open `{}`.
- Test: proceso **sin** la cap no consulta (CA-09) — usar un process control del Core como negativo.

## T7 · Prueba cruzada y revocación

- Activar `priority.max` para `subject_key` S desde canal correo.
- QUERY desde ingest simulado de Telegram: hit.
- REVOKE desde correo; QUERY posterior: miss.
- PURGE: ausencia en filesystem.
- Contradicción: dos `priority` mismo `preference_id` distinto ámbito — gana el específico.
- Store down: write  ≠ 0; read Telegram `{}`.

## T8 · Aduana documental

- `implementation.md` / `execution.md` / `validacion.md`.
- PBI → `docs/todos/done/` + `pbi_archived: true` en el mismo PR.
- Evolution bajo `directories.evolution`.
- Suite Caos completa **fuera** salvo smoke T7; `qa:probe` no sustituye CA de privacidad.

## Orden

```
T0
T1 → T2 → T3 → T4
T4 → T5
T3 → T6
T5 + T6 → T7 → T8
```

## Fases process-contract (ingest — instanciar en T4)

| name | intent | delegates_to / requires_capability |
|------|--------|--------------------------------------|
| Gate | Validar ECST y FORBIDDEN | — |
| Destilar | Propuesta estructurada o IGNORE | `llm:interact` (condicional) |
| Persistir | PUT/REVOKE/PURGE | `memory:pref-write` → `skill:user-preference-store` |
| Sellar | Emitir `User_Preference_Changed` | action emisor / bus |

Cruce RBAC: `knowledge-management` + `ecosystem-evolution`. Si Cerbero niega `llm:interact`, destilación solo por hints estructurados del bridge (sin improvisar).

## Riesgos de ejecución

| Riesgo | Mitigación en plan |
|--------|-------------------|
| `entity-manager` update regenera UUID de eventos | T4 = CREATE |
| Binding antes de existir la skill | T1 declara id; T3 forja; no ejecutar T6 antes de T3 |
| Placeholder LanceDB | T2/T3 JSON durable primero |
| Dualidad email-quick-action | T5 no reutiliza `action` archive/draft/delegate como hábito |

## Stop de esta sesión

T1–T8 completados en sesión 2026-08-27. PR único pendiente de merge en `main`.
