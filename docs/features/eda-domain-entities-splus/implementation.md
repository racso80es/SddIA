---
feature_name: eda-domain-entities-splus
created: "2026-05-20"
process: feature
items:
  - id: F0-topology
    touchpoint: SddIA/events/domain-entity-*.md, event-subscriptions.json, event-watcher.py
    status: done
  - id: F0-emit
    touchpoint: execute-action.py, emit-domain-mutation.md
    status: done
  - id: F0-idempotency
    touchpoint: eda_bus_utils.py, execute_process_capsules.py
    status: done
  - id: F0-audit
    touchpoint: audit-entity-eda-coverage.py, delivery-close-cycle.md
    status: done
  - id: FA-forges
    touchpoint: execute_process_forges.py, execute_process_capsules.py
    status: done
  - id: FA-governance
    touchpoint: entity-manager.md, *-creator.md (6 clases)
    status: done
  - id: FB-e2e
    touchpoint: run-eda-e2e-lab.py, event-watcher.py
    status: done
  - id: FB-argos
    touchpoint: execute_process_capsules.py delivery-close-cycle gate
    status: done
  - id: FC-backfill
    touchpoint: audit-entity-eda-coverage.py --emit --anchor-merkle
    status: done
---

# Implementación — EDA Domain Entities S+

## Fase 0 — Protocolo de Acero

| Pilar | Entregable | Estado |
|-------|------------|--------|
| 1 Topología | `origin_topology` en ECST, subscriptions, watcher filter | ✅ |
| 2 DLT | umbral watcher, skip backfill, docs mandato | ✅ |
| 3 Idempotencia | `find_existing_domain_event`, forja/sello idempotente | ✅ |
| 4 Argos | `audit-entity-eda-coverage.py`, fase close-cycle | ✅ |

## Fase A — Laboratorio

- `PILOT_ENTITY_CLASSES`: 8 clases.
- `FORGE_BY_ENTITY_CLASS`: tool, action, process, agent, norm, codex.
- `entity-manager` → forja directa lab + subprocess creator + `emit-domain-mutation`.

## Pendiente post-implementación

- Test `origin_topology=local` (no muta índices core).
- Merge PR `feat/eda-domain-entities-splus` → `main`.
- IOTA testnet real en acta Merkle (sustituir `SDDIA_LAB_SIMULATE_IOTA` en CI/producción).
