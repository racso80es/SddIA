---
feature_name: refactor-execute-process-engine
created: "2026-05-20"
process: feature
branch_name: feat/refactor-execute-process-engine
executed_at: "2026-05-20"
items_applied:
  - feature-init-execute-process
  - dynamic-interpreter-refactor
  - action-registry-expansion
  - pr9-merge-eda-close
  - event-subscriptions-iota-fix
---

# Ejecución — refactor execute-process engine

Registro de entrega, merge squash a `main` y cierre EDA con IOTA físico.

## Tareas realizadas

| # | Tarea | Estado |
|---|--------|--------|
| 1 | Inicialización feature vía `execute-process` (payload `tmp/refactor-execute-process-init.json`) | ✅ |
| 2 | Refactor a intérprete dinámico (`execute_process_core` + `execute_process_capsules`) | ✅ |
| 3 | CLI `--process` / `--inputs` / `--inputs-file` | ✅ |
| 4 | Shims Ola C `--input-file` y `--action` con warning | ✅ |
| 5 | Handler genérico `workspace-init` (git + `objectives.md`) | ✅ |
| 6 | Registry `CAPSULE_ACTION_REGISTRY` + handlers EDA en `execute-action.py` | ✅ |
| 7 | Contrato `emit-pr-presented-event.md` + índice acciones | ✅ |
| 8 | PR #9 creado, revisado y merge squash | ✅ |
| 9 | Emisión `PullRequest_Presented` + `PullRequest_Merged` | ✅ |
| 10 | Watcher `--once` sin `SDDIA_LAB_SIMULATE_IOTA` | ✅ |
| 11 | Fix `event-subscriptions.json` (IOTA en Presented y Domain_Entity_*) | ✅ |
| 12 | TODO deuda Ola C (retirada shims) | ✅ documentado |

## Commits y merge

| Ref | Descripción |
|-----|-------------|
| `31bae72` | Intérprete dinámico agnóstico (contenido squash PR #9) |
| `775114c` | Registry acciones EDA (historial rama feature) |
| `d2c9559` | Squash merge PR #9 → `main` |
| `18d80ea` | Fix `event-subscriptions.json` |
| `9b9d611` | Actualización manifiestos formales |

**PR:** https://github.com/racso80es/SddIA/pull/9 (MERGED, squash)

## Comandos reproducibles

### Inicialización (legado Ola C)

```powershell
python SddIA/scripts/qa/execute-process.py --input-file tmp/refactor-execute-process-init.json
```

### Intérprete canónico

```powershell
python SddIA/scripts/qa/execute-process.py --process feature --inputs-file tmp/smoke-feature-new-cli.json
```

### Registry ampliado

| Acción | Handler | Contrato |
|--------|---------|----------|
| `emit-domain-mutation` | `execute-action.py` | `SddIA/actions/emit-domain-mutation.md` |
| `emit-pr-presented-event` | `execute-action.py` | `SddIA/actions/emit-pr-presented-event.md` |
| `emit-pr-merged-event` | `execute-action.py` | `SddIA/actions/emit-pr-merged-event.md` |

### Cierre EDA (IOTA físico)

```powershell
python SddIA/scripts/qa/execute-action.py --action emit-pr-presented-event --input-file tmp/emit-pr-presented-refactor.json
python SddIA/scripts/qa/execute-action.py --action emit-pr-merged-event --input-file tmp/emit-pr-merged-refactor.json
Remove-Item Env:SDDIA_LAB_SIMULATE_IOTA -ErrorAction SilentlyContinue
python SddIA/scripts/daemons/event-watcher.py --once
```

| event_id | event_type | `delivery_state.cumulo` |
|----------|------------|-------------------------|
| `5d8716d5-ed2e-4657-bc07-7bf5a7e84a29` | `PullRequest_Presented` | `success` |
| `34f30fb4-1e72-4de1-a809-faec07af8b3b` | `PullRequest_Merged` | `success` |

## Handoff

Motor de procesos del laboratorio operativo como intérprete agnóstico en `main`. Pendiente: retirada de shims CLI (Ola C), documentación perfil laboratorio en `feature.md`, y acción `request-change-incorporation` para apertura formal de PR.
