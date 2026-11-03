---
feature_name: aut-68-equipo-desde-issue
process: refactorization
created: "2026-11-03"
pbi_ref: AUT-68
execution_id: "impl-aut-68-20261103"
---

# Ejecución — AUT-68

## Verificación

```bash
export CARGO_TARGET_DIR=/home/racso/Proyectos/SddIA/SddIA/target
cargo test -p linear-tracker-adapter aut68
```

## Criterios

1. Hijo con `parent_ref` MAN-* se crea bajo prefijo MAN aunque `team_key` sea AUT.
2. Label `pbi` casa con `Pbi` en el catálogo del equipo.
3. Label ausente en el equipo del issue falla nombrando equipo y nombre.
