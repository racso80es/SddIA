---
uuid: "8929eeea-5e5b-402a-aafa-4a9b429acaec"
name: "ephemeral-cache-purger"
version: "1.0.0"
contract: "tools-contract v1.5.0"
domain_origin: "SddIA"
context: "filesystem-ops"
capabilities:
  - "ephemeral_cache_purger"
hash_signature: "sha256:26e51a9edc694ff24d3c0abb79589e0c17368fb363c6298461e967b628d46c8e"
implementation_path_ref: "SddIA/tools/ephemeral-cache-purger"
---

# ephemeral-cache-purger

Cápsula nativa (excepción WASI --dir=.). Jail regex ^/tmp/cursor-sandbox-cache(/[a-f0-9]{16,64}(/.*)?)?$. Default candidatos …/<hash>/cargo-target. simulate true=inventario; false=purga. Envelope sddia-io. Symlink y prefijos de sistema vetados. EACCES en errors[].

## Propósito

Cápsula nativa (excepción WASI --dir=.). Jail regex ^/tmp/cursor-sandbox-cache(/[a-f0-9]{16,64}(/.*)?)?$. Default candidatos …/<hash>/cargo-target. simulate true=inventario; false=purga. Envelope sddia-io. Symlink y prefijos de sistema vetados. EACCES en errors[].

## Operación

Ejecución conforme al contrato de familia y topología Cúmulo; delegación en cápsulas autorizadas sin rutas cableadas.
