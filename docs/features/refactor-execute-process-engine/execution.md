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

## PR y merge

| Ref | Descripción |
|-----|-------------|
| `d2c9559` | Squash merge PR #9 → `main` |
| `775114c` | Registry acciones EDA (en rama feature) |
| `18d80ea` | Fix `event-subscriptions.json` (IOTA en `PullRequest_Presented`) |

**PR:** https://github.com/racso80es/SddIA/pull/9 (MERGED)

## Cierre EDA (IOTA físico, sin `SDDIA_LAB_SIMULATE_IOTA`)

```powershell
python SddIA/scripts/qa/execute-action.py --action emit-pr-presented-event --input-file tmp/emit-pr-presented-refactor.json
python SddIA/scripts/qa/execute-action.py --action emit-pr-merged-event --input-file tmp/emit-pr-merged-refactor.json
Remove-Item Env:SDDIA_LAB_SIMULATE_IOTA -ErrorAction SilentlyContinue
python SddIA/scripts/daemons/event-watcher.py --once
```

| event_id | event_type | `delivery_state.cumulo` |
|----------|------------|-------------------------|
| `5d8716d5-ed2e-4657-bc07-7bf5a7e84a29` | `PullRequest_Presented` | `success` (IOTA testnet) |
| `34f30fb4-1e72-4de1-a809-faec07af8b3b` | `PullRequest_Merged` | `success` (IOTA testnet) |
