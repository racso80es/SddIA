---
document_id: PBI-CENTINELAS-FRACTURE-OLA-20260716
title: "[FIX] centinelas EDA — ola fracturas heartbeat 2026-07-16"
format: markdown
version: "1.0.0"
created: "2026-07-16"
status: done
priority: alta
process: bug-fix
persist_ref: docs/fixes/centinelas-fracture-ola-20260716/
branch: fix/centinelas-fracture-ola-20260716
validacion_ref: docs/fixes/centinelas-fracture-ola-20260716/validacion.md
closed: "2026-07-16"
related:
  - docs/fixes/centinelas-heartbeat-fracture/
  - docs/fixes/event-sweeper-heartbeat-fracture-8b1ed140e48d/
  - docs/fixes/telegram-watcher-heartbeat-fracture-67a56998121e/
consolidated_from:
  - PBI-FIX-FRACTURE-a81b72a312d5
  - PBI-FIX-FRACTURE-d6e920aa4e69
  - PBI-FIX-FRACTURE-e42acd6cd3ee
  - PBI-FIX-FRACTURE-199a6a39f84e
  - PBI-FIX-FRACTURE-257a17f9cd13
  - PBI-FIX-FRACTURE-7c992f16dd56
  - PBI-FIX-FRACTURE-90980eeb438b
  - PBI-FIX-FRACTURE-fd4c909af43c
  - PBI-FIX-FRACTURE-c4bd0ecd3413
  - PBI-FIX-FRACTURE-cc7c92a22c17
  - PBI-FIX-FRACTURE-2a782d3b357c
  - PBI-FIX-FRACTURE-88a8717fcce5
  - PBI-FIX-FRACTURE-c68cccdf2152
---

# PBI-CENTINELAS-FRACTURE-OLA-20260716

## Qué

13 PBIs `System_Fracture_Detected` (heartbeat) consolidados.

| Centinela | PBIs |
|-----------|------|
| event-watcher | 5 |
| event-sweeper | 3 |
| telegram-watcher | 3 |
| github-bridge-watcher | 2 |

## Diagnóstico (Dedalo)

1. Keepalive (olas previas) ya existe en los 4 centinelas.
2. **Spam documental:** `materialize-fracture-pbi` hasheaba traza variable → PBI nuevo por incidente.
3. Corrección: idempotencia por `process_name` si existe PBI `abierto`.

## Criterio de cierre

- [x] Idempotencia: un solo PBI `abierto` por `process_name` ante fractura heartbeat
- [x] 13 PBIs satélite archivados en `done/` como duplicados de esta ola
- [x] `validacion.md` APTO + `pbi_archived: true`
