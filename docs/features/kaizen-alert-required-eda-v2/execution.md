---
feature_name: kaizen-alert-required-eda-v2
created: "2026-05-25"
process: feature
items_applied:
  - H1-H6 EDA v2 Kaizen_Alert_Required
  - smoke-pr-review-kaizen-alert
  - event-watcher E2E
---

# Ejecución — Kaizen_Alert_Required (EDA v2)

## Smoke DIA + emisión evento

```powershell
# Simular impacts_doc: false (solo durante smoke; spec restaurado a true post-validación)
# En spec.md temporal: impacts_doc: false

$env:SDDIA_LAB_SKIP_GIT_CHECKOUT = "1"
$env:SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF = "1"
python SddIA/scripts/qa/execute-process.py `
  --process pull-request-review `
  --inputs-file docs/features/kaizen-alert-required-eda-v2/_smoke-pr-review-kaizen-alert.json
```

Resultado triaje técnico (fase ejecutada):

| Campo | Valor |
|-------|-------|
| `alert_required` | `true` |
| `reason` | `impacts_doc_false_with_core_mutation` |
| `kaizen_alert_emitted` | `true` |
| `event_id` | `f8e579e9-b7d5-436c-888c-6324364ea103` |
| `target_path` | `.events/pending/f8e579e9-b7d5-436c-888c-6324364ea103.json` |
| `hash8` | `8bf3b3d1` |
| `kaizen_seeds` (Cosecha) | `[]` — cero escritura síncrona DIA |

## Smoke E2E watcher + Cúmulo

```powershell
$env:SDDIA_LAB_ROUTE_SYNC = "1"
$env:SDDIA_LAB_SIMULATE_IOTA = "1"
python SddIA/scripts/daemons/event-watcher.py --once
```

Resultado: `PENDING_AUDIT_DOC_8bf3b3d1.md` materializado por `materialize-kaizen-alert-doc`; evento purgado de pending.

## Integridad

```powershell
python SddIA/scripts/qa/recalc-process-hash-signatures.py --write
python SddIA/scripts/qa/verify-process-integrity.py
```

Resultado: OK (`pull-request-review` hash actualizado a `64480577eb56061a…`).

## Grep poda v1

```powershell
rg "_dia_audit_hash|PENDING_AUDIT_DOC" SddIA/scripts/qa/execute_process_capsules.py
```

Resultado: sin coincidencias (poda completa).

## Evidencia

| Check | Resultado |
|-------|-----------|
| Emisión ECST en `.events/pending/` | OK |
| Cosecha Kaizen sin seeds DIA | OK |
| Cúmulo materializa TODO async | OK |
| `verify-process-integrity` | OK |
