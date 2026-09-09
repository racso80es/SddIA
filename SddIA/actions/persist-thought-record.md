---
uuid: "a37f9f1d-8f2a-441f-9357-776d65553362"
name: "persist-thought-record"
version: "1.1.0"
contract: "actions-contract v1.3.0"
context: "ecosystem-evolution"
capabilities:
  - "persist_thought_record"
inputs:
  - "prompt": "string; obligatorio; estímulo"
  - "response_text": "string; obligatorio; texto del LLM"
  - "metadata": "object; opcional; default {status: ACTIVE}"
outputs:
  - "success": "boolean"
  - "thought_id": "string; node_id SHA-256"
  - "persisted": "boolean"
hash_signature: "sha256:5f78928f2cbee19a3d0e0c68e2801254504d5c5692306b218c3ab68f351f1320"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: persist-thought-record

## 1. Propósito

Persistir el par estímulo/respuesta en el grafo de pensamiento. **No** emite `Thought_Persisted`: el adaptador `lancedb-thought-repo` es el emisor autorizado.

## 2. Orquestación

Handler nativo `handlers::aiua_stimulus::persist_thought_record`.

### Paso 1 — Store (`tool:thought-graph-access`)

`content` = `stimulus:\n{prompt}\n\nresponse:\n{response_text}`.

```json
{
  "operation": "store",
  "content": "<content>",
  "metadata": "<metadata|{status: ACTIVE}>",
  "repository_path": "<repository_path resuelta por Cúmulo>"
}
```

- Abortar si `success` es `false` o `exitCode != 0`.
- `thought_id` ← `result.node_id`.

### Paso 2 — Cierre

```json
{
  "success": true,
  "thought_id": "<node_id>",
  "persisted": true
}
```

## 3. Límites

* Sin emisión ECST desde esta acción. Sin Kalma2 UI. Sin HTTP Gemini.
