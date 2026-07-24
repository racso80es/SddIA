---
context:
- ecosystem-evolution
- external-ingest
contract: process-contract v1.4.0
hash_signature: sha256:924537e45cb8cdc5d0b04054d00d98faad62d3fdcfb57dc28e7540a53d150c3e
inputs:
- text: Texto del mensaje Telegram (payload.text)
- chat_id: Chat destino para respuesta eferente
minteo_maximo: null
name: telegram-fallback-responder
outputs:
- filtered: Si Filtro C abortó silenciosamente
- synthesized: Texto orgánico generado por Mayeuta
- notified: Si send-telegram-notification tuvo éxito
phases:
- delegates_to:
  - agent:mayeuta
  intent: Evaluar si payload.text empieza por /, ! o palabras reservadas (TODO:, IDEA:). Abortar con success silencioso para ceder a especialistas.
  name: Filtro C
  requires_capability:
  - contract: llm.interact
    id: llm:interact
    version: '>=1.0.0'
- delegates_to:
  - agent:mayeuta
  intent: 'Invocar Mayeuta con prompt literal: [HARD OVERRIDE] Has recibido este estímulo externo: "{text}". Genera respuesta orgánica ≤2 líneas (Tormentosa/Aiúa).'
  name: Síntesis
  requires_capability:
  - contract: llm.interact
    id: llm:interact
    version: '>=1.0.0'
- delegates_to:
  - tool:send-telegram-notification
  intent: Ejecutar send-telegram-notification con message=output Síntesis y chat_id del payload.
  name: Materialización
porcentaje_de_exito: null
uuid: c9d0e1f2-a3b4-4c5d-6e7f-8a9b0c1d2e3f
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# telegram-fallback-responder

Red de seguridad sensorial: **triaje inverso** para entropía conversacional no estructurada en Telegram.

## Fase 1 — Filtro C

| Condición | Acción |
|-----------|--------|
| `text` empieza por `/` o `!` | `success`, sin notificación |
| `text` coincide `TODO:` o `IDEA:` (i) | `success`, ceder a `telegram-gateway` |
| resto | continuar a Síntesis |

## Fase 2 — Síntesis

Prompt inyectado literalmente a Mayeuta:

```text
[HARD OVERRIDE] Has recibido este estímulo externo: "{payload.text}". Genera una respuesta orgánica de máximo 2 líneas. Habla desde tu identidad arquitectónica (Tormentosa/Aiúa). Acusa recibo, asimila o cuestiona el estímulo. PROHIBIDO: Ser verboso, ofrecer asistencia genérica o actuar como herramienta esclava.
```

## Fase 3 — Materialización

Invocar **`send-telegram-notification`** con `message` = output Fase 2. Destino: `chat_id` del payload (fallback env `TELEGRAM_ALLOWED_CHAT_ID`).
