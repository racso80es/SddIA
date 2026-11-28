---
uuid: "6175f5cd-7844-4d0c-aa93-d2ce3a41d18e"
id: download-remote-asset
name: download-remote-asset
version: "1.0.1"
type: action
contract: "actions-contract v1.2.0"
context: knowledge-management
hash_signature: "sha256:a41761ebd2c6a6d7547147656d4d544d59e6c05fe41c4c8ec9f66c752eb4bbc0"
capabilities:
  - "remote-asset-reclamation"
  - "asset-integrity-declaration"
requires_capability:
  - id: asset:fetch
    contract: asset.fetch
    version: ">=1.0.0"
---

## Propósito

Reclamar activos `library_codexes` / `library_norms` del origen remoto vía DI `asset:fetch` sin nombrar proveedor (G7). Retorna `content`, `declared_hash`, `origin_kind`.

## Orquestación

1. Resolver path lógico desde `asset_id` + `asset_family` (cumulo).
2. Invocar `asset:fetch` (binding en `capability-bindings.md`).
3. Propagar salida al llamador (`sync-client-assets` aduana sha256).

## Límites

Sin ramificar por `origin_kind`; sin referencias a tools de fetch directos.
