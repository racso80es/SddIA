---
feature_name: eda-bus-e2e-smoke-wasi-build-block
created: "2026-08-26"
process: bug-fix
branch_name: fix/eda-bus-e2e-smoke-wasi-build-block
---

# Implementación

## Cambios

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/scripts/qa/build-wasi-capsules.sh` | Nuevo: descubre y compila cápsulas WASI; excluye daemons nativos |
| `.github/workflows/sddia-index-qa.yml` | Jobs `wasi-runtime-smoke` y `eda-bus-e2e-smoke`: sustituyen `cargo build --workspace` por el script |

## Exclusión explícita

```text
execute-process, sddia-qa,
event-watcher, event-sweeper, email-watcher, telegram-watcher, github-bridge-watcher
```

## Fix unificado

Mismo parche cierra `PBI-FIX-EDA-BUS-E2E-SMOKE-WASI-BUILD` y `PBI-FIX-WASI-RUNTIME-SMOKE` (causa raíz compartida: `F-CI-WASI-OPENSSL` / `F-CI-EDA-E2E-WASI-BUILD`).
