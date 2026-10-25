---
feature_name: ci-integrity-protoc-jobs
created: "2026-10-25"
process: bug-fix
phases:
  - id: T1
    title: Publicar los binarios que exige verify-compiled-capsules
  - id: T2
    title: Instalar protoc en eda-bus-e2e-smoke
execution_id: "1d05f105-00ba-4ecd-b2ac-f295ff6ca047"
---
# Plan

| Id | Tarea | Hecho |
|----|--------|--------|
| T1 | `stage-native-debug-bins.sh` y artefacto `native-debug-bins` con ese directorio. | AC-1 |
| T2 | `apt-get` de `protobuf-compiler` y `libprotobuf-dev` en `eda-bus-e2e-smoke`, antes del laboratorio. | AC-2 |
