---
uuid: "9b3259be-e7a0-4fb1-b5d9-620a46fbc18b"
name: "crypto-broker"
version: "1.0.0"
contract: "actions-contract v1.1.0"
context: "quality-assurance"
capabilities:
  - "cryptography-broker"
  - "delegate-cryptography-manager"
inputs:
  - "crypto_request": "Objeto JSON idéntico al stdin de `skill:cryptography-manager`: `operation` ∈ [GENERATE_SHA256, VALIDATE_HASH, GENERATE_UUID]; `target_type` ∈ [STRING, FILE_PATH] salvo GENERATE_UUID; `target_payload` según contrato de la skill"
outputs:
  - "crypto_response": "JSON stdout de `skill:cryptography-manager` (success, exitCode, data.result o error)"
  - "exitCode": "0 si la cápsula reporta éxito; 1 en caso contrario"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: crypto-broker

## 1. Propósito
Ser la **única puerta autorizada** hacia `skill:cryptography-manager` cuando el orquestador principal (p. ej. `execute-process` bajo contexto `ecosystem-evolution`) no posee en su `allowed_policies` el contexto `quality-assurance` de la skill. Cerbero evalúa esta acción con su propio `context`; la acción delega la física exclusivamente en la cápsula resuelta vía `cumulo.paths.json` → `execution_capsules.skills` → `cryptography-manager.py`.

## 2. Orquestación
1. Resolver la ruta del intérprete y del script `cryptography-manager.py` desde el SSOT (sin rutas absolutas inventadas).
2. Pasar `crypto_request` como **único JSON por stdin** al proceso de la cápsula.
3. Leer **exclusivamente** la respuesta JSON de stdout; reenviarla como `crypto_response`.
4. Propagar `exitCode` igual al `exitCode` devuelto por la cápsula (o 1 si el proceso falla).

## 3. Límites
* No implementar SHA-256, UUID ni validación en prosa dentro de esta acción: solo pipe stdin/stdout hacia la skill.
* No invocar terminal cruda arbitraria del host salvo el lanzamiento acotado de la cápsula declarada en cumulo.
