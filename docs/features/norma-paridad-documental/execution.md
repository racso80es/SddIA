---
feature_name: norma-paridad-documental
created: "2026-05-25"
process: feature
items_applied:
  - H1 plantilla spec-template
  - H2 pull-request-review v2.1.0
  - H3 audit-doc-parity.py
  - H4 cápsulas triaje + Kaizen
---

# Ejecución — Norma de Paridad Documental (DIA)

## Comandos de verificación

### Sensor aislado (declaración OK)

```powershell
python SddIA/scripts/qa/audit-doc-parity.py `
  --persist-ref docs/features/norma-paridad-documental `
  --base-ref main --head-ref HEAD --json
```

Resultado: `alert_required: false`, `reason: dia_declared_ok`, exit `0`.

### Sensor alerta (PBI §5 — impacts_doc simulado false)

Con `impacts_doc: false` temporal en `spec.md` y diff en rutas monitorizadas:

```text
alert_required: true
reason: impacts_doc_false_with_core_mutation
exit: 0
```

### Aduana lab (DIA + Kaizen)

```powershell
$env:SDDIA_LAB_SKIP_GIT_CHECKOUT = "1"
$env:SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF = "1"
python SddIA/scripts/qa/execute-process.py `
  --process pull-request-review `
  --inputs-file docs/features/norma-paridad-documental/_smoke-pr-review-dia.json
```

Con `impacts_doc: false` simulado: genera `docs/todos/pending/PENDING_AUDIT_DOC_482dcaf8.md`.

### Integridad

```powershell
python SddIA/scripts/qa/verify-process-integrity.py
```

Resultado: OK.

## Evidencia

| Check | Resultado |
|-------|-----------|
| `verify-process-integrity` | OK |
| Sensor sin refs Cúmulo | grep vacío |
| Alerta exit 0 | OK |
| TODO Kaizen `PENDING_AUDIT_DOC_*` | OK (smoke lab) |
