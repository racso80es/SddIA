---
feature_name: kaizen-alert-required-eda-v2
created: "2026-05-25"
process: feature
items:
  - H1 ECST kaizen-alert-required.md + events/index
  - H2 suscripción Kaizen_Alert_Required → Cúmulo
  - H3 emisión _emit_kaizen_alert_required en triaje técnico
  - H4 poda _dia_audit_hash y escritura DIA en capsule_pr_review_kaizen
  - H5 cumulo.md + cumulo.instructions.json mandato reactivo
  - H6 materialize-kaizen-alert-doc + handler execute-action
  - pull-request-review v2.2.0 DIA-2/DIA-3
---

# Implementación — Kaizen_Alert_Required (EDA v2)

## Touchpoints

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/events/kaizen-alert-required.md` | Nueva Clase ECST |
| `SddIA/events/index.md` | Fila catálogo (7 ECST) |
| `SddIA/actions/materialize-kaizen-alert-doc.md` | Nueva acción Cúmulo |
| `SddIA/actions/index.md` | Fila catálogo (9 acciones) |
| `SddIA/core/event-subscriptions.json` | Suscriptor único Cúmulo |
| `SddIA/scripts/qa/execute_process_capsules.py` | `_emit_kaizen_alert_required`; poda puente v1 |
| `SddIA/scripts/qa/execute-action.py` | `_run_materialize_kaizen_alert_doc` |
| `SddIA/process/pull-request-review.md` | v2.2.0 — DIA vía evento |
| `SddIA/agents/cumulo.md` | §6 mandato reactivo |
| `SddIA/agents/cumulo.instructions.json` | `MANDATO [KAIZEN_ALERT_REQUIRED]` |

## Decisiones Tekton

- Reutilizar `write_pending_event` + `validate_domain_mutation_event` (mismo patrón Domain_Entity_*).
- Hash idempotente alineado a PBI §7 M2: `review_id + sorted(implicated_files)`.
- Cosecha Kaizen conserva solo Kaizen genérico (`SDDIA_LAB_PR_REVIEW_KAIZEN`); DIA fuera de cápsula.
