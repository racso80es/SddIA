---
document_id: PBI-FIX-REVISION-GESTION-EVENTOS-KAIZEN
title: "[FIX] revision-gestion-eventos-kaizen — eventos DL post-merge tras kaizen single-PR"
format: markdown
version: "1.0.0"
created: "2026-05-23"
status: "completado"
closed: "2026-05-23"
priority: alta
process: bug-fix
incident_ref: "Eventos PullRequest_Presented #30/#31 en dead-letter con padre duplicado en pending/"
feature_ref: docs/fixes/revision-gestion-eventos-kaizen
validacion_ref: docs/fixes/revision-gestion-eventos-kaizen/validacion.md
branch: fix/revision-gestion-eventos-kaizen
---

# [FIX] revision-gestion-eventos-kaizen — eventos DL post-merge tras kaizen single-PR

**Estado:** ✅ Completado en rama `fix/revision-gestion-eventos-kaizen` (cierre documental pre-merge).

## Contexto

Tras el kaizen de cierre documental en un solo PR (#34), quedaron eventos gestionados con error: `PullRequest_Presented` retroactivos de PRs #30/#31 (flujo post-merge obsoleto) con testigo dead-letter en `argos.pull-request-review` y copia stale del padre en `pending/`.

## Entregables

| Ítem | Resultado |
|------|-----------|
| Diagnóstico | Residual pre-kaizen, no regresión single-PR |
| `finalize_kaizen_terminal` | Terminaliza Kaizen cuando suscriptores cerrados |
| `try_sweep_event` | Nuevo status `kaizen-finalized` |
| Sweeper / Watcher | Reporte y logs distintos Kaizen activo vs terminalizado |
| `events-contract.md` | §4 pasos 6–7 actualizados |
| Manifiesto | `eda-legacy-manifest.json` con UUIDs #30/#31 |

## Invocación canónica

```powershell
python SddIA/scripts/daemons/event-sweeper.py --once --json
python SddIA/scripts/qa/run-eda-e2e-lab.py
```
