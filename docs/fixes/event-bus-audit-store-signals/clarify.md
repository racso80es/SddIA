---
feature_name: event-bus-audit-store-signals
created: "2026-10-31"
purpose: decisiones de alcance AUT-27
process: bug-fix
branch_name: Fix_Events_Multiple
tracker_ref: AUT-27
execution_id: "6cd0a99f-1d58-43ee-8e30-31e9e13a090b"
---

# Clarificación — AUT-27

## Decisiones

1. La auditoría observa. No reescribe `reconcile_unstamped` ni el despacho de AUT-26.
2. `success_sin_efecto` junta tres hechos: estado `success`, agente con tier `high`, `medium` o `low`, y directorio `.SddIA/workspaces/eda-dispatch/{event_id}/{agente}` ausente o sin ficheros. Si la cápsula es `tracker-stamp`, hace falta además que no exista `.SddIA/state/tracker-outbound/{tracker_ref}.jsonl`. Un jsonl presente apaga esa marca para esa fila.
3. `store_unavailable` solo cuando `EventStore::open` devuelve error. Si el fichero de la base no existe, `open` no se llama y el escaneo vacío se mantiene.
4. El inventario residual lista `*.json` de `.events/pending/` en modo redb y cita el `event_id` de la fractura si el store ya tiene `System_Fracture_Detected` con `friction_id` `F-EDA-RESIDUAL-FILESYSTEM` y la traza nombra ese fichero. No borra ni reinyecta.
5. Las cuatro señales no entran en `needs_kaizen`. El predicado sigue siendo `circuit_alert` o pending estancado accionable.
6. Rama de trabajo: `Fix_Events_Multiple`. Sin checkout a `fix/event-bus-audit-store-signals`.
