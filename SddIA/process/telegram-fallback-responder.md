---
uuid: "c9d0e1f2-a3b4-4c5d-6e7f-8a9b0c1d2e3f"
name: telegram-fallback-responder
version: "1.0.0"
contract: process-contract v1.4.0
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
context:
- ecosystem-evolution
- external-ingest
hash_signature: sha256:aa025ef2f104dca174c495a3c384aab437ec255389c47ddaab6d9f8f8e4c48fe
inputs:
- text: Texto del mensaje Telegram (payload.text)
- chat_id: Chat destino para respuesta eferente
outputs:
- filtered: Si Filtro C abortó silenciosamente
- synthesized: Texto orgánico generado por Mayeuta
- notified: Si send-telegram-notification tuvo éxito
phases:
- name: Filtro C
  intent: "Evaluar si payload.text empieza por /, ! o palabras reservadas (TODO:, IDEA:). Abortar con success silencioso para ceder a especialistas."
  delegates_to:
  - agent:mayeuta
- name: Síntesis
  intent: 'Invocar Mayeuta con prompt literal: [HARD OVERRIDE] Has recibido este estímulo externo: "{text}". Genera respuesta orgánica ≤2 líneas (Tormentosa/Aiúa).'
  delegates_to:
  - agent:mayeuta
- name: Materialización
  intent: Ejecutar send-telegram-notification con message=output Síntesis y chat_id del payload.
  delegates_to:
  - tool:send-telegram-notification
minteo_maximo: null
porcentaje_de_exito: null
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
