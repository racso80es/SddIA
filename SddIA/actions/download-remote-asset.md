---
uuid: "6175f5cd-7844-4d0c-aa93-d2ce3a41d18e"
id: download-remote-asset
name: download-remote-asset
version: "1.0.0"
type: action
contract: actions-contract
context: knowledge-management
hash_signature: sha256:pending-forge
capabilities:
  - "remote-asset-reclamation"
  - "asset-integrity-declaration"
requires_capability:
  - id: asset:fetch
    contract: asset.fetch
    version: ">=1.0.0"
---

# Acción — `download-remote-asset`

Reclamación de activos del repositorio maestro con declaración de integridad. La acción no nombra ni inspecciona al proveedor concreto de `asset:fetch`; la abstracción es total (pivote DLT G7).

## Contrato de interfaz

### Input

| Campo | Obligatorio | Descripción |
|-------|:-----------:|-------------|
| `asset_id` | Sí | UUID del activo solicitado |
| `asset_family` | Sí | `library_codexes` \| `library_norms` |

### Output

| Campo | Descripción |
|-------|-------------|
| `content` | Texto plano del activo |
| `declared_hash` | `sha256:<hex>` declarado por el origen |
| `origin_kind` | Etiqueta opaca del proveedor (`git-raw`, `dlt-ipfs`, …) — solo telemetría |

## Restricción de acero (G7)

La acción **no** nombra al proveedor concreto de `asset:fetch` en ningún punto. No ramifica lógica según `origin_kind`. Verificable por construcción: el documento no referencia tools de fetch directos (G7).

## Flujo lógico

1. Resolver `asset_path` desde `asset_id` + `asset_family` consultando `cumulo.paths.json`.
2. Invocar `asset:fetch` (motor resuelve proveedor vía `capability-bindings.md`).
3. Propagar `{ content, declared_hash, origin_kind }` al llamador.
