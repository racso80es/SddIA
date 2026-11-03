---
feature_name: aut-70-fractura-project-id
process: bug-fix
created: "2026-11-03"
pbi_ref: AUT-70
execution_id: "impl-aut-70-20261103"
---

# Ejecución — AUT-70

```bash
export CARGO_TARGET_DIR=/home/racso/Proyectos/SddIA/SddIA/target
cargo test -p linear-tracker-adapter aut70
cargo test -p execute-process aut70
cargo build -p linear-tracker-adapter -p execute-process
```

Verificación manual opcional: enrutar `System_Fracture_Detected` con `project_id` y comprobar que `materialize-fracture-pbi` no falla por `GRAPHQL_VALIDATION_FAILED`.
