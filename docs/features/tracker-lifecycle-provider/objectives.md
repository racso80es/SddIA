---
feature_name: tracker-lifecycle-provider
created: "2026-10-22"
process: feature
branch_name: feat/tracker-lifecycle-provider
persist_ref: docs/features/tracker-lifecycle-provider
pbi_ref: docs/todos/done/[ARQUITECTURA] HU-003 PBI-02 — tracker lifecycle.md
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-02
pbi_uuid: "4b7aae4b-1fc6-477f-be07-d03216258493"
execution_id: "7a0b47aa-3afa-424e-afc9-a9cb3c67a038"
---

# Objetivos — tracker-lifecycle-provider

## Misión

Que `linear-tracker-adapter` declare `tracker:lifecycle` y que el anti-eco registre por el provider de la fila de binding, no por una constante con el nombre de la tool.

## Alcance

| Dentro | Fuera |
|--------|-------|
| `provides` id/contract/version iguales a la fila del PBI 01 | Operaciones nuevas en el crate |
| `outputs` `success` y `exitCode` para el gate del schema | WASI |
| Quitar `const TOOL`. El nombre sale de `capability-bindings` | Cambiar `ENTITY_ID` o el binario |
| `update_issue_description` en `RECORDED_OPS` | El peaje `LINEAR_TRACKER_TOOL` (PBI 11) |

## Criterios

| ID | Criterio |
|----|----------|
| AC-1 | `provides` coincide con la fila `tracker:lifecycle` → `tool:linear-tracker-adapter`. |
| AC-2 | Un `update_issue_description` aceptado escribe `.SddIA/state/tracker-outbound/{issue}.jsonl`. |
| AC-3 | El crate sigue con las seis operaciones vigentes. |

## Ley aplicada

- Alta taxonómica del PBI 01 ya está en `main`.
- `tool-creator` update sin parche recrea la entidad. El contenido se escribe y el sello es `entity-manager` con `hash_refresh_only`.
