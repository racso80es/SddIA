---
feature_name: start-sddia-native-contract
created: "2026-07-13"
process: bug-fix
items_applied:
  - native-launcher-selection
  - required-daemon-gate
  - kalma2-elf-check
  - startup-smoke
---

# Ejecución — contrato nativo de `start-sddia`

## Evidencia

| Comprobación | Resultado |
|---|---|
| `bash -n` launchers | Correcto |
| `cargo test` `kalma2-bridge` | 2/2 OK |
| Override `SDDIA_EXECUTE_PROCESS_BIN=start-sddia.sh` | Rechazado como no ELF |
| Gate requerido con ambos perfiles de watcher no ejecutables | Exit 1 antes de Kalma2 |
| Smoke `timeout --signal=INT 6 ./start-sddia.sh` | Centinelas y bridge debug nativos activos; apagado limpio |

El smoke expuso backlog de eventos con errores de configuración/ruteo, ajenos a la ignición. No se observó la referencia heredada `execute-process.py` al usar los binarios debug resueltos.
