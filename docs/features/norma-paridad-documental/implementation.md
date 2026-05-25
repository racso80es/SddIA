---
feature_name: norma-paridad-documental
created: "2026-05-25"
process: feature
items:
  - SddIA/templates/spec-template/spec.md
  - SddIA/templates/spec-template/spec.json
  - SddIA/templates/index.md
  - SddIA/scripts/qa/audit-doc-parity.py
  - SddIA/process/pull-request-review.md
  - SddIA/process/index.md
  - SddIA/scripts/qa/execute_process_capsules.py
  - SddIA/evolution/pull-request-review-v2.1-dia-20260525.md
---

# Implementación — Norma de Paridad Documental (DIA)

## Touchpoints

| Ámbito | Cambio |
|--------|--------|
| **Plantilla DIA** | `SddIA/templates/spec-template/` — `impacts_doc` + § Impacto en Documentación |
| **Sensor** | `audit-doc-parity.py` — diff vs spec; stdout JSON; exit 0 en alerta |
| **Genoma aduana** | `pull-request-review` v2.1.0 — reglas DIA-1..3 |
| **Cápsulas lab** | `_invoke_dia_audit` en triaje técnico; `PENDING_AUDIT_DOC_*` en Kaizen |
| **Evolución** | `pull-request-review-v2.1-dia-20260525.md` |

## Decisiones de implementación

1. **Ceguera espacial** — el sensor no importa agentes ni escribe en `docs/todos/` directamente.
2. **Fricción suave** — `alert_required` no eleva `passed: False` en triaje técnico.
3. **Puente lab v1** — cápsula parsea JSON → `state["dia_audit"]` → fase Cosecha Kaizen.
4. **Evento EDA v2** — `Kaizen_Alert_Required` documentado en spec; suscripción bus fuera de alcance.

## Propuestas no aplicadas (Kaizen)

- Suscripción bus `Kaizen_Alert_Required` en `event-subscriptions.json`.
- Integrar `audit-doc-parity` en `verify-process-integrity.py` o CI GitHub Actions.
