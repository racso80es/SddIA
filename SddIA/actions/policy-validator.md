---
uuid: "3f8c2b1a-9d0e-4f7a-b2c1-0e9d8c7b6a50"
name: "policy-validator"
version: "1.0.1"
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
  - "required_secrets": "(Opcional) Lista de nombres de variables de entorno requeridas; la acción solo audita **declaración** (lista no vacía, sin duplicados obvios), no valores — el cruce empírico con el Vértice Biológico queda en `skill:shell-executor` o delegación explícita al operador humano"
outputs:
  - "dictamen": "JSON agregado: `valid` (bool), `exitCode` (0 si todo conforme; 1 si bloqueo), `findings` (array de strings causales), `contexts_checked` (subset validado contra la normativa)"
  - "normative_ref": "Referencia estable al artefacto normativo resuelto vÃ­a SSOT (p. ej. clave `directories.norms` + `execution-contexts.md`)"
minteo_maximo: null
porcentaje_de_exito: null
hash_signature: "sha256:91ecaefb14418c20cfc64c710c48910b6cb930aed687de44cd5de1bffd931240"
---

## Propósito

Dictamen Cerbero vs `execution-contexts.md` vía Cúmulo (`policy_validator.rs`).

## Orquestación

Perfiles `AGENT_POLICIES` | `TOOL_DOMAIN` | `BOTH` → `dictamen` + `normative_ref`.

## Límites

Sin crypto ni lectura de secretos.
