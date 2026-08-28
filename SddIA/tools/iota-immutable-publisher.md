---
capabilities:
- iota-publish
- immutable-anchor
- capsule-json-io
context: system-operations
contract: tools-contract v1.2.0
contract_ref: SddIA/tools/tools-contract.md
domain_origin: SddIA
implementation_path_ref: SddIA/tools/iota-immutable-publisher
name: iota-immutable-publisher
source_sha256: sha256:30027ec820e44a0c80d2665c6bdf92edd6d2ae53dd91d194ba847f1bb05a80fa
uuid: 7c8be7da-d080-4ad0-b0b0-df43be376e46
version: 1.0.0
---


# iota-immutable-publisher

**Descripción:** Cápsula de ejecución atómica para anclar un dato inmutable en la red IOTA Rebased (Testnet).

## Interface

Define las entradas y salidas de la herramienta, siguiendo el estándar `capsule-json-io`.

### Inputs

Entradas esperadas en el payload.

```json
{
  "action": "string (Obligatorio. ej: 'publish_immutable_data')",
  "network": "string (Obligatorio. ej: 'testnet')",
  "payload": "string|array (Obligatorio. El dato a eternizar, o array de strings para agrupar en un Árbol de Merkle)"
}
```

### Outputs Envelope (capsule-json-io)

Estructura de la respuesta emitida por la cápsula.

```json
{
  "success": "boolean (Obligatorio. true si la ejecución fue exitosa, false en caso contrario)",
  "exitCode": "number (Obligatorio. 0 para éxito, >0 para errores)",
  "feedback": "string (Obligatorio. Mensaje detallado sobre el resultado o el error)",
  "result": {
    "transaction_digest": "string (Hash de la transacción inmutable, presente si success es true)",
    "object_id": "string (Opcional. ID del objeto de estado en MoveVM)",
    "merkle_proofs": "array (Opcional. Lista de Merkle proofs si el payload era un array de elementos)",
    "merkle_root": "string (Opcional. Raíz de Merkle si se usó batching)"
  }
}
```

## Security

Directrices de seguridad y aislamiento de la herramienta.

**Isolation Policy:** Obligatorio: Los secretos se cargan vía **Jerarquía de Bóvedas** (`.dev/.env` global, `.SddIA/.dev/.env` instancia) inyectada por el entrypoint Python antes de invocar la cápsula. Estrictamente prohibido requerir o aceptar llaves privadas o mnemónicos en el JSON de entrada del payload.

### Vault hierarchy (SSOT)

| Bóveda | Ruta |
|--------|------|
| Global | `.dev/.env` |
| Instancia | `.SddIA/.dev/.env` (prevalece) |

Referencia Cúmulo: `env_hierarchy` en `SddIA/core/cumulo.paths.json`.

### Secrets

```json
[
  "IOTA_WALLET_SECRET"
]
```

## Architecture

Definición de las dependencias y el motor de ejecución.

**Engine:** TypeScript. La cápsula debe leer el payload estrictamente desde `process.argv[2]` o `stdin` y emitir su salida final únicamente mediante `console.log(JSON.stringify(envelope))`. Cero logs de texto libre que rompan el parseo del orquestador.

### Dependencies

```json
[
  "@iota/iota-sdk"
]
```
