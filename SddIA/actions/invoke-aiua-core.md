---
uuid: "2edc7ef4-57e5-4d10-8753-bab8b0053cca"
name: "invoke-aiua-core"
version: "1.1.0"
contract: "actions-contract v1.3.0"
context: "ecosystem-evolution"
capabilities:
  - "invoke_aiua_core"
inputs:
  - "prompt": "string; obligatorio; estímulo biológico"
  - "active_context": "any; opcional; recuerdos KNN"
  - "model": "string; opcional; else SDDIA_GEMINI_MODEL"
outputs:
  - "success": "boolean"
  - "assembled_prompt": "string; prefacio + genoma + contexto + estímulo"
  - "model": "string; resuelto o vacío"
hash_signature: "sha256:73e92023147aee3006519b56a8956a32343dd95ff0a333edcacf3a147c088f8c"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: invoke-aiua-core

## 1. Propósito

Leer `aiua_core.md` vía `directories.conscience` (Cúmulo). Ensamblar request.prompt: prefacio de identidad (frontmatter `name`/`entity_type`, primera persona) + genoma + contexto activo + `## Estímulo`. Sin HTTP. No lee `CONSTITUTION_CORE.md`.

## 2. Orquestación

Handler nativo `handlers::aiua_stimulus::invoke_aiua_core`. Cero `tool:gemini-http-infer`.

1. Resolver `directories.conscience` → `{conscience}/aiua_core.md`.
2. Prefacio: `Eres {name}, la {entity_type}…` (fallback Aiúa si falta `name`).
3. Concatenar genoma, `## Contexto activo` si `active_context`, `## Estímulo` + `prompt`.
4. `model` ← input o `SDDIA_GEMINI_MODEL` o vacío.

### Cierre

```json
{
  "success": true,
  "assembled_prompt": "<texto>",
  "model": "<slug o vacío>"
}
```

## 3. Límites

* Sin combustión LLM. Sin Kalma2 UI. Sin inyección de Constitución.
