---
uuid: f5a6b7c8-d9e0-4f1a-b2c3-d4e5f6a7b8c9
name: telegram-gateway
version: 1.0.1
contract: process-contract v1.4.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
context:
- ecosystem-evolution
- external-ingest
hash_signature: sha256:4433ab87538bf549f7764c5eff775f30c56ea6fb3d4e398c9d61df9aa436e63b
inputs:
- text: Texto plano recibido del centinela telegram-watcher
outputs:
- emitted: Si se escribió instancia en ./.events/domain/
- event_type: Tipo ECST emitido
- event_id: UUID de la instancia
phases:
- name: Transmutación e inyección
  intent: Regex MVP (TODO prefix → Kaizen_Idea_Captured; resto Manual_Task_Requested)
    y write_fractal_event domain.
  requires_capability:
  - id: channel:ingest
    contract: channel.ingest
    version: '>=1.0.0'
  delegates_to:
  - tool:telegram-gateway
minteo_maximo: null
porcentaje_de_exito: null
---

# telegram-gateway

Aduana cognitiva del canal aferente Telegram. Recibe texto limpio desde Capa 0; no realiza long polling ni filtra chat_id.

Proveedor DI: fase Transmutación con `requires_capability` → `channel:ingest` (H11-D).

## Contrato

| Input | Obligatorio |
|-------|:-----------:|
| `text` | Sí |

