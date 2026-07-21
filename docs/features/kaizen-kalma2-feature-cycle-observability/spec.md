---
feature_name: kaizen-kalma2-feature-cycle-observability
created: "2026-07-21"
process: feature
branch_name: feat/kaizen-kalma2-feature-cycle-observability
persist_ref: docs/features/kaizen-kalma2-feature-cycle-observability
correlation_id: 6ae1b7be-54e5-4750-8888-5f19ac76551f
scope: "O1 early PEC + O2 failed PEC + O4 pr_url DEFAULTABLE + O3 checklist"
base: main
version_spec: "1.0.0"
agent: dedalo
---

# Spec — Kaizen observabilidad Kalma2-feature

## Arquitectura

```mermaid
sequenceDiagram
  participant K as Kalma2 UI
  participant B as kalma2-bridge
  participant T as task-queue-manager
  participant O as .events/orchestration
  participant F as feature hijo
  K->>B: POST /api/execute
  B->>T: Kalma2_Process_Requested
  T->>O: PEC initialized (correlation_id)
  T->>F: dispatch hijo
  F->>O: PEC completed|failed
  K->>B: GET /api/status
  B->>O: find_pec_by_correlation
  B->>K: initialized|completed|failed
```

## Cambios

| ID | Touchpoint | Cambio |
|----|------------|--------|
| O2/O1 | `thermodynamic.rs` | PEC en fallo si hay `correlation_id`; `emit_initialized_pec` |
| O1/O2 | `task_queue_manager.rs` | Emite PEC `initialized` antes de `invoke_process` hijo |
| O4 | `resolver.rs` | `pr_url` en `DEFAULTABLE` |
| O3 | `checklist-delivery-repro.md` | Norma operativa de entrega |

## Restricciones

- Sin mutar genoma `process/` vía IDE (contrato PPR ya declara `pr_url` opcional).
- Sin absorber FIX kalma2-bridge.
