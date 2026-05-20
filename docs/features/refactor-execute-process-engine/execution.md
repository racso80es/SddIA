---
feature_name: refactor-execute-process-engine
created: "2026-05-20"
process: feature
branch_name: feat/refactor-execute-process-engine
---

# Ejecución — refactor execute-process engine

## Registry ampliado

| Acción | Handler `execute-action.py` | Contrato |
|--------|------------------------------|----------|
| `emit-domain-mutation` | ✅ | `SddIA/actions/emit-domain-mutation.md` |
| `emit-pr-presented-event` | ✅ | `SddIA/actions/emit-pr-presented-event.md` (nuevo) |
| `emit-pr-merged-event` | ✅ | `SddIA/actions/emit-pr-merged-event.md` |

`execute_process_capsules.CAPSULE_ACTION_REGISTRY` enruta `action:*` → `execute-action.py`.

## Cierre PR + EDA (IOTA físico)

```powershell
# Tras merge a main — sustituir MERGE_HASH por el commit de squash/merge
python SddIA/scripts/qa/execute-action.py --action emit-pr-presented-event --input-file tmp/emit-pr-presented-refactor.json
python SddIA/scripts/qa/execute-action.py --action emit-pr-merged-event --input-file tmp/emit-pr-merged-refactor.json

Remove-Item Env:SDDIA_LAB_SIMULATE_IOTA -ErrorAction SilentlyContinue
python SddIA/scripts/daemons/event-watcher.py --once
```
