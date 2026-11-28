---
uuid: "66daf19f-217a-4874-b417-99e5be2571f3"
id: github-raw-fetcher
name: github-raw-fetcher
version: "1.0.0"
type: tool
contract: tools-contract
context: system-operations
hash_signature: "sha256:4c7d8119cdf8d23c86e53409c6b9215bb6f409b4b83f22331544d35b2931cce5"
capabilities:
  - "github-raw-fetcher"
  - "asset-fetch"
  - "capsule-json-io"
provides_capability:
  - id: asset:fetch
    contract: asset.fetch
    version: "1.0.0"
deprecation_pivot: "tool:iota-ipfs-fetcher (fase DLT)"
---

# Tool — `github-raw-fetcher`

## Propósito

Proveedor `asset:fetch` G7: descarga raw HTTPS del maestro con `declared_hash` sha256.

## Operación

Consumida por `action:download-remote-asset`; pivot futuro `iota-ipfs-fetcher`.

## Interfaz de entrada y salida (`capsule-json-io` 2.0)

### Petición (stdin)

```json
{
  "meta": {
    "schemaVersion": "2.0",
    "entityKind": "tool",
    "entityId": "github-raw-fetcher"
  },
  "request": {
    "asset_path": "<ruta relativa en el repo maestro>",
    "ref": "main"
  }
}
```

### Respuesta exitosa (stdout, exitCode 0)

```json
{
  "meta": { "schemaVersion": "2.0", "entityKind": "tool", "entityId": "github-raw-fetcher" },
  "success": true,
  "exitCode": 0,
  "result": {
    "content": "<texto plano del activo>",
    "declared_hash": "sha256:<hex64>",
    "origin_kind": "git-raw"
  }
}
```

### Respuesta fallida (stdout, exitCode != 0)

```json
{
  "meta": { "schemaVersion": "2.0", "entityKind": "tool", "entityId": "github-raw-fetcher" },
  "success": false,
  "exitCode": 1,
  "error": { "code": "<ERROR_CODE>", "message": "<descripción>" }
}
```

## Invariantes de cápsula

- `exitCode: 0 ⟺ success: true` (tubería hermética G6).
- `declared_hash` = SHA-256 sobre el texto plano de `content` (corpus idéntico al que verifica la aduana).
- Base remota configurable vía `SDDIA_GITHUB_RAW_BASE` (defecto: `https://raw.githubusercontent.com/racso80es/SddIA/`). Cero secretos; solo lectura pública.
- La cápsula **no** escribe en ningún fichero local; solo stdin → stdout.

## Pivote DLT (G7)

Permutar `provider` en `capability-bindings.md` de `tool:github-raw-fetcher` → `tool:iota-ipfs-fetcher` migra a IOTA Rebased sin tocar `download-remote-asset` ni `sync-client-assets`.
