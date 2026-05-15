---
uuid: "7c8be7da-d080-4ad0-b0b0-df43be376e46"
name: "iota-immutable-publisher"
version: "1.0.0"
contract: "tools-contract v1.2.0"
contract_ref: "SddIA/tools/tools-contract.md"
domain_origin: "SddIA"
context: "system-operations"
capabilities:
  - "iota-publish"
  - "immutable-anchor"
  - "capsule-json-io"
implementation_path_ref: "scripts/tools/iota-immutable-publisher"
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
  "payload": "string (Obligatorio. El dato a eternizar)"
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
    "object_id": "string (Opcional. ID del objeto de estado en MoveVM)"
  }
}
```

## Security

Directrices de seguridad y aislamiento de la herramienta.

**Isolation Policy:** Obligatorio: Los secretos deben cargarse vía variables de entorno locales (dotenv). Estrictamente prohibido requerir o aceptar llaves privadas o mnemónicos en el JSON de entrada del payload.

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
  "@iota/iota-sdk",
  "dotenv"
]
```
