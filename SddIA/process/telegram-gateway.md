---
uuid: "f5a6b7c8-d9e0-4f1a-b2c3-d4e5f6a7b8c9"
name: telegram-gateway
version: "1.0.0"
contract: process-contract v1.4.0
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
context:
- ecosystem-evolution
- external-ingest
hash_signature: sha256:7cd05f9cb5d25cac5321fbf7a17a464964dea69e7513ae578234708cfc22afd1
inputs:
- text: Texto plano recibido del centinela telegram-watcher
outputs:
- emitted: Si se escribiÃ³ instancia en ./.events/domain/
- event_type: Tipo ECST emitido
- event_id: UUID de la instancia
phases:
- name: TransmutaciÃ³n e inyecciÃ³n
  intent: "Regex MVP (TODO prefix â†’ Kaizen_Idea_Captured; resto Manual_Task_Requested) y write_fractal_event domain."
  delegates_to:
  - tool:telegram-gateway
minteo_maximo: null
porcentaje_de_exito: null
---

# telegram-gateway

Aduana cognitiva del canal aferente Telegram. Recibe texto limpio desde Capa 0; no realiza long polling ni filtra chat_id.

## Contrato

| Input | Obligatorio |
|-------|:-----------:|
| `text` | SÃ­ |

## TransmutaciÃ³n MVP

| PatrÃ³n | event_type |
|--------|------------|
| `^\s*TODO:\s*(.+)$` o `^\s*IDEA:\s*(.+)$` (i) | `Kaizen_Idea_Captured` |
| texto no vacÃ­o | `Manual_Task_Requested` |
| vacÃ­o | no emite |
