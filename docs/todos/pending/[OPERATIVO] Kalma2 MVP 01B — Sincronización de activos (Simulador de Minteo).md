---
document_id: PBI-KALMA2-MVP-01B
uuid: "ed2f20b8-6e3d-4dbf-931c-d62e53ddf7c4"
title: "[OPERATIVO] Kalma2 MVP 01B — Sincronización de activos (Simulador de Minteo)"
format: markdown
version: "1.0.0"
status: bloqueado
priority: alta
process: feature
parent_pbi: PBI-KALMA2-MVP-01
depends_on:
  - id: PBI-KALMA2-MVP-01A
    state: pendiente
    reason: "La carga a sincronizar es el códice codex-kalma2-assistant forjado en T2 de la ola 01A"
feature_slug: kalma2-mvp-sync-activos
persist_ref: docs/features/kalma2-mvp-sync-activos
dossier_ref: docs/features/kalma2-mvp-paciente-0
spec_ref: docs/features/kalma2-mvp-paciente-0/spec.md
plan_ref: docs/features/kalma2-mvp-paciente-0/plan.md
clarify_ref: docs/features/kalma2-mvp-paciente-0/clarify.md
phases: "T6-di-y-capsula,T7-tuberia-sync,T8-bridge-y-wui,T9b-aduana-sync"
created: "2026-08-17"
---

# Kalma2 MVP 01B — Sincronización de activos (Simulador de Minteo)

Segunda ola de `PBI-KALMA2-MVP-01`. El cliente reclama la versión vigente de sus activos desde el repositorio maestro, simulando el entorno DLT futuro.

Especificación en `spec.md` del dossier compartido; aquí solo alcance, gates y Done.

**Bloqueado hasta el merge de `PBI-KALMA2-MVP-01A`:** sin el códice `codex-kalma2-assistant` no existe activo que sincronizar.

## Alcance

| Fase | Entrega | Referencia |
|------|---------|------------|
| T6 | Tool `github-raw-fetcher` + binding `asset:fetch` | `spec.md` §8.3, §9.1 |
| T7 | Acción `download-remote-asset` + proceso `sync-client-assets` | `spec.md` §8.1, §8.2 |
| T8 | `POST /api/sync-assets` en `kalma2-bridge` + botón "Sincronizar Genoma" en la WUI | `spec.md` §10 |
| T9b | Aduana de sincronización + registro en `SddIA/evolution/` | `plan.md` T9 |

**Fuera de alcance:** minteo real en IOTA Rebased. Esta ola simula el ledger; el pivote a `iota-ipfs-fetcher` es una permuta de `provider` en `capability-bindings.md`.

## Entidades a forjar

`github-raw-fetcher` (tool, cápsula temporal), `download-remote-asset` (acción), `sync-client-assets` (proceso). Reutiliza `filesystem-manager` sin modificarla.

## Criterios de aceptación

- [ ] **Sincronización íntegra:** el botón de la WUI actualiza el códice en `{instancia}/.SddIA/library/codexes/`.
- [ ] **Aduana de integridad:** hash discordante aborta la operación **sin escribir** el fichero local.
- [ ] **Pivote DLT sin fractura (G7):** `grep` de `github-raw-fetcher` en `sync-client-assets.md` y `download-remote-asset.md` devuelve cero coincidencias; permutar el `provider` de `asset:fetch` por un stub completa el circuito sin editar proceso ni acción.
- [ ] **Abstracción de origen:** `download-remote-asset` exige `asset_id` y devuelve contenido sin conocer la procedencia; `origin_kind` viaja solo como etiqueta opaca de telemetría.
- [ ] **Tubería hermética (G6):** la cápsula cumple `capsule-json-io` schema 2.0 con `exitCode: 0 ⟺ success: true`.
- [ ] **Fire-and-forget (G8):** `POST /api/sync-assets` devuelve `202` con `correlation_id` sin bloquear la UI; progreso por el canal SSE existente.
- [ ] **Sin credenciales:** la cápsula opera en lectura pública; cero secretos en genoma.
- [ ] **Cicatriz Digital:** las 3 entidades nuevas con `uuid` v4, SemVer, `contract`, `hash_signature` y fila en su `index.md`.

## Done

Un único PR mergeado en `main`, con `validacion.md` en `global: APTO` y `pbi_archived: true`, y este PBI movido a `docs/todos/done/` en la misma rama. Al cerrar esta ola se archiva también el paraguas `PBI-KALMA2-MVP-01`.
