---
uuid: "9b3259be-e7a0-4fb1-b5d9-620a46fbc18b"
name: "crypto-broker"
version: "1.0.0"
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
---

# AcciÃ³n: crypto-broker

## 1. PropÃ³sito
Ser la **Ãºnica puerta autorizada** hacia `skill:cryptography-manager` cuando el orquestador principal (p. ej. `execute-process` bajo contexto `ecosystem-evolution`) no posee en su `allowed_policies` el contexto `quality-assurance` de la skill. Cerbero evalÃºa esta acciÃ³n con su propio `context`; la acciÃ³n delega la fÃ­sica exclusivamente en la cÃ¡psula resuelta vÃ­a `cumulo.paths.json` â†’ `execution_capsules.skills` â†’ `cryptography-manager.py`.

## 2. OrquestaciÃ³n
1. Resolver la ruta del intÃ©rprete y del script `cryptography-manager.py` desde el SSOT (sin rutas absolutas inventadas).
2. Pasar `crypto_request` como **Ãºnico JSON por stdin** al proceso de la cÃ¡psula.
3. Leer **exclusivamente** la respuesta JSON de stdout; reenviarla como `crypto_response`.
4. Propagar `exitCode` igual al `exitCode` devuelto por la cÃ¡psula (o 1 si el proceso falla).

## 3. LÃ­mites
* No implementar SHA-256, UUID ni validaciÃ³n en prosa dentro de esta acciÃ³n: solo pipe stdin/stdout hacia la skill.
* No invocar terminal cruda arbitraria del host salvo el lanzamiento acotado de la cÃ¡psula declarada en cumulo.
