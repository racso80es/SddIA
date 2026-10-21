---
feature_name: eda-redb-03-watcher-vitality
created: "2026-10-21"
process: feature
document_id: PBI-EDA-REDB-03-WATCHER-VITALITY
uuid: "9b4e7c25-8a1d-4e63-b5f0-2d8c6a1e4f97"
---

# Clarify — eda-redb-03-watcher-vitality

## Decisiones

- `filesystem` (default, vacío o desconocido) no abre el store. El barrido de `.events/pending/` no cambia.
- `dual` y `redb` listan pares `(event_id, subscriber_id)` en `Pending`. Si el JSON pending ya existe, el barrido de ficheros lo despacha y el reclamo no lo duplica.
- El watcher pasa solo la ruta `.events/pending/{event_id}.json`. No serializa el ECST.
- `route-domain-event`, si el fichero no está y el modo escribe store, carga el ECST con `load_dispatched_event`.
- El tier sale de `llm_profile.tier` en `SddIA/agents/{name}.md`. Sin bloque no se inventa un tier cognitivo: el camino es determinista.
- El workspace cognitivo es `.SddIA/workspaces/eda-dispatch/{event_id}/{agent}` y Cerbero lo niega si el agente está en `revoked_entities`.
- El despacho `AGENT_PHASE` solo aplica al evento respaldado por store (la ruta no es un fichero). Con JSON en disco el suscriptor cognitivo sigue su proceso o acción.
- Las sondas del sweeper se copian al tick del watcher. El sweeper las conserva. Borrar el binario es PBI 07.
- La ficha `event-watcher.md` sube jurisdicción y versión por `entity-manager` (`daemon_declaration_patch`), sin reescribir el UUID.

## Laudos

- L-REDB-06. Las sondas viven en el watcher antes de retirar el sweeper.
- L-REDB-08. Tres planos: fichero, store y tier cognitivo. `none` no cubre el plano cognitivo.
