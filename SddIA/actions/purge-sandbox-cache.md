---
uuid: "37454c3a-4590-4dfb-a439-2014876a0138"
name: "purge-sandbox-cache"
version: "1.0.1"
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
hash_signature: "sha256:95c6a167b654929d30fa976dafd2a59158f8d903ff8f5b4bf4043bcedb331160"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: purge-sandbox-cache

Dos tiempos. Cero agentes.

1. `tool:ephemeral-cache-purger` `simulate: true` (binario nativo).
2. Gate determinista: cada `candidate_targets[i]` debe matchear `^/tmp/cursor-sandbox-cache(/[a-f0-9]{16,64}(/.*)?)?$`.
3. Si `simulate_only` o lista vacía: fin.
4. Misma invocación `simulate: false`.

Handler nativo en `execute-process` (`purge_sandbox_cache`). UUID tool forjada: `8929eeea-5e5b-402a-aafa-4a9b429acaec`.
