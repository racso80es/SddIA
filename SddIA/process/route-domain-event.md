---
context: ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: sha256:4950244f3dd72ccabdf8deb8a72c77fb2535a6b9ef4d9369133bfe85a3f054c0
inputs:
- 'event_uri: Token lógico EventUri resuelto por EventManagerPort (eda://{family}/{event_id})'
- 'event_uris: Lista de tokens lógicos (batch); el ECST se carga por puerto'
- 'cumulo_topology: Topología SSOT inyectada (opcional en laboratorio)'
minteo_maximo: null
name: route-domain-event
outputs:
- success: boolean
- delivery_status: mapa subscriber_id → status
- event_uri: token lógico eda://{family}/{event_id}
- dispatch_mode: string; sync | async
- sweep: objeto resumen de barrido del puerto
- transaction_digest: string opcional; digest DLT cuando aplica
phases:
- intent: Resolver el ECST por EventManagerPort (token event_uri); validar frente a la Clase; Nack/DLQ en el puerto si falla el gate.
  name: Lectura y validación ECST
- delegates_to:
  - agent:cumulo
  intent: Cargar event-subscriptions.json; filtrar por applies_to_origin_topology.
  name: Resolución suscripciones
- intent: Marcar el evento en processing y abrir entregas de suscriptor vía record_delivery en el puerto; sin copia de cabecera ni JSON en subscribers/.
  name: Materialización processing
- delegates_to:
  - action:execute-process
  - action:execute-action
  intent: Despachar process/action/tool por suscriptor; async por defecto (sync con SDDIA_LAB_ROUTE_SYNC=1).
  name: Fan-out suscriptores
- intent: Cerrar entregas en processed o dead-letter del puerto (list_deliveries / record_delivery) cuando los suscriptores aplicables están terminales; sin mover JSON en disco.
  name: Promoción testigos
porcentaje_de_exito: null
uuid: c8e91f2a-4b6d-4e1a-9f03-2d7e5a684b10
version: 2.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# route-domain-event

Proceso orquestador del bus EDA local (**Ola C V3+**). Sustituye la acción homónima legacy.

## Responsabilidades

1. Padre ECST **inmutable** en cola `pending` del puerto (token lógico `pending/`).
2. Estados `processing` / `processed` / `dead-letter` y entregas consultables en el store; sin réplicas JSON en disco.
3. Testigos vía `list_deliveries` / `record_delivery` del puerto (sin JSON bajo `subscribers/`).
4. Fan-out **asíncrono** por suscriptor (modo sync solo en lab/regresión).
5. Promoción/purga vía sweeper sobre entregas del puerto cuando todos los suscriptores aplicables están terminales.

## Implementación física

Handler: `route_domain_event_core::route_domain_event()` resuelve el ECST vía `EventManagerPort`; `event_uri` es token lógico, no lectura de JSON en disco.

Watcher: delega en `execute-process --process route-domain-event`.

## Límites

* No emite eventos de dominio; no ancla DLT directamente.
* Sweeper (`event-sweeper`, binario Rust) purga `pending/` tras consenso en `processed/subscribers/`.
