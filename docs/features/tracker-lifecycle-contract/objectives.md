---
feature_name: tracker-lifecycle-contract
created: "2026-10-22"
process: feature
branch_name: feat/tracker-lifecycle-contract
persist_ref: docs/features/tracker-lifecycle-contract
pbi_ref: docs/todos/done/[ARQUITECTURA] HU-003 PBI-01 — tracker lifecycle.md
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-01
pbi_uuid: "1b17e6fa-a8f3-4ce3-91a4-7aff1954a910"
execution_id: "3e0a8c0f-1b97-4b9c-a467-b149494b3e28"
---

# Objetivos — tracker-lifecycle-contract

## Misión

Registrar la capacidad `tracker:lifecycle` antes de que ningún proceso la pida. El id del catálogo es `dominio:verbo`. El contrato de salida es `tracker.lifecycle` y valida el stdout real de `emit_v2`.

## Alcance

| Dentro | Fuera |
|--------|-------|
| Fila en `capability-taxonomy` v1.0.9 (`id`, `contract`, `version` 1.0.0) | Operaciones nuevas en la cápsula |
| Una fila en `capability-bindings` v1.7.0 hacia `tool:linear-tracker-adapter` | `provides` de la tool (PBI 02) |
| `tracker.lifecycle.schema.json` sobre `cap.body` de `emit_v2`, con `additionalProperties` | Enum `file` del manifiesto (PBI 13 y 14) |
| Prueba del validador de salida y del filtro A por id ausente | Resolución condicionada al provider efectivo |

## Criterios

| ID | Criterio |
|----|----------|
| AC-1 | `requires_capability: tracker:lifecycle` no falla el filtro A por id ausente. |
| AC-2 | El schema acepta un cuerpo `emit_v2` de éxito y rechaza uno sin `success`/`exitCode`. |
| AC-3 | Sigue habiendo una sola fila por `capability_id`. |

## Ley aplicada

- Alta taxonómica antes de uso (HU §3.1 y §3.8).
- `norm-creator` en update no preserva `catalog`. Materialización = write del `{name}.md` + sello `Domain_Entity_Updated`.
- Genoma bajo topología `docs/features/tracker-lifecycle-contract/` y registro en `SddIA/evolution/`.
