---
uuid: "8929eeea-5e5b-402a-aafa-4a9b429acaec"
name: "ephemeral-cache-purger"
version: "1.0.0"
contract: "tools-contract v1.5.0"
domain_origin: "SddIA"
context: "filesystem-ops"
capabilities:
  - "ephemeral_cache_purger"
hash_signature: "sha256:9f5c29a49f452b559630e1e0250e6b592d4bca9f9051b90be1071c2278ca5310"
implementation_path_ref: "SddIA/tools/ephemeral-cache-purger"
---

# ephemeral-cache-purger

## Propósito

Purga determinista de caches efímeras Cursor bajo `/tmp/cursor-sandbox-cache`.

## Operación

`simulate: true` inventario; `false` purga. Regex jail + veto symlinks. Invocada desde `action:purge-sandbox-cache`.
