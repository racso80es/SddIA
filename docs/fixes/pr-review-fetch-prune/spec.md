---
feature_name: pr-review-fetch-prune
created: "2026-05-22"
process: bug-fix
base: main
scope: pull-request-review-git-manager-fetch
version_spec: "1.0.0"
---

# Especificación — Fix fetch en aduana PR

## 1. Contexto

Durante el cierre de `ia-obrera-blindaje` (PR #16), `pull-request-review` abortó en **Preparación de rama**:

```
operation_payload_json for fetch must have exactly keys ['prune', 'remote'], got ['remote']
```

Causa: `capsule_pr_review_branch_prep` llamaba `invoke_git_manager(repo, "fetch", {"remote": "origin"})` sin `prune`. El patrón correcto ya existía en `run_workspace_init` (línea homóloga con `prune: True`).

## 2. Auditoría proceso `bug-fix` vs `feature`

| Aspecto | `feature` v1.2.0 | `bug-fix` v1.2.0 (antes) | Resolución |
|---------|------------------|--------------------------|------------|
| Fases documentales | objectives, clarify, spec, plan, impl, execution, validacion | spec, impl, execution, validacion (+ plan opcional) | ✅ Sin cambio genoma; subconjunto válido |
| `workspace-init` lab | `feature_name` obligatorio | **No disparaba** sin `feature_name` | ✅ Handler acepta `fix_name`, prefijo `fix/`, o `bug-fix` + `branch_name`/`persist_ref` |
| `persist_ref` default | `docs/features/{name}` | N/A (no init) | ✅ `docs/fixes/{name}` si `fix/` o `bug-fix` |
| `objectives.md` semilla | `refined_requirements` | N/A | ✅ También `bug_summary` |
| Perfil laboratorio en genoma | § en `feature.md` | Ausente | ✅ § añadido en `bug-fix.md` |
| Cierre | `delivery-close-cycle` + `source_process: feature` | `source_process: bug-fix` | ✅ Sin cambio |

## 3. Cambio técnico

### 3.1 Fix puntual (O1)

```python
# execute_process_capsules.py — capsule_pr_review_branch_prep
invoke_git_manager(repo, "fetch", {"remote": "origin", "prune": True})
```

### 3.2 Handler workspace-init (O3)

Nuevas funciones:

- `_workspace_task_name(inputs)` — resuelve slug desde `feature_name`, `fix_name` o `fix/*` / `feat/*`.
- `_workspace_process_label(inputs, branch_name)` — `bug-fix` si rama `fix/` o `source_process: bug-fix`.
- `is_workspace_init_phase(..., process_def)` — true para `bug-fix` con `branch_name` + `persist_ref`.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| CA-1 | Fetch incluye `prune` | grep + smoke Fase 1 |
| CA-2 | `pull-request-review` sin skip checkout | `execute-process.py` exit 0 en Fase 1 |
| CA-3 | `bug-fix` workspace-init | `execute-process.py --process bug-fix` fase 1 executed |
| CA-4 | `bug-fix.md` perfil lab | diff genoma |
| CA-5 | Regresión feature init | `feature_name` sigue funcionando |

## 5. Smoke tests

```powershell
# Fase 1 aduana (debe pasar fetch)
python SddIA/scripts/qa/execute-process.py --process pull-request-review --inputs-file tmp/smoke-pr-review-fetch.json

# Inicialización bug-fix
python SddIA/scripts/qa/execute-process.py --process bug-fix --inputs-file tmp/smoke-bug-fix-init.json
```
