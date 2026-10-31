---
feature_name: eda-subscriber-event-push
created: "2026-10-31"
purpose: decisiones de alcance AUT-21
process: refactorization
branch_name: Fix_Events_Multiple
tracker_ref: AUT-21
---

# Clarificación — AUT-21

## Decisiones

1. Precedencia de carga: objeto `event` en línea, después `event_id` con `load_dispatched_event`, después `event_file_path` solo para fixtures de laboratorio.
2. La heurística que toma el nombre del fichero como `event_id` deja de ser el camino principal.
3. Un ECST en línea que no coincide con el bus no escribe en el tracker remoto. El suscriptor recarga por `event_id`.
4. `subscriber_delivery` es el estado vivo. `delivery_state` del ECST es la foto del alta y no se actualiza.
5. El clasificador `classify_stamp_delivery` de AUT-46 se audita. No se reimplementa.
6. El origen del evento permanece en `sddia-event-store`. No hay cápsula externa al estilo `tracker.provider`.
7. AUT-26 queda fuera. El re-enrutado de `success` históricos es una pasada operativa después del puerto.
8. La espera del parche de `tracker_stamp.rs` quedó cumplida por AUT-46 (PR #412).

## Tipo en el tracker

El motor selló el ítem como `deuda` porque las etiquetas son `pbi` y `deuda` y el frontmatter no trae `type`. El cuerpo refinado cubre el DoR de deuda y conserva objetivo, alcance, criterios y dependencias del PBI.
