---
document_id: FIX-ACCEPT-PR-SILENT-DELETE-BRANCH
title: "[FIX] accept-pr — higiene silenciosa delete_branch tras merge"
format: markdown
version: "1.1.0"
created: "2026-05-23"
updated: "2026-05-25"
status: cerrado
priority: media
process: bug-fix
incident_ref: "PR #36 — pull-request-automation-dlt (merge 5878452)"
feature_ref: docs/features/pull-request-automation-dlt
resolved_by:
  - docs/features/vanguardia-soberania-local
  - docs/features/l1-o5-runbooks-paridad
related:
  - SddIA/process/accept-pr.md
  - SddIA/scripts/qa/execute_process_capsules.py
  - SddIA/norms/git-operations.md
  - docs/features/pull-request-automation-dlt/finalize-process.md
  - docs/features/l1-o5-runbooks-paridad/runbook-accept-pr.md
---

# [FIX] accept-pr — higiene silenciosa `delete_branch` tras merge

## Resolución (2026-05-25)

| Objetivo | Estado | Evidencia |
|----------|--------|-----------|
| O1 Visibilidad | ✅ | `hygiene_failure` + `operations[]` en Fase 4 |
| O2 Trazabilidad | ✅ | `closed_branch` condicionado a delete local+remoto |
| O3 Smoke | ✅ | `vanguardia-soberania-local/_smoke-accept-pr-hygiene-fail.json` |
| O4 Runbook | ✅ | `accept-pr.md` § Fase 4 + `runbook-accept-pr.md` |

**Código:** PR #37 — `docs/features/vanguardia-soberania-local/validacion.md` APTO.  
**Documentación:** PR feature `l1-o5-runbooks-paridad` — runbook SSOT + cierre FIX.

---

## Síntoma (histórico)

Tras ejecutar `pull-request-review` → handoff `accept-pr` sobre **PR #36** (`feat/pull-request-automation-dlt`), la fase **Sincronización y Limpieza** completó push de `main` pero **no eliminó** la rama feature (local ni remoto). El operador tuvo que borrar manualmente.

## Causa raíz (código)

En `capsule_accept_sync_cleanup` (`execute_process_capsules.py`):

```python
try:
    invoke_git_manager(repo, "delete_branch", {...})
    closed = source.strip()
except RuntimeError:
    closed = None  # ← fallo silencioso; sin log ni error_trace
```

Cualquier fallo de `delete_branch` se traga; `closed_branch: null` en salida del proceso **sin señal auditable**.

## Evidencia

| Campo | Valor |
|-------|-------|
| PR | https://github.com/racso80es/SddIA/pull/36 |
| Merge commit | `58784523405a62189bbd80d061878589686f2cb0` |
| Rama | `feat/pull-request-automation-dlt` |
| Remediación manual | `git branch -d feat/pull-request-automation-dlt` (2026-05-23) |
| Remote | Ya ausente en `origin` (merge GitHub); ref stale en `git branch -a` hasta `fetch --prune` |
