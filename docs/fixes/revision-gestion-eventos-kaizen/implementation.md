---
feature_name: revision-gestion-eventos-kaizen
created: "2026-05-23"
process: bug-fix
branch: fix/revision-gestion-eventos-kaizen
version_implementation: "1.0.0"
---

# Implementación — Terminalización Kaizen EDA

## Cambios de código

| Archivo | Cambio |
|---------|--------|
| `SddIA/scripts/qa/eda_bus_utils.py` | Nueva `finalize_kaizen_terminal()`; `try_sweep_event` retorna `kaizen-finalized` cuando DL + consenso terminal |
| `SddIA/scripts/daemons/event-sweeper.py` | Reporte `kaizen_finalized` separado de alertas activas |
| `SddIA/scripts/daemons/event-watcher.py` | Log distinto para `kaizen-finalized` vs Kaizen activo |
| `SddIA/events/events-contract.md` | §4 pasos 6–7: terminalización Kaizen |

## Helper `finalize_kaizen_terminal`

Cuando `dead_letter_subscribers` existen y todos los suscriptores requeridos están terminales:

1. Asegura cabecera `dead-letter/` (copia desde pending si falta).
2. Elimina padre de `pending/`.
3. Purga cabecera `processing/` vía `maybe_purge_processing_header`.

Estados `try_sweep_event` ampliados:

| status | purged | Semántica |
|--------|--------|-----------|
| `kaizen` | false | DL presente; suscriptores aún in-flight o incompletos |
| `kaizen-finalized` | true | DL terminal; padre retirado de pending |

## Artefactos documentales

| Archivo | Propósito |
|---------|-----------|
| `eda-legacy-manifest.json` | UUIDs retroactivos #30/#31 y procedimiento |
| `spec.md` | Diagnóstico y CA |
