---
feature_name: start-sddia-native-contract
created: "2026-07-13"
process: bug-fix
branch: fix/start-sddia-native-contract
global: APTO
pbi_archived: true
pbi_ref: docs/todos/done/[FIX] start-sddia — fractura sistémica (fcca5016574d).md
checks:
  CA1-debug-priority: pass
  CA2-required-daemon-gate: pass
  CA3-non-elf-overrides-rejected: pass
  CA4-kalma2-native-orchestrator: pass
  CA5-native-startup-smoke: pass
git_changes:
  - start-sddia.sh
  - start-sddia.md
  - SddIA/daemons/
  - SddIA/scripts/common/sddia_shell_lib.sh
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - docs/fixes/start-sddia-native-contract/
  - docs/todos/done/[FIX] start-sddia — fractura sistémica (fcca5016574d).md
---

# Validación — contrato nativo de `start-sddia`

**Veredicto global: APTO.**

| CA | Evidencia |
|---|---|
| CA1 | `event-watcher` debug no contiene `execute-process.py`; release heredado sí. Los launchers eligen debug primero. |
| CA2 | Al retirar ejecución temporalmente de ambos perfiles de `event-watcher`, `start-sddia.sh` terminó con exit 1 antes de Kalma2. |
| CA3 | `SDDIA_EXECUTE_PROCESS_BIN=start-sddia.sh` devolvió `no es un binario ELF nativo ejecutable`. |
| CA4 | `cargo test` de `kalma2-bridge`: 2/2, incluido rechazo de contenido script. |
| CA5 | Smoke de 6 s informó `event-watcher`, `event-sweeper`, opcionales y `kalma2-bridge` bajo `target/debug`; apagado limpio por SIGINT. |

## Límites

El smoke encontró errores históricos de configuración/ruteo en eventos pendientes. No son causados por la ignición ni por la selección de ejecutables y quedan fuera de este PBI.
