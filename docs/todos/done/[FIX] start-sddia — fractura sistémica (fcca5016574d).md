---

document_id: PBI-FIX-FRACTURE-fcca5016574d
title: "[FIX] start-sddia — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-07-13"
status: done
priority: alta
process: bug-fix
fracture_process: start-sddia
fracture_hash: fcca5016574d
incident_ref: "System_Fracture_Detected — fcca5016574d"
branch_name: fix/start-sddia-native-contract
fix_ref: docs/fixes/start-sddia-native-contract
validacion_ref: docs/fixes/start-sddia-native-contract/validacion.md
closed: "2026-07-13"


# [FIX] start-sddia — fractura sistémica

## Incidente

El perfil `release` de `event-watcher`, compilado el 2026-06-16, conservaba una referencia a `execute-process.py`. El perfil `debug` compilado el 2026-07-13 ya es nativo y no contiene esa referencia; sin embargo, los launchers preferían `release`.

## Corrección aplicada

- Se prioriza `debug`, coherente con el `cargo build` documentado; `release` queda como fallback.
- Los ejecutables y overrides deben ser ELF nativos ejecutables.
- Los centinelas obligatorios se validan de forma individual.
- El arranque expone los paths nativos elegidos y la documentación invoca el script desde la raíz.

## Criterio de cierre

- [x] Causa raíz resuelta
- [x] Argos APTO en `validacion.md` del fix
- [x] PBI archivado en `docs/todos/done/`
