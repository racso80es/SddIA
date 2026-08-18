---
document_id: PBI-FIX-FRACTURE-521b4f60d746
title: "[FIX] email-watcher — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-08-18"
updated: "2026-08-18"
status: done
priority: alta
process: bug-fix
incident_ref: "System_Fracture_Detected — 521b4f60d746"
resolution: "Cerrado in-ciclo PBI-KALMA2-MVP-01A — email-watcher en start-sddia.sh + A-02 lock huérfano"
pr_url: https://github.com/racso80es/SddIA/pull/182
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
---

# [FIX] email-watcher — fractura sistémica

## Cierre

Causa: centinela no arrancaba en ignición unificada + locks huérfanos tras `--once`.

Resuelto en **PR #182** (`PBI-KALMA2-MVP-01A`): A-05 integración `start-sddia.sh`, A-02 recuperación lock, daemon en loop con bóveda instancia.

## Criterio de cierre

- [x] Causa raíz resuelta
- [x] Circuito sensorial operativo (lab E2E)
- [x] TODO movido a `docs/todos/done/`
