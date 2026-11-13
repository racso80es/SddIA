---
context:
- event-routing
- ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: sha256:a37837b314d570dde698cae2ac20d2cec5a13f9a83236cc95b2083f3926d22aa
inputs:
- event_file_path: Token lógico `.events/pending/<event_id>.json` resuelto por EventManagerPort, sin JSON en disco
- event_file_paths: Lista de tokens lógicos (batch); el ECST se carga por puerto
- cumulo_topology: Topología SSOT inyectada (opcional en laboratorio)
minteo_maximo: null
name: route-domain-event
outputs:
- success: boolean
- delivery_status: mapa subscriber_id → status
- parent_path: ruta relativa del padre en pending/
- processing_header_path: ruta relativa de cabecera en processing/
phases:
- intent: Resolver el ECST por EventManagerPort (token event_file_path); validar frente a la Clase; Nack/DLQ en el puerto si falla el gate.
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
version: 1.0.2
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

Handler: `route_domain_event_core::route_domain_event()` resuelve el ECST vía `EventManagerPort`; `event_file_path` es token lógico, no lectura de JSON en disco.

Watcher: delega en `execute-process --process route-domain-event`.

## Límites

* No emite eventos de dominio; no ancla DLT directamente.
* Sweeper (`event-sweeper`, binario Rust) purga `pending/` tras consenso en `processed/subscribers/`.
