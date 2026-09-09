---
uuid: "afa0424b-cdd4-4810-9a29-d8e7c06d6a1f"
name: "retrieve-active-context"
version: "1.1.0"
contract: "actions-contract v1.3.0"
context: "ecosystem-evolution"
capabilities:
  - "retrieve_active_context"
inputs:
  - "query_text": "string; obligatorio; texto a embeber para KNN"
  - "limit": "number; opcional; default 5"
outputs:
  - "success": "boolean"
  - "memories": "array; recuerdos KNN; vacío no es error"
hash_signature: "sha256:81a9fd43c23f587c7a247657824ce0387425d9eef1347cfb4df3d10465874098"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: retrieve-active-context

## 1. Propósito

Recuperar recuerdos KNN del grafo de pensamiento previo a la inyección genómica. `memories` vacío no es error. No toca el adaptador ni emite ECST.

## 2. Orquestación

Rutas vía `cumulo.paths.json`. Handler nativo `handlers::aiua_stimulus::retrieve_active_context`.

### Paso 1 — Búsqueda (`tool:thought-graph-access`)

```json
{
  "operation": "search",
  "query_text": "<query_text>",
  "limit": "<limit|5>",
  "repository_path": "<repository_path resuelta por Cúmulo>"
}
```

- Abortar si `success` es `false` o `exitCode != 0`.
- `memories` ← `result.memories` (default `[]`).

### Paso 2 — Cierre

```json
{
  "success": true,
  "memories": []
}
```

## 3. Límites

* Sin HTTP Gemini. Sin emisión `Thought_Persisted`. Sin Kalma2 UI.
