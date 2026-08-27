---
feature_name: memoria-preferencias-usuario
created: "2026-08-27"
process: feature
items:
  - T1-cap-ssot
  - T2-domain-store
  - T3-skill-contract
  - T4-events-ingest
  - T5-kalma2-producer
  - T6-telegram-read
  - T7-cross-smoke
---

# Implementación — memoria-preferencias-usuario

## T1 · SSOT capacidades

| Artefacto | Cambio |
|-----------|--------|
| `capability-taxonomy.md` | v1.0.7 — `memory:pref-write`, `memory:pref-query` |
| `capability-bindings.md` | v1.6.0 — provider `skill:user-preference-store` |
| `memory.pref_*.schema.json` | Contratos DI nuevos |
| `cumulo.paths.json` | v1.6.4 — `paths.userPreferencesStore` |

## T2 · Store JSON durable

| Artefacto | Cambio |
|-----------|--------|
| `handlers/user_preference.rs` | Modelo `UserPreference`, store, query, precedencia, tests (3) |

Ruta: `.SddIA/vector_store/user_preferences/{revisions,head_index.json}`.

## T3 · Skill (contrato)

| Artefacto | Cambio |
|-----------|--------|
| `skills/user-preference-store.md` | `provides` write + query |
| `skills/index.md` | Fila indexada |

Handler nativo: `run_capsule` en mismo módulo (pendiente wiring cápsula WASM; MVP vía ingest directo).

## T4 · Eventos + ingest + emisor

| Artefacto | Cambio |
|-----------|--------|
| `events/domain/user-preference-*.md` | Clases ECST |
| `actions/emit-user-preference-change-requested.md` | Emisor |
| `process/user-preference-ingest.md` | Proceso ingest |
| `user_preference_change_requested.rs` | Handler emisor |
| `event-domain-subscriptions.json` | `User_Preference_Change_Requested` → ingest |
| `route_domain_core.rs` | Fan-out `event_file_path` para ingest |

## T6 · Consumidor Telegram (parcial)

| Artefacto | Cambio |
|-----------|--------|
| `telegram-fallback-responder.md` | Fase `memory:pref-query` v1.0.2 |
| `telegram_fallback.rs` | `query_context_block` fail-open antes de síntesis |

## T5 · Productor Kalma2

| Artefacto | Cambio |
|-----------|--------|
| `kalma2-bridge/src/main.rs` | `POST /api/user-preference-change` → evento domain `User_Preference_Change_Requested` |

Validación: `operation` ∈ {propose, activate, revoke, purge, ignore}; `subject_key` obligatorio salvo purge/ignore.

## T7 · Smoke cruzado (ingest)

| Verificación | Resultado |
|--------------|-----------|
| `cross_channel_activate_query_revoke_via_ingest` | activate (kalma2/email) → `query_context_block` hit → revoke → miss |

## T8 · Aduana documental

| Artefacto | Cambio |
|-----------|--------|
| `validacion.md` | `global: APTO`, `pbi_archived: true` |
| PBI | `docs/todos/done/[ARQUITECTURA] Globalización de la Consciencia del Usuario (Grafo de Pensamiento Universal).md` |
| Evolution | `SddIA/evolution/7ad2ef99-4c50-4b6d-9cc1-313d3338bb1b.md` |

## Deuda post-MVP

- Smoke runtime Telegram (operador)
- Sellos `entity-manager` / cobertura EDA pre-commit
- Test contradicción CA-10; wiring cápsula WASI
