---
feature_name: adecuar-ed-telemetry
created: "2026-05-29"
process: feature
base: main
scope: event-domain-subscriptions, domain ECST classes, emit-domain-mutation, radamanto-batch, cerbero-governance-react, fix-tool-process, ecst_validation, eda-coverage, tests QA Self-Healing
pbi_ref: docs/todos/pending/event_domain_subscriptions_Adecuar_ED_Telemetry.md
document_id: PBI-ADECUAR-ED-TELEMETRY
---

# Especificación técnica — Enrutamiento semántico agnóstico (Domain_Entity_*)

## 1. Contexto

Estado actual (post `eda-domain-entities-splus` + `telemetria-reactiva-eda-fase4`):

| Capa | Estado | Problema |
|------|--------|----------|
| CRUD genómico | `Domain_Entity_Created/Updated/Deleted` operativos vía `entity-manager` | Payload usa `entity_class` + `entity_uuid`; **faltan** `entity_type` + `entity_id` de routing |
| Self-Healing Radamanto | Emite `Tool_Degraded`, `Tool_Deprecated`, `Status_Restored` | Acoplamiento temprano al nombre «Tool»; suscripciones duplicarían por cada ED futura |
| Genoma `SddIA/events/domain/` | Clases fósiles `tool-degraded.md`, `tool-deprecated.md`, `status-restored.md` | Contradicen Simetría Fractal y Ceguera Espacial |
| `event-domain-subscriptions.json` | Claves `Tool_*` y `Status_Restored` | Fan-out no agnóstico |
| Consumidores | `cerbero_governance_react_core` ramifica por `event_type` acoplado | Debe filtrar por `entity_type` en payload |
| `emit-domain-mutation` | Traduce solo `create/update/delete` | Gobernanza Radamanto no pasa por esta acción (correcto); CRUD debe enriquecer payload |

Objetivo: **una clave de suscripción por acción semántica** (`Domain_Entity_Degraded`, etc.); la identidad concreta de la ED vive en el payload.

## 2. Arquitectura objetivo

```text
                    ┌─────────────────────────────────────────┐
                    │     event-domain-subscriptions.json      │
                    │  Domain_Entity_Degraded / Deprecated /     │
                    │  Restored  (+ CRUD existentes)           │
                    └──────────────────┬──────────────────────┘
                                       │ fan-out por event_type
         ┌─────────────────────────────┼─────────────────────────────┐
         ▼                             ▼                             ▼
 cerbero-governance-react      fix-tool-process              radamanto → DLT
 (filtra entity_type)           (gate entity_type=tool)       (exclusividad preservada)

Radamanto-batch emite:
  Domain_Entity_Degraded   { entity_type, entity_id, reason, … }
  Domain_Entity_Restored   { entity_type, entity_id, success_rate, … }
  Domain_Entity_Deprecated { entity_type, entity_id, recovery_attempts, … }

entity-manager / emit-domain-mutation emite CRUD con:
  entity_class, entity_uuid  (legacy)
  entity_type (= entity_class), entity_id (= entity_uuid)  (routing canónico)
```

## 3. Taxonomía de eventos

### 3.1 Mapa de migración (SSOT)

| event_type legado | event_type objetivo | Emisor autorizado |
|-------------------|---------------------|-------------------|
| `Tool_Degraded` | `Domain_Entity_Degraded` | `radamanto` |
| `Tool_Deprecated` | `Domain_Entity_Deprecated` | `radamanto` |
| `Status_Restored` | `Domain_Entity_Restored` | `radamanto` |
| `Domain_Entity_Created` | *(sin cambio de tipo)* | `entity-manager`, `emit-domain-mutation` |
| `Domain_Entity_Updated` | *(sin cambio)* | idem |
| `Domain_Entity_Deleted` | *(sin cambio)* | idem |

**Prohibido** introducir `{EntityClass}_Degraded` u otras variantes por entidad.

### 3.2 Nuevas Clases ECST (genoma)

| Archivo | event_type | context | capabilities |
|---------|------------|---------|--------------|
| `domain-entity-degraded.md` | `Domain_Entity_Degraded` | `quality-assurance` | `domain_entity_degraded`, `self_healing_trigger` |
| `domain-entity-deprecated.md` | `Domain_Entity_Deprecated` | `quality-assurance` | `domain_entity_deprecated`, `self_healing_death` |
| `domain-entity-restored.md` | `Domain_Entity_Restored` | `quality-assurance` | `domain_entity_restored`, `self_healing_redemption` |

Cabecera obligatoria: `event_family: domain`, `contract: events-contract v1.1.0`, UUID v4 único, `hash_signature` anclado post-forja.

### 3.3 Payload ECST — gobernanza (degraded / deprecated / restored)

#### `Domain_Entity_Degraded`

| Sección | Campos |
|---------|--------|
| **REQUIRED** | `entity_type`, `entity_id`, `reason`, `success_rate`, `recovery_attempt` |
| **OPTIONAL** | `avg_duration_ms` |
| **FORBIDDEN** | `branch`, `pr_url`, `target_entity_id` |

#### `Domain_Entity_Deprecated`

| Sección | Campos |
|---------|--------|
| **REQUIRED** | `entity_type`, `entity_id`, `recovery_attempts`, `reason` |
| **FORBIDDEN** | `branch`, `target_entity_id` |

#### `Domain_Entity_Restored`

| Sección | Campos |
|---------|--------|
| **REQUIRED** | `entity_type`, `entity_id`, `success_rate` |
| **OPTIONAL** | `consecutive_success_count` |
| **FORBIDDEN** | `structure_valid`, `target_entity_id` |

`entity_type` enum: `process | agent | skill | tool | action | norm | codex | event`.

En runtime Radamanto v1: **`entity_type` default `"tool"`** al migrar telemetría CLI (asset_id → tool); extensión futura vía metadatos telemetría sin nueva clave de suscripción.

### 3.4 Payload ECST — CRUD (enriquecimiento)

Clases existentes `domain-entity-created.md`, `domain-entity-updated.md`, `domain-entity-deleted.md`:

| Cambio | Detalle |
|--------|---------|
| Añadir REQUIRED | `entity_type`, `entity_id` |
| Mantener REQUIRED | `entity_class`, `entity_uuid`, resto campos actuales |
| Regla runtime | `entity_type := entity_class`; `entity_id := entity_uuid` en `emit_domain_mutation()` |

Bump versión Clase: `1.0.0` → `1.1.0` en las tres.

## 4. Suscripciones — `event-domain-subscriptions.json`

### 4.1 Eliminar claves

- `Tool_Degraded`
- `Tool_Deprecated`
- `Status_Restored`

### 4.2 Añadir / sustituir claves

```json
"Domain_Entity_Degraded": [
  { "agent": "cerbero", "process": "cerbero-governance-react", "intent": "Revocar RBAC entidad degradada (filtra entity_type)." },
  { "agent": "dedalo", "process": "fix-tool-process", "intent": "Reparación sandbox; gate entity_type=tool." },
  { "agent": "radamanto", "tool": "iota-immutable-publisher", "intent": "Sellar degradación en DLT (exclusividad Radamanto)." }
],
"Domain_Entity_Restored": [
  { "agent": "cerbero", "process": "cerbero-governance-react", "intent": "Rehabilitar RBAC tras redención Radamanto." },
  { "agent": "radamanto", "tool": "iota-immutable-publisher", "intent": "Sellar redención en DLT." }
],
"Domain_Entity_Deprecated": [
  { "agent": "cerbero", "process": "cerbero-governance-react", "intent": "Bloqueo permanente." },
  { "agent": "radamanto", "tool": "iota-immutable-publisher", "intent": "Sellar obsolescencia en DLT." }
]
```

Sin cambio en `Domain_Entity_Created/Updated/Deleted`, `PullRequest_*`, `Suite_*`, `System_*`, `Kaizen_*`, `System_Fracture_*`.

## 5. Runtime — emisores

### 5.1 `radamanto_batch_core.py`

| Función | Cambio |
|---------|--------|
| `build_domain_event` | Sin cambio de forma |
| Emisiones degradación | `"Tool_Degraded"` → `"Domain_Entity_Degraded"`; payload incluye `entity_type: "tool"`, `entity_id: entity_id` (sin `target_entity_id`) |
| Emisiones redención | `"Status_Restored"` → `"Domain_Entity_Restored"` |
| Emisiones muerte | `"Tool_Deprecated"` → `"Domain_Entity_Deprecated"` |
| `actions[].type` | Actualizar strings de log a tipos agnósticos |

Resolución `entity_id`: continúa desde bucket stats / telemetría `asset_id` (hoy siempre herramienta).

### 5.2 `emit_domain_mutation()` (`execute_process_capsules.py`)

Tras ensamblar payload CRUD, inyectar:

```python
payload["entity_type"] = payload["entity_class"]
payload["entity_id"] = payload["entity_uuid"]
```

Validación ECST vía `validate_domain_mutation_event` — debe pasar con schemas 1.1.0.

**Fuera de alcance:** ampliar `lifecycle_operation` a `degraded/deprecated/restored` en `emit-domain-mutation` — Radamanto emite directamente vía `write_fractal_event`; no usa lifecycle CRUD.

### 5.3 `SddIA/actions/emit-domain-mutation.md`

- Documentar campos `entity_type` / `entity_id` en payload Paso 4 (derivados de inputs).
- Bump versión acción `1.0.0` → `1.1.0`.
- Aclarar que nomenclatura archivo en `pending/` es `{event_id}.json` (UUID), no prefijo semántico.

## 6. Runtime — consumidores

### 6.1 `cerbero_governance_react_core.py`

```python
def _resolve_entity_id(payload: dict) -> str | None:
    eid = payload.get("entity_id") or payload.get("target_entity_id")
    ...

def react_to_domain_event(...):
    event_type in (
        "Domain_Entity_Degraded",
        "Domain_Entity_Restored",
        "Domain_Entity_Deprecated",
    )
    entity_type = payload.get("entity_type", "tool")  # compat transitoria
```

Misma semántica RBAC que hoy; solo cambia discriminación por `event_type` agnóstico.

### 6.2 `fix-tool-process` + handler lab

| Artefacto | Cambio |
|-----------|--------|
| `fix-tool-process.md` | Input `event_file_path` genérico «JSON Domain_Entity_Degraded»; bump `1.0.0` → `1.1.0` |
| Handler invocación | Gate: abortar si `payload.entity_type != "tool"` (no-op auditable, `exitCode: 0`, sin sandbox) |
| Suscripción | Solo vía clave `Domain_Entity_Degraded` |

### 6.3 `radamanto.instructions.json`

```json
"dlt_exclusive_events": [
  "Domain_Entity_Degraded",
  "Domain_Entity_Restored",
  "Domain_Entity_Deprecated",
  "System_Immunity_Certified"
],
"rules": {
  "R4.1": "... → Domain_Entity_Degraded",
  ...
}
```

### 6.4 `route-domain-event` / `route_fractal_event_core`

Sin cambio estructural: resuelven suscriptores por clave `event_type` del JSON instancia.

## 7. Higiene genoma (Fase D)

| Acción | Artefacto |
|--------|-----------|
| **DELETE** | `SddIA/events/domain/tool-degraded.md` |
| **DELETE** | `SddIA/events/domain/tool-deprecated.md` |
| **DELETE** | `SddIA/events/domain/status-restored.md` |
| **CREATE** | `domain-entity-degraded.md`, `domain-entity-deprecated.md`, `domain-entity-restored.md` |
| **UPDATE** | `SddIA/events/domain/index.md` — 13 → 13 filas (swap tipos, bump `indexed_at`) |
| **UPDATE** | `domain-entity-created/updated/deleted.md` v1.1.0 |

Referencias residuales en docs históricas (`telemetria-reactiva-eda-fase4/*`) — **no mutar** (auditoría); solo runtime y genoma activo.

## 8. EDA coverage

Tras forja de nuevas Clases:

1. Upsert UUIDs en `SddIA/core/eda-coverage.json` (`is_covered: true`).
2. Retirar entradas huérfanas de UUIDs fósiles eliminados (si indexados).
3. Gate pre-merge: `python SddIA/scripts/qa/audit-entity-eda-coverage.py --scan --json` → `orphan_count: 0`.

## 9. Tests QA

| Test | Cambio esperado |
|------|-----------------|
| `test_radamanto_self_healing.py` | Assert `Domain_Entity_Degraded`, `Domain_Entity_Restored`, `Domain_Entity_Deprecated` |
| `test_radamanto_dlt_tool_status.py` | Tipos agnósticos + payload `entity_type` |
| `test_eda_fractal_bus.py` | Si referencias Tool_* en fixtures, actualizar |
| **Nuevo** `test_cerbero_entity_type_filter.py` (opcional) | `Domain_Entity_Degraded` con `entity_type=skill` → Cerbero revoca; fix-tool no-op |

Suite mínima verde:

```bash
python -m unittest SddIA/scripts/qa/test_radamanto_self_healing.py
python -m unittest SddIA/scripts/qa/test_radamanto_dlt_tool_status.py
python SddIA/scripts/qa/audit-entity-eda-coverage.py --scan --json
```

## 10. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| **AC1** | Cero claves `Tool_*` / `Status_Restored` en `event-domain-subscriptions.json` | Diff + grep |
| **AC2** | Tres Clases agnósticas en genoma; fósiles eliminados | `domain/index.md` + ausencia `tool-degraded.md` |
| **AC3** | CRUD payload incluye `entity_type` + `entity_id` | ECST validation + smoke entity-manager |
| **AC4** | Radamanto emite tipos agnósticos | `test_radamanto_self_healing` |
| **AC5** | Cerbero RBAC operativo post-migración | Test Self-Healing end-to-end |
| **AC6** | DLT Radamanto exclusivo preservado | `test_radamanto_dlt_tool_status` |
| **AC7** | `orphan_count: 0` | `--scan` |
| **AC8** | PBI archivado + `validacion.md` APTO pre-merge | Cierre documental en rama |

## 11. Fuera de alcance

- Renombrar `entity_class` → solo `entity_type` (deuda consolidación futura).
- Generalizar `fix-tool-process` a `fix-entity-process` multi-ED.
- Migrar instancias históricas en `processed/` (convivencia D0.2).
- Cambiar exclusividad DLT Cúmulo vs Radamanto (handoff Fase 4 intacto).

## 12. Referencias

- `docs/features/adecuar-ed-telemetry/clarify.md` (D1–D8)
- `docs/features/telemetria-reactiva-eda-fase4/spec.md` (linaje Self-Healing)
- `docs/features/eda-domain-entities-splus/spec.md` (CRUD universal)
- `SddIA/core/event-domain-subscriptions.json`
- `SddIA/norms/entidades-dominio-ecosistema-sddia.md`
