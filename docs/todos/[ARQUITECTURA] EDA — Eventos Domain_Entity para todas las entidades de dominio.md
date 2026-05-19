---
document_id: TODO-EDA-DOMAIN-ENTITIES
title: "[ARQUITECTURA] EDA — Eventos Domain_Entity_* para todas las entidades de dominio"
format: markdown
version: "1.0.0"
created: "2026-05-19"
status: "pendiente"
priority: alta
blocks: "Cierre CA EDA / ampliación piloto entity-manager"
related:
  - SddIA/process/entity-manager.md
  - SddIA/actions/emit-domain-mutation.md
  - SddIA/core/event-subscriptions.json
  - docs/features/ola-c-event-entity/spec.md
---

# TODO: Emisión EDA universal en mutaciones de entidades de dominio

## Objetivo

Garantizar que **toda** mutación de entidad catalogada en el genoma SddIA (`create`, `update`, `delete`) emita de forma determinista el evento ECST correspondiente en el bus (`Domain_Entity_Created`, `Domain_Entity_Updated`, `Domain_Entity_Deleted`), sin forjas manuales ni huecos por clase de entidad.

## Estado actual (deuda)

| `entity_class` | Creator documentado | Piloto `entity-manager` (create/update) | Sello `emit-domain-mutation` en create/update | Delete → evento |
|----------------|---------------------|----------------------------------------|-----------------------------------------------|-----------------|
| `skill` | `skill-creator` | ✅ Piloto | ✅ (vía `execute-process.py`) | ✅ |
| `event` | `event-creator` | ✅ Piloto | ✅ | ✅ |
| `process` | `process-creator` | ⏳ Pendiente | ❌ | ✅ (delete físico) |
| `agent` | `agent-creator` | ⏳ Pendiente | ❌ | ✅ |
| `tool` | `tool-creator` | ⏳ Pendiente | ❌ | ✅ |
| `action` | `action-creator` | ⏳ Pendiente | ❌ | ✅ |
| `norm` | `norm-creator` | ⏳ Pendiente | ❌ | ✅ |
| `codex` | `codex-creator` | ⏳ Pendiente | ❌ | ✅ |

**Síntoma reciente:** `markdown-table-editor` forjada en Hito 2 PBI-005 sin pasar por `entity-manager` → **no** se generó `Domain_Entity_Created` (índice actualizado a mano; bus sin instancia).

**Invariante normativo:** solo `entity-manager` cierra con `action:emit-domain-mutation` (`SddIA/process/entity-manager.md`). Los `*-creator` no sustituyen el sello.

## Criterios de aceptación

1. **Cobertura de clases:** Las siete clases en la tabla anterior pasan por `entity-manager` en create/update/delete (o quedan explícitamente excluidas en norma con justificación).
2. **Emisión obligatoria:** Tras cada `lifecycle_operation` válida, existe un JSON en `docs/events/pending/` (ruta SSOT `cumulo.paths.json` → `eda_bus.pending`) con `event_type` correcto y payload ECST conforme a `SddIA/events/domain-entity-*.md`.
3. **Handlers físicos:** `execute-process.py` (o puerta sucesora) implementa delegación real a cada `*-creator` en piloto ampliado, no solo simulación de fases.
4. **Prohibición de atajos:** Forja directa de `.md` + fila en `index.md` sin `entity-manager` queda documentada como **Ruido de Sistema** en validación de features.
5. **Suscripciones:** `event-subscriptions.json` mantiene `sync-entity-index` (y DLT donde aplique) para los tres tipos de mutación; el watcher despacha vía `execute-action.py` (Hito 2 PBI-005).

## Tareas (backlog)

### Fase A — Ampliación del piloto `entity-manager`

- [ ] Extender `PILOT_ENTITY_CLASSES` y mapeo `semantic_seed` → `process_inputs` en `SddIA/scripts/qa/execute-process.py` para: `tool`, `action`, `process`, `agent`, `norm`, `codex`.
- [ ] Implementar handlers físicos mínimos por creator (o encadenar `execute-process` recursivo con contrato estable).
- [ ] Actualizar tabla de estado en `SddIA/process/entity-manager.md` (Piloto → Completado por clase).

### Fase B — Gobernanza y validación

- [ ] Checklist en `verify-process-integrity` o script QA dedicado: toda entidad nueva en `*/index.md` tiene evento `Domain_Entity_Created` correlacionado (uuid + nombre).
- [ ] Actualizar `features-documentation-pattern` / guías de Tekton: mutaciones de genoma **solo** vía `entity-manager`.
- [ ] Prueba E2E por clase: create → pending → watcher → `processed/` + auditoría `sync-entity-index`.

### Fase C — Remediación y deuda histórica

- [ ] Inventariar entidades forjadas sin evento (ej. `markdown-table-editor`) y decidir: emisión retroactiva manual, re-forja simulada, o acta de excepción documentada.
- [ ] Actualizar PBI-005 / Ola A cuando CA EDA cubra altas, no solo deletes.

### Deuda relacionada (laboratorio)

- [ ] Handler físico del proceso `feature` — ver `docs/todos/[ARQUITECTURA] Laboratorio — Handler físico proceso feature.md`.

## Referencias

| Artefacto | Ruta |
|-----------|------|
| Gestor de entidad | `SddIA/process/entity-manager.md` |
| Sello universal | `SddIA/actions/emit-domain-mutation.md` |
| Suscripciones | `SddIA/core/event-subscriptions.json` |
| Piloto laboratorio | `SddIA/scripts/qa/execute-process.py` (`PILOT_ENTITY_CLASSES`) |
| Ola C (ECST) | `docs/features/ola-c-event-entity/` |

## Definición de hecho

- [ ] Matriz de la sección «Estado actual» sin celdas ❌ en create/update.
- [ ] Al menos una prueba automatizada o script QA reproducible por `entity_class`.
- [ ] Documento operativo Ola A / PBI actualizado con enlace a este TODO.
