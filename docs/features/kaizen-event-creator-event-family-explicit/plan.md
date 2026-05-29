---
feature_name: kaizen-event-creator-event-family-explicit
created: "2026-05-29"
process: refactorization
---

# Plan — Kaizen event-creator event_family explícito

| Hito | Entregable | Estado |
|------|------------|--------|
| H0 | PBI + `_init-feature.json` + `objectives.md` + `clarify.md` + rama | [x] |
| H1 | `spec.md` — contrato runtime + inventario O2 | [ ] |
| H2 | `event-creator.md` — input obligatorio, sin default | [ ] |
| H3 | `run_event_forge` + `creator_inputs_from_entity` alineados fractal | [ ] |
| H4 | Migración seeds/smokes documentados | [ ] |
| H5 | Actualización docs Fase 1 (D1.9 cerrado) | [ ] |
| H6 | `implementation.md` / `execution.md` / `validacion.md` + PBI `done/` | [ ] |
| H7 | Un solo PR + `delivery-close-cycle` | [ ] |

## Orden Tekton (borrador)

H1 → H2 → H3 → H4 → H5 → H6 → H7

## Touchpoints código (previstos)

| Archivo | Cambio |
|---------|--------|
| `SddIA/process/event-creator.md` | Quitar default; validación estricta |
| `SddIA/process/entity-manager.md` | Documentar `semantic_seed.event_family` |
| `SddIA/scripts/qa/execute_process_capsules.py` | `run_event_forge`, `creator_inputs_from_entity` |
| Docs smokes / specs con forja event | Añadir `event_family` explícito |
| `docs/features/telemetria-reactiva-eda-fase1/` | Cerrar puntero Kaizen D1.9 |

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Handler legacy ocultaba ausencia de `event_family` | H3 obligatorio antes de retirar default en spec |
| Smokes `ola-c-event-entity` sin familia | H4 inventario + patch JSON ejemplo |
| Confundir instancia vs Clase | Mantener fuera de alcance envelope instancia (PBI §4) |

## Gate Argos (borrador)

- O1–O4 del PBI cumplidos
- `test_eda_bus_v3plus` / smokes forja sin regresión
- `validacion.md` APTO + `pbi_archived: true`
