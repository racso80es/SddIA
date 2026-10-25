---
feature_name: linear-e2e-hu-tracker-ref
created: "2026-10-25"
process: bug-fix
branch_name: fix/linear-e2e-hu-tracker-ref
persist_ref: docs/fixes/linear-e2e-hu-tracker-ref
---

# Objetivos

`run-linear-direct-cycle-e2e-lab` (job `eda-bus-e2e-smoke`) falla en AC-7, AC-8 y AC-9: `refine-hu` con `tracker.provider: linear` no deja `tracker_ref` en el markdown local de la HU.

## Criterio

Tras `refine-hu` / `refine-pbi`, el fichero local del ítem conserva el sello y, si hay `tracker_ref`, el issue remoto recibe la misma descripción.
