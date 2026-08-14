---
feature_name: norma-paridad-documental
created: "2026-05-25"
process: feature
branch: feat/norma-paridad-documental
global: APTO
pbi_archived: true
pr_url: https://github.com/racso80es/SddIA/pull/46
checks:
  DIA-CA1: pass
  DIA-CA2: pass
  DIA-CA3: pass
  DIA-CA4: pass
  DIA-CA5: pass
  DIA-CA6: pass
  DIA-CA7: pass
  DIA-CA8: pass
  verify-process-integrity: pass
  audit-doc-parity: pass
git_changes:
  - docs/features/norma-paridad-documental/
  - SddIA/templates/spec-template/
  - SddIA/templates/index.md
  - SddIA/scripts/qa/audit-doc-parity.py
  - SddIA/scripts/qa/execute_process_capsules.py
  - SddIA/process/pull-request-review.md
  - SddIA/process/index.md
  - SddIA/evolution/7a396904-dd3a-4e44-ba82-8df2c59430b6.md
  - docs/todos/done/norma-paridad-documental.md
---

# Validación — Norma de Paridad Documental (Argos)

**Veredicto global: APTO**

## Criterios de aceptación

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| DIA-CA1 | Plantilla `spec-template` en índice | ✅ | `SddIA/templates/index.md` |
| DIA-CA2 | `impacts_doc` + § DIA en plantilla | ✅ | `spec-template/spec.md` |
| DIA-CA3 | Alerta exit 0 (PBI §5) | ✅ | `impacts_doc: false` simulado → `alert_required: true`, exit 0 |
| DIA-CA4 | Exit 2 solo error operativo | ✅ | repo inválido → exit 2 |
| DIA-CA5 | Sensor sin refs agentes | ✅ | grep `cumulo`/`execute-action` vacío |
| DIA-CA6 | Genoma v2.1.0 reglas DIA | ✅ | hash `c0d8d748e7260e13…` |
| DIA-CA7 | Kaizen `PENDING_AUDIT_DOC_*` sin bloqueo DIA | ✅ | smoke `smoke-dia-parity-20260525` + `verdict: aprobado` |
| DIA-CA8 | `verify-process-integrity` | ✅ | OK |
| DIA-CA9 | PBI archivado | ✅ | `docs/todos/done/norma-paridad-documental.md` |

## Integridad

| Check | Estado |
|-------|--------|
| `verify-process-integrity.py` | ✅ OK |
| `audit-doc-parity.py` (declaración OK) | ✅ `dia_declared_ok` |

## Smoke aduana DIA

Correlación: `smoke-dia-parity-20260525`

Con artefactos documentales completos e `impacts_doc: false` simulado:

- `verdict: aprobado`
- `delivery_state: success`
- `kaizen_seeds`: `docs/todos/pending/PENDING_AUDIT_DOC_482dcaf8.md`

## Referencias

- `execution.md` — comandos reproducibles
- `spec.md` §3.2 — contrato JSON sensor
- Deuda EDA: evento `Kaizen_Alert_Required` (v2, no implementado)
