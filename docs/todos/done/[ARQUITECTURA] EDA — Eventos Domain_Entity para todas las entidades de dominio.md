---
document_id: TODO-EDA-DOMAIN-ENTITIES
title: "[ARQUITECTURA] EDA — Eventos Domain_Entity_* para todas las entidades de dominio"
format: markdown
version: "1.1.0"
created: "2026-05-19"
updated: "2026-05-20"
status: "cerrado_laboratorio"
priority: alta
blocks: "Cierre CA EDA / ampliación piloto entity-manager"
feature_ref: docs/features/eda-domain-entities-splus
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
| `skill` | `skill-creator` | ✅ Piloto | ✅ | ✅ |
| `event` | `event-creator` | ✅ Piloto | ✅ | ✅ |
| `process` | `process-creator` | ✅ Piloto S+ | ✅ | ✅ |
| `agent` | `agent-creator` | ✅ Piloto S+ | ✅ | ✅ |
| `tool` | `tool-creator` | ✅ Piloto S+ | ✅ | ✅ |
| `action` | `action-creator` | ✅ Piloto S+ | ✅ | ✅ |
| `norm` | `norm-creator` | ✅ Piloto S+ | ✅ | ✅ |
| `codex` | `codex-creator` | ✅ Piloto S+ | ✅ | ✅ |

**Remediación histórica (2026-05-20):** backfill Fase C en `docs/features/eda-domain-entities-splus/` — 40 huérfanas selladas; `orphan_count: 0` en audit post-lote.

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

- [x] Extender `PILOT_ENTITY_CLASSES` y mapeo `semantic_seed` → `process_inputs` en laboratorio para: `tool`, `action`, `process`, `agent`, `norm`, `codex`.
- [x] Implementar handlers físicos mínimos por creator (`execute_process_forges.py`, `execute_process_capsules.py`).
- [x] Actualizar tabla de estado en `SddIA/process/entity-manager.md`.

### Fase B — Gobernanza y validación

- [x] Script QA dedicado: `audit-entity-eda-coverage.py` (`--scan`, `--emit`, `--anchor-merkle`).
- [x] Actualizar `features-documentation-pattern` — Ruido de Sistema EDA.
- [x] Prueba E2E: `run-eda-e2e-lab.py` + aduana en `delivery-close-cycle`.

### Fase C — Remediación y deuda histórica

- [x] Inventario y emisión retroactiva (`--emit --skip-dlt`, 40 entidades); acta Merkle obligatoria al cierre.
- [x] Actualizar PBI-005 / Ola A con enlace a feature `eda-domain-entities-splus`.

### Deuda relacionada (laboratorio)

- [x] Handler físico del proceso `feature` — cerrado; ver `docs/todos/done/[ARQUITECTURA] Laboratorio — Handler físico proceso feature.md` (sustituido por `workspace-init`, PR #9).

## Referencias

| Artefacto | Ruta |
|-----------|------|
| Gestor de entidad | `SddIA/process/entity-manager.md` |
| Sello universal | `SddIA/actions/emit-domain-mutation.md` |
| Suscripciones | `SddIA/core/event-subscriptions.json` |
| Piloto laboratorio | `SddIA/scripts/qa/execute-process.py`, `execute_process_capsules.py` |
| Audit / backfill | `SddIA/scripts/qa/audit-entity-eda-coverage.py` |
| Feature S+ | `docs/features/eda-domain-entities-splus/` |
| Ola C (ECST) | `docs/features/ola-c-event-entity/` |

## Definición de hecho

- [x] Matriz de la sección «Estado actual» sin celdas ❌ en create/update.
- [x] Script QA reproducible: `audit-entity-eda-coverage.py`, `run-eda-e2e-lab.py`.
- [x] Documento operativo Ola A actualizado con enlace a feature `eda-domain-entities-splus`.
