---
feature_name: eda-redb-05-dlt-frontier
created: "2026-10-21"
base: iota-immutable-publisher
scope: frontera DLT
---

# Spec — eda-redb-05-dlt-frontier

## Despacho

`dispatch_subscriber`, ante `iota-immutable-publisher`:

| Clase | Efecto |
| :--- | :--- |
| `Domain_Entity_Telemetry_Captured` | Digest `local-`, prueba en `.SddIA/proofs/dlt-telemetry/`, sin publisher |
| Resto no declarado `external_public_interest` | `skipped-local-only` |
| `PullRequest_Merged`, `System_Immunity_Certified` | `invoke_iota_publisher` con `external_anchor_document` |

El documento de ancla lleva `event_id`, `event_type` y el `payload` completo. Con store activo, `seal_dlt` persiste `tx_hash`. `route_domain_batch` solo marca ancla si la clase es de interés público.

## Contrato

`events-contract` 1.2.0 documenta el enum y los opcionales `participating_agents` y `authorizer`. La clase `PullRequest_Merged` los lista en OPTIONAL. `System_Immunity_Certified` publica la tabla de suscripción que casa con el JSON.

## Norma

`pull-request-orchestration` 1.3.0: la aceptación es el ancla externa; los `Domain_Entity_*` del mismo cambio no van a IOTA.
