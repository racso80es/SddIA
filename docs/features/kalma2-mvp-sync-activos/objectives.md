---
feature_name: kalma2-mvp-sync-activos
created: "2026-08-19"
process: feature
phase: Estabilización de Requisitos
agents: mayeuta
document_id: PBI-KALMA2-MVP-01B
uuid: "ed2f20b8-6e3d-4dbf-931c-d62e53ddf7c4"
version: "1.0.0"
persist_ref: docs/features/kalma2-mvp-sync-activos
branch_name: feat/kalma2-mvp-sync-activos
pbi_ref: "docs/todos/pending/[OPERATIVO] Kalma2 MVP 01B — Sincronización de activos (Simulador de Minteo).md"
dossier_ref: docs/features/kalma2-mvp-paciente-0
spec_ref: docs/features/kalma2-mvp-paciente-0/spec.md
depends_on:
  - id: PBI-KALMA2-MVP-01A
    state: mergeado
    pr: "182"
mayeuta_verdict: ok
status: stabilized
---

# Objectives — Kalma2 MVP 01B · Sincronización de Activos

## Objetivo

Cerrar el ciclo de autonomía cognitiva de Kalma2 permitiendo que el cliente sincronice la versión vigente del `codex-kalma2-assistant` desde el repositorio maestro, simulando el entorno DLT futuro. La operación es autoiniciada desde la WUI, transparente para el usuario, segura por aduana de hash e irreversible solo en la dirección "cliente ← genoma".

## Alcance (Ola B — T6 a T8 + T9b)

| Fase | Entrega |
|------|---------|
| T6 | Tool `github-raw-fetcher` (cápsula `capsule-json-io` 2.0) + binding `asset:fetch` en `capability-bindings.md` |
| T7 | Acción `download-remote-asset` + proceso `sync-client-assets` (tubería hermética) |
| T8 | `POST /api/sync-assets` en `kalma2-bridge` + botón "Sincronizar Genoma" en WUI |
| T9b | Aduana de sincronización + registro cicatriz digital en `SddIA/evolution/` |

**Fuera de alcance:** minteo real en IOTA Rebased. El pivote `provider: asset:fetch` (T6) simula el ledger; permutar `github-raw-fetcher` → `iota-ipfs-fetcher` es operación de una línea sin tocar proceso ni acción.

## Ley aplicada

- `capsule-json-io` schema 2.0 — tubería hermética E/S.
- `features-documentation-pattern` v1.2.1 — patrón documental.
- `process-contract` v1.4.0 — contrato de proceso.
- `actions-contract` — contrato de acción.
- `tools-contract` — contrato de tool.
- `codex-contract` v1.2.0 — identidad de activo (NFT lógico); ya ratificado en ola A (R-01).
- DA-5 — fire-and-forget tras acuse JSON del CLI.

## Vector de éxito

Un único PR mergeado en `main` con:
1. Las 3 entidades nuevas (`github-raw-fetcher`, `download-remote-asset`, `sync-client-assets`) con `uuid` v4, SemVer, `contract`, `hash_signature` y fila en sus índices.
2. `POST /api/sync-assets` operativo en `kalma2-bridge` (202 + `correlation_id`).
3. Botón "Sincronizar Genoma" en WUI observable vía SSE.
4. Aduana de hash: discordancia aborta sin escribir fichero local.
5. `validacion.md` en `global: APTO` con `pbi_archived: true` y PBI movido a `docs/todos/done/`.
