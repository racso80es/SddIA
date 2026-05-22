---
feature_name: pull-request-review-redesign
created: "2026-05-22"
process: feature
---

# Ejecución — Aduana `pull-request-review` v2

## Hito 1 — Genoma v2.0.0

| Artefacto | Acción |
|-----------|--------|
| `SddIA/process/pull-request-review.md` | Reescritura 7 fases; retirada Dedalo |
| `SddIA/process/index.md` | v2.0.0 + contexto `pr-lifecycle` |
| Hash | `sha256:4408f797…` (verify-process-integrity OK) |

## Hito 2 — Cableado bus

| Artefacto | Acción |
|-----------|--------|
| `event-subscriptions.json` | Suscriptor `argos` + `process: pull-request-review` |
| `pull-request-presented.md` | Texto suscripciones actualizado |
| `event-watcher.py` | Fan-out `process` → `execute-process.py` |

## Hito 3 — Handler lab

| Módulo | Fases físicas |
|--------|----------------|
| `execute_process_capsules.py` | F1–F7 + `run_process` verdict blocking |

## Smokes ejecutados (2026-05-22)

```powershell
$env:SDDIA_LAB_SKIP_GIT_CHECKOUT='1'
$env:SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF='0'   # default watcher; encadena accept-pr
python SddIA/scripts/qa/execute-process.py --process pull-request-review `
  --inputs-file docs/features/pull-request-review-redesign/_smoke-pr-review-presented.json
```

| Escenario | Resultado |
|-----------|-----------|
| Aprobado (smoke positivo) | `verdict: aprobado`, `delivery_state: success` |
| Rechazado (`SDDIA_LAB_PR_REVIEW_DOC_FAIL`) | `verdict: rechazado`, `status_code: 1` |
| Kaizen (`SDDIA_LAB_PR_REVIEW_KAIZEN`) | TODO generado en `docs/todos/` |

## E2E bus + cierre PR #15

| Paso | Evidencia |
|------|-----------|
| Presented (aduana) | `ec7cd211-e88e-416c-bcd3-e43995a0131b` |
| Handoff `accept-pr` | `81e41e26ab9db9cc9dfb5787f89d31c056eec097` |
| Merged | `395cb57d-4d0d-473a-bc6c-d287575e964b` → `processed/` |
