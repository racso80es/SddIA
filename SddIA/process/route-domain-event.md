---
context:
- event-routing
- ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: sha256:265df9077cc868ce998267acfa7f5f0fca8ec54547f2a32533335f5c638758f9
inputs:
- event_file_path: Ruta relativa al JSON padre en eda_bus.pending (.events/pending/)
- event_file_paths: Lista de rutas relativas a los JSONs (opcional, habilita batching semántico)
- cumulo_topology: Topología SSOT inyectada (opcional en laboratorio)
minteo_maximo: null
name: route-domain-event
outputs:
- success: boolean
- delivery_status: mapa subscriber_id → status
- parent_path: ruta relativa del padre en pending/
- processing_header_path: ruta relativa de cabecera en processing/
phases:
- intent: Leer pending/; validar instancia frente a Clase ECST; gate ecst-gate en dead-letter si falla.
  name: Lectura y validación ECST
  requires_capability:
  - contract: fs.persist
    id: fs:persist
    version: '>=1.0.0'
- delegates_to:
  - agent:cumulo
  intent: Cargar event-subscriptions.json; filtrar por applies_to_origin_topology.
  name: Resolución suscripciones
- intent: Copiar cabecera a processing/; crear testigos processing/subscribers/.
  name: Materialización processing
  requires_capability:
  - contract: fs.persist
    id: fs:persist
    version: '>=1.0.0'
- delegates_to:
  - action:execute-process
  - action:execute-action
  intent: Despachar process/action/tool por suscriptor; async por defecto (sync con SDDIA_LAB_ROUTE_SYNC=1).
  name: Fan-out suscriptores
- intent: Mover testigos a processed/dead-letter; decorar; réplica cabecera; purge processing/.
  name: Promoción testigos
  requires_capability:
  - contract: fs.persist
    id: fs:persist
    version: '>=1.0.0'
porcentaje_de_exito: null
uuid: c8e91f2a-4b6d-4e1a-9f03-2d7e5a684b10
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# route-domain-event

Proceso orquestador del bus EDA local (**Ola C V3+**). Sustituye la acción homónima legacy.

## Responsabilidades

1. Padre ECST **inmutable** en `pending/`.
2. Cabecera réplica en `processing/`, `processed/`, `dead-letter/` según avance.
3. Testigos en `{estado}/subscribers/[UUID].[subscriber_id].json`.
4. Fan-out **asíncrono** por suscriptor (modo sync solo en lab/regresión).
5. Purga de `processing/[UUID].json` cuando todos los suscriptores aplicables están terminales.

## Implementación física

Handler laboratorio: `route_domain_event_core::route_domain_event()` invocado desde el binario `execute-process`.

Watcher: delega en `execute-process --process route-domain-event`.

## Límites

* No emite eventos de dominio; no ancla DLT directamente.
* Sweeper (`event-sweeper`, binario Rust) purga `pending/` tras consenso en `processed/subscribers/`.
