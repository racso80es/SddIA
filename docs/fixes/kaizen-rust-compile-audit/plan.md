---
feature_name: kaizen-rust-compile-audit
created: "2026-10-25"
process: bug-fix
phases:
  - higiene
  - gates
  - pruebas
---

# Plan — AUT-3

1. Higiene de símbolos solo vivos en test, imports residuales y asignaciones no leídas. Sin cambiar el valor que producción ya escribe.
2. Gates KZ-N1..KZ-N6 en el camino que el warning había dejado mudo.
3. Pruebas de catálogo, replay, sandbox, fagocito, scope y F5. `cargo build` de los seis crates del criterio KZ-CA1.
