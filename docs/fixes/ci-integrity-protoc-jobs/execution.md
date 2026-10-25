---
feature_name: ci-integrity-protoc-jobs
created: "2026-10-25"
process: bug-fix
items_applied:
  - T1
  - T2
tracker_ref: AUT-4
execution_id: "1d05f105-00ba-4ecd-b2ac-f295ff6ca047"
---
# Ejecución

| Tarea | Resultado |
|-------|-----------|
| T1 | `stage-native-debug-bins.sh` copia cada `name` de `[package]` con `src/main.rs`. `native-debug-bins` publica `SddIA/target/native-debug-bins/`. |
| T2 | `eda-bus-e2e-smoke` instala `protobuf-compiler` y `libprotobuf-dev` antes de `run-linear-direct-cycle-e2e-lab`. |

AC-1, AC-2 y AC-3 se comprueban en el run de GitHub Actions de este PR. Hasta ese run quedan `PENDIENTE-CI`.
