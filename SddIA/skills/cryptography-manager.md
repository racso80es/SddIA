---
uuid: "a1f2e3d4-c5b6-4789-a012-3456789abc0"
name: "cryptography-manager"
version: "1.0.0"
contract: "skills-contract v1.1.0"
context: "quality-assurance"
capabilities:
  - "sha256-generation"
  - "hash-validation"
  - "uuid-generation"
hash_signature: "sha256:7ec7062404536588b243b99108cfdc7519aecb4c850c304186ba0dfc04991eb6"
inputs:
  - "operation": "Enum: [GENERATE_SHA256, VALIDATE_HASH, GENERATE_UUID]"
  - "target_type": "Enum: [STRING, FILE_PATH]; omitido o ignorado cuando operation es GENERATE_UUID"
  - "target_payload": "STRING: cadena a hashear (GENERATE_SHA256+STRING) o ruta relativa al workspace (GENERATE_SHA256+FILE_PATH). VALIDATE_HASH: objeto JSON {\"expected_sha256\":\"<hex64>\",\"subject\":\"...\"} si target_type es STRING, o {\"expected_sha256\":\"<hex64>\",\"path\":\"...\"} si es FILE_PATH. GENERATE_UUID: null o cadena vacía."
outputs:
  - "success": "boolean"
  - "exitCode": "0 éxito, 1 error"
  - "data": "objeto con `result`: digest hex64 (GENERATE_SHA256), boolean (VALIDATE_HASH) o UUID v4 en minúsculas (GENERATE_UUID)"
  - "error": "cadena de diagnóstico si success es false"
---

# Skill: cryptography-manager (cápsula física)

## 1. Propósito y Naturaleza
`cryptography-manager` centraliza operaciones criptográficas deterministas (SHA-256, validación de hash, UUID v4) para eliminar cálculos mentales de la IA y garantizar bytes exactos en auditorías de calidad.

## 2. Motor de ejecución (no LLM-Native)
Esta skill **no** es LLM-Native. Su uso requiere invocar la cápsula homónima resuelta vía `cumulo.paths.json` → `execution_capsules.skills` → `cryptography-manager.py`, pasando **un único objeto JSON por stdin** y leyendo **exclusivamente** la respuesta JSON de stdout.

Comando orientativo (desde la raíz del workspace):

`python {paths.execution_capsules.skills}/cryptography-manager.py`

## 3. Contrato I/O (stdin / stdout)
* **Entrada:** JSON UTF-8 con `operation`, `target_type` (salvo `GENERATE_UUID`) y `target_payload` según el esquema del frontmatter.
* **Salida:** JSON con `success`, `exitCode`, y `data.result` en éxito; o `error` en fallo. El proceso debe terminar con código de salida igual a `exitCode`.

## 4. Límites termodinámicos
* Rutas `FILE_PATH` deben ser **relativas** al directorio de trabajo actual; rutas absolutas y escapes fuera del workspace (`..`) se rechazan con `exitCode: 1`.
* `expected_sha256` en `VALIDATE_HASH` debe ser hex minúsculas o mayúsculas de longitud 64; la comparación es contra el digest calculado en minúsculas.
