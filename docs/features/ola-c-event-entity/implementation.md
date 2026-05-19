---
feature_name: ola-c-event-entity
created: "2026-05-19"
process: feature
branch_name: feat/ola-c-event-entity
---

# Implementación — Hito 1 Ola C

## Touchpoints materializados

| # | Artefacto | Acción |
|---|-----------|--------|
| H1.1 | `SddIA/CONSTITUTION_CORE.md` | §3.1 Evento de Dominio + distinción Clase/Instancia/Personalización |
| H1.2 | `README.md` | Enlaces a `events-contract.md`, `index.md`, Constitución §3.1 |
| H1.3 | `SddIA/events/events-contract.md` | Contrato familia v1.0.0: ECST, forense, ciclo bus |
| H1.4 | `SddIA/events/index.md` | Índice vacío con columnas obligatorias |
| H1.5 | `SddIA/core/cumulo.paths.json` | `contracts.events` registrado |

## Reglas forenses incorporadas en contrato

- `PullRequest_Merged`: `merge_commit_hash` REQUIRED; `hash_signature` en payload FORBIDDEN
- `Domain_Entity_Created`: `hash_signature_new` REQUIRED; `payload_schema_hash` OPTIONAL

## Pendiente (Fases 2–4)

- `event-creator`, piloto `entity-manager` para `entity_class: event`
- Clases ECST (`pull-request-merged.md`, `domain-entity-*.md`)
