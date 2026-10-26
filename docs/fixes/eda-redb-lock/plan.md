---
feature_name: eda-redb-lock
created: "2026-10-26"
process: bug-fix
phases:
  - id: T1
    title: Retry/backoff en EventStore::open + StoreError::Locked
  - id: T2
    title: scan_presented_for_branch lee el store
  - id: T3
    title: system_vitality emite por sddia-event-store
  - id: T4
    title: probe_bus_topology no fractura por Locked
execution_id: "0aab3583-5c34-40e9-ba20-d8d9a527d2b8"
pbi_ref: AUT-16
---

# Plan

Descomposición de implementación (no blueprint de proceso). Un commit por tarea vía `skill:git-manager`.

| Id | Tarea | Hecho |
|----|--------|--------|
| T1 | `EventStore::open` con reintento solo ante lock; `StoreError::Locked`; presupuesto `SDDIA_EVENT_STORE_OPEN_BUDGET_MS` default 2000. Tests REDB-CA1/CA2. | C1 |
| T2 | Predicado compartido `presented_in_store`; `scan_presented_for_branch` lo consulta cuando `writes_store`. Tests REDB-CA3 + `t_adu_ca7_*`. | C2 |
| T3 | `emit_vitality_event` → `fractal::write_fractal_event` (telemetry); `emit_probe_fracture` → `record_ecst` domain. Test REDB-CA4. | C3 |
| T4 | `probe_bus_topology`: `Locked` → `ok: true` / `store ocupado`. Test REDB-CA5. REDB-CA6 en lab post-commits. | C4 |
