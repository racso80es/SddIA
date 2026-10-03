---
feature_name: work-initiated-event
created: "2026-10-02"
process: feature
---

# Especificación — work-initiated-event

## Evento `Work_Initiated`

- Clase: `SddIA/events/domain/work-initiated.md`
- Payload ECST: `work_kind`, `branch`, `persist_ref`, etc.

## Emisión

- Acción `emit-work-initiated-event`.
- Hook en `workspace-init` tras inicializar feature | bug-fix | refactorization (fail-soft).
