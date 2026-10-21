---

feature_name: eda-redb-04-readers
created: "2026-10-21"
phases:
  - store
  - lectores
  - cli
  - gate
  - pruebas
execution_id: "06f21693-9776-4537-a9fe-00cc76dda132"
---
# Plan — eda-redb-04-readers

1. Extender `sddia-event-store` con listados y purga.
2. Desviar kalma2, event-bus-audit y `list_witnesses` cuando `writes_store`.
3. Añadir los tres flags en `execute-process` sin abrir la base en `filesystem`.
4. Apuntar el gate de `start-sddia.sh` al side-channel y aceptar ese JSON en la auditoría.
5. Probar cada criterio con un filtro de `cargo test`, sin la suite completa de `execute-process`.
