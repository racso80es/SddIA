---
feature_name: tracker-lifecycle-provider
created: "2026-10-22"
process: feature
purpose: Laudos de provides, provider resuelto y operación registrada
execution_id: "7a0b47aa-3afa-424e-afc9-a9cb3c67a038"
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-02
---

# Clarificación — tracker-lifecycle-provider

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. `execution_id` `7a0b47aa-3afa-424e-afc9-a9cb3c67a038`. Rama `feat/tracker-lifecycle-provider`.

## Decisiones

| ID | Laudo |
|----|-------|
| L-PROVIDES | Misma terna que el binding: `tracker:lifecycle` / `tracker.lifecycle` / `1.0.0`. |
| L-OUTPUTS | El gate exige que el proveedor declare las claves `required` del schema (`success`, `exitCode`). Van en `outputs` del mismo `.md`. No son operaciones. |
| L-NAME | `should_record_linear_tool` lee `capability_di.bindings`, fila `tracker:lifecycle`, y se queda con el nombre tras `tool:`. Sin fila, no registra. |
| L-CALL | El único llamador pasa `repo`. No se toca `LINEAR_TRACKER_TOOL`. |
| L-DESC | `update_issue_description` usa `issue_ref` del request. Entra en `RECORDED_OPS` junto a las tres que ya estaban. |
| L-FORGE | `hash_refresh_only` vía `entity-manager`. El update completo del creator regeneraría el UUID. |
