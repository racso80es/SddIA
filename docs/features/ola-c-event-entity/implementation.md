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

## Fase 6 — Argos y cierre (completada)

| Artefacto | Estado |
|-----------|--------|
| `execution.md` | ✅ registro de commits y comandos reproducibles |
| `validacion.md` | ✅ checklist Fase 6 APTO |
| E2E bus | ✅ `PullRequest_Presented` → `processed/` |

Handoff: `delivery-close-cycle` + PR desde `feat/ola-c-event-entity`.

## Fase 5 — Validación cruzada (completada)

| Artefacto | Cambio |
|-----------|--------|
| `event-watcher.py` | Paso 2b: validación instancia ↔ Clase ECST antes de fan-out |
| `route-domain-event.md` | Documentado Paso 2b (REQUIRED/FORBIDDEN) |
| `events-contract.md` | §7 validación runtime en dead-letter |
| `SddIA/templates/eda-instance-events/README.md` | Plantilla Vía C `.SddIA/events/` |

## Fase 4 — Clases ECST (completada)

| Archivo | `event_type` | Forense clave |
|---------|--------------|---------------|
| `pull-request-merged.md` | `PullRequest_Merged` | `merge_commit_hash` REQUIRED; `hash_signature` FORBIDDEN |
| `pull-request-presented.md` | `PullRequest_Presented` | no-op (suscripción vacía) |
| `domain-entity-created.md` | `Domain_Entity_Created` | `hash_signature_new` REQUIRED; `payload_schema_hash` OPTIONAL |
| `domain-entity-updated.md` | `Domain_Entity_Updated` | old+new hashes REQUIRED |
| `domain-entity-deleted.md` | `Domain_Entity_Deleted` | `hash_signature_old` REQUIRED; `hash_signature_new` FORBIDDEN |

Forjadas vía `event-creator` (execute-process.py). Índice `events/index.md` con 5 filas sincronizadas.

## Fase 3 — entity-manager (completada)

| Artefacto | Cambio |
|-----------|--------|
| `entity-manager.md` | Piloto `event` → `event-creator`; mapeo `semantic_seed`; hash recalculado |
| `execute-process.py` | `_run_event_creator`, piloto `skill` + `event`, handler `event-creator` |
| `emit-domain-mutation.md` | `entity_class` incluye `event` |

| Artefacto | Estado |
|-----------|--------|
| `SddIA/process/event-creator.md` | ✅ v1.0.0, hash fases verificado |
| `SddIA/process/index.md` | ✅ fila catalogada |
| `SddIA/norms/interaction-triggers.json` | ✅ `intent.create_event` |
