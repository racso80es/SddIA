---
uuid: "9b3259be-e7a0-4fb1-b5d9-620a46fbc18b"
name: "crypto-broker"
version: "1.0.1"
contract: "actions-contract v1.2.0"
context: "quality-assurance"
capabilities:
  - "cryptography-broker"
  - "delegate-cryptography-manager"
inputs:
  - "crypto_request": "Objeto JSON idÃ©ntico al stdin de `skill:cryptography-manager`: `operation` âˆˆ [GENERATE_SHA256, VALIDATE_HASH, GENERATE_UUID]; `target_type` âˆˆ [STRING, FILE_PATH] salvo GENERATE_UUID; `target_payload` segÃºn contrato de la skill"
outputs:
  - "crypto_response": "JSON stdout de `skill:cryptography-manager` (success, exitCode, data.result o error)"
  - "exitCode": "0 si la cÃ¡psula reporta Ã©xito; 1 en caso contrario"
minteo_maximo: null
porcentaje_de_exito: null
hash_signature: "sha256:4a73b3b59071f02b5bbb864e7d362f65072f926af62fe7d08f16a339e49bcda4"
---

## Propósito

Única puerta Cerbero hacia `skill:cryptography-manager` cuando el orquestador carece de `quality-assurance` en `allowed_policies`; pipe JSON stdin/stdout sin criptografía en prosa.

## Orquestación

1. Resolver binario vía `compiled_capsules` → `cryptography-manager`.
2. Pasar `crypto_request` (`GENERATE_SHA256` | `VALIDATE_HASH` | `GENERATE_UUID`) por stdin.
3. Propagar `crypto_response` y `exitCode` de la cápsula.

## Límites

Prohibido SHA/UUID local; solo delegación acotada a la skill.
