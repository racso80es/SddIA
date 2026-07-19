---
feature_name: telemetria-activa-domain-entity-updated
created: "2026-07-17"
process: feature
purpose: Estabilización Mayeuta del PBI de ingesta telemetría activa vía Domain_Entity_Updated
---

# Clarificación — telemetria-activa-domain-entity-updated

Transcript Mayeuta (2026-07-17). Semilla: PBI v1.0.0 «Por Refinar» → v1.1.0 **Refinado**.

---

## D0 — Apertura formal

| Pregunta | Decisión |
|----------|----------|
| Proceso | `feature` v1.3.0 |
| `feature_name` | `telemetria-activa-domain-entity-updated` |
| Rama | `feat/telemetria-activa-domain-entity-updated` |
| `persist_ref` | `docs/features/telemetria-activa-domain-entity-updated` |
| `document_id` | `PBI-TELEMETRIA-ACTIVA-DOMAIN-ENTITY-UPDATED` |
| Init lab | `./sddia-run.sh --process feature` + `SDDIA_LAB_SKIP_PBI_ARCHIVE=1` + `SDDIA_LAB_SKIP_DELIVERY_CLOSE=1` → `execution_id` `c58a386d-f5aa-4538-bb2b-bc4a2b4a8936` |
| Fase actual | Tekton T0–T5.1 ejecutado — pendiente Argos/cierre (T5.2) |
| Laudo Dedalo | **Plan B** — `Domain_Entity_Telemetry_Captured` (Plan A rechazado) |

---

## D1 — Emisor (incongruencia corregida)

| Borrador v1.0 | Hecho en `main` | Decisión |
|---------------|-----------------|----------|
| «Orquestador inerte o cápsula de aduana» / tocar `telemetry_batch_stub` | Hot path: `Raw_Execution_Finished` → `route-telemetry` → **`radamanto-batch`** | Emisor = **Radamanto** |
| Stub como refactor target | Stub = residual Fase 3 (purga ciega) | **Fuera de alcance** salvo Kaizen de poda |

---

## D2 — Topología de bus (incongruencia corregida)

| Borrador v1.0 | SSOT Cúmulo | Decisión |
|---------------|-------------|---------|
| `.SddIA/events/pending/domain_entity_updated_[UUID].json` | `eda_fractal.domain` = `./.events/domain` | Escribir vía `write_fractal_event(..., "domain")` |
| Nombre archivo con prefijo tipo | Runtime usa `event_id` como stem | No imponer convención legacy V3+ |

---

## D3 — Contrato Domain_Entity_Updated vs telemetría

| Tensión | Resolución |
|---------|------------|
| Clase actual = CRUD genómico (`hash_signature_*`, `lifecycle_operation` create/update/delete, emisores entity-manager) | **Plan A:** extender con `lifecycle_operation: telemetry_snapshot` + campos métricos |
| Suscriptores actuales (`sync-entity-index`, IOTA) romperían / ruido | Filtro `applies_to_lifecycle` — index/IOTA solo CRUD; memory solo snapshot |
| Payload v1.0 con solo `mutation_type` / `state_*` | Sustituir `mutation_type` por discriminador alineado a schema existente (`lifecycle_operation`) |
| Plan B (laudo Dedalo) | Nueva clase `Domain_Entity_Telemetry_Captured` si validadores ECST no admiten Plan A |

---

## D4 — Persistencia vectorial (incongruencia corregida)

| Borrador v1.0 | Genoma real | Decisión |
|---------------|-------------|----------|
| `EvolutionNode` inventado en pseudocódigo | `EvolutionEvent` en `core/memory` | Usar modelo existente |
| Handler suelto en `core/memory/src/services/` | `EvolutionProxyService` + port `EvolutionStore` | Cablear proxy ↔ `LanceDbEvolutionAdapter` |
| «lancedb_evolution_repo» sin ruta | `SddIA/infrastructure/adapters/lancedb_evolution_repo` | SSOT adapter |
| Adapter hoy mock (`Ok(())`) | Persistencia real = parte del valor del PBI | Tekton debe materializar store o declarar deuda explícita en spec |

Sello post-ingesta: preferir `Vector_Memory_Indexed` (clase existente) frente a inventar evento nuevo.

---

## D5 — Ortogonalidad Self-Healing

| Flujo | Rol |
|-------|-----|
| Umbrales → Degraded / Restored / Deprecated | Gobernanza (sin cambio) |
| Snapshot por ejecución → Domain_Entity_Updated + LanceDB | Trazabilidad / memoria (este PBI) |
| `telemetry-compliance-audit` | Cumplimiento termodinámico (sin cambio) |

---

## D6 — Dependencias

| Feature | Por qué |
|---------|---------|
| `telemetria-reactiva-eda-fase4` | Radamanto + bus fractal telemetry |
| `adecuar-ed-telemetry` | Taxonomía `Domain_Entity_*` agnóstica |
| `memoria-vectorial` / `boveda-evolucion-epigenetica` | Puerto memory + adapter evolution |

---

## D7 — Criterios de aceptación (estabilizados)

1. Chispa domain `telemetry_snapshot` tras telemetría real.
2. Route-domain → memory ingest; **no** sync-entity-index.
3. Registro en store evolution correlacionable.
4. Sin regresión CRUD genómico.
5. Cierre documental single-PR (`pbi_archived: true` en rama).

---

## Preguntas cerradas / abiertas

| ID | Estado |
|----|--------|
| Emisor | **Cerrada** — Radamanto |
| Ruta bus | **Cerrada** — `./.events/domain/` |
| Modelo persistencia | **Cerrada** — `EvolutionEvent` + adapter |
| Plan A vs Plan B event_type | **Cerrada (Dedalo)** — Plan B; ECST REQUIRED plano bloquea Plan A |
| Embeddings obligatorios en v1 | **Cerrada** — metadata-first; `embedding: None` OK |

---

## D8 — Laudo Dedalo (2026-07-19)

| Evidencia | Conclusión |
|-----------|------------|
| `domain-entity-updated.md` lista REQUIRED genómicos planos | Snapshot no puede satisfacerlos sin mentir hashes |
| `ecst_validation.rs` sin REQUIRED condicional | Extender Updated = romper gate o debilitar CRUD |
| Patrón Degraded/Restored/Deprecated | Clase semántica propia es canónica |

`event_type` de entrega: **`Domain_Entity_Telemetry_Captured`**. Título PBI se conserva como narrativa de negocio.
