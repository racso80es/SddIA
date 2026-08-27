---
feature_name: memoria-preferencias-usuario
created: "2026-08-27"
updated: "2026-08-27"
process: feature
branch: feat/memoria-preferencias-usuario
branch_name: feat/memoria-preferencias-usuario
persist_ref: docs/features/memoria-preferencias-usuario
pbi_ref: docs/todos/done/[ARQUITECTURA] Globalización de la Consciencia del Usuario (Grafo de Pensamiento Universal).md
document_id: PBI-ARQ-CONSCIENCIA-UNIVERSAL
uuid: "7ad2ef99-4c50-4b6d-9cc1-313d3338bb1b"
evolution_id: "7ad2ef99-4c50-4b6d-9cc1-313d3338bb1b"
execution_id: "56eb29e0-e2f5-46d1-90c6-48b918a1af8a"
global: APTO
pbi_archived: true
checks:
  CA-01: APTO
  CA-02: APTO
  CA-03: APTO
  CA-04: APTO
  CA-05: APTO
  CA-06: APTO
  CA-07: APTO
  CA-08: APTO
  CA-09: APTO
  CA-10: PARCIAL
  CA-11: APTO
  CA-12: APTO
  CA-13: APTO
  CA-14: APTO
  CA-15: APTO
  E-T2: APTO
  E-T5: APTO
  E-T7: APTO
  E-T6-RUNTIME: OPERADOR
  ENTITY-MANAGER-SEAL: DEUDA
git_changes:
  - SddIA/library/norms/capability-taxonomy.md
  - SddIA/core/capability-bindings.md
  - SddIA/library/norms/capability-contracts/memory.pref_write.schema.json
  - SddIA/library/norms/capability-contracts/memory.pref_query.schema.json
  - SddIA/core/cumulo.paths.json
  - SddIA/engine/execute-process/src/engine/handlers/user_preference.rs
  - SddIA/engine/execute-process/src/engine/user_preference_change_requested.rs
  - SddIA/engine/execute-process/src/engine/handlers/telegram_fallback.rs
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/engine/execute-process/src/engine/mod.rs
  - SddIA/engine/execute-process/src/engine/actions.rs
  - SddIA/core/event-domain-subscriptions.json
  - SddIA/skills/user-preference-store.md
  - SddIA/skills/index.md
  - SddIA/events/domain/user-preference-change-requested.md
  - SddIA/events/domain/user-preference-changed.md
  - SddIA/events/domain/index.md
  - SddIA/actions/emit-user-preference-change-requested.md
  - SddIA/actions/index.md
  - SddIA/process/user-preference-ingest.md
  - SddIA/process/telegram-fallback-responder.md
  - SddIA/process/index.md
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - docs/features/memoria-preferencias-usuario/
  - docs/todos/done/[ARQUITECTURA] Globalización de la Consciencia del Usuario (Grafo de Pensamiento Universal).md
  - SddIA/evolution/7ad2ef99-4c50-4b6d-9cc1-313d3338bb1b.md
  - SddIA/evolution/Evolution_log.md
---

# Validación — memoria-preferencias-usuario (Argos)

**Veredicto global: APTO** — MVP memoria soberana de preferencias del usuario (`UserPreference`), local a instancia, opt-in, separada de `ThoughtNode`. Rama `feat/memoria-preferencias-usuario`. PBI archivado en `docs/todos/done/`.

## Criterios de aceptación (PBI §11)

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| CA-01 | Modelo `UserPreference` versionado | ✅ APTO | `handlers/user_preference.rs`; skill + schemas |
| CA-02 | Sin reutilizar `ThoughtNode` | ✅ APTO | Ontología separada; spec L-ONTOLOGY-SPLIT |
| CA-03 | SSOT única; canales sin store propio | ✅ APTO | Store bajo `paths.userPreferencesStore`; kalma2 → ECST → ingest |
| CA-04 | Inferencia solo `proposed` | ✅ APTO | `proposed_not_returned_by_default` |
| CA-05 | Revisión + tombstone en revoke | ✅ APTO | `revoke_stops_query`; `put_revision` + head_index |
| CA-06 | Purga física | ✅ APTO | `purge_preference` + `purge_removes_revision_from_store` |
| CA-07 | Persistencia durable / reapertura | ✅ APTO | `put_and_reopen_active_preference` |
| CA-08 | Cruce productor/consumidor | ✅ APTO | `cross_channel_activate_query_revoke_via_ingest`; kalma2 + Telegram handler |
| CA-09 | Opt-in consumidor | ✅ APTO | `telegram-fallback-responder` declara `memory:pref-query`; sin test negativo de proceso control |
| CA-10 | Precedencia determinista | 🟡 PARCIAL | `scope_rank` / `authority_rank` en query; sin test de contradicción explícita |
| CA-11 | Sin value en ECST changed / sin DLT | ✅ APTO | `emit_changed_event` payload; sin suscriptor IOTA |
| CA-12 | Capsule-json-io + taxonomía | ✅ APTO | `memory:pref-write` / `memory:pref-query` v1.0.7; bindings v1.6.0 |
| CA-13 | Rutas por Cúmulo | ✅ APTO | `cumulo.paths.json` `userPreferencesStore` v1.6.4 |
| CA-14 | Fail-open read / fail-closed write | ✅ APTO | `query_context_block` Err→`{}`; ingest sin path → error |
| CA-15 | Export / revocación usuario | ✅ APTO | `run_capsule` EXPORT; revoke vía kalma2 + ingest |

## Evidencia automatizada

```bash
cd SddIA && cargo test -p execute-process user_preference   # 5/5
cd SddIA && cargo test -p kalma2-bridge                     # 21/21 (ruta /api/user-preference-change)
```

| ID | Verificación | Resultado |
|----|--------------|-----------|
| E-T2 | Tests store/query/ingest | 5/5 OK |
| E-T5 | Ruta kalma2 `POST /api/user-preference-change` | OK |
| E-T7 | Smoke activate → query → revoke | OK |

## Deuda explícita (no bloquea APTO MVP)

| ID | Deuda | Nota |
|----|-------|------|
| E-T6-RUNTIME | Smoke Telegram en instancia viva | Verificación operador (mismo patrón que `telegram-fallback-responder`) |
| ENTITY-MANAGER-SEAL | Sellos EDA vía `entity-manager` | Genoma forjado in-ciclo; pre-commit puede exigir cobertura en PR |
| CA-10 | Test contradicción mismo predicado | Seguimiento post-MVP |
| WASM | Wiring cápsula WASI `user-preference-store` | Handler nativo cubre MVP |

## Paridad documental

| Artefacto | Estado |
|-----------|--------|
| objectives.md | ✅ |
| clarify.md | ✅ |
| spec.md | ✅ |
| plan.md | ✅ |
| implementation.md | ✅ |
| execution.md | ✅ |
| validacion.md | ✅ (este) |
