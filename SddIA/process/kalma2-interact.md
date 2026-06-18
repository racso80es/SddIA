---
uuid: "acdb6c88-f0d9-4e10-9d2f-7e4b5401a892"
name: kalma2-interact
version: "1.0.0"
contract: process-contract v1.4.0
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
context:
- ecosystem-evolution
hash_signature: sha256:a6fa47b152b369b63da8c00e9923035c91d7cd20fd542ee6f292eefa6a50ac42
inputs:
- prompt: Texto del operador desde el cliente Kalma2
outputs:
- response: Respuesta sintética Mayeuta (lab) para la UI
phases:
- name: Síntesis
  intent: Transmutar prompt en respuesta orgánica breve vía síntesis Mayeuta (laboratorio, ≤2 líneas).
  delegates_to:
  - agent:mayeuta
- name: Respuesta
  intent: Devolver response al invocador (puente HTTP o CLI execute-process).
minteo_maximo: null
porcentaje_de_exito: null
---

# kalma2-interact

Proceso PoC **Kalma2**: recibe `prompt` del cliente web desacoplado y materializa `response` mediante síntesis Mayeuta de laboratorio (determinista, alineada al patrón `telegram-fallback-responder`).

## Contrato

| Input | Obligatorio |
|-------|:-----------:|
| `prompt` | Sí |

| Output | Descripción |
|--------|-------------|
| `response` | Texto ≤2 líneas (Tormentosa/Aiúa) |

## Fase 1 — Síntesis

Reutiliza plantilla Mayeuta lab (`synthesize_mayeuta_response`) — sin LLM externo en PoC.

## Fase 2 — Respuesta

Retorna JSON con `response` en `data` para consumo del puente `.SddIA/client/sddia-client-bridge.py`.
