---
uuid: c8e91f2a-4b6d-4e1a-9f03-2d7e5a684b10
name: route-domain-event
version: 1.0.0
contract: process-contract v1.4.0
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
context:
- event-routing
- ecosystem-evolution
hash_signature: sha256:de74f3575f10117320729fd7c63e8567801867c5ce63918204c986157c3331cb
inputs:
- event_file_path: Ruta relativa al JSON padre en eda_bus.pending (.events/pending/)
- cumulo_topology: Topología SSOT inyectada (opcional en laboratorio)
outputs:
- success: boolean
- delivery_status: mapa subscriber_id → status
- parent_path: ruta relativa del padre en pending/
- processing_header_path: ruta relativa de cabecera en processing/
phases:
- name: Lectura y validación ECST
  intent: Leer pending/; validar instancia frente a Clase ECST; gate ecst-gate en dead-letter si falla.
  delegates_to:
  - skill:filesystem-manager
- name: Resolución suscripciones
  intent: Cargar event-subscriptions.json; filtrar por applies_to_origin_topology.
  delegates_to:
  - agent:cumulo
- name: Materialización processing
  intent: Copiar cabecera a processing/; crear testigos processing/subscribers/.
  delegates_to:
  - skill:filesystem-manager
- name: Fan-out suscriptores
  intent: Despachar process/action/tool por suscriptor; async por defecto (sync con SDDIA_LAB_ROUTE_SYNC=1).
  delegates_to:
  - action:execute-process
  - action:execute-action
- name: Promoción testigos
  intent: Mover testigos a processed/dead-letter; decorar; réplica cabecera; purge processing/.
  delegates_to:
  - skill:filesystem-manager
minteo_maximo: null
porcentaje_de_exito: null
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

Handler laboratorio: `route_domain_event_core.route_domain_event()` invocado desde `execute-process.py`.

Watcher: delega en `execute-process --process route-domain-event`.

## Límites

* No emite eventos de dominio; no ancla DLT directamente.
* Sweeper (`event-sweeper.py`) purga `pending/` tras consenso en `processed/subscribers/`.
