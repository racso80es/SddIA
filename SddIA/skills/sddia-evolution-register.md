---
uuid: "f9d6ad5c-6f7a-49f6-89fb-60d6119776b4"
name: "sddia-evolution-register"
version: "1.0.1"
contract: "skills-contract v1.4.0"
context: "ecosystem-evolution"
capabilities:
  - "evolution-verdict"
  - "evolution-record-compute"
hash_signature: "sha256:52215e87c70cffdf84821a43dd3ceb2e7c9e05111714e9262d3f3f94ef2b52bb"
inputs:
  - "operation": "verdict | alta | modificacion | baja | rehash"
  - "diff": "object inyectado por CLI nativo (paths+status); obligatorio en verdict. La cápsula no calcula Git."
  - "registry": "snapshot JSON records+index inyectado; obligatorio en verdict"
  - "id_cambio": "UUID v4; opcional en alta"
  - "fecha": "ISO; obligatoria en mutaciones; cápsula no inventa"
  - "descripcion_breve": "string"
  - "relacionado": "string[]"
  - "dry_run": "bool"
outputs:
  - "success": "boolean"
  - "exitCode": "integer; 0 iff success"
  - "reason_codes": "string[]"
  - "findings": "array {path, reason_code, detail}"
  - "detail": "markdown propuesto (mutaciones); persistencia host"
  - "index": "índice propuesto (mutaciones)"
  - "hash_integrity": "sha256:..."
---

# Skill: sddia-evolution-register

## Propósito

Registro evolution (verdict/alta/modificación) sobre registry inyectado; cápsula WASI.

## Operación

`verdict` coteja diff+registry sin Git; `rehash` vía canonical_hash.

- `verdict`: coteja `request.diff` contra `request.registry` (JSON inyectado). Cero Git. `audit: universe` valida todos los registros inyectados.
- `rehash`: re-ancla `hash_integrity` in situ vía `canonical_hash`; no regenera el cuerpo.
- `alta` / `modificacion` / `baja`: calcula detalle canónico v1.1.1 + índice. El CLI nativo (`sddia-qa`) persiste.

Sustrato: `compiled_capsules.wasm_root`. Invocación: envelope `capsule-json-io` v2.0 (`--request-file <ruta>`, `SDDIA_CAPSULE_REQUEST` o `stdin` no-TTY con Fail-Fast TTY).

Ejemplo mínimo CLI (`sddia-qa`):

```bash
sddia-qa evolution-register --json --dry-run --request-file request.json
```
