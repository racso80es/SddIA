---
feature_name: delivery-close-hook-eda-governance
created: "2026-05-22"
process: bug-fix
branch_name: fix/delivery-close-hook-eda-governance
---

# Ejecución — delivery-close-hook-eda-governance

## Fase 1 — Anti-recursión (lab)

| Paso | Comando / verificación | Resultado |
|------|------------------------|-----------|
| Guarda hook | `SDDIA_HOOK_DELIVERY_CLOSE=1` + `pre_push_gate.py` | exit 0, mensaje `SKIPPED (delivery-close-cycle guard)` |
| Smoke DCC | Lab: `SDDIA_LAB_SIMULATE_GH_PR=1`, payload `git-hook-pre-push` | `PullRequest_Presented` minteado (fase sello OK; gate EDA genómica puede bloquear por huérfanas nuevas entidades) |

## Fase 2 — Retroactivo PR #20

```powershell
# Presented
python -c "import json,subprocess,sys; ..."
# → event_id 868d1b8f-0171-4f8f-ab72-19382941523d

# Merged
# → event_id 75b8e950-9366-4ce5-bf22-b4b56430736e

python SddIA/scripts/daemons/event-watcher.py --once
```

## Fase 2 — Retroactivo PR #20

Event IDs: Presented `868d1b8f-…`, Merged `75b8e950-…` → `processed/`.

## Fase 4 — Smoke Kintsugi dual

```powershell
# Emitir System_Fracture_Detected en pending → event-watcher --once
# Verificar delivery_state: { "cumulo": "success", "mayeuta": "success" }
# PBI en docs/todos/pending/ con sección Mayeuta completa
```

## Fase 4b — Backfill Fase C

```powershell
python SddIA/scripts/qa/audit-entity-eda-coverage.py --emit --skip-dlt --json --correlation-id delivery-close-hook-eda-governance
python SddIA/scripts/daemons/event-watcher.py --once
# orphan_count_after: 0
```
