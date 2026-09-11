---
context:
- ecosystem-evolution
- external-ingest
contract: process-contract v1.4.0
hash_signature: "sha256:08f46818616eb8f3efc05564f888423d08cf1368fa64712233eb31141c88f0e0"
inputs:
- 'text: Texto plano (XOR con callback_data)'
- 'callback_data: Payload de botón inline ≤ 64 bytes (XOR con text)'
- 'chat_id: Chat origen del callback (opt)'
- 'message_id: Mensaje ancla del callback (opt)'
minteo_maximo: null
name: telegram-gateway
outputs:
- emitted: Si se escribió instancia en ./.events/domain/
- event_type: Tipo ECST emitido
- event_id: UUID de la instancia
phases:
- delegates_to:
  - tool:telegram-gateway
  intent: XOR text→Kaizen/Manual_Task+TelegramMessage_Received; callback_data→TelegramCallback_Received. write_fractal_event domain.
  name: Transmutación e inyección
  requires_capability:
  - contract: channel.ingest
    id: channel:ingest
    version: '>=1.0.0'
porcentaje_de_exito: null
uuid: f5a6b7c8-d9e0-4f1a-b2c3-d4e5f6a7b8c9
version: 1.0.2
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# telegram-gateway

Aduana cognitiva del canal aferente Telegram. XOR `text` / `callback_data`. No realiza long polling ni filtra chat_id (eso es el watcher).

Proveedor DI: fase Transmutación con `requires_capability` → `channel:ingest` (H11-D).

## Contrato

| Input | Obligatorio |
|-------|:-----------:|
| `text` | XOR con `callback_data` |
| `callback_data` | XOR con `text` |
| `chat_id` | No (callback) |
| `message_id` | No (callback) |

