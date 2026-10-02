# Spike — migración gate Done a Linear

## Alcance

Documento de spike (sin cambio productivo): comparar gate actual `feature-pbi-archive` / `validacion.md` con criterio Done en Linear (laudo D1 pendiente de productización).

## Conclusión provisional

- **SSOT documental Done** sigue siendo `validacion.md` APTO + PBI en `docs/todos/done/` en el PR (D1).
- **Linear** refleja estado vía `tracker-stamp` en merge; no sustituye el gate del motor hasta laudo explícito de migración.
- **Riesgo:** desalinear gate CI y estado Linear si se mueve Done solo a Linear sin mover `feature-pbi-archive`.

## Recomendación

Mantener gate actual; usar Linear como espejo operativo. Revisitar spike tras E2E mock del ciclo completo (AC-8).
