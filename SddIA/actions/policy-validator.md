---
uuid: "3f8c2b1a-9d0e-4f7a-b2c1-0e9d8c7b6a50"
name: "policy-validator"
version: "1.0.0"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "execution-contexts-validation"
  - "allowed-policies-audit"
  - "tool-context-compliance"
  - "secrets-declaration-audit"
inputs:
  - "validation_profile": "Enum lÃ³gico: `AGENT_POLICIES` | `TOOL_DOMAIN` | `BOTH` â€” quÃ© reglas aplicar en esta invocaciÃ³n"
  - "allowed_policies": "(Opcional) Array de identificadores a contrastar con la matriz S+ de `execution-contexts.md` (secciones 2.x); obligatorio cuando el perfil incluye `AGENT_POLICIES`"
  - "tool_context": "(Opcional) Contexto RBAC Cerbero declarado para una tool; obligatorio cuando el perfil incluye `TOOL_DOMAIN`"
  - "domain_origin": "(Opcional) AcotaciÃ³n del dominio (p. ej. origen del proyecto) para trazabilidad en dictamen"
  - "required_secrets": "(Opcional) Lista de nombres de variables de entorno requeridas; la acciÃ³n solo audita **declaraciÃ³n** (lista no vacÃ­a, sin duplicados obvios), no valores â€” la lectura del entorno queda en `skill:environment-reader` u otra cÃ¡psula"
outputs:
  - "dictamen": "JSON agregado: `valid` (bool), `exitCode` (0 si todo conforme; 1 si bloqueo), `findings` (array de strings causales), `contexts_checked` (subset validado contra la normativa)"
  - "normative_ref": "Referencia estable al artefacto normativo resuelto vÃ­a SSOT (p. ej. clave `directories.norms` + `execution-contexts.md`)"
minteo_maximo: null
porcentaje_de_exito: null
---

# AcciÃ³n: policy-validator

## 1. PropÃ³sito
Emitir un **dictamen normativo** contrastando entradas de forja (agentes, tools) con la matriz de contextos S+ definida en `execution-contexts.md`, sin rutas hardcodeadas: la norma se localiza exclusivamente vÃ­a topologÃ­a CÃºmulo (`directories.norms`). Es la acciÃ³n invocada por procesos como `agent-creator` y `tool-creator` en fases de auditorÃ­a previas a la forja material.

## 2. OrquestaciÃ³n lÃ³gica
1. **ResoluciÃ³n SSOT:** Obtener la ruta canÃ³nica de normas desde `cumulo.paths.json` (clave `directories.norms`) y componer la referencia a `execution-contexts.md`. Si el mapa o el archivo no son accesibles, devolver `exitCode: 1` y `findings` con causa de integridad espacial (alineado a tolerancia cero de Cerbero).
2. **Conjunto permitido:** Extraer el conjunto cerrado de contextos vÃ¡lidos: `source-control`, `filesystem-ops`, `knowledge-management`, `quality-assurance`, `ecosystem-evolution` (identificadores de la normativa vigente; cualquier otro valor en solicitud implica bloqueo).
3. **Perfil `AGENT_POLICIES`:** Para cada elemento de `allowed_policies`, comprobar pertenencia al conjunto permitido. Faltante de clave cuando el perfil la exige â†’ `exitCode: 1`.
4. **Perfil `TOOL_DOMAIN`:** Comprobar que `tool_context` pertenece al conjunto permitido. Si se entrega `required_secrets`, comprobar que es una lista bien formada (p. ej. lista de strings no vacÃ­a si la forja declara secretos obligatorios; polÃ­tica exacta acordada con el proceso invocante).
5. **Perfil `BOTH`:** Ejecutar las reglas 3 y 4 con los campos presentes.
6. **Salida:** Armar `dictamen` con `valid: true` solo si ninguna regla falla; en caso contrario `valid: false` y `exitCode: 1`. Incluir `normative_ref` para trazabilidad del dictamen.

## 3. LÃ­mites
* No invoca `skill:cryptography-manager` ni opera criptografÃ­a; no sustituye a `action:crypto-broker`.
* No lee valores secretos del entorno: solo validaciones declarativas acordadas; el cruce con el runtime del VÃ©rtice BiolÃ³gico es responsabilidad de `skill:environment-reader` (u otra cÃ¡psula) en la fase del proceso que corresponda.
* No modifica normativas ni `allowed_policies` de terceros: solo lectura normativa y emisiÃ³n de dictamen.
