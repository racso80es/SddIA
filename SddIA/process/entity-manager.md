---
uuid: "62f08bbd-e9ce-479d-8d1b-792684e1bd26"
name: "entity-manager"
version: "1.0.0"
contract: "process-contract v1.3.0"
context: "ecosystem-evolution"
hash_signature: "sha256:5adacc0a536e9347c6283a7ff2ecd09572b577ce29bfa93195b6495cd5234bb8"
inputs:
  - "entity_class": "string; enum: process | agent | skill | tool | action | norm | codex"
  - "entity_name": "string; identificador kebab-case de la entidad"
  - "lifecycle_operation": "string; enum: create | update | delete"
  - "semantic_seed": "object|null; parámetros de forja para el creator hijo; ignorado en delete"
  - "cumulo_topology": "Topología SSOT inyectada (paths, contratos, directorios)"
outputs:
  - "event_id": "UUID v4 del evento Domain_Entity_* en eda_bus.pending"
  - "target_path": "Ruta relativa del JSON emitido en pending/"
  - "handoff_entity_uuid": "UUID de la entidad afectada (desde creator o frontmatter en delete)"
  - "handoff_hash_signature_new": "Sello post-mutación o null en delete"
  - "handoff_hash_signature_old": "Sello pre-mutación o null en create"
  - "handoff_version": "Versión SemVer resultante"
phases:
  - name: "Delegación al creator"
    intent: "En create/update, invocar action:execute-process con el *-creator según entity_class. Piloto v1: skill → skill-creator. Resto: abortar con mensaje hasta ampliación de mapeo."
    delegates_to:
      - "action:execute-process"
  - name: "Delete físico"
    intent: "Solo en delete: READ_FILE del artefacto para extraer uuid/version/hash_signature; DELETE_FILE del .md bajo directories.*."
    delegates_to:
      - "skill:filesystem-manager"
  - name: "Sello universal"
    intent: "Invocar action:emit-domain-mutation con emitter_agent entity-manager y handoff del creator o metadatos de delete."
    delegates_to:
      - "action:emit-domain-mutation"
minteo_maximo: null
porcentaje_de_exito: null
---

# entity-manager

Proceso orquestador (**Gestor de Entidad**) que envuelve los `*-creator` del genoma y cierra siempre con el sello EDA `emit-domain-mutation`. Patrón de delegación análogo a `feature` → `delivery-close-cycle`.

## Tabla de delegación

| `entity_class` | Proceso hijo | Estado v1 |
|----------------|--------------|-----------|
| `skill` | `skill-creator` | **Piloto** |
| `process` | `process-creator` | Pendiente |
| `agent` | `agent-creator` | Pendiente |
| `tool` | `tool-creator` | Pendiente |
| `action` | `action-creator` | Pendiente |
| `norm` | `norm-creator` | Pendiente |
| `codex` | `codex-creator` | Pendiente |

Entradas bajo `SddIA/evolution/` **no** pasan por este proceso (no emiten `Domain_Entity_*`).

## Fase 1 — Delegación al creator

**Omitida** cuando `lifecycle_operation` es `delete`.

1. Resolver `process_name` desde la tabla anterior.
2. Si `entity_class` no está en piloto v1: `status_code: 1`, error documentado.
3. Mapear `semantic_seed` → `process_inputs` del hijo (piloto `skill`):

| Campo `semantic_seed` | Input `skill-creator` |
|-----------------------|-------------------------|
| `skill_name` o `entity_name` | `skill_name` |
| `skill_context` | `skill_context` (default `ecosystem-evolution`) |
| `skill_description` | `skill_description` |
| `skill_inputs_schema` | obligatorio en semilla |
| `skill_outputs_schema` | obligatorio en semilla |
| `skill_version` | default `1.0.0` |
| `skills_contract_version` | desde contrato vigente |

4. Invocar `action:execute-process` con `process_name` canónico y `process_inputs` mapeados.
5. Extraer del `execution_report` / outputs del hijo: `handoff_entity_uuid`, `handoff_hash_signature_new`, `handoff_hash_signature_old`, `handoff_version`.

Orden: forja + indexación síncrona del creator → sello en Fase 3.

## Fase 2 — Delete físico

Solo cuando `lifecycle_operation` es `delete`:

1. Resolver ruta `{directories.<entity_class>}/{entity_name}.md` vía `cumulo_topology`.
2. `filesystem-manager` → `READ_FILE` para parsear frontmatter (`uuid`, `version`, `hash_signature`).
3. `filesystem-manager` → `DELETE_FILE` del artefacto.
4. Poblar handoff: `handoff_hash_signature_old` desde frontmatter; `handoff_hash_signature_new` = `null`; `handoff_entity_uuid` y `handoff_version` desde frontmatter.

## Fase 3 — Sello universal

Invocar `action:emit-domain-mutation`:

| Campo | Valor |
|-------|--------|
| `entity_class` | input |
| `entity_name` | input |
| `lifecycle_operation` | input |
| `entity_uuid` | `handoff_entity_uuid` |
| `version` | `handoff_version` (null permitido en delete) |
| `hash_signature_new` | `handoff_hash_signature_new` |
| `hash_signature_old` | `handoff_hash_signature_old` |
| `changes_summary` | `"{lifecycle_operation} {entity_class} {entity_name}"` (≤ 2048) |
| `emitter_agent` | `entity-manager` |

Propagar `event_id` y `target_path` a outputs del proceso.

## Límites

* No calcula hashes de entidades en create/update (delegado al creator hijo).
* No enruta el bus (`route-domain-event` es asíncrono vía watcher).
* Tekton no materializa forja Core directamente; solo orquesta hijos vía `execute-process`.
