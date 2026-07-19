---
feature_name: telemetria-activa-domain-entity-updated
created: "2026-07-19"
process: feature
branch: feat/telemetria-activa-domain-entity-updated
global: APTO
pbi_archived: true
pbi_ref: docs/todos/done/[OPERATIVO] PBI: Gestión e Ingesta de Telemetría Activa mediante Domain_Entity_Updated.md
document_id: PBI-TELEMETRIA-ACTIVA-DOMAIN-ENTITY-UPDATED
checks:
  AC1: pass
  AC2: pass
  AC3: pass
  AC4: pass
  AC5: pass
  AC6: pass
git_changes:
  - README.md
  - SddIA/agents/radamanto.md
  - SddIA/agents/radamanto.instructions.json
  - SddIA/core/event-domain-subscriptions.json
  - SddIA/core/eda-coverage.json
  - SddIA/events/domain/domain-entity-telemetry-captured.md
  - SddIA/process/memory-evolution-ingest.md
  - SddIA/process/radamanto-batch.md
  - SddIA/engine/execute-process/src/engine/memory_evolution_ingest_core.rs
  - SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs
  - SddIA/infrastructure/adapters/lancedb_evolution_repo/src/lib.rs
  - docs/features/telemetria-activa-domain-entity-updated/
  - docs/todos/done/[OPERATIVO] PBI: Gestión e Ingesta de Telemetría Activa mediante Domain_Entity_Updated.md
---

# Validación — telemetria-activa-domain-entity-updated

**Veredicto global: APTO**

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| AC1 | Chispa `Domain_Entity_Telemetry_Captured` tras consumo Radamanto | ✅ | Smoke lab `radamanto-batch` + `SDDIA_LAB_ROUTE_SYNC=1` |
| AC2 | `route-domain` → `memory-evolution-ingest`; sin `sync-entity-index` | ✅ | `delivery_status.cumulo.memory-evolution-ingest=success` |
| AC3 | Registro en `.SddIA/vector_store/evolution/` | ✅ | `stim-*.json` con `entity_id` + métricas |
| AC4 | CRUD `Domain_Entity_Updated` intacto (Plan B) | ✅ | Clase/suscripciones CRUD sin cambio semántico |
| AC5 | EDA coverage `orphan_count: 0` | ✅ | `sddia-qa audit-eda-coverage --scan` |
| AC6 | PBI archivado en rama + `pbi_archived: true` | ✅ | Este documento + PBI en `docs/todos/done/` |

## Comandos (laboratorio)

```bash
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo test -p execute-process memory_evolution_ingest
SDDIA_LAB_ROUTE_SYNC=1 ./sddia-run.sh --process radamanto-batch \
  --inputs '{"event_file_path":".events/telemetry/<id>.json"}'
SddIA/target/debug/sddia-qa audit-eda-coverage --scan --json
```

## Laudo

Plan B Dedalo: `Domain_Entity_Telemetry_Captured` (no contaminar CRUD `Domain_Entity_Updated`).
