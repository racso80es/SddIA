---
feature_name: linear-e2e-hu-tracker-ref
created: "2026-10-25"
process: bug-fix
---

# Spec

Run [40186227424](https://github.com/racso80es/SddIA/actions/runs/40186227424) en `main` y run [40187481893](https://github.com/racso80es/SddIA/actions/runs/40187481893) en el PR de AUT-5. `run-eda-e2e-lab` sale 0. `run-linear-direct-cycle-e2e-lab` aborta en `linear_direct_cycle_e2e.rs`: `tracker_ref HU` / `hu tracker`.

`557d6332` dejó de persistir el sello en el fichero cuando el provider es `linear`. El lab lee ese fichero.

## Corte

1. `refine-hu` escribe el markdown sellado en el path local si el fichero existe, y actualiza la descripción del issue cuando hay `tracker_ref`.
2. `refine-pbi` usa la descripción remota si trae frontmatter; si no, el fichero local. El update remoto usa `tracker_ref`, no la ruta.
