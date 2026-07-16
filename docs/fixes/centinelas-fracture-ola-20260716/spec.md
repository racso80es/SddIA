---
feature_name: centinelas-fracture-ola-20260716
created: "2026-07-16"
process: bug-fix
base: main
branch: fix/centinelas-fracture-ola-20260716
uuid: 8fc4c8d1-3752-42a8-a97d-c6e4f143f70c
scope: materialize-fracture-pbi, docs/todos consolidation
---

# Spec — Ola fracturas heartbeat centinelas (2026-07-16)

## Decisión

No reabrir keepalive (ya en main). Corregir **materialización de PBI** para que fracturas del mismo centinela no multipliquen TODOs mientras exista uno `abierto`. Archivar los 13 satélites bajo este fix.

## Causa raíz (empírica)

| Hecho | Evidencia |
|-------|-----------|
| Keepalive presente | `spawn_heartbeat_worker` en sweeper/watcher/telegram/github-bridge |
| Latidos vivos tras reinicio | `heartbeat-audit.json` missed_cycles=0 (2026-07-16T16:37Z) |
| 13 PBIs distintos | hash SHA256[:12] de traza con `missed_cycles` + timestamp variable |
| Idempotencia actual insuficiente | solo exact-path (`error_trace` idéntico) |

## Cambios

1. `materialize-fracture-pbi`: si existe PBI pending `abierto` para el mismo `process_name` (slug), reutilizar path — no crear otro.
2. Consolidar/archivar 13 PBIs satélite → `docs/todos/done/` con veredicto duplicado → esta ola.
3. Evolution + validacion APTO.

## CA

| ID | Criterio |
|----|----------|
| CA1 | Segunda materialización mismo `process_name` con traza distinta → mismo `target_path`, sin archivo nuevo |
| CA2 | Build/test `materialize_fracture_pbi` OK |
| CA3 | 13 satélites en `done/` + PBI ola en `done/` al cierre |
| CA4 | `validacion.md` global APTO, `pbi_archived: true` |
