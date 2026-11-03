---
feature_name: aut-69-unificar-etiqueta-fix-bug
created: "2026-11-03"
purpose: decision
process: refactorization
branch_name: refactor/man-26-ciclo-vida-software
persist_ref: docs/features/aut-69-unificar-etiqueta-fix-bug
pbi_ref: AUT-69
execution_id: "plan-aut-69-20261103"
---

# Clarificaciones — AUT-69

## Decisiones

1. La clave de contrato `tracker.labels.fix` no cambia. El valor pasa de `Fix` a `Bug`.
2. `Bug` es la label de workspace que ya existe (`6154e3e4-42d8-403e-9795-7779cfbc9df8`). No se crea otra.
3. La migración es por issue del equipo AUT: añadir `Bug`, quitar `Fix`. La label de equipo `Fix` se borra solo cuando ningún issue la lleve.
4. El resto del mapa (`HU`, `Pbi`, `Kaizen`, `Deuda`, `Spike`) y `editable: sddia-editable` quedan como están.
5. La entrega se planifica en `refactor/man-26-ciclo-vida-software`. No se abre otra rama.
