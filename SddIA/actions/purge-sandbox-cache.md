---
uuid: "37454c3a-4590-4dfb-a439-2014876a0138"
name: "purge-sandbox-cache"
version: "1.0.2"
contract: "actions-contract v1.3.0"
context: "filesystem-ops"
capabilities:
  - "purge_sandbox_cache"
  - "delegate-ephemeral-cache-purger"
inputs:
  - "simulate_only": "boolean opcional"
  - "target_dir": "string opcional"
  - "older_than_hours": "number opcional"
  - "purge_sandbox_root": "boolean opcional"
outputs:
  - "success": "boolean"
  - "dry_run": "object"
  - "purge": "object|null"
hash_signature: "sha256:ccd249073994a86a8c6d645cdf05eeec327b33f60f904af140c54704ac250231"
minteo_maximo: null
porcentaje_de_exito: null
---

## Propósito

Purgar `/tmp/cursor-sandbox-cache/**` tras simulación con `tool:ephemeral-cache-purger`.

## Orquestación

1. `simulate: true` → candidatos.
2. Gate regex `^/tmp/cursor-sandbox-cache(/[a-f0-9]{16,64}(/.*)?)?$`.
3. Purga real salvo `simulate_only` o lista vacía.

## Límites

Sin agentes; dos tiempos deterministas.
