---
document_id: PBI-CENTINELAS-KALMA2-FRACTURE-OLA-20260722
title: "[FIX] centinelas+kalma2 — ola fracturas start-sddia 2026-07-22"
format: markdown
version: "1.0.0"
created: "2026-07-22"
status: done
priority: alta
process: bug-fix
persist_ref: docs/fixes/centinelas-kalma2-fracture-ola-20260722/
branch: fix/centinelas-kalma2-fracture-ola-20260722
validacion_ref: docs/fixes/centinelas-kalma2-fracture-ola-20260722/validacion.md
closed: "2026-07-22"
consolidated_from:
  - PBI-FIX-FRACTURE-dd1aea4a9a29
  - PBI-FIX-FRACTURE-84eb0394cd44
  - PBI-FIX-FRACTURE-a669741ed066
  - PBI-FIX-FRACTURE-522e3a40e3de
  - PBI-FIX-FRACTURE-cbe0c30b3695
---

# PBI-CENTINELAS-KALMA2-FRACTURE-OLA-20260722

## Qué

5 PBIs `System_Fracture_Detected` (4 heartbeat + 1 kalma2 prótesis) consolidados.

| Componente | document_id |
|------------|-------------|
| event-sweeper | PBI-FIX-FRACTURE-dd1aea4a9a29 |
| event-watcher | PBI-FIX-FRACTURE-84eb0394cd44 |
| github-bridge-watcher | PBI-FIX-FRACTURE-a669741ed066 |
| telegram-watcher | PBI-FIX-FRACTURE-522e3a40e3de |
| kalma2-bridge | PBI-FIX-FRACTURE-cbe0c30b3695 |

## Diagnóstico

1. Keepalive centinelas ya presente; heartbeats OK con ecosistema vivo.
2. `start-sddia` cleanup dejaba locks con PID muerto.
3. `start-sddia` no cargaba bóveda → bridge/mayeuta sin `SDDIA_LLM_*` → `mayeuta-llm/prótesis exit 1`.

## Criterio de cierre

- [x] Vault + cleanup locks + gate heartbeat en `start-sddia`
- [x] Validación empírica APTO
- [x] 5 satélites en `done/`
