---
uuid: "62f08bbd-e9ce-479d-8d1b-792684e1bd26"
name: "entity-manager"
version: "1.0.0"
contract: "process-contract v1.3.0"
context: "ecosystem-evolution"
hash_signature: sha256:0d5784440047ba18b0df605ecf27b5ec8fbe1ae72293bc553abc4210d9d2b857
inputs:
  - "entity_class": "string; enum: process | agent | skill | tool | action | norm | codex | event"
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
    intent: "En create/update, invocar action:execute-process con el *-creator según entity_class. Piloto S+: las 8 clases (skill, event, process, agent, tool, action, norm, codex)."
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

| `entity_class` | Proceso hijo | Estado S+ |
|----------------|--------------|-----------|
| `skill` | `skill-creator` | **Piloto** |
| `event` | `event-creator` | **Piloto** |
| `process` | `process-creator` | **Piloto** |
| `agent` | `agent-creator` | **Piloto** |
| `tool` | `tool-creator` | **Piloto** |
| `action` | `action-creator` | **Piloto** |
| `norm` | `norm-creator` | **Piloto** |
| `codex` | `codex-creator` | **Piloto** |

Entradas bajo `SddIA/evolution/` **no** pasan por este proceso (no emiten `Domain_Entity_*`).

## Fase 1 — Delegación al creator

**Omitida** cuando `lifecycle_operation` es `delete`.

1. Resolver `process_name` desde la tabla anterior.
2. Si `entity_class` no está en la tabla: `status_code: 1`, error documentado.
3. Resolver `origin_topology` desde `semantic_seed.scope` (`core` → `core`, `local` → `local`) o desde `semantic_seed.origin_topology` explícito. Propagar al handoff y a Fase 3.
4. Mapear `semantic_seed` → `process_inputs` del hijo según tablas por clase:

| Campo `semantic_seed` | Input `skill-creator` |
|-----------------------|-------------------------|
| `skill_name` o `entity_name` | `skill_name` |
| `skill_context` | `skill_context` (default `ecosystem-evolution`) |
| `skill_description` | `skill_description` |
| `skill_inputs_schema` | obligatorio en semilla |
| `skill_outputs_schema` | obligatorio en semilla |
| `skill_version` | default `1.0.0` |
| `skills_contract_version` | desde contrato vigente |

| Campo `semantic_seed` | Input `event-creator` |
|-----------------------|-------------------------|
| `event_name` o `entity_name` | `event_name` |
| `event_type` | obligatorio (PascalCase_Snake) |
| `event_context` | default `ecosystem-evolution` |
| `event_description` | descripción de la Clase |
| `payload_required` | array de campos ECST |
| `payload_optional` | array |
| `payload_forbidden` | array |
| `emitter_agents` | array de emisores autorizados |
| `event_version` | default `1.0.0` |
| `events_contract_version` | desde `events-contract.md` |

| Campo `semantic_seed` | Input `tool-creator` |
|-----------------------|----------------------|
| `tool_name` o `entity_name` | `tool_name` |
| `scope` | `core` \| `local` (default `core`) |
| `domain_origin` | default `SddIA` |
| `tool_context` | default `ecosystem-evolution` |
| `required_secrets`, `dependencies`, `tool_outputs`, `execution_logic` | según contrato tools |

| Campo `semantic_seed` | Input `action-creator` |
|-----------------------|------------------------|
| `action_name` o `entity_name` | `action_name` |
| `action_context` | default `ecosystem-evolution` |
| `action_inputs`, `action_outputs`, `orchestration_logic` | según contrato actions |

| Campo `semantic_seed` | Input `process-creator` |
|-----------------------|-------------------------|
| `process_name` o `entity_name` | `process_name` |
| `process_description`, `process_context`, `process_phases` | según contrato process |
| `process_contract_version` | default `1.3.0` |

| Campo `semantic_seed` | Input `agent-creator` |
|-----------------------|-----------------------|
| `agent_name` o `entity_name` | `agent_name` |
| `allowed_policies`, `agent_inputs`, `agent_outputs`, `agent_purpose` | según contrato agents |

| Campo `semantic_seed` | Input `norm-creator` |
|-----------------------|----------------------|
| `tactical_norm_name` o `entity_name` | `tactical_norm_name` |
| `tactical_norm_version`, `tactical_norm_friction`, `tactical_norm_author` | según contrato norms |
| `norm_scope`, `norm_category` | opcionales |

| Campo `semantic_seed` | Input `codex-creator` |
|-----------------------|-----------------------|
| `domain_codex_slug` o `entity_name` | `domain_codex_slug` |
| `domain_codex_name`, `target_environment`, `tactical_norm_inventory` | según contrato codex |

5. Invocar `action:execute-process` con `process_name` canónico y `process_inputs` mapeados.
6. Extraer del `execution_report` / outputs del hijo: `handoff_entity_uuid`, `handoff_hash_signature_new`, `handoff_hash_signature_old`, `handoff_version`.

Orden: forja + indexación síncrona del creator → sub-fase 2.5 → sello en Fase 3.

## Fase 2.5 — Idempotencia (Protocolo Acero Pilar 3)

Antes del sello universal:

1. Si `lifecycle_operation` es `create`: comprobar en `eda_bus` si ya existe `Domain_Entity_Created` para `handoff_entity_uuid`. Si existe → omitir Fase 3; propagar `event_id` existente.
2. Si forja idempotente detectó artefacto preexistente → mismo criterio (un sello por UUID).
3. En `update`/`delete`: comprobar duplicado de mismo `lifecycle_operation` + `hash_signature_new`/`hash_signature_old` cuando aplique.

Implementación lab: `execute_process_capsules.py` → `find_existing_domain_event`, `assert_idempotent_emit`.

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
| `origin_topology` | resuelto en Fase 1 (`core` \| `local`) |

Propagar `event_id` y `target_path` a outputs del proceso.

### Mandato DLT (Pilar 2 — post-sello core)

Tras persistir el JSON en `eda_bus.pending`:

1. El **watcher** (`event-watcher.py` → `route-domain-event`) filtra suscriptores por `applies_to_origin_topology`.
2. Solo eventos con `origin_topology=core` y `event_type=Domain_Entity_Created` disparan `iota-immutable-publisher` si el umbral DLT está satisfecho (`hash_signature_new` válido, no placeholder).
3. Emisiones con `emitter_agent=cumulo-eda-backfill` omiten DLT por entidad (Fase C); el cierre exige `--anchor-merkle` con acta IOTA del lote.
4. Eventos `origin_topology=local` no mutan índices canónicos bajo `SddIA/` (fan-out acotado por suscripción).

## Límites

* No calcula hashes de entidades en create/update (delegado al creator hijo).
* No enruta el bus (`route-domain-event` es asíncrono vía watcher).
* Tekton no materializa forja Core directamente; solo orquesta hijos vía `execute-process`.
