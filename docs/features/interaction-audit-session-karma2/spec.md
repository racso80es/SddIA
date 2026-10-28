---
feature_name: interaction-audit-session-karma2
created: "2026-10-28"
process: feature
pbi_ref: AUT-10
tracker_ref: AUT-10
---

# Spec — interaction_audit karma2 (AUT-10)

## Tabla de interacciones (v2)

Cabecera canónica en `docs/audits/{tracker_ref}.md`:

| entity_type | entity_id | invoked_by | timestamp |
| --- | --- | --- | --- |

- `entity_type` ∈ `skill` \| `tool` \| `action` \| `process`
- `timestamp`: ISO-8601 UTC con resolución de traza (segundos mínimo; no redondeo a minuto)
- Dedup: `(entity_type, entity_id, invoked_by, timestamp)`

## Frontmatter del artefacto

| Campo | Regla |
| --- | --- |
| `uuid` | UUID v4 obligatorio en alta |
| `tracker_ref` | Identificador Linear |
| `type` | `audit` |
| `process` | `llm-interaction-audit` |

## API `llm-interaction-audit` / `record_interaction`

Inputs obligatorios: `tracker_ref`, `entity_type`, `entity_id`, `invoked_by`, `timestamp`.

Retirado del contrato de fila: `rol`, `tipo`, `resumen` (legacy AUT-7).

## Peaje termodinámico

Tras cada invocación de `feature` \| `bug-fix` \| `refactorization` con `llm_audit: active`, registrar una fila:

- `entity_type`: `process`
- `entity_id`: nombre del proceso
- `invoked_by`: `execute-process`
- `timestamp`: instante del peaje (UTC)

La fricción en fallo (`record_friction`) se mantiene sin cambio.

## Fuera de alcance

- Reescritura de `Interaccones_llm.md`
- Cambios en frontmatter YAML de `karma2-token.md`
