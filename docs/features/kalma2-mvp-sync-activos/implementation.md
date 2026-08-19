---
feature_name: kalma2-mvp-sync-activos
created: "2026-08-19"
process: feature
branch_name: feat/kalma2-mvp-sync-activos
persist_ref: docs/features/kalma2-mvp-sync-activos
document_id: PBI-KALMA2-MVP-01B
uuid: "ed2f20b8-6e3d-4dbf-931c-d62e53ddf7c4"
status: implementing
agent: tekton
---

# Implementación — kalma2-mvp-sync-activos

## T6 · Tool + binding

| Ítem | Estado |
|------|--------|
| `SddIA/tools/github-raw-fetcher.md` | hecho |
| Cápsula Rust `SddIA/tools/github-raw-fetcher/` | hecho · `capsule-json-io` 2.0 |
| `capability-bindings.md` v1.5.0 · `asset:fetch` | hecho |
| `SddIA/tools/index.md` | hecho |

## T7 · Acción + proceso

| Entidad | UUID | Handler |
|---------|------|---------|
| `download-remote-asset` | `6175f5cd-7844-4d0c-aa93-d2ce3a41d18e` | contrato atómico |
| `sync-client-assets` | `0f6bf2ff-a067-46fb-9175-ee97e6a5dcd8` | `handlers/sync_client_assets.rs` |

G7: cero coincidencias `grep` del proveedor fetch en `.md` de acción/proceso.

## T8 · Bridge + WUI

| Ítem | Estado |
|------|--------|
| `POST /api/sync-assets` · 202 + `correlation_id` en `--inputs` | hecho |
| Botón **Sincronizar Genoma** · `interfaces/kalma2/` | hecho |
| `syncGenome()` · SSE + poll status | hecho |

## T9b · Aduana + cicatriz

| Ítem | Estado |
|------|--------|
| Aduana SHA-256 pre-escritura (Fase 3 handler) | hecho |
| `SddIA/evolution/kalma2-mvp-sync-activos-ola-b.md` | hecho |

## Pendiente cierre

- `validacion.md` (Argos)
- PR único + PBI → `docs/todos/done/`
