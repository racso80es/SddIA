---
uuid: "3f8c2b1a-9d0e-4f7a-b2c1-0e9d8c7b6a50"
name: "policy-validator"
version: "1.0.0"
contract: "actions-contract v1.1.0"
context: "ecosystem-evolution"
capabilities:
  - "execution-contexts-validation"
  - "allowed-policies-audit"
  - "tool-context-compliance"
  - "secrets-declaration-audit"
inputs:
  - "validation_profile": "Enum lógico: `AGENT_POLICIES` | `TOOL_DOMAIN` | `BOTH` — qué reglas aplicar en esta invocación"
  - "allowed_policies": "(Opcional) Array de identificadores a contrastar con la matriz S+ de `execution-contexts.md` (secciones 2.x); obligatorio cuando el perfil incluye `AGENT_POLICIES`"
  - "tool_context": "(Opcional) Contexto RBAC Cerbero declarado para una tool; obligatorio cuando el perfil incluye `TOOL_DOMAIN`"
  - "domain_origin": "(Opcional) Acotación del dominio (p. ej. origen del proyecto) para trazabilidad en dictamen"
  - "required_secrets": "(Opcional) Lista de nombres de variables de entorno requeridas; la acción solo audita **declaración** (lista no vacía, sin duplicados obvios), no valores — la lectura del entorno queda en `skill:environment-reader` u otra cápsula"
outputs:
  - "dictamen": "JSON agregado: `valid` (bool), `exitCode` (0 si todo conforme; 1 si bloqueo), `findings` (array de strings causales), `contexts_checked` (subset validado contra la normativa)"
  - "normative_ref": "Referencia estable al artefacto normativo resuelto vía SSOT (p. ej. clave `directories.norms` + `execution-contexts.md`)"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: policy-validator

## 1. Propósito
Emitir un **dictamen normativo** contrastando entradas de forja (agentes, tools) con la matriz de contextos S+ definida en `execution-contexts.md`, sin rutas hardcodeadas: la norma se localiza exclusivamente vía topología Cúmulo (`directories.norms`). Es la acción invocada por procesos como `agent-creator` y `tool-creator` en fases de auditoría previas a la forja material.

## 2. Orquestación lógica
1. **Resolución SSOT:** Obtener la ruta canónica de normas desde `cumulo.paths.json` (clave `directories.norms`) y componer la referencia a `execution-contexts.md`. Si el mapa o el archivo no son accesibles, devolver `exitCode: 1` y `findings` con causa de integridad espacial (alineado a tolerancia cero de Cerbero).
2. **Conjunto permitido:** Extraer el conjunto cerrado de contextos válidos: `source-control`, `filesystem-ops`, `knowledge-management`, `quality-assurance`, `ecosystem-evolution` (identificadores de la normativa vigente; cualquier otro valor en solicitud implica bloqueo).
3. **Perfil `AGENT_POLICIES`:** Para cada elemento de `allowed_policies`, comprobar pertenencia al conjunto permitido. Faltante de clave cuando el perfil la exige → `exitCode: 1`.
4. **Perfil `TOOL_DOMAIN`:** Comprobar que `tool_context` pertenece al conjunto permitido. Si se entrega `required_secrets`, comprobar que es una lista bien formada (p. ej. lista de strings no vacía si la forja declara secretos obligatorios; política exacta acordada con el proceso invocante).
5. **Perfil `BOTH`:** Ejecutar las reglas 3 y 4 con los campos presentes.
6. **Salida:** Armar `dictamen` con `valid: true` solo si ninguna regla falla; en caso contrario `valid: false` y `exitCode: 1`. Incluir `normative_ref` para trazabilidad del dictamen.

## 3. Límites
* No invoca `skill:cryptography-manager` ni opera criptografía; no sustituye a `action:crypto-broker`.
* No lee valores secretos del entorno: solo validaciones declarativas acordadas; el cruce con el runtime del Vértice Biológico es responsabilidad de `skill:environment-reader` (u otra cápsula) en la fase del proceso que corresponda.
* No modifica normativas ni `allowed_policies` de terceros: solo lectura normativa y emisión de dictamen.
