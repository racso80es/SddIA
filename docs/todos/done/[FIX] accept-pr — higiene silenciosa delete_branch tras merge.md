---
document_id: FIX-ACCEPT-PR-SILENT-DELETE-BRANCH
title: "[FIX] accept-pr — higiene silenciosa delete_branch tras merge"
format: markdown
version: "1.1.0"
created: "2026-05-23"
closed_at: "2026-05-24"
status: cerrado
priority: media
process: bug-fix
incident_ref: "PR #36 — pull-request-automation-dlt (merge 5878452)"
feature_ref: docs/features/vanguardia-soberania-local
resolved_by: docs/features/vanguardia-soberania-local/validacion.md
related:
  - SddIA/process/accept-pr.md
  - SddIA/scripts/qa/execute_process_capsules.py
  - SddIA/norms/git-operations.md
  - docs/features/pull-request-automation-dlt/finalize-process.md
---

# [FIX] accept-pr — higiene silenciosa `delete_branch` tras merge

**Estado: CERRADO** — absorbido en feature `vanguardia-soberania-local` (L.1, `validacion.md` APTO).

## Síntoma

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

Payload inválido adicional: `"remote": "origin"` (string) vs contrato frozen `remote: boolean`.

## Resolución (2026-05-24)

| ID | Estado | Evidencia |
|----|--------|-----------|
| O1 Visibilidad | ✅ | Nodo `hygiene_failure.operations[].error` |
| O2 Trazabilidad | ✅ | `closed_branch` solo si local+remoto OK |
| O3 Smoke | ✅ | `_smoke-accept-pr-hygiene-fail.json` |
| O4 Runbook | ✅ | `accept-pr.md` § Fase 4 |

## Evidencia incidente

| Campo | Valor |
|-------|-------|
| PR | https://github.com/racso80es/SddIA/pull/36 |
| Merge commit | `58784523405a62189bbd80d061878589686f2cb0` |
| Rama | `feat/pull-request-automation-dlt` |
