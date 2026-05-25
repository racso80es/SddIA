---
feature_name: kaizen-higiene-ficheros-temporales
created: "2026-05-25"
process: feature
branch: feat/kaizen-higiene-ficheros-temporales
global: APTO
pbi_archived: true
checks:
  CA1-norma-tmp: pass
  CA2-e2e-cleanup: pass
  CA3-hook-tmp: pass
  CA4-helper: pass
  CA5-barrido: pass
  CA6-pbi-done: pass
git_changes:
  - .gitignore
  - SddIA/norms/git-operations.md
  - SddIA/library/norms/features-documentation-pattern.md
  - SddIA/scripts/qa/tmp_paths.py
  - SddIA/scripts/qa/lab_teardown.py
  - SddIA/scripts/qa/run-eda-e2e-lab.py
  - SddIA/scripts/qa/git-hooks/hook_common.py
  - docs/features/kaizen-higiene-ficheros-temporales/
  - docs/todos/done/[Kaizen] higiene ficheros temporales — .tmp fuera de control y limpieza post-uso.md
---

# Validación — Kaizen higiene ficheros temporales

**Veredicto global: APTO**

## CA1 — Norma `git-operations.md` v1.1.0

§3 taxonomía efímero/fixture; SSOT `.tmp/`; referencia a `tmp_paths.py`.

## CA2 — E2E lab sin ruido Core

```powershell
python SddIA/scripts/qa/run-eda-e2e-lab.py --entity-class tool --json
git status --short SddIA/tools/ .SddIA/tools/
```

Evidencia: `cleaned: true`, `cleanup.artifact_removed: true`; `git status` vacío en rutas genómicas. El exit code 1 por `sweep` vacío en modo async es preexistente; el objetivo Kaizen (higiene git) se cumple.

## CA3 — Hooks autolimpiantes

`hook_common.invoke_process` delega en `write_ephemeral_json` + `cleanup_path` en `finally`.

## CA4 — Helper `tmp_paths.py`

Módulo compartido con `SDDIA_KEEP_TMP` para depuración.

## CA5 — Barrido inicial

Eliminados 5× `eda-e2e-tool-*.md` Core y JSON one-shot sueltos (vanguardia, pull-request-automation-dlt).

## CA6 — Cierre documental

PBI archivado en `docs/todos/done/` en la misma rama; `pbi_archived: true`.
