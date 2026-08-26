---
feature_name: eda-bus-e2e-smoke-wasi-build-block
created: "2026-08-26"
process: bug-fix
branch_name: fix/eda-bus-e2e-smoke-wasi-build-block
---

# Ejecución

1. Rama `fix/eda-bus-e2e-smoke-wasi-build-block` desde `main`.
2. Script `build-wasi-capsules.sh` con descubrimiento dinámico de 20 paquetes (skills/tools/interfaces).
3. Workflow actualizado en ambos jobs WASI.
4. Verificación local: build script + smokes nativos pendientes de CI GHA.
