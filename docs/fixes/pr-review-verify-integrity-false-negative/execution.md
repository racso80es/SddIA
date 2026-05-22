---
feature_name: pr-review-verify-integrity-false-negative
created: "2026-05-22"
process: bug-fix
---

# Ejecución — pr-review-verify-integrity-false-negative

## Hito 1–2 (local 2026-05-22)

| Comando | Resultado |
|---------|-----------|
| `verify-process-integrity.py` | OK |
| `smoke-pr-review-verify-integrity.py` (sync) | `verify_exit=0`, `mode=local-checkout` |
| `pull-request-review` + `tmp/smoke-pr-review-verify-integrity.json` | `verdict: aprobado`, triaje técnico `passed: true` |

```powershell
$env:PYTHONUTF8 = "1"
python SddIA/scripts/qa/verify-process-integrity.py
python SddIA/scripts/qa/smoke-pr-review-verify-integrity.py --branch fix/pr-review-verify-integrity-false-negative
$env:SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF = "1"
python SddIA/scripts/qa/execute-process.py --process pull-request-review --inputs-file tmp/smoke-pr-review-verify-integrity.json
```

## Hito 3 — dead-letter PR #23

Pendiente tras merge del fix: re-enrutar `docs/events/dead-letter/c2573529-ca49-4716-bbf9-ae77135be8fe.json`.
