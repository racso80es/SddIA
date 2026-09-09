---
feature_name: async-fracture-clarification-mayeuta
created: "2026-09-09"
process: feature
items:
  - event-fracture-clarification-requested
  - action-append-mayeuta-hypothesis
  - enrich-unclassified-emit
  - handler-append-mayeuta-hypothesis
  - orchestration-subscription
---

# Implementation — async-fracture-clarification-mayeuta

## Touchpoints

| Ítem | Path | Vehículo |
|------|------|----------|
| Clase ECST | `SddIA/events/orchestration/fracture-clarification-requested.md` | entity-manager create `7d08d6c4-2ee3-4a70-953f-6f49375913d8` |
| Acción | `SddIA/actions/append-mayeuta-hypothesis.md` | entity-manager create+update `d5c792b5-33a1-4287-a66f-ea3b2ad5716d` |
| Suscripción | `SddIA/core/event-orchestration-subscriptions.json` | directo (no DA-2) |
| Disparador | `enrich_fracture_pbi_kaizen.rs` | `unclassified` + `write_fractal_event` fail-soft |
| Handler | `append_mayeuta_hypothesis.rs` + `actions.rs` | nativo |
| Forja agente | `forges/factory.rs` `run_agent_forge` update | replacements only (no reescritura) |
| Conteo códice | `sync_event_family_class_count` | regex `N clases ECST catalogadas` |

## Propuestas aplicadas

- Prompt `mayeuta-llm` SYNTHESIZE; fail-open `success: true`.
- H2 hipótesis entre Conclusión y Criterio.
- Carta `mayeuta.md` **no** mutada: `hash_signature: opcional_en_desarrollo` bloquea `Domain_Entity_Updated`; binario CLI raíz owned por root no absorbió el parche de forja. Restaurado desde git.
