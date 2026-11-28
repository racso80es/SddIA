---
context:
- ecosystem-evolution
- external-ingest
contract: process-contract v1.4.0
hash_signature: "sha256:59cfd7bd2cdab434720bdc30ca79a74f08991dbbd5310b4c509307ed187f5681"
inputs:
- 'text: Texto plano (XOR con callback_data)'
- 'callback_data: Payload de botón inline ≤ 64 bytes (XOR con text)'
- 'chat_id: Chat origen del callback (opt)'
- 'message_id: Mensaje ancla del callback (opt)'
minteo_maximo: null
name: telegram-gateway
outputs:
- emitted: Si se emitió instancia eda://domain/<event_id> vía EventManagerPort
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
  id: transmutacion_e_inyeccion
porcentaje_de_exito: null
uuid: f5a6b7c8-d9e0-4f1a-b2c3-d4e5f6a7b8c9
version: 1.0.2
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# telegram-gateway

## Propósito

Aduana cognitiva del canal aferente Telegram. XOR `text` / `callback_data`. No realiza long polling ni filtra chat_id (eso es el watcher).

## Operación

Proveedor DI: fase Transmutación con `requires_capability` → `channel:ingest` (H11-D); invocación `./sddia-run.sh --process telegram-gateway`.

## Contrato

| Input | Obligatorio |
|-------|:-----------:|
| `text` | XOR con `callback_data` |
| `callback_data` | XOR con `text` |
| `chat_id` | No (callback) |
| `message_id` | No (callback) |

