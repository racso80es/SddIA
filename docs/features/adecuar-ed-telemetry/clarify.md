---
feature_name: adecuar-ed-telemetry
created: "2026-05-29"
process: feature
purpose: Cierre de decisiones arquitectónicas — enrutamiento agnóstico Domain_Entity_*
version_clarify: "1.0.0"
---

# Clarificación — Enrutamiento semántico agnóstico

Transcript de decisiones (2026-05-29) para estabilizar requisitos antes de `spec.md` / `plan.md`.

---

## D1 — Inicio formal

| Pregunta | Decisión |
|----------|----------|
| ¿Proceso de inicio? | **`feature`** v1.3.0 |
| Rama | `feat/adecuar-ed-telemetry` |
| `persist_ref` | `docs/features/adecuar-ed-telemetry` |
| PBI | `docs/todos/pending/event_domain_subscriptions_Adecuar_ED_Telemetry.md` |
| Inputs SSOT | `_init-feature.json` |
| Dependencias lógicas | `eda-domain-entities-splus` (CRUD universal), `telemetria-reactiva-eda-fase4` (Radamanto Self-Healing) |

---

## D2 — Campos de enrutamiento en payload

| Pregunta | Decisión |
|----------|----------|
| PBI nombra `entity_type` + `entity_id` | Adoptar como **par canónico de routing** en todos los eventos `Domain_Entity_*` |
| ECST actual usa `entity_class` + `entity_uuid` (CRUD) | **Mantener** en eventos create/update/delete por compatibilidad; **añadir** `entity_type` (= `entity_class`) e `entity_id` (= `entity_uuid`) como REQUIRED duplicado semántico hasta migración única en fase posterior |
| Radamanto usa `target_entity_id` | En degraded/deprecated/restored: `entity_id` **alias** de `target_entity_id`; retirar `target_entity_id` como REQUIRED tras migración runtime |
| Enum `entity_type` | Mismo universo que `entity_class` en `emit-domain-mutation`: `process \| agent \| skill \| tool \| action \| norm \| codex \| event` |

**Motivo:** cumplir decreto PBI sin romper consumidores CRUD ya desplegados; Tekton consolida en una sola pasada.

---

## D3 — Mapa de eventos acoplados → agnósticos

| Evento fósil (actual) | Evento objetivo | Suscriptores actuales | Notas |
|------------------------|-----------------|----------------------|-------|
| *(ya migrado)* | `Domain_Entity_Created` | Cúmulo sync + DLT | Sin cambio |
| *(ya migrado)* | `Domain_Entity_Updated` | Cúmulo sync + DLT | Sin cambio |
| *(ya migrado)* | `Domain_Entity_Deleted` | Cúmulo sync + DLT | Sin cambio |
| `Tool_Degraded` | `Domain_Entity_Degraded` | Cerbero, Dedalo (`fix-tool-process`), Radamanto DLT | Filtrar `entity_type=tool` en handler si aplica |
| `Tool_Deprecated` | `Domain_Entity_Deprecated` | Cerbero, Radamanto DLT | Idem |
| `Status_Restored` | `Domain_Entity_Restored` | Cerbero, Radamanto DLT | Simetría con degraded/deprecated; PBI no lo nombra pero es acoplamiento equivalente |

**Regla:** ninguna clave nueva por entidad (`Skill_Degraded`, etc.). Un solo suscriptor por acción semántica; discriminación vía payload.

---

## D4 — Artefactos genoma (Fase D)

| Decisión | Detalle |
|----------|---------|
| Hard override fósiles | Eliminar `tool-degraded.md`, `tool-deprecated.md`, `status-restored.md` |
| Reemplazo | Forjar `domain-entity-degraded.md`, `domain-entity-deprecated.md`, `domain-entity-restored.md` (nombres kebab alineados a familia existente) |
| Índice | Actualizar `SddIA/events/domain/index.md`; bump `indexed_at` |
| UUID | Nuevas Clases ECST con UUID v4; acta Merkle si backfill coverage |

---

## D5 — Válvula emisora (Fase C)

| Pregunta | Decisión |
|----------|----------|
| Nomenclatura archivos `pending/` | **UUID** (`{event_id}.json`) — ya canónico; PBI menciona `domain-entity-created_*.json` como contraste conceptual con legado `tool-created_*`, no como nuevo formato |
| `emit-domain-mutation` | Ampliar tabla lifecycle → incluir `deprecated`, `degraded`, `restored` **solo si** el invocante es Radamanto o proceso autorizado; CRUD sigue vía `entity-manager` |
| Radamanto batch | Retarget `build_domain_event("Tool_*")` → `Domain_Entity_*`; payload con `entity_type` + `entity_id` |

---

## D6 — Consumidores y filtrado

| Consumidor | Cambio |
|------------|--------|
| `cerbero-governance-react` | Resolver `entity_id` desde payload; ramificar RBAC por `entity_type` |
| `fix-tool-process` | Suscrito a `Domain_Entity_Degraded`; gate: `entity_type == "tool"` |
| `radamanto_batch_core` | Emisor + DLT bajo tipos agnósticos |
| `route-domain-event` | Sin cambio estructural — fan-out por clave JSON suscripciones |

---

## D7 — Triaje de bloqueos

| Área | Estado | Veredicto |
|------|--------|-----------|
| CRUD `Domain_Entity_*` en main | ✅ | Sin bloqueo |
| Self-Healing Fase 4 mergeado | ✅ | Base Radamanto operativa |
| Clases ECST degraded/deprecated/restored agnósticas | ❌ No existen | **Alcance Tekton B+D** |
| Tests `test_radamanto_dlt_tool_status.py` | ⚠️ Referencias `Tool_*` | Actualizar en ejecución |
| `eda-coverage.json` | ⚠️ Requiere upsert post-forja | Gate pre-merge |

**Conclusión:** ejecutable sin ambigüedad bloqueante. Riesgo principal: **regresión Self-Healing** si suscriptores no filtran payload correctamente.

---

## D8 — Cierre documental

| Pregunta | Decisión |
|----------|----------|
| ¿Archivar PBI al merge? | **Sí** — feature única; `pbi_archived: true` en `validacion.md` |
| ¿Segundo PR documental? | **Prohibido** — regla task-closure-documental |

---

## Referencias

- PBI: `docs/todos/pending/event_domain_subscriptions_Adecuar_ED_Telemetry.md`
- Suscripciones SSOT: `SddIA/core/event-domain-subscriptions.json`
- Decisión Radamanto D4.x: `docs/features/telemetria-reactiva-eda-fase4/clarify.md`
- CRUD universal: `docs/features/eda-domain-entities-splus/objectives.md`
